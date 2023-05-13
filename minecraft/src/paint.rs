use darling::FromMeta;
use konst::{parser_method, parsing::ParseValueResult, Parser};
use once_cell::sync::Lazy;
use quote::quote;

macro_rules! paint_colour {
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
paint_colour!(BLACK, (255, 0, 0, 0));
paint_colour!(DARK_BLUE, (255, 0, 0, 170));
paint_colour!(DARK_GREEN, (255, 0, 170, 0));
paint_colour!(DARK_AQUA, (255, 0, 170, 170));
paint_colour!(DARK_RED, (255, 170, 0, 0));
paint_colour!(DARK_PURPLE, (255, 170, 0, 170));
paint_colour!(GOLD, (255, 255, 170, 0));
paint_colour!(GRAY, (255, 170, 170, 170));
paint_colour!(DARK_GRAY, (255, 85, 85, 85));
paint_colour!(BLUE, (255, 85, 85, 255));
paint_colour!(GREEN, (255, 85, 255, 85));
paint_colour!(AQUA, (255, 85, 255, 255));
paint_colour!(RED, (255, 255, 85, 85));
paint_colour!(LIGHT_PURPLE, (255, 255, 85, 255));
paint_colour!(YELLOW, (255, 255, 255, 85));
paint_colour!(WHITE, (255, 255, 255, 255));

// Utility colours
paint_colour!(BACKGROUND, (255, 21, 33, 43));
paint_colour!(CANVAS_BACKGROUND, (255, 31, 48, 64));

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, FromMeta)]
#[darling(default)]
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
