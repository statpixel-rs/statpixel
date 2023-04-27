#![feature(once_cell)]

mod http;
mod ratelimit;

use thiserror::Error;
use uuid::Uuid;

pub mod cache;
pub mod game;
pub mod player;

#[derive(Error, Debug)]
pub enum Error {
	#[error("An internal error occurred when sending a request.")]
	Reqwest(#[from] reqwest::Error),
	#[error("An internal error occurred when deserializing JSON.")]
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
}
