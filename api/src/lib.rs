#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]
#![feature(let_chains)]
#![feature(exclusive_range_pattern)]

mod extras;
mod http;
mod ratelimit;

use thiserror::Error;

pub mod cache;
pub mod canvas;
pub mod game;
pub mod player;

pub use extras::*;
pub use translate::ApiError as Error;
