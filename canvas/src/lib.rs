mod data;

pub use data::*;

use minecraft::paint;
use skia_safe::{EncodedImageFormat, Path, Point, RRect, Rect, Surface};

pub const WIDTH: i32 = 750;

pub const WIDTH_F: f32 = WIDTH as f32;
pub const GAP: f32 = 7.;
pub const PADDING: f32 = 15.;

pub const ITEMS_PER_ROW: f32 = 3.;
pub const ITEM_WIDTH: f32 = (WIDTH_F - PADDING * 2. - GAP * 2.) / ITEMS_PER_ROW;
pub const ITEM_HEIGHT: f32 = 85.;

pub const HEADER_HEIGHT: f32 = 175.;
pub const HEADER_LEFT_END_X: f32 = PADDING + ITEM_WIDTH * 1.5 - GAP / 2.;
pub const HEADER_MIDDLE_END_X: f32 = HEADER_LEFT_END_X + ITEM_WIDTH + GAP;

pub const HEADER_NAME_HEIGHT: f32 = 60.;

pub fn create_surface(rows: u8) -> Surface {
	let height = (PADDING * 2. + HEADER_HEIGHT + (GAP + ITEM_HEIGHT) * rows as f32) as i32;
	let mut surface = Surface::new_raster_n32_premul((WIDTH, height)).unwrap();

	let mut path = Path::new();
	let mut rect = RRect::new();

	// Header, left block top
	rect.set_rect_radii(
		Rect::new(
			PADDING,
			PADDING,
			HEADER_LEFT_END_X,
			HEADER_NAME_HEIGHT + PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	// Header, left block bottom
	rect.set_rect_radii(
		Rect::new(
			PADDING,
			PADDING + HEADER_NAME_HEIGHT + GAP,
			HEADER_LEFT_END_X,
			HEADER_HEIGHT + PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	// Header, middle block
	rect.set_rect_radii(
		Rect::new(
			HEADER_LEFT_END_X + GAP,
			PADDING,
			HEADER_MIDDLE_END_X,
			HEADER_HEIGHT + PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	// Header, right block
	rect.set_rect_radii(
		Rect::new(
			HEADER_MIDDLE_END_X + GAP,
			PADDING,
			WIDTH_F - PADDING,
			HEADER_HEIGHT + PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	let mut y = PADDING + HEADER_HEIGHT + GAP;

	for _ in 0..rows {
		let mut x = PADDING;

		for _ in 0..3 {
			rect.set_rect_radii(
				Rect::new(x, y, x + ITEM_WIDTH, y + ITEM_HEIGHT),
				&[
					Point::new(20., 20.),
					Point::new(20., 20.),
					Point::new(20., 20.),
					Point::new(20., 20.),
				],
			);

			path.add_rrect(rect, None);

			x += ITEM_WIDTH + GAP;
		}

		y += ITEM_HEIGHT + GAP;
	}

	surface.canvas().draw_path(&path, &paint::BACKGROUND);

	surface
}

pub fn to_png(surface: &mut Surface) -> Vec<u8> {
	surface
		.image_snapshot()
		.encode_to_data(EncodedImageFormat::PNG)
		.unwrap()
		.to_vec()
}

pub fn get_item_center(i: u16) -> (f32, f32) {
	let x = i % 3;
	let y = i / 3;

	let x = PADDING + (GAP + ITEM_WIDTH) * x as f32 + ITEM_WIDTH / 2.;
	let y = PADDING + HEADER_HEIGHT + GAP + (ITEM_HEIGHT + GAP) * y as f32 + ITEM_HEIGHT / 2.;

	(x, y)
}
