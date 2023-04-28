use konst::{parser_method, parsing::ParseValueResult, Parser};
use serde::Deserialize;

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

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(try_from = "&str")]
pub enum MinecraftColour {
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
	White,
}

impl From<MinecraftColour> for &skia_safe::Color {
	fn from(colour: MinecraftColour) -> Self {
		match colour {
			MinecraftColour::Black => &BLACK,
			MinecraftColour::DarkBlue => &DARK_BLUE,
			MinecraftColour::DarkGreen => &DARK_GREEN,
			MinecraftColour::DarkAqua => &DARK_AQUA,
			MinecraftColour::DarkRed => &DARK_RED,
			MinecraftColour::DarkPurple => &DARK_PURPLE,
			MinecraftColour::Gold => &GOLD,
			MinecraftColour::Gray => &GRAY,
			MinecraftColour::DarkGray => &DARK_GRAY,
			MinecraftColour::Blue => &BLUE,
			MinecraftColour::Green => &GREEN,
			MinecraftColour::Aqua => &AQUA,
			MinecraftColour::Red => &RED,
			MinecraftColour::LightPurple => &LIGHT_PURPLE,
			MinecraftColour::Yellow => &YELLOW,
			MinecraftColour::White => &WHITE,
		}
	}
}

impl From<MinecraftColour> for skia_safe::Color {
	fn from(colour: MinecraftColour) -> Self {
		match colour {
			MinecraftColour::Black => BLACK,
			MinecraftColour::DarkBlue => DARK_BLUE,
			MinecraftColour::DarkGreen => DARK_GREEN,
			MinecraftColour::DarkAqua => DARK_AQUA,
			MinecraftColour::DarkRed => DARK_RED,
			MinecraftColour::DarkPurple => DARK_PURPLE,
			MinecraftColour::Gold => GOLD,
			MinecraftColour::Gray => GRAY,
			MinecraftColour::DarkGray => DARK_GRAY,
			MinecraftColour::Blue => BLUE,
			MinecraftColour::Green => GREEN,
			MinecraftColour::Aqua => AQUA,
			MinecraftColour::Red => RED,
			MinecraftColour::LightPurple => LIGHT_PURPLE,
			MinecraftColour::Yellow => YELLOW,
			MinecraftColour::White => WHITE,
		}
	}
}

impl TryFrom<char> for MinecraftColour {
	type Error = &'static str;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'0' => Ok(MinecraftColour::Black),
			'1' => Ok(MinecraftColour::DarkBlue),
			'2' => Ok(MinecraftColour::DarkGreen),
			'3' => Ok(MinecraftColour::DarkAqua),
			'4' => Ok(MinecraftColour::DarkRed),
			'5' => Ok(MinecraftColour::DarkPurple),
			'6' => Ok(MinecraftColour::Gold),
			'7' => Ok(MinecraftColour::Gray),
			'8' => Ok(MinecraftColour::DarkGray),
			'9' => Ok(MinecraftColour::Blue),
			'a' | 'A' => Ok(MinecraftColour::Green),
			'b' | 'B' => Ok(MinecraftColour::Aqua),
			'c' | 'C' => Ok(MinecraftColour::Red),
			'd' | 'D' => Ok(MinecraftColour::LightPurple),
			'e' | 'E' => Ok(MinecraftColour::Yellow),
			'f' | 'F' => Ok(MinecraftColour::White),
			_ => Err("invalid colour code"),
		}
	}
}

impl TryFrom<&str> for MinecraftColour {
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"BLACK" => Ok(MinecraftColour::Black),
			"DARK_BLUE" => Ok(MinecraftColour::DarkBlue),
			"DARK_GREEN" => Ok(MinecraftColour::DarkGreen),
			"DARK_AQUA" => Ok(MinecraftColour::DarkAqua),
			"DARK_RED" => Ok(MinecraftColour::DarkRed),
			"DARK_PURPLE" => Ok(MinecraftColour::DarkPurple),
			"GOLD" => Ok(MinecraftColour::Gold),
			"GRAY" => Ok(MinecraftColour::Gray),
			"DARK_GRAY" => Ok(MinecraftColour::DarkGray),
			"BLUE" => Ok(MinecraftColour::Blue),
			"GREEN" => Ok(MinecraftColour::Green),
			"AQUA" => Ok(MinecraftColour::Aqua),
			"RED" => Ok(MinecraftColour::Red),
			"LIGHT_PURPLE" => Ok(MinecraftColour::LightPurple),
			"YELLOW" => Ok(MinecraftColour::Yellow),
			"WHITE" => Ok(MinecraftColour::White),
			_ => Err("invalid colour code"),
		}
	}
}

pub const fn parse_colour(mut parser: Parser<'_>) -> ParseValueResult<'_, MinecraftColour> {
	let paint = parser_method! {parser, strip_prefix;
		"0" => MinecraftColour::Black,
		"1" => MinecraftColour::DarkBlue,
		"2" => MinecraftColour::DarkGreen,
		"3" => MinecraftColour::DarkAqua,
		"4" => MinecraftColour::DarkRed,
		"5" => MinecraftColour::DarkPurple,
		"6" => MinecraftColour::Gold,
		"7" => MinecraftColour::Gray,
		"8" => MinecraftColour::DarkGray,
		"9" => MinecraftColour::Blue,
		"a" => MinecraftColour::Green,
		"b" => MinecraftColour::Aqua,
		"c" => MinecraftColour::Red,
		"d" => MinecraftColour::LightPurple,
		"e" => MinecraftColour::Yellow,
		"f" => MinecraftColour::White,
		_ => return Err(parser.into_other_error(&"could not parse paint")),
	};

	Ok((paint, parser))
}
