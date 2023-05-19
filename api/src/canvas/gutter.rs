use std::borrow::Cow;

use super::{GAP, HEADER_MIDDLE_END_X, ITEM_WIDTH, PADDING};

use minecraft::{
	paint::{self, Paint},
	text::{draw, Text},
};
use skia_safe::{textlayout::TextAlign, Rect, Surface};

pub type Line<'c> = (Cow<'c, str>, Paint);

/// (name, value, colour, percent)
pub fn item(surface: &mut Surface, line: &Line<'_>, idx: u8) {
	let y = PADDING + f32::from(idx) * 21.2;
	let x = HEADER_MIDDLE_END_X + GAP;

	let rect = Rect::from_xywh(x, y, ITEM_WIDTH, 21.2).with_offset((17., 13.));

	draw(
		surface,
		&[
			Text {
				text: &format!("{}. ", idx + 1),
				paint: paint::Paint::White,
				..Default::default()
			},
			Text {
				text: line.0.as_ref(),
				paint: line.1,
				..Default::default()
			},
		],
		17.,
		rect,
		TextAlign::Left,
		true,
	);
}
