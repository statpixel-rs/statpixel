use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ApiError {
	#[error("An internal error occurred while sending a request.")]
	Http(#[from] HttpError),
	#[error("An internal error occurred while sending a request.")]
	Reqwest(#[from] reqwest::Error),
	#[error("Failed to parse UUID.")]
	Uuid(#[from] uuid::Error),
	#[error("A profile belonging to `{0}` was not found.")]
	PlayerNotFound(String),
	#[error("A session belonging to `{0}` was not found.")]
	SnapshotNotFound(String),
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
	#[error("An error occurred while interacting with Diesel: {0:?}")]
	Diesel(#[from] diesel::result::Error),
	#[error("An error occurred while interacting with the database.")]
	Database(#[from] diesel_async::pooled_connection::deadpool::PoolError),
	#[error("An error occurred with serde_json. {0:?}")]
	SerdeJson(#[from] serde_json::Error),
	#[error("An internal error occurred while decoding bincode.")]
	BincodeDeserialize(#[from] bincode::error::DecodeError),
	#[error("An internal error occurred while encoding bincode.")]
	BincodeSerialize(#[from] bincode::error::EncodeError),
	#[error("An error occurred while handling io.")]
	Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Api(#[from] Arc<ApiError>),
	#[error(transparent)]
	ApiRaw(#[from] ApiError),
	#[error("An error occurred while interacting with Diesel: {0:?}")]
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
	#[error("A leaderboard with the name `{0}` was not found.")]
	LeaderboardNotFound(usize),
	#[error("An internal error occurred while interacting with the canvas.")]
	Canvas,
	#[error("An internal error occurred while decoding base64.")]
	Base64(#[from] base64::DecodeError),
	#[error("The generated identifier is too long.")]
	IdentifierTooLong,
	#[error("A track for this channel and uuid already exists.")]
	TrackAlreadyExists,
	#[error("The number of tracks for this user has been reached, at {0}.")]
	UserTrackLimitReached(i16),
	#[error("The tracks for this guild has been reached, at {0}.")]
	GuildTrackLimitReached(i64),
	#[error("The boosts for this user has been reached, at {0}.")]
	BoostLimitReached(i16),
	#[error("You are not a premium user.")]
	NotPremium,
	#[error("You are not in a guild.")]
	NotInAGuild,
	#[error("You have already boosted this guild.")]
	BoostAlreadyExists,
	#[error("The provided time is invalid.")]
	TimeParse(#[from] humantime::DurationError),
	#[error("This branch is not implemented yet.")]
	NotImplemented,
	#[error("The provided session was not found.")]
	SessionNotFound,
	#[error("The provided session name already exists.")]
	SessionAlreadyExists,
	#[error("The provided player was not found in our leaderboards.")]
	LeaderboardPlayerNotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
	#[error("invalid utf8 when parsing header")]
	InvalidHeaderUtf8,
	#[error("invalid format when parsing header")]
	InvalidHeaderFormat,
	#[error("http error")]
	Http(#[from] reqwest::Error),
}

impl Error {
	pub fn api(&self) -> Option<&ApiError> {
		match self {
			Error::Api(e) => Some(e),
			Error::ApiRaw(e) => Some(e),
			_ => None,
		}
	}
}
