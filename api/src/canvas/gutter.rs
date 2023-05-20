use std::borrow::Cow;

use crate::canvas::WIDTH_F;

use super::{GAP, HEADER_MIDDLE_END_X, ITEM_WIDTH, PADDING};

use minecraft::{
	paint::Paint,
	text::{draw, Text},
};
use skia_safe::{textlayout::TextAlign, ClipOp, Rect, Surface};

pub type Line<'c> = (Cow<'c, str>, Paint);

/// (name, value, colour, percent)
pub fn item(surface: &mut Surface, line: &Line<'_>, idx: u8) {
	let y = PADDING + f32::from(idx) * 21.2;
	let x = HEADER_MIDDLE_END_X + GAP;

	let mut rect =
		Rect::from_xywh(x, y, (ITEM_WIDTH - GAP) / 2. - 17., 21.2).with_offset((17., 13.));

	surface.canvas().save();
	surface.canvas().clip_rect(rect, ClipOp::Intersect, false);

	rect.right = WIDTH_F * 2.;

	draw(
		surface,
		&[Text {
			text: line.0.as_ref(),
			paint: line.1,
			..Default::default()
		}],
		17.,
		rect,
		TextAlign::Left,
		true,
	);

	surface.canvas().restore();
}
