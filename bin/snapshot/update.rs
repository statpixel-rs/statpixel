use std::ops::Mul;

use super::encode;
use api::player::Player;
use chrono::{DateTime, Datelike, Timelike, Utc};
use database::{
	schema::{schedule, snapshot},
	PostgresPool,
};
use diesel::{
	Connection, ExpressionMethods, NullableExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use futures::StreamExt;
use tracing::warn;
use translate::Error;
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

async fn update(
	connection: &mut PgConnection,
	uuid: Uuid,
	timestamp: DateTime<Utc>,
	snapshots: i32,
	hash: i64,
	weekly_schedule: i32,
) -> Result<(), Error> {
	let player = Player::from_uuid_unchecked(uuid);
	let data = player.get_data().await?;

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

		let block = weekly_schedule >> (weekday * 3) & 0b111;
		#[allow(clippy::cast_sign_loss)]
		let hour = block as u32 * 3;

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
			.get_results::<(DateTime<Utc>, bool)>(connection)?;

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
				let three_hour_block = time.hour() / 3;

				bitfield |= three_hour_block << (weekday * 3);
			}
		}

		Some(bitfield)
	} else {
		None
	};

	// Make sure both the snapshot is inserted and the player is updated.
	connection.transaction::<(), Error, _>(|conn| {
		if let Some(bitfield) = bitfield {
			diesel::update(schedule::table)
				.filter(schedule::columns::uuid.eq(player.uuid))
				.set((
					schedule::update_at.eq(next),
					schedule::snapshots.eq(schedule::snapshots + 1),
					schedule::prev_hash.eq(schedule::hash.nullable()),
					schedule::hash.eq(hash),
					schedule::weekly_schedule.eq(bitfield as i32),
				))
				.execute(conn)?;
		} else {
			diesel::update(schedule::table)
				.filter(schedule::columns::uuid.eq(player.uuid))
				.set((
					schedule::update_at.eq(next),
					schedule::snapshots.eq(schedule::snapshots + 1),
					schedule::prev_hash.eq(schedule::hash.nullable()),
					schedule::hash.eq(hash),
				))
				.execute(conn)?;
		}

		diesel::insert_into(snapshot::table)
			.values((
				snapshot::uuid.eq(player.uuid),
				snapshot::data.eq(encoded),
				snapshot::hash.eq(new_hash),
				snapshot::did_update.eq(did_update),
			))
			.execute(conn)?;

		Ok(())
	})?;

	Ok(())
}

/// Begins the update loop for the snapshot system.
/// This function will return an Err if
pub async fn begin(pool: &PostgresPool) -> Result<(), Error> {
	loop {
		let mut connection = pool.get()?;

		// We can afford fetching a lot of records since all of them update with the same
		// frequency, so it's impossible to insert one that would fit inside of these.
		let players = schedule::table
			.select((
				schedule::uuid,
				schedule::update_at,
				schedule::snapshots,
				schedule::hash,
				schedule::weekly_schedule,
			))
			.order(schedule::update_at.asc())
			.limit(PLAYER_BATCH_LIMIT)
			.get_results::<(Uuid, DateTime<Utc>, i32, i64, i32)>(&mut connection)?;

		if players.is_empty() {
			// Sleep for an hour, since that's the earliest a profile could be added.
			tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;

			continue;
		}

		futures::stream::iter(players)
			.map(
				|(uuid, update_at, snapshots, hash, weekly_schedule)| async move {
					// Wait until `update_at` to update the player.
					// If it's an Err, then the time has already passed so we don't need to wait.
					if let Ok(duration) = update_at.signed_duration_since(Utc::now()).to_std() {
						tokio::time::sleep(duration).await;
					}

					let mut connection = loop {
						match pool.get() {
							Ok(connection) => break connection,
							Err(e) => {
								warn!("Failed to get connection: {}", e);

								tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
							}
						}
					};

					// This will never fail since it's an md5 hash.
					while let Err(e) = update(
						&mut connection,
						uuid,
						update_at,
						snapshots,
						hash,
						weekly_schedule,
					)
					.await
					{
						warn!("Failed to update player {uuid}: {e}");

						tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
					}
				},
			)
			.buffer_unordered(PLAYER_BUFFER_LIMIT)
			.count()
			.await;
	}
}
