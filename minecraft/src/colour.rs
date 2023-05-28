use darling::FromMeta;
use konst::{parser_method, parsing::ParseValueResult, Parser};
use quote::quote;
use serde::{Deserialize, Serialize};

macro_rules! colour {
	($name: ident, $colour: expr) => {
		pub const $name: skia_safe::Color =
			skia_safe::Color::from_rgb($colour.0, $colour.1, $colour.2);
	};
}

// Minecraft `Paint` colours
colour!(BLACK, (0, 0, 0));
colour!(DARK_BLUE, (0, 0, 170));
colour!(DARK_GREEN, (0, 170, 0));
colour!(DARK_AQUA, (0, 170, 170));
colour!(DARK_RED, (170, 0, 0));
colour!(DARK_PURPLE, (170, 0, 170));
colour!(GOLD, (255, 170, 0));
colour!(GRAY, (170, 170, 170));
colour!(DARK_GRAY, (85, 85, 85));
colour!(BLUE, (85, 85, 255));
colour!(GREEN, (85, 255, 85));
colour!(AQUA, (85, 255, 255));
colour!(RED, (255, 85, 85));
colour!(LIGHT_PURPLE, (255, 85, 255));
colour!(YELLOW, (255, 255, 85));
colour!(WHITE, (255, 255, 255));

// Utility colours
colour!(BACKGROUND, (31, 48, 64));

#[derive(
	bincode::Decode,
	bincode::Encode,
	Deserialize,
	Serialize,
	Default,
	Clone,
	Copy,
	Debug,
	PartialEq,
	Eq,
	FromMeta,
)]
#[serde(try_from = "&str")]
#[darling(default)]
pub enum Colour {
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
}

impl darling::ToTokens for Colour {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		tokens.extend(quote! {
			::minecraft::colour::Colour::
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
		}
	}
}

impl From<Colour> for &skia_safe::Color {
	fn from(colour: Colour) -> Self {
		match colour {
			Colour::Black => &BLACK,
			Colour::DarkBlue => &DARK_BLUE,
			Colour::DarkGreen => &DARK_GREEN,
			Colour::DarkAqua => &DARK_AQUA,
			Colour::DarkRed => &DARK_RED,
			Colour::DarkPurple => &DARK_PURPLE,
			Colour::Gold => &GOLD,
			Colour::Gray => &GRAY,
			Colour::DarkGray => &DARK_GRAY,
			Colour::Blue => &BLUE,
			Colour::Green => &GREEN,
			Colour::Aqua => &AQUA,
			Colour::Red => &RED,
			Colour::LightPurple => &LIGHT_PURPLE,
			Colour::Yellow => &YELLOW,
			Colour::White => &WHITE,
		}
	}
}

impl From<Colour> for skia_safe::Color {
	fn from(colour: Colour) -> Self {
		match colour {
			Colour::Black => BLACK,
			Colour::DarkBlue => DARK_BLUE,
			Colour::DarkGreen => DARK_GREEN,
			Colour::DarkAqua => DARK_AQUA,
			Colour::DarkRed => DARK_RED,
			Colour::DarkPurple => DARK_PURPLE,
			Colour::Gold => GOLD,
			Colour::Gray => GRAY,
			Colour::DarkGray => DARK_GRAY,
			Colour::Blue => BLUE,
			Colour::Green => GREEN,
			Colour::Aqua => AQUA,
			Colour::Red => RED,
			Colour::LightPurple => LIGHT_PURPLE,
			Colour::Yellow => YELLOW,
			Colour::White => WHITE,
		}
	}
}

impl TryFrom<char> for Colour {
	type Error = &'static str;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'0' => Ok(Self::Black),
			'1' => Ok(Self::DarkBlue),
			'2' => Ok(Self::DarkGreen),
			'3' => Ok(Self::DarkAqua),
			'4' => Ok(Self::DarkRed),
			'5' => Ok(Self::DarkPurple),
			'6' => Ok(Self::Gold),
			'7' => Ok(Self::Gray),
			'8' => Ok(Self::DarkGray),
			'9' => Ok(Self::Blue),
			'a' | 'A' => Ok(Self::Green),
			'b' | 'B' => Ok(Self::Aqua),
			'c' | 'C' => Ok(Self::Red),
			'd' | 'D' => Ok(Self::LightPurple),
			'e' | 'E' => Ok(Self::Yellow),
			'f' | 'F' => Ok(Self::White),
			_ => Err("invalid colour code"),
		}
	}
}

impl From<Colour> for char {
	fn from(colour: Colour) -> Self {
		match colour {
			Colour::Black => '0',
			Colour::DarkBlue => '1',
			Colour::DarkGreen => '2',
			Colour::DarkAqua => '3',
			Colour::DarkRed => '4',
			Colour::DarkPurple => '5',
			Colour::Gold => '6',
			Colour::Gray => '7',
			Colour::DarkGray => '8',
			Colour::Blue => '9',
			Colour::Green => 'a',
			Colour::Aqua => 'b',
			Colour::Red => 'c',
			Colour::LightPurple => 'd',
			Colour::Yellow => 'e',
			Colour::White => 'f',
		}
	}
}

impl TryFrom<&str> for Colour {
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"BLACK" => Ok(Self::Black),
			"DARK_BLUE" => Ok(Self::DarkBlue),
			"DARK_GREEN" | "EMERALD" => Ok(Self::DarkGreen),
			"DARK_AQUA" | "CYAN" => Ok(Self::DarkAqua),
			"DARK_RED" => Ok(Self::DarkRed),
			"DARK_PURPLE" | "PURPLE" => Ok(Self::DarkPurple),
			"GOLD" | "ORANGE" => Ok(Self::Gold),
			"GRAY" | "SILVER" => Ok(Self::Gray),
			"DARK_GRAY" => Ok(Self::DarkGray),
			"BLUE" => Ok(Self::Blue),
			"GREEN" => Ok(Self::Green),
			"AQUA" | "DIAMOND" => Ok(Self::Aqua),
			"RED" => Ok(Self::Red),
			"LIGHT_PURPLE" | "PINK" => Ok(Self::LightPurple),
			"YELLOW" => Ok(Self::Yellow),
			"WHITE" => Ok(Self::White),
			_ => Err("invalid colour code"),
		}
	}
}

/// # Errors
/// Returns an error if the string is not a Minecraft colour code
pub const fn parse(mut parser: Parser<'_>) -> ParseValueResult<'_, Colour> {
	let paint = parser_method! {parser, strip_prefix;
		"0" => Colour::Black,
		"1" => Colour::DarkBlue,
		"2" => Colour::DarkGreen,
		"3" => Colour::DarkAqua,
		"4" => Colour::DarkRed,
		"5" => Colour::DarkPurple,
		"6" => Colour::Gold,
		"7" => Colour::Gray,
		"8" => Colour::DarkGray,
		"9" => Colour::Blue,
		"a" => Colour::Green,
		"b" => Colour::Aqua,
		"c" => Colour::Red,
		"d" => Colour::LightPurple,
		"e" => Colour::Yellow,
		"f" => Colour::White,
		_ => return Err(parser.into_other_error(&"could not parse paint")),
	};

	Ok((paint, parser))
}
