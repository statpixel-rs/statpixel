#![warn(clippy::pedantic)]
#![feature(exclusive_range_pattern)]
#![feature(assert_matches)]
#![feature(const_precise_live_drops)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(const_option)]

use thiserror::Error;

pub mod calc;
pub mod colour;
pub mod paint;
pub mod style;
pub mod text;
pub mod username;

pub use konst;

#[derive(Error, Debug)]
pub enum Error {
	#[error("The username `{0}` is invalid.")]
	InvalidUsername(String),
}
