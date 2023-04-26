pub mod colour;
pub mod data;
pub mod font;
pub mod paint;
pub mod text;

use skia_safe::{EncodedImageFormat, Surface};

pub fn create_surface() -> Surface {
	let mut surface = Surface::new_raster_n32_premul((750, 500)).unwrap();

	surface.canvas().clear(colour::BACKGROUND);

	surface
}

pub fn to_png(surface: &mut Surface) -> Vec<u8> {
	surface
		.image_snapshot()
		.encode_to_data(EncodedImageFormat::PNG)
		.unwrap()
		.to_vec()
}
