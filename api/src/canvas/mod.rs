pub mod body;
mod builder;
pub mod chart;
pub mod diff;
pub mod label;
pub mod shape;
pub mod text;
pub(crate) mod util;

pub use builder::*;

use skia_safe::{EncodedImageFormat, Surface};

/// # Panics
/// Panics if the canvas cannot be encoded to a png
pub fn to_png(surface: &mut Surface) -> Vec<u8> {
	surface
		.image_snapshot()
		.encode_to_data(EncodedImageFormat::PNG)
		.unwrap()
		.to_vec()
}
