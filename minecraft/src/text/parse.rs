use konst::{
	parsing::{ParseValueResult, Parser},
	try_, unwrap_ctx,
};

use super::MinecraftText;
use crate::{
	font::{self, parse_font},
	paint::{self, parse_paint},
};

pub const ESCAPE: &str = "§";

#[macro_export]
macro_rules! minecraft_text {
	($text: expr) => {
		$crate::text::parse::_const_parse_minecraft_strings($text)
	};
}

pub use minecraft_text;

pub const fn _const_parse_minecraft_strings<const LEN: usize>(
	string: &str,
) -> [MinecraftText<'_>; LEN] {
	unwrap_ctx!(const_parse_minecraft_strings::<LEN>(Parser::new(string))).0
}

const fn const_parse_minecraft_strings<const LEN: usize>(
	mut parser: Parser<'_>,
) -> ParseValueResult<'_, [MinecraftText<'_>; LEN]> {
	let mut font = font::MinecraftFont::Normal;
	let mut paint = paint::MinecraftPaint::White;
	let mut result = [MinecraftText {
		text: "",
		font,
		paint,
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
	prev_font: font::MinecraftFont,
	prev_paint: paint::MinecraftPaint,
) -> ParseValueResult<'_, MinecraftText<'_>> {
	if let Ok((paint, parser)) = parse_paint(parser) {
		let (text, parser) = unwrap_ctx!(parser.split(ESCAPE));

		Ok((
			MinecraftText {
				text,
				font: font::MinecraftFont::Normal,
				paint,
			},
			parser,
		))
	} else {
		match parse_font(parser) {
			Ok((font @ font::MinecraftFont::Normal, parser)) => {
				let (text, parser) = unwrap_ctx!(parser.split(ESCAPE));

				Ok((
					MinecraftText {
						text,
						font,
						paint: paint::MinecraftPaint::White,
					},
					parser,
				))
			}
			Ok((font, parser)) => {
				let (text, parser) = unwrap_ctx!(parser.split(ESCAPE));

				Ok((
					MinecraftText {
						text,
						font,
						paint: prev_paint,
					},
					parser,
				))
			}
			Err(_) => {
				let (text, parser) = unwrap_ctx!(parser.split(ESCAPE));

				Ok((
					MinecraftText {
						text,
						font: prev_font,
						paint: prev_paint,
					},
					parser,
				))
			}
		}
	}
}

pub fn parse_minecraft_string(text: &str) -> impl Iterator<Item = MinecraftText<'_>> {
	let mut prev_paint = paint::MinecraftPaint::White;
	let mut prev_font = font::MinecraftFont::Normal;
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
			// When the colour changes, the text effects are reset
			prev_font = font::MinecraftFont::Normal;

			(paint, prev_font)
		} else {
			match font::MinecraftFont::try_from(hex) {
				Ok(font @ font::MinecraftFont::Normal) => {
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

		Some(MinecraftText { text, paint, font })
	})
}

pub fn draw_minecraft_string(surface: &mut skia_safe::Surface, string: &str, size: f32) {
	let mut x = 0.0;
	let y = 0.0;

	for text in parse_minecraft_string(string) {
		let blob = text.get_blob(size);

		if let Some(blob) = blob {
			surface
				.canvas()
				.draw_text_blob(&blob, (x, y), text.paint.into());

			x += blob.bounds().width();
		}
	}
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
			vec![MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::White,
				font: font::MinecraftFont::Normal,
			}]
		);
	}

	#[test]
	fn test_colour_string() {
		let text = "§cHello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::Red,
				font: font::MinecraftFont::Normal,
			}]
		);
	}

	#[test]
	fn test_font_string() {
		let text = "§lHello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::White,
				font: font::MinecraftFont::Bold,
			}]
		);
	}

	#[test]
	fn test_colour_font_string() {
		let text = "§c§lHello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::Red,
				font: font::MinecraftFont::Bold,
			}]
		);
	}

	#[test]
	fn test_font_reset_string() {
		let text = "§c§l§dHello, world!";
		let parsed = parse_minecraft_string(text).collect::<Vec<_>>();

		assert_eq!(
			parsed,
			vec![MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::LightPurple,
				font: font::MinecraftFont::Normal,
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
				MinecraftText {
					text: "Hello, ",
					paint: paint::MinecraftPaint::Red,
					font: font::MinecraftFont::Normal,
				},
				MinecraftText {
					text: "world",
					paint: paint::MinecraftPaint::Red,
					font: font::MinecraftFont::Bold,
				},
				MinecraftText {
					text: "!",
					paint: paint::MinecraftPaint::White,
					font: font::MinecraftFont::Normal,
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
			[MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::White,
				font: font::MinecraftFont::Normal,
			}]
		);
	}

	#[test]
	fn test_colour_string_macro() {
		let parsed = minecraft_text!("§cHello, world!");

		assert_eq!(
			parsed,
			[MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::Red,
				font: font::MinecraftFont::Normal,
			}]
		);
	}

	#[test]
	fn test_font_string_macro() {
		let parsed = minecraft_text!("§lHello, world!");

		assert_eq!(
			parsed,
			[MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::White,
				font: font::MinecraftFont::Bold,
			}]
		);
	}

	#[test]
	fn test_colour_font_string_macro() {
		let parsed = minecraft_text!("§c§lHello, world!");

		assert_eq!(
			parsed,
			[MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::Red,
				font: font::MinecraftFont::Bold,
			}]
		);
	}

	#[test]
	fn test_font_reset_string_macro() {
		let parsed = minecraft_text!("§c§l§dHello, world!");

		assert_eq!(
			parsed,
			[MinecraftText {
				text: "Hello, world!",
				paint: paint::MinecraftPaint::LightPurple,
				font: font::MinecraftFont::Normal,
			}]
		);
	}

	#[test]
	fn test_reset_string_macro() {
		let parsed = minecraft_text!("§cHello, §lworld§r!");

		assert_eq!(
			parsed,
			[
				MinecraftText {
					text: "Hello, ",
					paint: paint::MinecraftPaint::Red,
					font: font::MinecraftFont::Normal,
				},
				MinecraftText {
					text: "world",
					paint: paint::MinecraftPaint::Red,
					font: font::MinecraftFont::Bold,
				},
				MinecraftText {
					text: "!",
					paint: paint::MinecraftPaint::White,
					font: font::MinecraftFont::Normal,
				},
			]
		);
	}
}
