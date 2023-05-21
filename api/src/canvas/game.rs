use super::{
	label::ToFormatted, util, GAP, HEADER_DATA_HEIGHT, HEADER_DATA_RAD, HEADER_HEIGHT,
	HEADER_LABEL_HEIGHT, HEADER_LEFT_END_X, HEADER_NAME_HEIGHT, PADDING,
};

use minecraft::{
	paint::Paint,
	style::MinecraftFont,
	text::{draw, parse::minecraft_string, Text},
};
use skia_safe::{gradient_shader, textlayout::TextAlign, Color, Handle, RRect, Rect, Surface};
use translate::{tr, Context};

#[allow(clippy::needless_pass_by_value)]
pub fn apply_data(
	ctx: Context<'_>,
	surface: &mut Surface,
	level: &str,
	progress: f32,
	current: impl ToFormatted,
	needed: impl ToFormatted,
	colors: &[Color; 2],
) {
	let label = format!("{}: ", tr!(ctx, "level"));
	let mut text = vec![Text {
		text: &label,
		paint: Paint::White,
		..Default::default()
	}];

	text.extend(minecraft_string(level));
	text.reserve_exact(8);

	let label = format!("\n{}: ", tr!(ctx, "progress"));
	let current = current.to_formatted_label(ctx);
	let needed = needed.to_formatted_label(ctx);

	text.push(Text {
		text: &label,
		paint: Paint::White,
		..Default::default()
	});

	text.push(Text {
		text: &current,
		paint: Paint::Aqua,
		..Default::default()
	});

	text.push(Text {
		text: "/",
		paint: Paint::White,
		..Default::default()
	});

	text.push(Text {
		text: &needed,
		paint: Paint::Green,
		..Default::default()
	});

	let rect = Rect::from_xywh(
		PADDING,
		PADDING + HEADER_NAME_HEIGHT + GAP * 2. + HEADER_LABEL_HEIGHT,
		HEADER_LEFT_END_X - PADDING,
		HEADER_DATA_HEIGHT,
	);

	draw(surface, text.as_slice(), 20., rect, TextAlign::Center, true);

	let path = util::progress::rrect(
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

	let mut paint: skia_safe::Paint = Handle::default();

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

#[allow(clippy::needless_pass_by_value)]
pub fn bubble(
	ctx: Context<'_>,
	surface: &mut Surface,
	value: impl ToFormatted,
	label: &str,
	paint: Paint,
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
			text: &value.to_formatted_label(ctx),
			paint,
			font: MinecraftFont::Normal,
			size: None,
		},
	];

	let rect = super::get_item_rect(index);

	draw(surface, &text, 40., rect, TextAlign::Center, true);
}
