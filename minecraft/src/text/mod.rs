pub mod parse;
pub mod rank;

use crate::{font, paint};
use skia_safe::TextBlob;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinecraftText<'a> {
	pub text: &'a str,
	pub paint: paint::MinecraftPaint,
	pub font: font::MinecraftFont,
}

impl MinecraftText<'_> {
	pub fn get_blob(&self, size: f32) -> Option<TextBlob> {
		TextBlob::from_str(self.text, &self.font.get_font(size))
	}
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

			x += blob.bounds().width();
		}
	}
}
