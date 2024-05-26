#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::get_first)]
#![feature(let_chains)]
#![feature(assert_matches)]
#![feature(associated_type_defaults)]

mod extras;

#[cfg(feature = "game")]
pub mod builder;
#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "canvas")]
pub mod canvas;
#[cfg(feature = "game")]
pub mod command;
pub(crate) mod de;
pub mod guild;
pub mod http;
#[cfg(feature = "game")]
pub mod id;
#[cfg(feature = "locale")]
pub mod leaderboard;
pub mod nbt;
pub mod player;
pub mod player_old;
pub mod ratelimiter;
#[cfg(feature = "redis")]
pub mod redis;
pub mod skyblock;
pub mod snapshot;

pub use extras::*;
pub use translate::ApiError as Error;

#[cfg(feature = "canvas")]
pub use canvas::shape;
pub use guild::member::Member;
pub use guild::Guild;
pub use guild::Rank;
pub use player::data::Data;
pub use player::status::Session;
pub use player::Player;

pub type Result<T> = std::result::Result<T, translate::Error>;
