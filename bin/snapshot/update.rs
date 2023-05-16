use std::ops::Mul;

use super::encode;
use api::player::Player;
use chrono::{DateTime, Utc};
use database::{
	extend,
	schema::{schedule, snapshot},
	PostgresPool,
};
use diesel::{
	BoolExpressionMethods, Connection, ExpressionMethods, JoinOnDsl, NullableExpressionMethods,
	PgConnection, QueryDsl, RunQueryDsl,
};
use futures::StreamExt;
use tracing::{info, warn};
use translate::Error;
use uuid::Uuid;

async fn update(
	connection: &mut PgConnection,
	uuid: Uuid,
	timestamp: DateTime<Utc>,
	snapshots: i32,
	hash: &str,
) -> Result<(), Error> {
	let player = Player::from_uuid_unchecked(uuid);
	let data = player.get_data().await?;

	let encoded = encode(&data)?;
	let did_update = format!("{:x}", md5::compute(encoded.as_slice())) != hash;

	let increase = match snapshots {
		..0 => unreachable!("snapshots cannot be less than 0"),
		// For the first few snapshots, get frequent updates until we know
		// (within 1 hour) when the player is online.
		0..24 if !did_update => chrono::Duration::hours(1),
		// If it has been too long or the data updated, then we can
		// update it once per day at the same time.
		_ => chrono::Duration::days(1),
	};

	let now = Utc::now();
	let next = {
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
	connection.transaction::<(), Error, _>(|conn| {
		diesel::update(schedule::table)
			.filter(schedule::columns::uuid.eq(player.uuid))
			.set((
				schedule::columns::update_at.eq(next),
				schedule::columns::snapshots.eq(schedule::columns::snapshots + 1),
			))
			.execute(conn)?;

		diesel::insert_into(snapshot::table)
			.values((
				snapshot::columns::uuid.eq(player.uuid),
				snapshot::columns::data.eq(encoded),
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
			.inner_join(
				snapshot::table.on(snapshot::columns::uuid.eq(schedule::columns::uuid).and(
					snapshot::columns::id
						.nullable()
						.eq(diesel::dsl::max(snapshot::columns::id)),
				)),
			)
			.select((
				schedule::columns::uuid,
				schedule::columns::update_at,
				schedule::columns::snapshots,
				extend::md5(snapshot::columns::data),
			))
			.order(schedule::columns::update_at.asc())
			.limit(1_000)
			.get_results::<(Uuid, DateTime<Utc>, i32, String)>(&mut connection)?;

		if players.is_empty() {
			info!("No players to update, sleeping for 10 seconds");

			tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

			continue;
		}

		futures::stream::iter(players)
			.map(|(uuid, update_at, snapshots, hash)| async move {
				// Wait until `update_at` to update the player.
				// If it's an Err, then the time has already passed so we don't need to wait.
				if let Ok(duration) = update_at.signed_duration_since(Utc::now()).to_std() {
					info!(
						"Waiting for {} seconds to update player {}",
						duration.as_secs_f64(),
						&uuid
					);

					tokio::time::sleep(duration).await;
				}

				info!("Updating player {}", &uuid);

				let mut connection = loop {
					match pool.get() {
						Ok(connection) => break connection,
						Err(e) => {
							warn!("Failed to get connection: {}", e);

							tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
						}
					}
				};

				while let Err(e) = update(&mut connection, uuid, update_at, snapshots, &hash).await
				{
					warn!("Failed to update player {uuid}: {e}");

					tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
				}
			})
			.buffer_unordered(20)
			.count()
			.await;
	}
}
