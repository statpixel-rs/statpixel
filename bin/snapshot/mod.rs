pub mod update;

use api::player::{data::Data, Player};
use chrono::{DateTime, Utc};
use database::schema::{schedule, snapshot};
use diesel::{Connection, ExpressionMethods, NullableExpressionMethods, QueryDsl, RunQueryDsl};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use translate::{Context, Error};

use self::update::DEFAULT_SCHEDULE;

pub enum Status {
	Found((Box<Data>, DateTime<Utc>)),
	Inserted,
}

pub fn encode(data: &Data) -> Result<Vec<u8>, Error> {
	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

	bincode::encode_into_std_write(data, &mut encoder, bincode::config::standard())?;

	Ok(encoder.finish()?)
}

pub fn decode(data: &[u8]) -> Result<Data, Error> {
	let mut decoder = ZlibDecoder::new(data);

	Ok(bincode::decode_from_std_read(
		&mut decoder,
		bincode::config::standard(),
	)?)
}

/// Gets the earliest snapshot of a given player within a timeframe.
pub fn get(
	ctx: Context<'_>,
	player: &Player,
	timeframe: DateTime<Utc>,
) -> Result<Option<(Data, DateTime<Utc>)>, Error> {
	let result = snapshot::table
		.filter(snapshot::columns::created_at.ge(timeframe))
		.filter(snapshot::columns::uuid.eq(player.uuid))
		.select((snapshot::columns::data, snapshot::columns::created_at))
		.order(snapshot::columns::created_at.asc())
		.first::<(Vec<u8>, DateTime<Utc>)>(&mut ctx.data().pool.get()?);

	match result {
		Ok((data, created_at)) => Ok(Some((decode(data.as_slice())?, created_at))),
		Err(diesel::NotFound) => Ok(None),
		Err(e) => Err(e.into()),
	}
}

pub fn get_or_insert(
	ctx: Context<'_>,
	player: &Player,
	data: &Data,
	timeframe: DateTime<Utc>,
) -> Result<Status, Error> {
	// If a snapshot exists within the given timeframe, return it.
	if let Some(snapshot) = get(ctx, player, timeframe)? {
		return Ok(Status::Found((Box::new(snapshot.0), snapshot.1)));
	}

	let encoded = encode(data)?;
	let hash = fxhash::hash64(&encoded) as i64;
	let mut connection = ctx.data().pool.get()?;

	connection.transaction::<(), Error, _>(|conn| {
		// Otherwise, insert the current data into the database.
		let prev_hash = diesel::insert_into(schedule::table)
			.values((
				schedule::columns::uuid.eq(player.uuid),
				// Schedule the first update for one hour from now.
				// The first few updates should be more frequent to calculate the
				// timezone of the player.
				schedule::columns::update_at.eq(Utc::now() + chrono::Duration::hours(3)),
				// Set the number of snapshots to 1, since we just inserted one.
				schedule::columns::snapshots.eq(1),
				schedule::columns::hash.eq(hash),
				schedule::columns::weekly_schedule.eq(DEFAULT_SCHEDULE),
			))
			.on_conflict(schedule::columns::uuid)
			.do_update()
			.set((
				schedule::columns::snapshots.eq(schedule::columns::snapshots + 1),
				schedule::columns::prev_hash.eq(schedule::columns::hash.nullable()),
				schedule::columns::hash.eq(hash),
			))
			.returning(schedule::columns::prev_hash.nullable())
			.get_result::<Option<i64>>(conn)?;

		diesel::insert_into(snapshot::table)
			.values((
				snapshot::columns::uuid.eq(player.uuid),
				snapshot::columns::data.eq(encoded),
				snapshot::columns::hash.eq(hash),
				snapshot::columns::did_update.eq(match prev_hash {
					Some(previous) => previous != hash,
					None => true,
				}),
			))
			.execute(conn)?;

		Ok(())
	})?;

	// And return nothing.
	Ok(Status::Inserted)
}
