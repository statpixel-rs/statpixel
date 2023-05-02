use konst::{
	parsing::{ParseValueResult, Parser},
	try_, unwrap_ctx,
};

use super::Text;
use crate::{
	paint::{self, parse_paint},
	style::{self, parse_font},
};

pub const ESCAPE: char = '§';

#[macro_export]
macro_rules! minecraft_text {
	($text: expr) => {
		$crate::text::parse::_const_parse_minecraft_strings($text)
	};
}

pub use minecraft_text;

pub const fn _const_parse_minecraft_strings<const LEN: usize>(string: &str) -> [Text<'_>; LEN] {
	unwrap_ctx!(const_parse_minecraft_strings::<LEN>(Parser::new(string))).0
}

const fn const_parse_minecraft_strings<const LEN: usize>(
	mut parser: Parser<'_>,
) -> ParseValueResult<'_, [Text<'_>; LEN]> {
	let mut font = style::MinecraftFont::Normal;
	let mut paint = paint::MinecraftPaint::White;
	let mut result = [Text {
		text: "",
		font: style::MinecraftFont::Normal,
		paint: paint::MinecraftPaint::White,
		size: None,
	}; LEN];

	try_!(parser.find_skip(ESCAPE));

	let mut i = 0;

	while i < LEN {
		(result[i], parser) = try_!(const_parse_minecraft_string(parser, font, paint));

		font = result[i].font;
		paint = result[i].paint;

		if !result[i].text.is_empty() {
			i += 1;
		}
	}

	Ok((result, parser))
}

const fn const_parse_minecraft_string(
	parser: Parser<'_>,
	prev_font: style::MinecraftFont,
	prev_paint: paint::MinecraftPaint,
) -> ParseValueResult<'_, Text<'_>> {
	if let Ok((paint, parser)) = parse_paint(parser) {
		let (text, parser) = unwrap_ctx!(parser.split(ESCAPE));

		Ok((
			Text {
				text,
				font: style::MinecraftFont::Normal,
				paint,
				size: None,
			},
			parser,
		))
	} else {
		match parse_font(parser) {
			Ok((font @ style::MinecraftFont::Normal, parser)) => {
				let (text, parser) = unwrap_ctx!(parser.split(ESCAPE));

				Ok((
					Text {
						text,
						font,
						paint: paint::MinecraftPaint::White,
						size: None,
					},
					parser,
				))
			}
			Ok((font, parser)) => {
				let (text, parser) = unwrap_ctx!(parser.split(ESCAPE));

				Ok((
					Text {
						text,
						font,
						paint: prev_paint,
						size: None,
					},
					parser,
				))
			}
			Err(_) => {
				let (text, parser) = unwrap_ctx!(parser.split(ESCAPE));

				Ok((
					Text {
						text,
						font: prev_font,
						paint: prev_paint,
						size: None,
					},
					parser,
				))
			}
		}
	}
}

pub fn parse_minecraft_string(text: &str) -> impl Iterator<Item = Text<'_>> {
	let mut prev_paint = paint::MinecraftPaint::White;
	let mut prev_font = style::MinecraftFont::Normal;
	let mut first = true;

	if text.starts_with(ESCAPE) {
		first = false;
	}

	text.split(ESCAPE).filter_map(move |s| {
		if s.is_empty() {
			return None;
		}

		let mut chars = s.chars();
		let hex = if first {
			first = false;

			'f'
		} else {
			chars.next().unwrap()
		};

		let (paint, font) = if let Ok(paint) = paint::MinecraftPaint::try_from(hex) {
			prev_paint = paint;
			// When the paint changes, the text effects are reset
			prev_font = style::MinecraftFont::Normal;

			(paint, prev_font)
		} else {
			match style::MinecraftFont::try_from(hex) {
				Ok(font @ style::MinecraftFont::Normal) => {
					prev_font = font;
					prev_paint = paint::MinecraftPaint::White;

					(prev_paint, font)
				}
				Ok(font) => {
					prev_font = font;

					(prev_paint, prev_font)
				}
				Err(_) => (prev_paint, prev_font),
			}
		};

		let text = chars.as_str();

		if text.is_empty() {
			return None;
		}

		Some(Text {
			text,
			paint,
			font,
			size: None,
		})
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_plain_string() {
		let text = "Hello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::White,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_paint_string() {
		let text = "§cHello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::Red,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_font_string() {
		let text = "§lHello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::White,
				font: style::MinecraftFont::Bold,
				size: None,
			}]
		);
	}

	#[test]
	fn test_paint_font_string() {
		let text = "§c§lHello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::Red,
				font: style::MinecraftFont::Bold,
				size: None,
			}]
		);
	}

	#[test]
	fn test_font_reset_string() {
		let text = "§c§l§dHello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::LightPurple,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_reset_string() {
		let text = "§cHello, §lworld§r!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![
				Text {
					text: "Hello, ",
					paint: paint::MinecraftPaint::Red,
					font: style::MinecraftFont::Normal,
					size: None,
				},
				Text {
					text: "world",
					paint: paint::MinecraftPaint::Red,
					font: style::MinecraftFont::Bold,
					size: None,
				},
				Text {
					text: "!",
					paint: paint::MinecraftPaint::White,
					font: style::MinecraftFont::Normal,
					size: None,
				},
			]
		);
	}

	// Macro tests
	#[test]
	fn test_plain_string_macro() {
		let parsed = minecraft_text!("Hello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::White,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_paint_string_macro() {
		let parsed = minecraft_text!("§cHello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::Red,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_font_string_macro() {
		let parsed = minecraft_text!("§lHello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::White,
				font: style::MinecraftFont::Bold,
				size: None,
			}]
		);
	}

	#[test]
	fn test_paint_font_string_macro() {
		let parsed = minecraft_text!("§c§lHello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::Red,
				font: style::MinecraftFont::Bold,
				size: None,
			}]
		);
	}

	#[test]
	fn test_font_reset_string_macro() {
		let parsed = minecraft_text!("§c§l§dHello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::LightPurple,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_reset_string_macro() {
		let parsed = minecraft_text!("§cHello, §lworld§r!");

		assert_eq!(
			parsed,
			[
				Text {
					text: "Hello, ",
					paint: paint::MinecraftPaint::Red,
					font: style::MinecraftFont::Normal,
					size: None,
				},
				Text {
					text: "world",
					paint: paint::MinecraftPaint::Red,
					font: style::MinecraftFont::Bold,
					size: None,
				},
				Text {
					text: "!",
					paint: paint::MinecraftPaint::White,
					font: style::MinecraftFont::Normal,
					size: None,
				},
			]
		);
	}
}
