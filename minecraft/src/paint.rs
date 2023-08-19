use darling::FromMeta;
use konst::{parser_method, parsing::ParseValueResult, Parser};
use once_cell::sync::Lazy;
use plotters::style::RGBColor;
use quote::quote;
use skia_safe::Color;

macro_rules! colour {
	($name: ident, $colour: expr) => {
		pub static $name: Lazy<skia_safe::Paint> = Lazy::new(|| {
			let mut paint = skia_safe::Paint::default();

			paint
				.set_color(skia_safe::Color::from_argb(
					$colour.0, $colour.1, $colour.2, $colour.3,
				))
				.set_anti_alias(false);

			paint
		});
	};
}

// Minecraft `Paint` colours
colour!(BLACK, (255, 0, 0, 0));
colour!(DARK_BLUE, (255, 0, 0, 170));
colour!(DARK_GREEN, (255, 0, 170, 0));
colour!(DARK_AQUA, (255, 0, 170, 170));
colour!(DARK_RED, (255, 170, 0, 0));
colour!(DARK_PURPLE, (255, 170, 0, 170));
colour!(GOLD, (255, 255, 170, 0));
colour!(GRAY, (255, 170, 170, 170));
colour!(DARK_GRAY, (255, 85, 85, 85));
colour!(BLUE, (255, 85, 85, 255));
colour!(GREEN, (255, 85, 255, 85));
colour!(AQUA, (255, 85, 255, 255));
colour!(RED, (255, 255, 85, 85));
colour!(LIGHT_PURPLE, (255, 255, 85, 255));
colour!(YELLOW, (255, 255, 255, 85));
colour!(WHITE, (255, 255, 255, 255));
colour!(BRONZE, (255, 205, 127, 50));

// Utility colours
colour!(BACKGROUND, (128, 15, 24, 32));
colour!(CANVAS_BACKGROUND, (255, 31, 48, 64));

#[derive(
	serde::Serialize,
	serde::Deserialize,
	Default,
	Clone,
	Copy,
	Debug,
	PartialEq,
	Eq,
	FromMeta,
	bincode::Encode,
	bincode::Decode,
	bitcode::Encode,
	bitcode::Decode,
)]
#[darling(default)]
#[serde(rename_all = "snake_case")]
pub enum Paint {
	Black,
	DarkBlue,
	DarkGreen,
	DarkAqua,
	DarkRed,
	DarkPurple,
	Gold,
	Gray,
	DarkGray,
	Blue,
	Green,
	Aqua,
	Red,
	LightPurple,
	Yellow,
	#[default]
	White,
	/// Special non-Minecraft colour for "3rd place" colour
	Bronze,
}

impl Paint {
	#[must_use]
	pub fn shadow(&self) -> Color {
		match self {
			Self::Black => Color::from_argb(255, 0, 0, 0),
			Self::DarkBlue => Color::from_argb(255, 0, 0, 0x2a),
			Self::DarkGreen => Color::from_argb(255, 0, 0x2a, 0),
			Self::DarkAqua => Color::from_argb(255, 0, 0x2a, 0x2a),
			Self::DarkRed => Color::from_argb(255, 0x2a, 0, 0),
			Self::DarkPurple => Color::from_argb(255, 0x2a, 0, 0x2a),
			Self::Gold => Color::from_argb(255, 0x3f, 0x2a, 0),
			Self::Gray => Color::from_argb(255, 0x2a, 0x2a, 0x2a),
			Self::DarkGray => Color::from_argb(255, 0x15, 0x15, 0x15),
			Self::Blue => Color::from_argb(255, 0x15, 0x15, 0x3f),
			Self::Green => Color::from_argb(255, 0x15, 0x3f, 0x15),
			Self::Aqua => Color::from_argb(255, 0x15, 0x3f, 0x3f),
			Self::Red => Color::from_argb(255, 0x3f, 0x15, 0x15),
			Self::LightPurple => Color::from_argb(255, 0x3f, 0x15, 0x3f),
			Self::Yellow => Color::from_argb(255, 0x3f, 0x3f, 0x15),
			Self::White => Color::from_argb(255, 0x3f, 0x3f, 0x3f),
			Self::Bronze => Color::from_argb(255, 77, 45, 14),
		}
	}
}

impl Paint {
	#[must_use]
	pub fn as_plotters(&self) -> RGBColor {
		match self {
			Self::Black => RGBColor(0, 0, 0),
			Self::DarkBlue => RGBColor(0, 0, 170),
			Self::DarkGreen => RGBColor(0, 170, 0),
			Self::DarkAqua => RGBColor(0, 170, 170),
			Self::DarkRed => RGBColor(170, 0, 0),
			Self::DarkPurple => RGBColor(170, 0, 170),
			Self::Gold => RGBColor(255, 170, 0),
			Self::Gray => RGBColor(170, 170, 170),
			Self::DarkGray => RGBColor(85, 85, 85),
			Self::Blue => RGBColor(85, 85, 255),
			Self::Green => RGBColor(85, 255, 85),
			Self::Aqua => RGBColor(85, 255, 255),
			Self::Red => RGBColor(255, 85, 85),
			Self::LightPurple => RGBColor(255, 85, 255),
			Self::Yellow => RGBColor(255, 255, 85),
			Self::White => RGBColor(255, 255, 255),
			Self::Bronze => RGBColor(205, 127, 50),
		}
	}
}

impl darling::ToTokens for Paint {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		tokens.extend(quote! {
			::minecraft::paint::Paint::
		});

		match self {
			Self::Black => tokens.extend(quote!(Black)),
			Self::DarkBlue => tokens.extend(quote!(DarkBlue)),
			Self::DarkGreen => tokens.extend(quote!(DarkGreen)),
			Self::DarkAqua => tokens.extend(quote!(DarkAqua)),
			Self::DarkRed => tokens.extend(quote!(DarkRed)),
			Self::DarkPurple => tokens.extend(quote!(DarkPurple)),
			Self::Gold => tokens.extend(quote!(Gold)),
			Self::Gray => tokens.extend(quote!(Gray)),
			Self::DarkGray => tokens.extend(quote!(DarkGray)),
			Self::Blue => tokens.extend(quote!(Blue)),
			Self::Green => tokens.extend(quote!(Green)),
			Self::Aqua => tokens.extend(quote!(Aqua)),
			Self::Red => tokens.extend(quote!(Red)),
			Self::LightPurple => tokens.extend(quote!(LightPurple)),
			Self::Yellow => tokens.extend(quote!(Yellow)),
			Self::White => tokens.extend(quote!(White)),
			Self::Bronze => tokens.extend(quote!(Bronze)),
		}
	}
}

impl From<Paint> for &skia_safe::Paint {
	fn from(paint: Paint) -> Self {
		match paint {
			Paint::Black => &BLACK,
			Paint::DarkBlue => &DARK_BLUE,
			Paint::DarkGreen => &DARK_GREEN,
			Paint::DarkAqua => &DARK_AQUA,
			Paint::DarkRed => &DARK_RED,
			Paint::DarkPurple => &DARK_PURPLE,
			Paint::Gold => &GOLD,
			Paint::Gray => &GRAY,
			Paint::DarkGray => &DARK_GRAY,
			Paint::Blue => &BLUE,
			Paint::Green => &GREEN,
			Paint::Aqua => &AQUA,
			Paint::Red => &RED,
			Paint::LightPurple => &LIGHT_PURPLE,
			Paint::Yellow => &YELLOW,
			Paint::White => &WHITE,
			Paint::Bronze => &BRONZE,
		}
	}
}

impl From<Paint> for skia_safe::Color {
	fn from(paint: Paint) -> Self {
		match paint {
			Paint::Black => super::colour::BLACK,
			Paint::DarkBlue => super::colour::DARK_BLUE,
			Paint::DarkGreen => super::colour::DARK_GREEN,
			Paint::DarkAqua => super::colour::DARK_AQUA,
			Paint::DarkRed => super::colour::DARK_RED,
			Paint::DarkPurple => super::colour::DARK_PURPLE,
			Paint::Gold => super::colour::GOLD,
			Paint::Gray => super::colour::GRAY,
			Paint::DarkGray => super::colour::DARK_GRAY,
			Paint::Blue => super::colour::BLUE,
			Paint::Green => super::colour::GREEN,
			Paint::Aqua => super::colour::AQUA,
			Paint::Red => super::colour::RED,
			Paint::LightPurple => super::colour::LIGHT_PURPLE,
			Paint::Yellow => super::colour::YELLOW,
			Paint::White => super::colour::WHITE,
			Paint::Bronze => super::colour::BRONZE,
		}
	}
}

impl TryFrom<char> for Paint {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'0' => Ok(Paint::Black),
			'1' => Ok(Paint::DarkBlue),
			'2' => Ok(Paint::DarkGreen),
			'3' => Ok(Paint::DarkAqua),
			'4' => Ok(Paint::DarkRed),
			'5' => Ok(Paint::DarkPurple),
			'6' => Ok(Paint::Gold),
			'7' => Ok(Paint::Gray),
			'8' => Ok(Paint::DarkGray),
			'9' => Ok(Paint::Blue),
			'a' | 'A' => Ok(Paint::Green),
			'b' | 'B' => Ok(Paint::Aqua),
			'c' | 'C' => Ok(Paint::Red),
			'd' | 'D' => Ok(Paint::LightPurple),
			'e' | 'E' => Ok(Paint::Yellow),
			'f' | 'F' => Ok(Paint::White),
			_ => Err(()),
		}
	}
}

impl TryFrom<&str> for Paint {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"BLACK" => Ok(Paint::Black),
			"DARK_BLUE" => Ok(Paint::DarkBlue),
			"DARK_GREEN" => Ok(Paint::DarkGreen),
			"DARK_AQUA" => Ok(Paint::DarkAqua),
			"DARK_RED" => Ok(Paint::DarkRed),
			"DARK_PURPLE" => Ok(Paint::DarkPurple),
			"GOLD" => Ok(Paint::Gold),
			"GRAY" => Ok(Paint::Gray),
			"DARK_GRAY" => Ok(Paint::DarkGray),
			"BLUE" => Ok(Paint::Blue),
			"GREEN" => Ok(Paint::Green),
			"AQUA" => Ok(Paint::Aqua),
			"RED" => Ok(Paint::Red),
			"LIGHT_PURPLE" => Ok(Paint::LightPurple),
			"YELLOW" => Ok(Paint::Yellow),
			"WHITE" => Ok(Paint::White),
			_ => Err(()),
		}
	}
}

/// # Errors
/// Returns an error if the string is not a Minecraft colour code
pub const fn parse(mut parser: Parser<'_>) -> ParseValueResult<'_, Paint> {
	let paint = parser_method! {parser, strip_prefix;
		"0" => Paint::Black,
		"1" => Paint::DarkBlue,
		"2" => Paint::DarkGreen,
		"3" => Paint::DarkAqua,
		"4" => Paint::DarkRed,
		"5" => Paint::DarkPurple,
		"6" => Paint::Gold,
		"7" => Paint::Gray,
		"8" => Paint::DarkGray,
		"9" => Paint::Blue,
		"a" => Paint::Green,
		"b" => Paint::Aqua,
		"c" => Paint::Red,
		"d" => Paint::LightPurple,
		"e" => Paint::Yellow,
		"f" => Paint::White,
		_ => return Err(parser.into_other_error(&"could not parse paint")),
	};

	Ok((paint, parser))
}

#[cfg(test)]
mod tests {
	use std::assert_matches::assert_matches;

	use super::*;

	#[test]
	fn test_try_from_char() {
		assert_matches!(Paint::try_from('0'), Ok(Paint::Black));
		assert_matches!(Paint::try_from('1'), Ok(Paint::DarkBlue));
		assert_matches!(Paint::try_from('2'), Ok(Paint::DarkGreen));
		assert_matches!(Paint::try_from('3'), Ok(Paint::DarkAqua));
		assert_matches!(Paint::try_from('4'), Ok(Paint::DarkRed));
		assert_matches!(Paint::try_from('5'), Ok(Paint::DarkPurple));
		assert_matches!(Paint::try_from('6'), Ok(Paint::Gold));
		assert_matches!(Paint::try_from('7'), Ok(Paint::Gray));
		assert_matches!(Paint::try_from('8'), Ok(Paint::DarkGray));
		assert_matches!(Paint::try_from('9'), Ok(Paint::Blue));
		assert_matches!(Paint::try_from('a'), Ok(Paint::Green));
		assert_matches!(Paint::try_from('b'), Ok(Paint::Aqua));
		assert_matches!(Paint::try_from('c'), Ok(Paint::Red));
		assert_matches!(Paint::try_from('d'), Ok(Paint::LightPurple));
		assert_matches!(Paint::try_from('e'), Ok(Paint::Yellow));
		assert_matches!(Paint::try_from('f'), Ok(Paint::White));
		assert_matches!(Paint::try_from('g'), Err(()));
	}
}
