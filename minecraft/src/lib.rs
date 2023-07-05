#![warn(clippy::pedantic)]
#![feature(exclusive_range_pattern)]
#![feature(assert_matches)]
#![feature(const_precise_live_drops)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(const_option)]
// Temporarily suppress a few clippy warnings
// See: https://github.com/SoftbearStudios/bitcode/issues/7
// TODO: Remember to remove this once it has been fixed
#![allow(clippy::verbose_bit_mask)]
#![allow(unused_must_use)]

use thiserror::Error;

pub mod calc;
pub mod colour;
pub mod paint;
pub mod style;
pub mod text;
pub mod username;

pub use konst;

pub use colour::Colour;
pub use text::ESCAPE;

#[derive(Error, Debug)]
pub enum Error {
	#[error("The username `{0}` is invalid.")]
	InvalidUsername(String),
}
