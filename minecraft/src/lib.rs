#![warn(clippy::pedantic)]
#![feature(exclusive_range_pattern)]
#![feature(assert_matches)]

use thiserror::Error;

pub mod calc;
pub mod colour;
pub mod paint;
pub mod style;
pub mod text;
pub mod username;

#[derive(Error, Debug)]
pub enum Error {
	#[error("The username `{0}` is invalid.")]
	InvalidUsername(String),
}
