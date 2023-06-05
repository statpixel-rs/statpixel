use std::fmt;
use std::sync::Arc;

use database::PostgresPool;
use thiserror::Error;
use uuid::Uuid;

mod locale;
pub mod prelude;

pub use fluent;
pub use locale::*;
pub use uuid;

pub struct Data {
	pub pool: PostgresPool,
	pub locale: locale::Locale,
}

impl fmt::Debug for Data {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Data").finish()
	}
}

pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Error, Debug)]
pub enum ApiError {
	#[error("An internal error occurred while sending a request.")]
	Http,
	#[error("An internal error occurred while sending a request.")]
	Reqwest(#[from] reqwest::Error),
	#[error("Failed to parse UUID.")]
	Uuid(#[from] uuid::Error),
	#[error("A profile belonging to `{0}` was not found.")]
	PlayerNotFound(String),
	#[error("A session belonging to `{0}` was not found.")]
	SessionNotFound(String),
	#[error("The `{0}` SkyBlock profile belonging to `{1}` has its API disabled.")]
	ProfileNotFound(String, String),
	#[error("The uuid `{0}` was not found.")]
	UuidNotFound(Uuid),
	#[error("The username `{0}` was not found.")]
	UsernameNotFound(String),
	#[error("The player `{0}` does not belong to a guild.")]
	GuildByMemberUuidNotFound(Uuid),
	#[error("The player `{0}` does not belong to a guild.")]
	GuildByMemberUsernameNotFound(String),
	#[error("The guild `{0}` was not found.")]
	GuildNotFound(String),
	#[error("An internal error occurred while interacting with Redis.")]
	Redis(#[from] redis::RedisError),
}

#[derive(Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Api(#[from] Arc<ApiError>),
	#[error("An error occurred while interacting with Diesel.")]
	Diesel(#[from] diesel::result::Error),
	#[error("An error occurred while interacting with the database.")]
	Database(#[from] diesel_async::pooled_connection::deadpool::PoolError),
	#[error("An internal error occurred.")]
	Framework(#[from] poise::serenity_prelude::Error),
	#[error("An internal error occurred during setup.")]
	Setup,
	#[error("You are not linked to a Minecraft account. Use `/link` to link your account.")]
	NotLinked,
	#[error("An error occurred while negotiating game mode.")]
	GameMode,
	#[error("The uuid `{0}` is invalid.")]
	InvalidUuid(String),
	#[error("The username `{0}` is invalid.")]
	InvalidUsername(String),
	#[error("An error occurred while handling io.")]
	Io(#[from] std::io::Error),
	#[error("An internal error occurred while decoding bincode.")]
	BincodeDeserialize(#[from] bincode::error::DecodeError),
	#[error("An internal error occurred while encoding bincode.")]
	BincodeSerialize(#[from] bincode::error::EncodeError),
	#[error("An internal error occurred while drawing a plot.")]
	Plotters,
	#[error("`{0}` is not a member of the provided profile.")]
	MemberPlayerNotFound(String),
	#[error("A SkyBlock profile belonging to `{0}` was not found.")]
	SkyBlockProfileNotFound(String),
	#[error("No snapshots were found for `{0}`.")]
	PlayerSnapshotNotFound(String),
	#[error("A leaderboard with the name `{0}` was not found.")]
	LeaderboardNotFound(String),
	#[error("An internal error occurred while interacting with the canvas.")]
	Canvas,
}
