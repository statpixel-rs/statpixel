#![feature(let_chains)]

mod http;
mod ratelimit;

use thiserror::Error;

pub mod cache;
pub mod canvas;
pub mod game;
pub mod player;

pub use translate::ApiError as Error;
