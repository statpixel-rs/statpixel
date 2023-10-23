use konst::{
	parsing::{ParseValueResult, Parser},
	try_, unwrap_ctx,
};

use super::Text;
use crate::{
	paint::{self, parse},
	style::{self, parse_font},
	text::DEFAULT_TEXT,
};

pub const ESCAPE: char = '§';

#[must_use]
pub const fn minecraft_text<const LEN: usize>(text: &str) -> [Text<'_>; LEN] {
	_const_parse_minecraft_strings(text)
}

const fn _const_parse_minecraft_strings<const LEN: usize>(string: &str) -> [Text<'_>; LEN] {
	unwrap_ctx!(const_minecraft_strings::<LEN>(Parser::new(string))).0
}

const fn const_minecraft_strings<const LEN: usize>(
	mut parser: Parser<'_>,
) -> ParseValueResult<'_, [Text<'_>; LEN]> {
	let mut font = style::MinecraftFont::Normal;
	let mut paint = paint::Paint::White;
	let mut result = [DEFAULT_TEXT; LEN];

	try_!(parser.find_skip(ESCAPE));

	let mut i = 0;

	while i < LEN {
		(result[i], parser) = try_!(const_minecraft_string(parser, font, paint));

		font = result[i].font;
		paint = result[i].paint;

		if !result[i].text.is_empty() {
			i += 1;
		}
	}

	Ok((result, parser))
}

#[allow(clippy::unnecessary_wraps)]
const fn const_minecraft_string(
	parser: Parser<'_>,
	prev_font: style::MinecraftFont,
	prev_paint: paint::Paint,
) -> ParseValueResult<'_, Text<'_>> {
	if let Ok((paint, parser)) = parse(parser) {
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
						paint: paint::Paint::White,
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

pub fn minecraft_string(text: &str) -> impl Iterator<Item = Text<'_>> {
	let mut prev_paint = paint::Paint::White;
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
			chars.next()?
		};

		let (paint, font) = if let Ok(paint) = paint::Paint::try_from(hex) {
			prev_paint = paint;
			// When the paint changes, the text effects are reset
			prev_font = style::MinecraftFont::Normal;

			(paint, prev_font)
		} else {
			match style::MinecraftFont::try_from(hex) {
				Ok(font @ style::MinecraftFont::Normal) => {
					prev_font = font;
					prev_paint = paint::Paint::White;

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
		let parsed = minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::Paint::White,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_paint_string() {
		let text = "§cHello, world!";
		let parsed = minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::Paint::Red,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_font_string() {
		let text = "§lHello, world!";
		let parsed = minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::Paint::White,
				font: style::MinecraftFont::Bold,
				size: None,
			}]
		);
	}

	#[test]
	fn test_paint_font_string() {
		let text = "§c§lHello, world!";
		let parsed = minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::Paint::Red,
				font: style::MinecraftFont::Bold,
				size: None,
			}]
		);
	}

	#[test]
	fn test_font_reset_string() {
		let text = "§c§l§dHello, world!";
		let parsed = minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![Text {
				text: "Hello, world!",
				paint: paint::Paint::LightPurple,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_reset_string() {
		let text = "§cHello, §lworld§r!";
		let parsed = minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![
				Text {
					text: "Hello, ",
					paint: paint::Paint::Red,
					font: style::MinecraftFont::Normal,
					size: None,
				},
				Text {
					text: "world",
					paint: paint::Paint::Red,
					font: style::MinecraftFont::Bold,
					size: None,
				},
				Text {
					text: "!",
					paint: paint::Paint::White,
					font: style::MinecraftFont::Normal,
					size: None,
				},
			]
		);
	}

	// Macro tests
	#[test]
	#[should_panic(expected = "expected `§`")]
	fn test_plain_string_macro() {
		let _: [Text; 1] = minecraft_text("Hello, world!");
	}

	#[test]
	fn test_paint_string_macro() {
		let parsed = minecraft_text("§cHello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::Paint::Red,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_font_string_macro() {
		let parsed = minecraft_text("§lHello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::Paint::White,
				font: style::MinecraftFont::Bold,
				size: None,
			}]
		);
	}

	#[test]
	fn test_paint_font_string_macro() {
		let parsed = minecraft_text("§c§lHello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::Paint::Red,
				font: style::MinecraftFont::Bold,
				size: None,
			}]
		);
	}

	#[test]
	fn test_font_reset_string_macro() {
		let parsed = minecraft_text("§c§l§dHello, world!");

		assert_eq!(
			parsed,
			[Text {
				text: "Hello, world!",
				paint: paint::Paint::LightPurple,
				font: style::MinecraftFont::Normal,
				size: None,
			}]
		);
	}

	#[test]
	fn test_reset_string_macro() {
		let parsed = minecraft_text("§cHello, §lworld§r!");

		assert_eq!(
			parsed,
			[
				Text {
					text: "Hello, ",
					paint: paint::Paint::Red,
					font: style::MinecraftFont::Normal,
					size: None,
				},
				Text {
					text: "world",
					paint: paint::Paint::Red,
					font: style::MinecraftFont::Bold,
					size: None,
				},
				Text {
					text: "!",
					paint: paint::Paint::White,
					font: style::MinecraftFont::Normal,
					size: None,
				},
			]
		);
	}
}
