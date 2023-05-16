mod update;

use api::player::{data::Data, Player};
use chrono::{DateTime, Utc};
use database::schema::snapshot;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use translate::{Context, Error};

pub enum Status {
	Found((Box<Data>, DateTime<Utc>)),
	Inserted,
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
		Ok((data, created_at)) => {
			let mut decoder = ZlibDecoder::new(&data[..]);
			let data: Data =
				bincode::decode_from_std_read(&mut decoder, bincode::config::standard())?;

			Ok(Some((data, created_at)))
		}
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

	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

	bincode::encode_into_std_write(data, &mut encoder, bincode::config::standard())?;

	// Otherwise, insert the current data into the database.
	diesel::insert_into(snapshot::table)
		.values((
			snapshot::columns::uuid.eq(player.uuid),
			snapshot::columns::data.eq(encoder.finish()?),
		))
		.execute(&mut ctx.data().pool.get()?)?;

	// And return nothing.
	Ok(Status::Inserted)
}
