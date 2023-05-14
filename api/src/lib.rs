#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]
#![feature(let_chains)]
#![feature(exclusive_range_pattern)]

mod extras;

pub use macros;
use thiserror::Error;

pub mod cache;
pub mod canvas;
pub mod game;
pub mod http;
pub mod key;
pub mod player;
pub mod ratelimit;

pub use extras::*;
pub use translate::ApiError as Error;
