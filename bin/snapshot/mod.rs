mod update;

use std::io::{Read, Write};

use api::player;
use chrono::{DateTime, Utc};
use database::schema::snapshot;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use translate::{Context, Error};

pub enum Status {
	Found((Box<player::data::Data>, DateTime<Utc>)),
	Inserted,
}

/// Gets the earliest snapshot of a given player within a timeframe.
pub fn get(
	ctx: Context<'_>,
	player: &player::Player,
	timeframe: DateTime<Utc>,
) -> Result<Option<(player::data::Data, DateTime<Utc>)>, Error> {
	let result = snapshot::table
		.filter(snapshot::columns::created_at.ge(timeframe))
		.filter(snapshot::columns::uuid.eq(player.uuid))
		.select((snapshot::columns::data, snapshot::columns::created_at))
		.order(snapshot::columns::created_at.asc())
		.first::<(Vec<u8>, DateTime<Utc>)>(&mut ctx.data().pool.get()?);

	match result {
		Ok((data, created_at)) => {
			let mut z = ZlibDecoder::new(&data[..]);
			let mut d = Vec::new();

			z.read_to_end(&mut d)?;

			Ok(Some((bson::from_slice(&d[..])?, created_at)))
		}
		Err(diesel::NotFound) => Ok(None),
		Err(e) => Err(e.into()),
	}
}

pub fn get_or_insert(
	ctx: Context<'_>,
	player: &player::Player,
	data: &player::data::Data,
	timeframe: DateTime<Utc>,
) -> Result<Status, Error> {
	// If a snapshot exists within the given timeframe, return it.
	if let Some(snapshot) = get(ctx, player, timeframe)? {
		return Ok(Status::Found((Box::new(snapshot.0), snapshot.1)));
	}

	let ser = bson::to_vec(data)?;
	let mut z = ZlibEncoder::new(Vec::new(), Compression::default());

	z.write_all(&ser[..])?;

	// Otherwise, insert the current data into the database.
	diesel::insert_into(snapshot::table)
		.values((
			snapshot::columns::uuid.eq(player.uuid),
			snapshot::columns::data.eq(z.finish()?),
		))
		.execute(&mut ctx.data().pool.get()?)?;

	// And return nothing.
	Ok(Status::Inserted)
}
