use crate::{get_item_center, GAP, HEADER_LEFT_END_X, PADDING};

pub mod header;
pub mod skywars;

use minecraft::{
	font::{MinecraftFont, FONT_ICON},
	paint::{self, MinecraftPaint},
	text::{draw_minecraft_text_ref, measure_minecraft_text_ref, MinecraftText},
};
use num_format::ToFormattedString;
use skia_safe::Surface;

fn apply_item(surface: &mut Surface, count: u32, icon: &str, colour: MinecraftPaint, index: u16) {
	let text = [
		MinecraftText {
			text: &count.to_formatted_string(&num_format::Locale::en),
			paint: colour,
			font: MinecraftFont::Normal,
		},
		MinecraftText {
			text: icon,
			paint: paint::MinecraftPaint::White,
			font: MinecraftFont::Other(&FONT_ICON),
		},
	];

	let (x, y) = get_item_center(index);

	draw_minecraft_text_ref(
		surface,
		text.iter(),
		x - measure_minecraft_text_ref(text.iter(), 40.) / 2.,
		y + 15.,
		40.,
	)
}

fn apply_item_float(
	surface: &mut Surface,
	count: f32,
	icon: &str,
	colour: MinecraftPaint,
	index: u16,
) {
	let text = [
		MinecraftText {
			text: &format!("{count:.2}"),
			paint: colour,
			font: MinecraftFont::Normal,
		},
		MinecraftText {
			text: icon,
			paint: paint::MinecraftPaint::White,
			font: MinecraftFont::Other(&FONT_ICON),
		},
	];

	let (x, y) = get_item_center(index);

	draw_minecraft_text_ref(
		surface,
		text.iter(),
		x - measure_minecraft_text_ref(text.iter(), 40.) / 2.,
		y + 15.,
		40.,
	)
}

pub fn apply_extras<'a, 'b>(
	surface: &mut Surface,
	lines: [impl Iterator<Item = &'b MinecraftText<'a>>; 7],
) where
	'a: 'b,
{
	let mut y = PADDING + 30.;
	let x = HEADER_LEFT_END_X + GAP + 15.;

	for line in lines {
		draw_minecraft_text_ref(surface, line, x, y, 17.);

		y += 21.2;
	}
}
