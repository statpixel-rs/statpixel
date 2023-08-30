pub mod body;
mod builder;
pub mod chart;
pub mod diff;
#[cfg(feature = "game")]
pub mod prelude;
pub mod project;
pub mod shape;
pub mod text;
pub mod util;

pub use builder::*;
pub use label;

use skia_safe::{EncodedImageFormat, Surface};

#[cfg(target_os = "linux")]
const IMAGE_FORMAT: EncodedImageFormat = EncodedImageFormat::PNG;
#[cfg(not(target_os = "linux"))]
const IMAGE_FORMAT: EncodedImageFormat = EncodedImageFormat::PNG;

/// # Panics
/// Panics if the canvas cannot be encoded to a png
pub fn to_png(surface: &mut Surface) -> Vec<u8> {
	surface
		.image_snapshot()
		.encode(None, IMAGE_FORMAT, 100)
		.unwrap()
		.to_vec()
}
