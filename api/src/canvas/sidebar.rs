use std::borrow::Cow;

use super::{label::ToFormatted, GAP, HEADER_LEFT_END_X, ITEM_WIDTH, PADDING};

use minecraft::{
	paint::{self, Paint},
	text::Text,
};
use skia_safe::{textlayout::TextAlign, Rect, Surface};
use translate::Context;

/// (name, value, colour, percent)
pub fn item(
	ctx: Context<'_>,
	surface: &mut Surface,
	line: &(Cow<'_, str>, impl ToFormatted, Paint),
	idx: u8,
) {
	let y = PADDING + f32::from(idx) * 21.2;
	let x = HEADER_LEFT_END_X + GAP;

	let rect = Rect::from_xywh(x, y, ITEM_WIDTH, 21.2).with_offset((17., 13.));
	let text = line.1.to_formatted_label(ctx);

	minecraft::text::draw(
		surface,
		&[
			Text {
				text: "• ",
				paint: line.2,
				..Default::default()
			},
			Text {
				text: &format!("{}: ", line.0),
				paint: paint::Paint::White,
				..Default::default()
			},
			Text {
				text: &text,
				paint: line.2,
				..Default::default()
			},
		],
		17.,
		rect,
		TextAlign::Left,
		true,
	);
}
