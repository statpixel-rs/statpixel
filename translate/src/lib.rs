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

pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Error, Debug)]
pub enum ApiError {
	#[error("An internal error occurred while sending a request. {0:?}")]
	Reqwest(#[from] reqwest::Error),
	#[error("An internal error occurred while deserializing JSON.")]
	Json(#[from] serde_json::Error),
	#[error("Failed to parse UUID.")]
	Uuid(#[from] uuid::Error),
	#[error("A profile belonging to `{0}` was not found.")]
	PlayerNotFound(String),
	#[error("A session belonging to `{0}` was not found.")]
	SessionNotFound(String),
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
}

#[derive(Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Api(#[from] Arc<ApiError>),
	#[error("An error occurred while interacting with Diesel.\n\n```\n{0:?}```")]
	Diesel(#[from] diesel::result::Error),
	#[error("An error occurred while interacting with the database. \n\n```\n{0:?}```")]
	Database(#[from] diesel_async::pooled_connection::deadpool::PoolError),
	#[error("An internal error occurred. \n\n```\n{0:?}```")]
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
	#[error("An internal error occurred while decoding bincode.\n\n```\n{0:?}```")]
	BincodeDeserialize(#[from] bincode::error::DecodeError),
	#[error("An internal error occurred while encoding bincode.\n\n```\n{0:?}```")]
	BincodeSerialize(#[from] bincode::error::EncodeError),
	#[error("An internal error occurred while drawing a plot.")]
	Plotters,
	#[error("{0}")]
	Custom(&'static str),
}
