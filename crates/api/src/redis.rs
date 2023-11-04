use std::sync::Arc;

use chrono::Utc;
#[cfg(feature = "database")]
use database::schema::schedule;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use redis::AsyncCommands;
use translate::context;

use crate::{
	player::{data::Data, Player},
	Error,
};

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
		if let Ok(display) = ctx.data().redis().get(self.uuid.as_bytes()).await {
			return Ok(display);
		}

		let data = self.get_data(ctx).await?;
		let display = if let Some(display) = data.get_rank().as_coloured_str() {
			format!("{} {}", display, data.username)
		} else {
			format!("§7{}", data.username)
		};

		ctx.data()
			.redis()
			.set_ex(self.uuid.as_bytes(), &display, 60 * 60 * 24 * 7)
			.await
			.map_err(|e| Arc::new(Error::Redis(e)))?;

		Ok(display)
	}

	/// # Errors
	/// Returns an error if the Redis cache is unavailable
	pub async fn set_display_str(
		&self,
		ctx: &context::Context<'_>,
		data: &Data,
	) -> Result<(), Error> {
		let display = if let Some(display) = data.get_rank().as_coloured_str() {
			format!("{} {}", display, data.username)
		} else {
			format!("§7{}", data.username)
		};

		ctx.data()
			.redis()
			.set_ex(self.uuid.as_bytes(), &display, 60 * 60 * 24 * 7)
			.await?;

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
		let mut pipeline = redis::pipe();

		// Add all player data to the pipeline
		data.add_to_pipeline(&mut pipeline);

		// Push everything to their respective leaderboards
		pipeline.query_async(&mut ctx.data().redis()).await?;

		Ok(())
	}

	/// # Errors
	/// Returns an error if the database has an error.
	pub async fn update_activity(&self, ctx: &context::Context<'_>) -> Result<(), Error> {
		let result = diesel::update(schedule::table.filter(schedule::uuid.eq(&self.uuid)))
			.set(schedule::active_at.eq(Utc::now()))
			.execute(&mut ctx.connection().await?)
			.await;

		if let Err(e) = result
			&& e != diesel::result::Error::NotFound
		{
			return Err(e.into());
		}

		Ok(())
	}
}
