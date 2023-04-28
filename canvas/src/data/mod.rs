use crate::{
	GAP, HEADER_LABEL_HEIGHT, HEADER_LEFT_END_X, HEADER_NAME_HEIGHT, ITEM_HEIGHT, ITEM_WIDTH,
	PADDING,
};

pub mod header;
pub mod skywars;

use minecraft::{
	paint::{self, MinecraftPaint},
	style::MinecraftFont,
	text::{draw, Text},
};
use num_format::ToFormattedString;
use skia_safe::{textlayout::TextAlign, Rect, Surface};
use translate::{prelude::GetNumFormatLocale, Context};

fn apply_label(surface: &mut Surface, label: &[Text<'_>]) {
	draw(
		surface,
		label,
		20.,
		Rect::from_xywh(
			PADDING,
			PADDING + HEADER_NAME_HEIGHT + GAP,
			HEADER_LEFT_END_X,
			HEADER_LABEL_HEIGHT,
		),
		TextAlign::Center,
		true,
	);
}

fn apply_item(
	ctx: &Context<'_>,
	surface: &mut Surface,
	value: u32,
	icon: &str,
	paint: MinecraftPaint,
	index: u16,
) {
	let text = [
		Text {
			text: &value.to_formatted_string(&ctx.get_num_format_locale()),
			paint,
			font: MinecraftFont::Normal,
		},
		Text {
			text: icon,
			paint: paint::MinecraftPaint::Gray,
			font: MinecraftFont::Icon,
		},
	];

	let rect = super::get_item_rect(index);

	draw(surface, &text, 40., rect, TextAlign::Center, true);
}

fn apply_item_float(
	surface: &mut Surface,
	value: f32,
	icon: &str,
	paint: MinecraftPaint,
	index: u16,
) {
	let text = [
		Text {
			text: &format!("{value:.2}"),
			paint,
			font: MinecraftFont::Normal,
		},
		Text {
			text: icon,
			paint: paint::MinecraftPaint::Gray,
			font: MinecraftFont::Icon,
		},
	];

	let rect = super::get_item_rect(index);

	draw(surface, &text, 40., rect, TextAlign::Center, true);
}

fn apply_extras(surface: &mut Surface, lines: [&[Text<'_>]; 7]) {
	let mut y = PADDING + 15.;
	let x = HEADER_LEFT_END_X + GAP + 15.;

	for line in lines {
		let rect = Rect::from_xywh(x, y, ITEM_WIDTH, ITEM_HEIGHT);

		draw(surface, line, 17., rect, TextAlign::Left, false);

		y += 21.2;
	}
}
