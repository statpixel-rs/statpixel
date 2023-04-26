#![feature(async_closure)]

use thiserror::Error;

mod http;
pub mod player;

#[derive(Error, Debug)]
pub enum Error {
	#[error("reqwest error")]
	Middleware(#[from] reqwest_middleware::Error),
	#[error("tower error")]
	Tower,
	#[error("reqwest error")]
	Reqwest(#[from] reqwest::Error),
	#[error("json error")]
	Json(#[from] serde_json::Error),
	#[error("uuid error")]
	Uuid(#[from] uuid::Error),
	#[error("data not found")]
	NotFound,
}
