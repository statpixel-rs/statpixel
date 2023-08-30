use std::sync::Arc;

use chrono::Utc;
#[cfg(feature = "database")]
use database::schema::{leaderboard, schedule};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use once_cell::sync::Lazy;
use redis::{aio::Connection, AsyncCommands, Client, RedisError};
use translate::context;

use crate::{
	player::{data::Data, Player},
	Error,
};

static REDIS: Lazy<Client> = Lazy::new(|| {
	let url = dotenvy_macro::dotenv!("REDIS_URL");

	Client::open(url).expect("failed to connect to Redis")
});

async fn get_connection() -> Result<Connection, RedisError> {
	REDIS.get_async_connection().await
}

impl Player {
	/// # Errors
	/// Returns an error if the player's data could not be fetched
	pub async fn get_display_string_owned(
		self,
		ctx: &context::Context<'_>,
	) -> Result<String, Arc<Error>> {
		self.get_display_string(ctx).await
	}

	/// # Errors
	/// Returns an error if the player's data could not be fetched
	pub async fn get_display_string(
		&self,
		ctx: &context::Context<'_>,
	) -> Result<String, Arc<Error>> {
		let mut conn = get_connection()
			.await
			.map_err(|e| Arc::new(Error::Redis(e)))?;

		if let Ok(display) = conn.get(self.uuid.as_bytes()).await {
			return Ok(display);
		}

		let data = self.get_data(ctx).await?;
		let display = if let Some(display) = data.get_rank().as_coloured_str() {
			format!("{} {}", display, data.username)
		} else {
			format!("§7{}", data.username)
		};

		conn.set(self.uuid.as_bytes(), &display)
			.await
			.map_err(|e| Arc::new(Error::Redis(e)))?;

		Ok(display)
	}

	/// # Errors
	/// Returns an error if the Redis cache is unavailable
	pub async fn set_display_str(&self, data: &Data) -> Result<(), Error> {
		let mut conn = get_connection().await?;

		let display = if let Some(display) = data.get_rank().as_coloured_str() {
			format!("{} {}", display, data.username)
		} else {
			format!("§7{}", data.username)
		};

		conn.set(self.uuid.as_bytes(), &display).await?;

		Ok(())
	}

	/// # Errors
	/// Returns an error if the player's data could not be saved to the leaderboard.
	#[cfg(feature = "database")]
	pub async fn update_leaderboard(
		&self,
		ctx: &context::Context<'_>,
		data: &Data,
	) -> Result<(), Error> {
		let value = serde_json::to_value(data)?;

		diesel::insert_into(leaderboard::table)
			.values((
				leaderboard::uuid.eq(&self.uuid),
				leaderboard::data.eq(&value),
			))
			.on_conflict(leaderboard::uuid)
			.do_update()
			.set(leaderboard::data.eq(&value))
			.execute(&mut ctx.data().pool.get().await?)
			.await?;

		Ok(())
	}

	/// # Errors
	/// Returns an error if the database has an error.
	pub async fn update_activity(&self, ctx: &context::Context<'_>) -> Result<(), Error> {
		let result = diesel::update(schedule::table.filter(schedule::uuid.eq(&self.uuid)))
			.set(schedule::active_at.eq(Utc::now()))
			.execute(&mut ctx.data().pool.get().await?)
			.await;

		if let Err(e) = result && e != diesel::result::Error::NotFound {
			return Err(e.into());
		}

		Ok(())
	}
}
