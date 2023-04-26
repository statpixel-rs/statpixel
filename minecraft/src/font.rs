use konst::{parser_method, parsing::ParseValueResult, Parser};
use once_cell::sync::Lazy;
use skia_safe::{Font, Typeface};

#[allow(dead_code)]
pub static FONT_NORMAL: Lazy<Font> = Lazy::new(|| {
	Font::from_typeface(
		Typeface::from_name("Minecraft", skia_safe::FontStyle::normal()).unwrap(),
		None,
	)
});

#[allow(dead_code)]
pub static FONT_BOLD: Lazy<Font> = Lazy::new(|| {
	Font::from_typeface(
		Typeface::from_name("Minecraft", skia_safe::FontStyle::bold()).unwrap(),
		None,
	)
});

#[allow(dead_code)]
pub static FONT_ITALIC: Lazy<Font> = Lazy::new(|| {
	Font::from_typeface(
		Typeface::from_name("Minecraft", skia_safe::FontStyle::italic()).unwrap(),
		None,
	)
});

#[allow(dead_code)]
pub static FONT_BOLD_ITALIC: Lazy<Font> = Lazy::new(|| {
	Font::from_typeface(
		Typeface::from_name("Minecraft", skia_safe::FontStyle::bold_italic()).unwrap(),
		None,
	)
});

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinecraftFont {
	Normal,
	Bold,
	Italic,
	BoldItalic,
}

impl MinecraftFont {
	pub fn get_font(&self, size: f32) -> Font {
		match self {
			Self::Normal => FONT_NORMAL.with_size(size).unwrap(),
			Self::Bold => FONT_BOLD.with_size(size).unwrap(),
			Self::Italic => FONT_ITALIC.with_size(size).unwrap(),
			Self::BoldItalic => FONT_BOLD_ITALIC.with_size(size).unwrap(),
		}
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
