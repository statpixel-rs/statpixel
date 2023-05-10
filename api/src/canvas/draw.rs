use super::{
	label::ToFormattedLabel, util, GAP, HEADER_DATA_HEIGHT, HEADER_DATA_RAD, HEADER_HEIGHT,
	HEADER_LABEL_HEIGHT, HEADER_LEFT_END_X, HEADER_NAME_HEIGHT, ITEM_WIDTH, PADDING,
};

use minecraft::{
	paint::{self, MinecraftPaint},
	style::MinecraftFont,
	text::{draw, parse::parse_minecraft_string, Text},
};
use skia_safe::{gradient_shader, textlayout::TextAlign, Color, Paint, RRect, Rect, Surface};
use translate::{prelude::GetNumFormatLocale, tr, Context};

pub fn apply_data(
	ctx: Context<'_>,
	surface: &mut Surface,
	level: &str,
	progress: f32,
	current: impl ToFormattedLabel,
	needed: impl ToFormattedLabel,
	colors: &[Color; 2],
) {
	let locale = ctx.get_num_format_locale();
	let label = format!("{}: ", tr!(ctx, "level"));
	let mut text = vec![Text {
		text: &label,
		paint: paint::MinecraftPaint::White,
		..Default::default()
	}];

	text.extend(parse_minecraft_string(level));
	text.reserve_exact(8);

	let label = format!("\n{}: ", tr!(ctx, "progress"));
	let current = current.to_formatted_label(&locale, false);
	let needed = needed.to_formatted_label(&locale, false);

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

	let rect = Rect::from_xywh(
		PADDING,
		PADDING + HEADER_NAME_HEIGHT + GAP * 2. + HEADER_LABEL_HEIGHT,
		HEADER_LEFT_END_X - PADDING,
		HEADER_DATA_HEIGHT,
	);

	draw(surface, text.as_slice(), 20., rect, TextAlign::Center, true);

	let path = util::progress::rrect_progress(
		RRect::new_rect_xy(
			rect.with_inset((1.5, 1.5)),
			HEADER_DATA_RAD,
			HEADER_DATA_RAD,
		),
		progress,
	)
	.with_offset((
		PADDING + 1.5,
		PADDING + HEADER_NAME_HEIGHT + HEADER_LABEL_HEIGHT + GAP * 2. + 1.5,
	));

	let mut paint: Paint = Default::default();

	paint
		.set_stroke_width(3.)
		.set_style(skia_safe::paint::Style::Stroke)
		.set_stroke_cap(skia_safe::paint::Cap::Round)
		.set_alpha(64)
		.set_shader(gradient_shader::linear(
			(
				(
					PADDING,
					PADDING + HEADER_NAME_HEIGHT + HEADER_LABEL_HEIGHT + GAP * 2.,
				),
				(HEADER_LEFT_END_X, HEADER_HEIGHT),
			),
			colors.as_ref(),
			None,
			skia_safe::TileMode::Clamp,
			None,
			None,
		));

	surface.canvas().draw_path(&path, &paint);
}

pub fn apply_label(surface: &mut Surface, label: &[Text<'_>]) {
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

pub fn apply_item(
	ctx: Context<'_>,
	surface: &mut Surface,
	value: impl ToFormattedLabel,
	label: &str,
	paint: MinecraftPaint,
	percent: Option<bool>,
	index: usize,
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
			text: &value.to_formatted_label(&ctx.get_num_format_locale(), percent.unwrap_or(false)),
			paint,
			font: MinecraftFont::Normal,
			size: None,
		},
	];

	let rect = super::get_item_rect(index);

	draw(surface, &text, 40., rect, TextAlign::Center, true);
}

pub fn apply_extras(
	ctx: Context<'_>,
	surface: &mut Surface,
	lines: &[(String, Box<dyn ToFormattedLabel>, MinecraftPaint, bool)],
) {
	let mut y = PADDING;
	let x = HEADER_LEFT_END_X + GAP;

	for line in lines {
		let rect = Rect::from_xywh(x, y, ITEM_WIDTH, 21.2).with_offset((17., 13.));
		let text = line
			.1
			.to_formatted_label(&ctx.get_num_format_locale(), line.3);

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
