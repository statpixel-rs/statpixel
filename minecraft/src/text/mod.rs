pub mod parse;
pub mod rank;

use crate::{font, paint};
use skia_safe::TextBlob;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinecraftText<'a> {
	pub text: &'a str,
	pub paint: paint::MinecraftPaint,
	pub font: font::MinecraftFont<'a>,
}

impl MinecraftText<'_> {
	pub fn get_blob(&self, size: f32) -> Option<TextBlob> {
		TextBlob::from_str(self.text, &self.font.get_font(size))
	}
}

pub fn measure_minecraft_text_ref<'a, 'b>(
	text: impl Iterator<Item = &'b MinecraftText<'a>>,
	size: f32,
) -> f32
where
	'a: 'b,
{
	text.map(|text| text.font.get_font(size).measure_str(text.text, None).0)
		.sum()
}

pub fn measure_minecraft_text<'a>(text: impl Iterator<Item = MinecraftText<'a>>, size: f32) -> f32 {
	text.map(|text| text.font.get_font(size).measure_str(text.text, None).0)
		.sum()
}

pub fn draw_minecraft_text<'a>(
	surface: &mut skia_safe::Surface,
	text: impl Iterator<Item = MinecraftText<'a>>,
	mut x: f32,
	y: f32,
	size: f32,
) {
	for text in text {
		let blob = text.get_blob(size);

		if let Some(blob) = blob {
			surface
				.canvas()
				.draw_text_blob(&blob, (x, y), text.paint.into());

			x += text.font.get_font(size).measure_str(text.text, None).0;
		}
	}
}

pub fn draw_minecraft_text_ref<'a, 'b>(
	surface: &mut skia_safe::Surface,
	text: impl Iterator<Item = &'b MinecraftText<'a>>,
	mut x: f32,
	y: f32,
	size: f32,
) where
	'a: 'b,
{
	for text in text {
		let blob = text.get_blob(size);

		if let Some(blob) = blob {
			surface
				.canvas()
				.draw_text_blob(&blob, (x, y), text.paint.into());

			x += text.font.get_font(size).measure_str(text.text, None).0;
		}
	}
}
