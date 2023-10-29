pub mod upgrade;

use std::{ops::Mul, sync::Arc};

use api::{
	canvas::diff::DiffLog,
	player::{data::Data, Player, VERSION},
	snapshot::user::{decode, encode},
};
use chrono::{DateTime, Datelike, Timelike, Utc};
use database::{
	models::TrackState,
	schema::{schedule, session, snapshot, track},
	PostgresPool,
};
use diesel::{BoolExpressionMethods, ExpressionMethods, NullableExpressionMethods, QueryDsl};
use diesel_async::{
	scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use futures::StreamExt;
use poise::serenity_prelude::{
	self as serenity, ChannelId, CreateEmbed, CreateEmbedAuthor, CreateMessage, Embed,
};
use tracing::{info, warn};
use translate::{
	context::{self, Context},
	Error,
};
use uuid::Uuid;

const HOURS_PER_DAY: i32 = 24;
const DAYS_PER_WEEK: i32 = 7;
// Number of days to take snapshots every day after the calculation period ends.
const REGULAR_PERIOD_SNAPSHOTS: i32 = 94;
const _REGULAR_PERIOD_TIME_STEP_HOURS: i32 = 24;
// Number of hours to wait between snapshots during calculation period.
// This number should not be changed, as it is depended on for the entire
// algorithm.
const CALCULATION_WEEK_TIME_STEP_HOURS: i32 = 3;
const CALCULATION_PERIOD_SNAPSHOTS: i32 =
	DAYS_PER_WEEK * HOURS_PER_DAY / CALCULATION_WEEK_TIME_STEP_HOURS;
// Number of days in a full period (calculation + regular period)
const FULL_PERIOD_SNAPSHOTS: i32 = CALCULATION_PERIOD_SNAPSHOTS + REGULAR_PERIOD_SNAPSHOTS;

// The number of players to update in a single batch.
const PLAYER_BATCH_LIMIT: i64 = 1_000;
// The number of players to update at the same time.
// This number only needs to be increased if there are a lot of
// players that need to be updated at the same time.
const PLAYER_BUFFER_LIMIT: usize = 20;

// Hour 0 (midnight) of every day of the week
#[allow(clippy::unusual_byte_groupings)]
pub const DEFAULT_SCHEDULE: i32 = 0b00000000000_000_000_000_000_000_000_000;

#[allow(clippy::too_many_lines)]
async fn update(
	ctx: &context::Context<'_>,
	connection: &mut AsyncPgConnection,
	uuid: Uuid,
	timestamp: DateTime<Utc>,
	snapshots: i32,
	hash: i64,
	weekly_schedule: i32,
) -> Result<Option<(Vec<ChannelId>, Data, Arc<Data>)>, Error> {
	let player = Player::from_uuid_unchecked(uuid);
	let data = player.get_data(ctx).await?;

	let encoded = encode(&data)?;
	// Converting the u64 to i64 is OK since we're always comparing
	// hashes in the same way.
	let new_hash = fxhash::hash64(&encoded) as i64;
	let did_update = new_hash != hash;

	let now = Utc::now();

	// Calculate the timezone of the player after the first week of snapshots.
	//
	// Then, use the "beginning" of each daily session as the time to update (once daily)
	// for 3 months. After that, go back to one update every 3 hours for a week and re-calculate
	// and begin the loop again.
	//
	// 56 is the number of snapshots taken every 3 hours for the first week.
	// If they have more than 56 snapshots, subtract 56 and see if it's
	// a multiple of 56 + 94 = 150.
	//
	// If it's within the first week after the three-month period, take
	// snapshots every 3 hours. Otherwise, take one per day based on their
	// weekly schedule.
	let next = if snapshots < CALCULATION_PERIOD_SNAPSHOTS
		|| (snapshots - CALCULATION_PERIOD_SNAPSHOTS) % FULL_PERIOD_SNAPSHOTS
			> REGULAR_PERIOD_SNAPSHOTS
	{
		let increase = chrono::Duration::hours(i64::from(CALCULATION_WEEK_TIME_STEP_HOURS));
		let next = timestamp + increase;

		if next > now {
			next
		} else {
			let mul = (now - timestamp).num_seconds() / increase.num_seconds() + 1;

			if mul < 0 {
				now + increase
			} else if let Ok(mul) = mul.try_into() {
				timestamp + increase.mul(mul)
			} else {
				now + increase
			}
		}
	} else {
		let time = timestamp + chrono::Duration::days(1);
		let weekday = time.weekday();
		let weekday = weekday.num_days_from_monday();

		let block = weekly_schedule >> (weekday * CALCULATION_WEEK_TIME_STEP_HOURS as u32)
			& ((1 << CALCULATION_WEEK_TIME_STEP_HOURS) - 1);
		#[allow(clippy::cast_sign_loss)]
		let hour = (block * CALCULATION_WEEK_TIME_STEP_HOURS) as u32;

		// We can safely unwrap this as the hour and minute is always within range
		time.with_hour(hour).unwrap().with_minute(0).unwrap()
	};

	let bitfield = if snapshots == CALCULATION_PERIOD_SNAPSHOTS
		|| (snapshots > CALCULATION_PERIOD_SNAPSHOTS
			&& (snapshots - CALCULATION_PERIOD_SNAPSHOTS) % FULL_PERIOD_SNAPSHOTS == 0)
	{
		let snapshots = snapshot::table
			.select((snapshot::created_at, snapshot::did_update))
			.filter(snapshot::uuid.eq(uuid))
			.order(snapshot::created_at.desc())
			.limit(i64::from(CALCULATION_PERIOD_SNAPSHOTS))
			.get_results::<(DateTime<Utc>, bool)>(connection)
			.await?;

		let time = Utc::now();
		let weekday = time.weekday();

		weekday.num_days_from_monday();
		time.hour();

		// we only need the last 7 blocks of 3:
		// xxxxxxxxxxx 000 000 000 000 000 000 000
		let mut bitfield = 0u32;

		for snapshot in snapshots {
			// If the data changed from the previous snapshot, use the 3-hour block
			// of this snapshot as the time that it should be updated on this week day.
			if snapshot.1 {
				let weekday = snapshot.0.weekday();
				let weekday = weekday.num_days_from_monday();
				//  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23
				//  0       1        2        3        4        5        6        7
				let three_hour_block = time.hour() / CALCULATION_WEEK_TIME_STEP_HOURS as u32;

				bitfield |= three_hour_block << (weekday * CALCULATION_WEEK_TIME_STEP_HOURS as u32);
			}
		}

		Some(bitfield)
	} else {
		None
	};

	// Make sure both the snapshot is inserted and the player is updated.
	let result = connection
		.transaction::<_, Error, _>(|conn| {
			async move {
				if let Some(bitfield) = bitfield {
					diesel::update(schedule::table.filter(schedule::uuid.eq(player.uuid)))
						.set((
							schedule::update_at.eq(next),
							schedule::snapshots.eq(schedule::snapshots + 1),
							schedule::prev_hash.eq(schedule::hash.nullable()),
							schedule::hash.eq(hash),
							schedule::weekly_schedule.eq(bitfield as i32),
						))
						.execute(conn)
						.await?;
				} else {
					diesel::update(schedule::table.filter(schedule::uuid.eq(player.uuid)))
						.set((
							schedule::update_at.eq(next),
							schedule::snapshots.eq(schedule::snapshots + 1),
							schedule::prev_hash.eq(schedule::hash.nullable()),
							schedule::hash.eq(hash),
						))
						.execute(conn)
						.await?;
				}

				let result = if did_update {
					let data: Vec<u8> = snapshot::table
						.filter(snapshot::uuid.eq(player.uuid))
						.order(snapshot::created_at.desc())
						.select(snapshot::data)
						.first(conn)
						.await?;

					let channels: Vec<i64> = track::table
						.filter(
							track::uuid
								.eq(player.uuid)
								.and(track::state.eq(i16::from(TrackState::Active))),
						)
						.distinct_on(track::channel_id)
						.select(track::channel_id)
						.get_results(conn)
						.await?;

					Some((channels, decode(&data)?))
				} else {
					None
				};

				diesel::insert_into(snapshot::table)
					.values((
						snapshot::uuid.eq(player.uuid),
						snapshot::data.eq(encoded),
						snapshot::hash.eq(new_hash),
						snapshot::did_update.eq(did_update),
						snapshot::version.eq(VERSION),
					))
					.execute(conn)
					.await?;

				Ok(result)
			}
			.scope_boxed()
		})
		.await?;

	#[allow(clippy::cast_sign_loss)]
	Ok(result.map(|(channels, old)| {
		(
			channels
				.into_iter()
				.map(|id| ChannelId::new(id as u64))
				.collect(),
			old,
			data,
		)
	}))
}

/// Begins the update loop for the snapshot system.
/// This function will return an Err if
#[allow(clippy::too_many_lines)]
pub async fn begin(
	pool: &PostgresPool,
	ctx: &context::Context<'_>,
	http: &serenity::Http,
) -> Result<(), Error> {
	loop {
		let mut connection = pool.get().await?;
		let now = Utc::now();

		// We can afford fetching a lot of records since all of them update with the same
		// frequency, so it's impossible to insert one that would fit inside of these.
		//
		// However, we can only fetch ones that update within 3 hours, since other profiles
		// could be added while this is active that might need to update in 3 hours.
		let players = schedule::table
			.filter(schedule::update_at.le(now + chrono::Duration::hours(3)))
			.filter(schedule::active_at.gt(now - chrono::Duration::weeks(1)))
			.select((
				schedule::uuid,
				schedule::update_at,
				schedule::snapshots,
				schedule::hash,
				schedule::weekly_schedule,
			))
			.order(schedule::update_at.asc())
			.limit(PLAYER_BATCH_LIMIT)
			.get_results::<(Uuid, DateTime<Utc>, i32, i64, i32)>(&mut connection)
			.await?;

		if players.is_empty() {
			// Sleep for an hour, since that's the earliest a profile could be added.
			tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 3)).await;

			continue;
		}

		info!(snapshots = players.len(), "updating player snapshots");

		futures::stream::iter(players)
			.map(
				|(uuid, update_at, snapshots, hash, weekly_schedule)| async move {
					// Wait until `update_at` to update the player.
					// If it's an Err, then the time has already passed so we don't need to wait.
					if let Ok(duration) = update_at.signed_duration_since(Utc::now()).to_std() {
						tokio::time::sleep(duration).await;
					}

					let mut connection = loop {
						match pool.get().await {
							Ok(connection) => break connection,
							Err(e) => {
								warn!("Failed to get connection: {}", e);

								tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
							}
						}
					};

					loop {
						let result = Box::pin(update(
							ctx,
							&mut connection,
							uuid,
							update_at,
							snapshots,
							hash,
							weekly_schedule,
						))
						.await;

						match result {
							Ok(Some((channels, data_lhs, data_rhs))) if !channels.is_empty() => {
								let embed =
									Data::diff_log(&data_lhs, &data_rhs, ctx, Embed::default());

								if !embed.fields.is_empty() {
									let player = Player::new(uuid, None);
									let message = CreateMessage::default().embed(
										<Embed as Into<CreateEmbed>>::into(embed)
											.author(
												CreateEmbedAuthor::new(&data_rhs.username)
													.icon_url(player.get_head_url()),
											)
											.colour(crate::EMBED_COLOUR),
									);

									for channel in channels {
										if let Err(e) =
											channel.send_message(http, message.clone()).await
										{
											match e {
												serenity::Error::Http(
													serenity::HttpError::UnsuccessfulRequest(
														serenity::ErrorResponse {
															status_code:
																s @ (serenity::StatusCode::NOT_FOUND
																| serenity::StatusCode::FORBIDDEN),
															..
														},
													),
												) => {
													diesel::update(track::table)
														.filter(track::uuid.eq(uuid))
														.filter(
															track::channel_id
																.eq(channel.get() as i64),
														)
														.set(track::state.eq(i16::from(
															if s == serenity::StatusCode::NOT_FOUND
															{
																TrackState::ChannelNotFound
															} else {
																TrackState::InsufficientPermission
															},
														)))
														.execute(&mut connection)
														.await
														.ok();
												}
												e => {
													warn!("Failed to send message: {}", e);
												}
											}
										}
									}
								}

								break;
							}
							Ok(..) => {
								break;
							}
							Err(e) => {
								warn!("Failed to update player {uuid}: {e}");

								tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
							}
						}
					}
				},
			)
			.buffer_unordered(PLAYER_BUFFER_LIMIT)
			.count()
			.await;
	}
}

pub enum Status {
	Found((Box<Data>, DateTime<Utc>)),
	Inserted,
}

impl Status {
	pub fn ok(self) -> Option<(Box<Data>, DateTime<Utc>)> {
		match self {
			Status::Found(data) => Some(data),
			Status::Inserted => None,
		}
	}
}

/// Gets the earliest snapshot of a given player within a timeframe.
pub async fn get(
	ctx: &Context<'_>,
	player: &Player,
	timeframe: DateTime<Utc>,
) -> Result<Option<(Data, DateTime<Utc>)>, Error> {
	// Get the oldest snapshot after the timeframe
	let first = snapshot::table
		.filter(snapshot::created_at.ge(timeframe))
		.filter(snapshot::uuid.eq(player.uuid))
		.select((snapshot::data, snapshot::created_at))
		.order(snapshot::created_at.asc())
		.first::<(Vec<u8>, DateTime<Utc>)>(&mut ctx.connection().await?)
		.await;

	// Get the newest snapshot before the timeframe
	let last = snapshot::table
		.filter(snapshot::created_at.le(timeframe))
		.filter(snapshot::uuid.eq(player.uuid))
		.select((snapshot::data, snapshot::created_at))
		.order(snapshot::created_at.desc())
		.first::<(Vec<u8>, DateTime<Utc>)>(&mut ctx.connection().await?)
		.await;

	// Use the one that is closest to the timeframe
	let result = match (first, last) {
		(Ok(first), Ok(last)) => {
			if (timeframe.timestamp_millis() - first.1.timestamp_millis()).abs()
				< (timeframe.timestamp_millis() - last.1.timestamp_millis()).abs()
			{
				Ok(first)
			} else {
				Ok(last)
			}
		}
		(Ok(first), Err(_)) => Ok(first),
		(Err(_), Ok(last)) => Ok(last),
		(Err(e), Err(_)) => Err(e),
	};

	match result {
		Ok((data, created_at)) => Ok(Some((decode(data.as_slice())?, created_at))),
		Err(diesel::NotFound) => Ok(None),
		Err(e) => Err(e.into()),
	}
}

/// Gets a session by its id
pub async fn get_snapshot_by_session_id(
	ctx: &Context<'_>,
	id: Uuid,
) -> Result<Option<(Data, DateTime<Utc>)>, Error> {
	// Get the oldest snapshot after the timeframe
	let snapshot_id = match session::table
		.filter(session::id.eq(id))
		.select(session::snapshot_id)
		.first::<i64>(&mut ctx.connection().await?)
		.await
	{
		Ok(id) => id,
		Err(diesel::NotFound) => return Ok(None),
		Err(e) => return Err(e.into()),
	};

	let snapshot = snapshot::table
		.filter(snapshot::id.eq(snapshot_id))
		.select((snapshot::data, snapshot::created_at))
		.first::<(Vec<u8>, DateTime<Utc>)>(&mut ctx.connection().await?)
		.await;

	match snapshot {
		Ok((data, created_at)) => Ok(Some((decode(data.as_slice())?, created_at))),
		Err(diesel::NotFound) => Ok(None),
		Err(e) => Err(e.into()),
	}
}

pub async fn get_or_insert(
	ctx: &Context<'_>,
	player: &Player,
	data: &Data,
	timeframe: DateTime<Utc>,
) -> Result<Status, Error> {
	// If a snapshot exists within the given timeframe, return it.
	if let Some(snapshot) = get(ctx, player, timeframe).await? {
		return Ok(Status::Found((Box::new(snapshot.0), snapshot.1)));
	}

	insert(ctx, player, data).await?;

	// And return nothing.
	Ok(Status::Inserted)
}

pub async fn insert(ctx: &Context<'_>, player: &Player, data: &Data) -> Result<(), Error> {
	let encoded = encode(data)?;
	let hash = fxhash::hash64(&encoded) as i64;
	let mut connection = ctx.connection().await?;

	connection
		.transaction::<(), Error, _>(|conn| {
			async move {
				// Otherwise, insert the current data into the database.
				let prev_hash = diesel::insert_into(schedule::table)
					.values((
						schedule::uuid.eq(player.uuid),
						// Schedule the first update for one hour from now.
						// The first few updates should be more frequent to calculate the
						// timezone of the player.
						schedule::update_at.eq(Utc::now() + chrono::Duration::hours(3)),
						// Set the number of snapshots to 1, since we just inserted one.
						schedule::snapshots.eq(1),
						schedule::hash.eq(hash),
						schedule::weekly_schedule.eq(DEFAULT_SCHEDULE),
					))
					.on_conflict(schedule::uuid)
					.do_update()
					.set((
						schedule::snapshots.eq(schedule::snapshots + 1),
						schedule::prev_hash.eq(schedule::hash.nullable()),
						schedule::hash.eq(hash),
					))
					.returning(schedule::prev_hash.nullable())
					.get_result::<Option<i64>>(conn)
					.await?;

				diesel::insert_into(snapshot::table)
					.values((
						snapshot::uuid.eq(player.uuid),
						snapshot::data.eq(encoded),
						snapshot::hash.eq(hash),
						snapshot::did_update.eq(match prev_hash {
							Some(previous) => previous != hash,
							None => true,
						}),
						snapshot::version.eq(VERSION),
					))
					.execute(conn)
					.await?;

				Ok(())
			}
			.scope_boxed()
		})
		.await?;

	Ok(())
}

pub async fn insert_with_session(
	ctx: &Context<'_>,
	player: &Player,
	data: &Data,
	name: Option<&str>,
) -> Result<Uuid, Error> {
	let encoded = encode(data)?;
	let hash = fxhash::hash64(&encoded) as i64;
	let mut connection = ctx.connection().await?;

	connection
		.transaction::<_, Error, _>(|conn| {
			async move {
				// Otherwise, insert the current data into the database.
				let prev_hash = diesel::insert_into(schedule::table)
					.values((
						schedule::uuid.eq(player.uuid),
						// Schedule the first update for one hour from now.
						// The first few updates should be more frequent to calculate the
						// timezone of the player.
						schedule::update_at.eq(Utc::now() + chrono::Duration::hours(3)),
						// Set the number of snapshots to 1, since we just inserted one.
						schedule::snapshots.eq(1),
						schedule::hash.eq(hash),
						schedule::weekly_schedule.eq(DEFAULT_SCHEDULE),
					))
					.on_conflict(schedule::uuid)
					.do_update()
					.set((
						schedule::prev_hash.eq(schedule::hash.nullable()),
						schedule::hash.eq(hash),
					))
					.returning(schedule::prev_hash.nullable())
					.get_result::<Option<i64>>(conn)
					.await?;

				let id = diesel::insert_into(snapshot::table)
					.values((
						snapshot::uuid.eq(player.uuid),
						snapshot::data.eq(encoded),
						snapshot::hash.eq(hash),
						snapshot::did_update.eq(match prev_hash {
							Some(previous) => previous != hash,
							None => true,
						}),
						snapshot::version.eq(VERSION),
					))
					.returning(snapshot::id)
					.get_result::<i64>(conn)
					.await?;

				let id = diesel::insert_into(session::table)
					.values((
						session::snapshot_id.eq(id),
						session::user_id.eq(ctx.author().unwrap().id.get() as i64),
						session::kind.eq(0),
						session::uuid.eq(player.uuid),
						session::name.eq(name),
					))
					.returning(session::id)
					.get_result::<Uuid>(conn)
					.await?;

				Ok(id)
			}
			.scope_boxed()
		})
		.await
}
