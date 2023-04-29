use crate::{
	GAP, HEADER_DATA_HEIGHT, HEADER_LABEL_HEIGHT, HEADER_LEFT_END_X, HEADER_NAME_HEIGHT,
	ITEM_WIDTH, PADDING,
};

pub mod bedwars;
pub mod header;
pub mod skywars;

use minecraft::{
	paint::{self, MinecraftPaint},
	style::MinecraftFont,
	text::{draw, parse::parse_minecraft_string, Text},
};
use num_format::ToFormattedString;
use skia_safe::{textlayout::TextAlign, Rect, Surface};
use translate::{prelude::GetNumFormatLocale, tr, Context};

fn apply_data(
	ctx: Context<'_>,
	surface: &mut Surface,
	level: &str,
	progress: f32,
	current: u32,
	needed: u32,
) {
	let locale = ctx.get_num_format_locale();
	let num_boxes = (progress * 10.).round() as usize;
	let label = format!("{}: ", tr!(ctx, "level"));
	let mut text = vec![Text {
		text: &label,
		paint: paint::MinecraftPaint::White,
		..Default::default()
	}];

	text.extend(parse_minecraft_string(level));
	text.reserve_exact(8);

	let label = format!("\n{}: ", tr!(ctx, "progress"));
	let current = current.to_formatted_string(&locale);
	let needed = needed.to_formatted_string(&locale);

	text.push(Text {
		text: &label,
		paint: paint::MinecraftPaint::White,
		..Default::default()
	});

	text.push(Text {
		text: &current,
		paint: paint::MinecraftPaint::Aqua,
		..Default::default()
	});

	text.push(Text {
		text: "/",
		paint: paint::MinecraftPaint::White,
		..Default::default()
	});

	text.push(Text {
		text: &needed,
		paint: paint::MinecraftPaint::Green,
		..Default::default()
	});

	text.push(Text {
		text: "\n[",
		paint: paint::MinecraftPaint::DarkGray,
		..Default::default()
	});

	let boxes = "■".repeat(num_boxes);

	text.push(Text {
		text: &boxes,
		paint: paint::MinecraftPaint::Aqua,
		..Default::default()
	});

	let boxes = "■".repeat(10 - num_boxes);

	text.push(Text {
		text: &boxes,
		paint: paint::MinecraftPaint::Gray,
		..Default::default()
	});

	text.push(Text {
		text: "]",
		paint: paint::MinecraftPaint::DarkGray,
		..Default::default()
	});

	draw(
		surface,
		text.as_slice(),
		20.,
		Rect::from_xywh(
			PADDING,
			PADDING + HEADER_NAME_HEIGHT + GAP * 2. + HEADER_LABEL_HEIGHT,
			HEADER_LEFT_END_X,
			HEADER_DATA_HEIGHT,
		),
		TextAlign::Center,
		true,
	);
}

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
	ctx: Context<'_>,
	surface: &mut Surface,
	value: u32,
	label: &str,
	paint: MinecraftPaint,
	index: u16,
) {
	let text = [
		Text {
			text: label,
			paint,
			font: MinecraftFont::Normal,
			size: Some(20.),
		},
		Text {
			text: "\n",
			size: Some(20.),
			..Default::default()
		},
		Text {
			text: &value.to_formatted_string(&ctx.get_num_format_locale()),
			paint,
			font: MinecraftFont::Normal,
			size: None,
		},
	];

	let rect = super::get_item_rect(index);

	draw(surface, &text, 40., rect, TextAlign::Center, true);
}

fn apply_item_float(
	ctx: Context<'_>,
	surface: &mut Surface,
	value: f32,
	label: &str,
	paint: MinecraftPaint,
	index: u16,
) {
	let sep = tr!(ctx, "decimal-sep");

	let text = [
		Text {
			text: label,
			paint,
			font: MinecraftFont::Normal,
			size: Some(20.),
		},
		Text {
			text: "\n",
			size: Some(20.),
			..Default::default()
		},
		Text {
			text: &if &sep != "." {
				format!("{value:.2}").replacen('.', &sep, 1)
			} else {
				format!("{value:.2}")
			},
			paint,
			..Default::default()
		},
	];

	let rect = super::get_item_rect(index);

	draw(surface, &text, 40., rect, TextAlign::Center, true);
}

fn apply_extras(
	ctx: Context<'_>,
	surface: &mut Surface,
	lines: [(String, impl ToFormattedString, MinecraftPaint, Option<char>); 7],
) {
	let mut y = PADDING;
	let x = HEADER_LEFT_END_X + GAP;

	for line in lines {
		let rect = Rect::from_xywh(x, y, ITEM_WIDTH, 21.2).with_offset((17., 13.));
		let text = if let Some(c) = line.3 {
			format!(
				"{}{c}",
				line.1.to_formatted_string(&ctx.get_num_format_locale())
			)
		} else {
			line.1.to_formatted_string(&ctx.get_num_format_locale())
		};

		draw(
			surface,
			&[
				Text {
					text: "• ",
					paint: line.2,
					..Default::default()
				},
				Text {
					text: &format!("{}: ", line.0),
					paint: paint::MinecraftPaint::White,
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

		y += 21.2;
	}
}
