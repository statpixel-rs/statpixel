use konst::{parser_method, parsing::ParseValueResult, Parser};
use once_cell::sync::Lazy;

macro_rules! paint_colour {
	($name: ident, $colour: expr) => {
		pub static $name: Lazy<skia_safe::Paint> = Lazy::new(|| {
			let mut paint = skia_safe::Paint::default();

			paint
				.set_color(skia_safe::Color::from_rgb($colour.0, $colour.1, $colour.2))
				.set_anti_alias(false);

			paint
		});
	};
}

// Minecraft `Paint` colours
paint_colour!(BLACK, (0, 0, 0));
paint_colour!(DARK_BLUE, (0, 0, 170));
paint_colour!(DARK_GREEN, (0, 170, 0));
paint_colour!(DARK_AQUA, (0, 170, 170));
paint_colour!(DARK_RED, (170, 0, 0));
paint_colour!(DARK_PURPLE, (170, 0, 170));
paint_colour!(GOLD, (255, 170, 0));
paint_colour!(GRAY, (170, 170, 170));
paint_colour!(DARK_GRAY, (85, 85, 85));
paint_colour!(BLUE, (85, 85, 255));
paint_colour!(GREEN, (85, 255, 85));
paint_colour!(AQUA, (85, 255, 255));
paint_colour!(RED, (255, 85, 85));
paint_colour!(LIGHT_PURPLE, (255, 85, 255));
paint_colour!(YELLOW, (255, 255, 85));
paint_colour!(WHITE, (255, 255, 255));

// Utility colours
// paint_colour!(BACKGROUND, (31, 48, 64));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MinecraftPaint {
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

impl From<MinecraftPaint> for &skia_safe::Paint {
	fn from(colour: MinecraftPaint) -> Self {
		match colour {
			MinecraftPaint::Black => &BLACK,
			MinecraftPaint::DarkBlue => &DARK_BLUE,
			MinecraftPaint::DarkGreen => &DARK_GREEN,
			MinecraftPaint::DarkAqua => &DARK_AQUA,
			MinecraftPaint::DarkRed => &DARK_RED,
			MinecraftPaint::DarkPurple => &DARK_PURPLE,
			MinecraftPaint::Gold => &GOLD,
			MinecraftPaint::Gray => &GRAY,
			MinecraftPaint::DarkGray => &DARK_GRAY,
			MinecraftPaint::Blue => &BLUE,
			MinecraftPaint::Green => &GREEN,
			MinecraftPaint::Aqua => &AQUA,
			MinecraftPaint::Red => &RED,
			MinecraftPaint::LightPurple => &LIGHT_PURPLE,
			MinecraftPaint::Yellow => &YELLOW,
			MinecraftPaint::White => &WHITE,
		}
	}
}

impl TryFrom<char> for MinecraftPaint {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'0' => Ok(MinecraftPaint::Black),
			'1' => Ok(MinecraftPaint::DarkBlue),
			'2' => Ok(MinecraftPaint::DarkGreen),
			'3' => Ok(MinecraftPaint::DarkAqua),
			'4' => Ok(MinecraftPaint::DarkRed),
			'5' => Ok(MinecraftPaint::DarkPurple),
			'6' => Ok(MinecraftPaint::Gold),
			'7' => Ok(MinecraftPaint::Gray),
			'8' => Ok(MinecraftPaint::DarkGray),
			'9' => Ok(MinecraftPaint::Blue),
			'a' | 'A' => Ok(MinecraftPaint::Green),
			'b' | 'B' => Ok(MinecraftPaint::Aqua),
			'c' | 'C' => Ok(MinecraftPaint::Red),
			'd' | 'D' => Ok(MinecraftPaint::LightPurple),
			'e' | 'E' => Ok(MinecraftPaint::Yellow),
			'f' | 'F' => Ok(MinecraftPaint::White),
			_ => Err(()),
		}
	}
}

impl TryFrom<&str> for MinecraftPaint {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"BLACK" => Ok(MinecraftPaint::Black),
			"DARK_BLUE" => Ok(MinecraftPaint::DarkBlue),
			"DARK_GREEN" => Ok(MinecraftPaint::DarkGreen),
			"DARK_AQUA" => Ok(MinecraftPaint::DarkAqua),
			"DARK_RED" => Ok(MinecraftPaint::DarkRed),
			"DARK_PURPLE" => Ok(MinecraftPaint::DarkPurple),
			"GOLD" => Ok(MinecraftPaint::Gold),
			"GRAY" => Ok(MinecraftPaint::Gray),
			"DARK_GRAY" => Ok(MinecraftPaint::DarkGray),
			"BLUE" => Ok(MinecraftPaint::Blue),
			"GREEN" => Ok(MinecraftPaint::Green),
			"AQUA" => Ok(MinecraftPaint::Aqua),
			"RED" => Ok(MinecraftPaint::Red),
			"LIGHT_PURPLE" => Ok(MinecraftPaint::LightPurple),
			"YELLOW" => Ok(MinecraftPaint::Yellow),
			"WHITE" => Ok(MinecraftPaint::White),
			_ => Err(()),
		}
	}
}

pub const fn parse_paint(mut parser: Parser<'_>) -> ParseValueResult<'_, MinecraftPaint> {
	let paint = parser_method! {parser, strip_prefix;
		"0" => MinecraftPaint::Black,
		"1" => MinecraftPaint::DarkBlue,
		"2" => MinecraftPaint::DarkGreen,
		"3" => MinecraftPaint::DarkAqua,
		"4" => MinecraftPaint::DarkRed,
		"5" => MinecraftPaint::DarkPurple,
		"6" => MinecraftPaint::Gold,
		"7" => MinecraftPaint::Gray,
		"8" => MinecraftPaint::DarkGray,
		"9" => MinecraftPaint::Blue,
		"a" => MinecraftPaint::Green,
		"b" => MinecraftPaint::Aqua,
		"c" => MinecraftPaint::Red,
		"d" => MinecraftPaint::LightPurple,
		"e" => MinecraftPaint::Yellow,
		"f" => MinecraftPaint::White,
		_ => return Err(parser.into_other_error(&"could not parse paint")),
	};

	Ok((paint, parser))
}
