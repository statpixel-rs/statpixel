use std::{borrow::Cow, mem::ManuallyDrop};

use super::{Text, ESCAPE};

use crate::{
	paint::{self, parse},
	style::{self, parse_font},
	text::{parse::const_minecraft_string, DEFAULT_TEXT},
};
use konst::{
	parsing::{ParseValueResult, Parser},
	try_, unwrap_ctx,
};

pub const fn const_minecraft_strings_1(
	mut parser: Parser<'_>,
) -> ParseValueResult<'_, [Text<'_>; 1]> {
	let mut font = style::MinecraftFont::Normal;
	let mut paint = paint::Paint::White;
	let mut result: [Text; 1] = [DEFAULT_TEXT];

	try_!(parser.find_skip(ESCAPE));

	let mut i = 0;

	while i < 1 {
		(result[i], parser) = try_!(const_minecraft_string(parser, font, paint));

		font = result[i].font;
		paint = result[i].paint;

		match result[i].text {
			Cow::Borrowed(s) => {
				if !s.is_empty() {
					result[i] = Text {
						text: Cow::Borrowed(s),
						font,
						paint,
						size: None,
					};

					i += 1;
				}
			}
			_ => {
				panic!("minecraft_text! should not create owned strings")
			}
		}
	}

	Ok((result, parser))
}

pub const fn const_minecraft_strings_2(
	mut parser: Parser<'_>,
) -> ParseValueResult<'_, [Text<'_>; 2]> {
	let mut font = style::MinecraftFont::Normal;
	let mut paint = paint::Paint::White;
	let mut result: [Text; 2] = [DEFAULT_TEXT, DEFAULT_TEXT];

	try_!(parser.find_skip(ESCAPE));

	let mut i = 0;

	while i < 1 {
		(result[i], parser) = try_!(const_minecraft_string(parser, font, paint));

		font = result[i].font;
		paint = result[i].paint;

		match result[i].text {
			Cow::Borrowed(s) => {
				if !s.is_empty() {
					result[i] = Text {
						text: Cow::Borrowed(s),
						font,
						paint,
						size: None,
					};

					i += 1;
				}
			}
			_ => {
				panic!("minecraft_text! should not create owned strings")
			}
		}
	}

	Ok((result, parser))
}

pub const fn const_minecraft_strings_3(
	mut parser: Parser<'_>,
) -> ParseValueResult<'_, [Text<'_>; 3]> {
	let mut font = style::MinecraftFont::Normal;
	let mut paint = paint::Paint::White;
	let mut result: [Text; 3] = [DEFAULT_TEXT, DEFAULT_TEXT, DEFAULT_TEXT];

	try_!(parser.find_skip(ESCAPE));

	let mut i = 0;

	while i < 1 {
		(result[i], parser) = try_!(const_minecraft_string(parser, font, paint));

		font = result[i].font;
		paint = result[i].paint;

		match result[i].text {
			Cow::Borrowed(s) => {
				if !s.is_empty() {
					result[i] = Text {
						text: Cow::Borrowed(s),
						font,
						paint,
						size: None,
					};

					i += 1;
				}
			}
			_ => {
				panic!("minecraft_text! should not create owned strings")
			}
		}
	}

	Ok((result, parser))
}

pub const fn const_minecraft_strings_4(
	mut parser: Parser<'_>,
) -> ParseValueResult<'_, [Text<'_>; 4]> {
	let mut font = style::MinecraftFont::Normal;
	let mut paint = paint::Paint::White;
	let mut result: [Text; 4] = [DEFAULT_TEXT, DEFAULT_TEXT, DEFAULT_TEXT, DEFAULT_TEXT];

	try_!(parser.find_skip(ESCAPE));

	let mut i = 0;

	while i < 1 {
		(result[i], parser) = try_!(const_minecraft_string(parser, font, paint));

		font = result[i].font;
		paint = result[i].paint;

		match result[i].text {
			Cow::Borrowed(s) => {
				if !s.is_empty() {
					result[i] = Text {
						text: Cow::Borrowed(s),
						font,
						paint,
						size: None,
					};

					i += 1;
				}
			}
			_ => {
				panic!("minecraft_text! should not create owned strings")
			}
		}
	}

	Ok((result, parser))
}
