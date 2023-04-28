use konst::{parser_method, parsing::ParseValueResult, Parser};
use once_cell::sync::Lazy;
use skia_safe::{
	font_arguments::{variation_position::Coordinate, VariationPosition},
	textlayout::TextStyle,
	FontArguments, FourByteTag,
};

use crate::paint::MinecraftPaint;

#[allow(dead_code)]
pub static STYLE_NORMAL: Lazy<TextStyle> = Lazy::new(|| {
	let mut style = TextStyle::new();

	style.set_font_families(&["Minecraft"]);
	style.set_font_style(skia_safe::FontStyle::normal());

	style
});

#[allow(dead_code)]
pub static STYLE_BOLD: Lazy<TextStyle> = Lazy::new(|| {
	let mut style = TextStyle::new();

	style.set_font_families(&["Minecraft"]);
	style.set_font_style(skia_safe::FontStyle::bold());

	style
});

#[allow(dead_code)]
pub static STYLE_ITALIC: Lazy<TextStyle> = Lazy::new(|| {
	let mut style = TextStyle::new();

	style.set_font_families(&["Minecraft"]);
	style.set_font_style(skia_safe::FontStyle::italic());

	style
});

#[allow(dead_code)]
pub static STYLE_BOLD_ITALIC: Lazy<TextStyle> = Lazy::new(|| {
	let mut style = TextStyle::new();

	style.set_font_families(&["Minecraft"]);
	style.set_font_style(skia_safe::FontStyle::bold_italic());

	style
});

#[allow(dead_code)]
pub static STYLE_ICON: Lazy<TextStyle> = Lazy::new(|| {
	let coordinates = [Coordinate {
		axis: FourByteTag::from_chars('F', 'I', 'L', 'L'),
		value: 1.,
	}];

	let args = FontArguments::new().set_variation_design_position(VariationPosition {
		coordinates: &coordinates,
	});

	let mut style = TextStyle::new();

	style.set_font_arguments(Some(&args));
	style.set_font_families(&["Material Symbols Outlined"]);
	style.set_font_style(skia_safe::FontStyle::normal());
	style.set_baseline_shift(5.);

	style
});

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MinecraftFont {
	Normal,
	Bold,
	Italic,
	BoldItalic,
	Icon,
}

impl MinecraftFont {
	pub fn get_style(&self, paint: MinecraftPaint, size: f32) -> TextStyle {
		let mut style = match self {
			Self::Normal => STYLE_NORMAL.clone(),
			Self::Bold => STYLE_BOLD.clone(),
			Self::Italic => STYLE_ITALIC.clone(),
			Self::BoldItalic => STYLE_BOLD_ITALIC.clone(),
			Self::Icon => STYLE_ICON.clone(),
		};

		style.set_font_size(size);
		style.set_foreground_color(paint.into());

		style
	}
}

impl From<char> for MinecraftFont {
	fn from(c: char) -> Self {
		match c {
			'r' | 'R' => Self::Normal,
			'l' | 'L' => Self::Bold,
			'o' | 'O' => Self::Italic,
			_ => Self::Normal,
		}
	}
}

pub const fn parse_font(mut parser: Parser<'_>) -> ParseValueResult<'_, MinecraftFont> {
	let font = parser_method! {parser, strip_prefix;
		"r" => MinecraftFont::Normal,
		"l" => MinecraftFont::Bold,
		"o" => MinecraftFont::Italic,
		_ => return Err(parser.into_other_error(&"could not parse font")),
	};

	Ok((font, parser))
}
