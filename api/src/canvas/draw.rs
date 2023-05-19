use minecraft::paint;
use skia_safe::{Point, RRect, Rect, Surface};

pub fn rect(surface: &mut Surface, top_left: Point, width: f32, height: f32) {
	let mut rect = RRect::new_empty();

	rect.set_rect_radii(
		Rect::new(
			top_left.x,
			top_left.y,
			top_left.x + width,
			top_left.y + height,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	surface.canvas().draw_rrect(rect, &paint::BACKGROUND);
}
