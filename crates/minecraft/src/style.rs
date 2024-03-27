#[cfg(feature = "diesel")]
use diesel::{
	backend::Backend,
	deserialize::FromSql,
	pg::Pg,
	serialize::{self, Output, ToSql},
	sql_types::SmallInt,
};
use konst::{parser_method, parsing::ParseValueResult, Parser};
#[cfg(feature = "skia")]
use once_cell::sync::Lazy;
#[cfg(feature = "skia")]
use skia_safe::{
	textlayout::{TextShadow, TextStyle},
	FontStyle,
};

#[cfg(feature = "skia")]
fn style_with_name(name: &str, font: FontStyle) -> TextStyle {
	let mut style = TextStyle::new();

	style.set_font_families(&[name]);
	style.set_font_style(font);

	style
}

#[cfg(feature = "skia")]
pub static MINECRAFT_NORMAL: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Minecraft", FontStyle::normal()));

#[cfg(feature = "skia")]
pub static MINECRAFT_BOLD: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Minecraft", FontStyle::bold()));

#[cfg(feature = "skia")]
pub static MINECRAFT_ITALIC: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Minecraft", FontStyle::italic()));

#[cfg(feature = "skia")]
pub static MINECRAFT_BOLD_ITALIC: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Minecraft", FontStyle::bold_italic()));

#[cfg(feature = "skia")]
pub static FAITHFUL_NORMAL: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Faithful 32x", FontStyle::normal()));

#[cfg(feature = "skia")]
pub static FAITHFUL_BOLD: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Faithful 32x", FontStyle::bold()));

#[cfg(feature = "skia")]
pub static FAITHFUL_ITALIC: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Faithful 32x", FontStyle::italic()));

#[cfg(feature = "skia")]
pub static FAITHFUL_BOLD_ITALIC: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Faithful 32x", FontStyle::bold_italic()));

#[cfg(feature = "skia")]
pub static ROBOTO_NORMAL: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Roboto", FontStyle::normal()));

#[cfg(feature = "skia")]
pub static ROBOTO_BOLD: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Roboto", FontStyle::bold()));

#[cfg(feature = "skia")]
pub static ROBOTO_ITALIC: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Roboto", FontStyle::italic()));

#[cfg(feature = "skia")]
pub static ROBOTO_BOLD_ITALIC: Lazy<TextStyle> =
	Lazy::new(|| style_with_name("Roboto", FontStyle::bold_italic()));

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MinecraftFont {
	Normal,
	Bold,
	Italic,
	BoldItalic,
}

#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::AsExpression, diesel::FromSqlRow), diesel(sql_type = SmallInt))]
pub enum Family {
	#[cfg_attr(feature = "serde", serde(rename = "minecraft"))]
	Minecraft,
	#[default]
	#[cfg_attr(feature = "serde", serde(rename = "faithful"))]
	Faithful,
	#[cfg_attr(feature = "serde", serde(rename = "roboto"))]
	Roboto,
}

#[cfg(feature = "diesel")]
impl FromSql<SmallInt, Pg> for Family {
	fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
		Ok(match i16::from_sql(bytes)? {
			0 => Family::Minecraft,
			1 => Family::Faithful,
			2 => Family::Roboto,
			_ => unreachable!(),
		})
	}
}

#[cfg(feature = "diesel")]
impl<Db> ToSql<SmallInt, Db> for Family
where
	Db: Backend,
	i16: ToSql<SmallInt, Db>,
{
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Db>) -> serialize::Result {
		match self {
			Family::Minecraft => 0.to_sql(out),
			Family::Faithful => 1.to_sql(out),
			Family::Roboto => 2.to_sql(out),
		}
	}
}

impl Family {
	#[must_use]
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Minecraft => "Minecraft",
			Self::Faithful => "Faithful 32x",
			Self::Roboto => "Roboto",
		}
	}
}

impl MinecraftFont {
	#[cfg(feature = "skia")]
	pub fn get_style(&self, family: Family, paint: super::paint::Paint, size: f32) -> TextStyle {
		let mut style = match family {
			Family::Minecraft => match self {
				Self::Normal => MINECRAFT_NORMAL.clone(),
				Self::Bold => MINECRAFT_BOLD.clone(),
				Self::Italic => MINECRAFT_ITALIC.clone(),
				Self::BoldItalic => MINECRAFT_BOLD_ITALIC.clone(),
			},
			Family::Faithful => match self {
				Self::Normal => FAITHFUL_NORMAL.clone(),
				Self::Bold => FAITHFUL_BOLD.clone(),
				Self::Italic => FAITHFUL_ITALIC.clone(),
				Self::BoldItalic => FAITHFUL_BOLD_ITALIC.clone(),
			},
			Family::Roboto => match self {
				Self::Normal => ROBOTO_NORMAL.clone(),
				Self::Bold => ROBOTO_BOLD.clone(),
				Self::Italic => ROBOTO_ITALIC.clone(),
				Self::BoldItalic => ROBOTO_BOLD_ITALIC.clone(),
			},
		};

		#[allow(clippy::cast_possible_truncation)]
		let offset = (size / 9.) as i32;

		style.add_shadow(TextShadow::new(paint.shadow(), (offset, offset), 0.));
		style.set_foreground_paint(paint.into());
		style
	}
}

impl From<char> for MinecraftFont {
	fn from(c: char) -> Self {
		match c {
			'l' | 'L' => Self::Bold,
			'o' | 'O' => Self::Italic,
			_ => Self::Normal,
		}
	}
}

/// # Errors
/// Returns an error if the char is not a valid font modifier (r, l, o)
pub const fn parse_font(mut parser: Parser<'_>) -> ParseValueResult<'_, MinecraftFont> {
	let font = parser_method! {parser, strip_prefix;
		"r" => MinecraftFont::Normal,
		"l" => MinecraftFont::Bold,
		"o" => MinecraftFont::Italic,
		_ => return Err(parser.into_other_error(&"could not parse font")),
	};

	Ok((font, parser))
}

#[cfg(test)]
mod tests {
	use std::assert_matches::assert_matches;

	use super::*;

	#[test]
	fn test_const_parse_font() {
		assert_matches!(parse_font(Parser::new("r")), Ok((MinecraftFont::Normal, _)));
		assert_matches!(parse_font(Parser::new("l")), Ok((MinecraftFont::Bold, _)));
		assert_matches!(parse_font(Parser::new("o")), Ok((MinecraftFont::Italic, _)));
		assert_matches!(parse_font(Parser::new("")), Err(_));
	}

	#[test]
	fn test_font_from_char() {
		assert_eq!(MinecraftFont::from('r'), MinecraftFont::Normal);
		assert_eq!(MinecraftFont::from('l'), MinecraftFont::Bold);
		assert_eq!(MinecraftFont::from('o'), MinecraftFont::Italic);
	}
}
