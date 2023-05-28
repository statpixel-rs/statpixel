#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]
#![feature(let_chains)]
#![feature(exclusive_range_pattern)]
#![feature(assert_matches)]

mod extras;

pub use macros;

pub mod cache;
pub mod canvas;
pub mod game;
pub mod guild;
pub mod http;
pub mod leaderboard;
pub mod player;
pub mod player_old;
pub mod ratelimiter;
pub mod snapshot;

pub use extras::*;
pub use translate::ApiError as Error;
