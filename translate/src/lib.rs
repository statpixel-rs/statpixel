use std::sync::Arc;

use database::PostgresPool;
use thiserror::Error;

mod locale;
pub mod prelude;

pub use diesel;
pub use fluent;
pub use locale::*;
pub use r2d2;

#[derive(Debug)]
pub struct Data {
	pub pool: PostgresPool,
	pub locale: locale::Locale,
}

pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Api(#[from] Arc<api::Error>),
	#[error("An error occurred while interacting with Diesel.")]
	Diesel(#[from] diesel::result::Error),
	#[error("An error occurred while interacting with the database.")]
	Database(#[from] r2d2::Error),
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
}
