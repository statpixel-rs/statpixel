use thiserror::Error;

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
