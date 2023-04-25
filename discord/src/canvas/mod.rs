mod colour;
mod font;
mod paint;

use skia_bindings::SkSurface;
use skia_safe::surface::Surface;

pub fn create_surface() -> skia_safe::RCHandle<SkSurface> {
	let mut surface = Surface::new_raster_n32_premul((750, 500)).unwrap();

	surface.canvas().clear(colour::BACKGROUND);

	surface
}
