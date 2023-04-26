pub mod rank;
pub mod parse;

use crate::{font, paint};
use skia_safe::TextBlob;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinecraftText<'a> {
	text: &'a str,
	paint: paint::MinecraftPaint,
	font: font::MinecraftFont,
}

impl MinecraftText<'_> {
	pub fn get_blob(&self) -> Option<TextBlob> {
		TextBlob::from_str(self.text, self.font.into())
	}
}

pub fn draw_minecraft_text<'a>(
	surface: &mut skia_safe::Surface,
	text: impl Iterator<Item = &'a MinecraftText<'a>>,
) {
	let mut x = 0.0;
	let y = 0.0;

	for text in text {
		let blob = text.get_blob();

		if let Some(blob) = blob {
			surface
				.canvas()
				.draw_text_blob(&blob, (x, y), text.paint.into());

			x += blob.bounds().width();
		}
	}
}
