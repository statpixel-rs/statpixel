use std::ops::Mul;

use api::guild::{Guild, VERSION};
use chrono::{DateTime, Utc};
use database::{
	schema::{guild_schedule, guild_snapshot},
	PostgresPool,
};
use diesel::{ExpressionMethods, NullableExpressionMethods, QueryDsl};
use diesel_async::{
	scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use futures::StreamExt;
use tracing::warn;
use translate::{Context, Error};
use uuid::Uuid;

const GUILD_BATCH_LIMIT: i64 = 1_000;
const GUILD_BUFFER_LIMIT: usize = 20;

async fn update(
	connection: &mut AsyncPgConnection,
	uuid: Uuid,
	timestamp: DateTime<Utc>,
	hash: i64,
) -> Result<(), Error> {
	let guild = Guild::from_uuid(uuid).await?;

	let encoded = encode(&guild)?;
	// Converting the u64 to i64 is OK since we're always comparing
	// hashes in the same way.
	let new_hash = fxhash::hash64(&encoded) as i64;
	let did_update = new_hash != hash;

	let now = Utc::now();
	#[allow(clippy::cast_possible_truncation)]
	let days = (now.timestamp() / 60 / 60 / 24) as i32;

	let next = {
		let increase = chrono::Duration::hours(12);
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
	};

	// Make sure both the snapshot is inserted and the player is updated.
	connection
		.transaction::<(), Error, _>(|conn| {
			async move {
				diesel::update(guild_schedule::table)
					.filter(guild_schedule::uuid.eq(&uuid))
					.set((
						guild_schedule::update_at.eq(next),
						guild_schedule::snapshots.eq(guild_schedule::snapshots + 1),
						guild_schedule::prev_hash.eq(guild_schedule::hash.nullable()),
						guild_schedule::hash.eq(hash),
					))
					.execute(conn)
					.await?;

				diesel::insert_into(guild_snapshot::table)
					.values((
						guild_snapshot::uuid.eq(&uuid),
						guild_snapshot::data.eq(encoded),
						guild_snapshot::hash.eq(new_hash),
						guild_snapshot::did_update.eq(did_update),
						guild_snapshot::days_since_epoch.eq(days),
						guild_snapshot::version.eq(VERSION),
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

/// Begins the update loop for the snapshot system.
/// This function will return an Err if
pub async fn begin(pool: &PostgresPool) -> Result<(), Error> {
	loop {
		let mut connection = pool.get().await?;

		// We can afford fetching a lot of records since all of them update with the same
		// frequency, so it's impossible to insert one that would fit inside of these.
		let guilds = guild_schedule::table
			.select((
				guild_schedule::uuid,
				guild_schedule::update_at,
				guild_schedule::hash,
			))
			.order(guild_schedule::update_at.asc())
			.limit(GUILD_BATCH_LIMIT)
			.load_stream::<(Uuid, DateTime<Utc>, i64)>(&mut connection)
			.await?;

		guilds
			.filter_map(|row| async { row.ok() })
			.map(|(uuid, update_at, hash)| async move {
				// Wait until `update_at` to update the guild.
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

				// This will never fail since it's an md5 hash.
				while let Err(e) = update(&mut connection, uuid, update_at, hash).await {
					warn!("Failed to update guild {uuid}: {e}");

					tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
				}
			})
			.buffer_unordered(GUILD_BUFFER_LIMIT)
			.count()
			.await;
	}
}

pub enum Status {
	Found((Box<Guild>, DateTime<Utc>)),
	Inserted,
}

pub fn encode(guild: &Guild) -> Result<Vec<u8>, Error> {
	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

	bincode::encode_into_std_write(guild, &mut encoder, bincode::config::standard())?;

	Ok(encoder.finish()?)
}

pub fn decode(guild: &[u8]) -> Result<Guild, Error> {
	let mut decoder = ZlibDecoder::new(guild);

	Ok(bincode::decode_from_std_read(
		&mut decoder,
		bincode::config::standard(),
	)?)
}

/// Gets the earliest snapshot of a given player within a timeframe.
pub async fn get(
	ctx: Context<'_>,
	guild: &Guild,
	timeframe: DateTime<Utc>,
) -> Result<Option<(Guild, DateTime<Utc>)>, Error> {
	let result = guild_snapshot::table
		.filter(guild_snapshot::created_at.ge(timeframe))
		.filter(guild_snapshot::uuid.eq(Uuid::from_u128(guild.id)))
		.select((guild_snapshot::data, guild_snapshot::created_at))
		.order(guild_snapshot::created_at.asc())
		.first::<(Vec<u8>, DateTime<Utc>)>(&mut ctx.data().pool.get().await?)
		.await;

	match result {
		Ok((data, created_at)) => Ok(Some((decode(data.as_slice())?, created_at))),
		Err(diesel::NotFound) => Ok(None),
		Err(e) => Err(e.into()),
	}
}

pub async fn get_or_insert(
	ctx: Context<'_>,
	guild: &Guild,
	timeframe: DateTime<Utc>,
) -> Result<Status, Error> {
	// If a snapshot exists within the given timeframe, return it.
	if let Some(snapshot) = get(ctx, guild, timeframe).await? {
		return Ok(Status::Found((Box::new(snapshot.0), snapshot.1)));
	}

	insert(ctx, guild).await?;

	// And return nothing.
	Ok(Status::Inserted)
}

pub async fn insert(ctx: Context<'_>, guild: &Guild) -> Result<(), Error> {
	let encoded = encode(guild)?;
	let hash = fxhash::hash64(&encoded) as i64;
	let mut connection = ctx.data().pool.get().await?;

	let uuid = Uuid::from_u128(guild.id);

	let now = Utc::now();
	#[allow(clippy::cast_possible_truncation)]
	let days = (now.timestamp() / 60 / 60 / 24) as i32;

	connection
		.transaction::<(), Error, _>(|conn| {
			async move {
				// Otherwise, insert the current data into the database.
				let prev_hash = diesel::insert_into(guild_schedule::table)
					.values((
						guild_schedule::uuid.eq(&uuid),
						// Schedule the first update for one hour from now.
						// The first few updates should be more frequent to calculate the
						// timezone of the player.
						guild_schedule::update_at.eq(now + chrono::Duration::hours(12)),
						// Set the number of snapshots to 1, since we just inserted one.
						guild_schedule::snapshots.eq(1),
						guild_schedule::hash.eq(hash),
					))
					.on_conflict(guild_schedule::uuid)
					.do_update()
					.set((
						guild_schedule::snapshots.eq(guild_schedule::snapshots + 1),
						guild_schedule::prev_hash.eq(guild_schedule::hash.nullable()),
						guild_schedule::hash.eq(hash),
					))
					.returning(guild_schedule::prev_hash.nullable())
					.get_result::<Option<i64>>(conn)
					.await?;

				diesel::insert_into(guild_snapshot::table)
					.values((
						guild_snapshot::uuid.eq(&uuid),
						guild_snapshot::data.eq(encoded),
						guild_snapshot::hash.eq(hash),
						guild_snapshot::did_update.eq(match prev_hash {
							Some(previous) => previous != hash,
							None => true,
						}),
						guild_snapshot::days_since_epoch.eq(days),
						guild_snapshot::version.eq(VERSION),
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
