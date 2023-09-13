#[cfg(feature = "skia")]
mod draw;
pub mod parse;
pub mod rank;

#[cfg(feature = "skia")]
use skia_safe::textlayout::TextStyle;

use crate::{paint::Paint, style::MinecraftFont};
#[cfg(feature = "skia")]
pub use draw::draw;

pub const ESCAPE: char = '§';

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text<'t> {
	pub text: &'t str,
	pub font: MinecraftFont,
	pub paint: Paint,
	pub size: Option<f32>,
}

const DEFAULT_TEXT: Text<'static> = Text {
	text: "",
	font: MinecraftFont::Normal,
	paint: Paint::White,
	size: None,
};

impl<'t> Default for Text<'t> {
	fn default() -> Self {
		DEFAULT_TEXT
	}
}

impl<'t> Text<'t> {
	pub const NEW_LINE: Text<'static> = Text {
		text: "\n",
		font: MinecraftFont::Normal,
		paint: Paint::White,
		size: None,
	};
	pub const SPACE: Text<'static> = Text {
		text: " ",
		font: MinecraftFont::Normal,
		paint: Paint::White,
		size: None,
	};

	#[cfg(feature = "skia")]
	#[must_use]
	pub fn get_style(
		&self,
		family: crate::style::Family,
		paint: Paint,
		default_size: f32,
	) -> TextStyle {
		let size = self.size.unwrap_or(default_size);
		let mut style = self.font.get_style(family, paint, size);

		style.set_font_size(size);
		style.set_foreground_paint(paint.into());
		style
	}

	#[must_use]
	pub fn new_with_paint(text: &'t str, paint: Paint) -> Self {
		Self {
			text,
			paint,
			..Self::default()
		}
	}
}
