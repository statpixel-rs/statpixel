use std::borrow::Cow;

use super::{parse::minecraft_text, parse::ESCAPE, Text};
use crate::{colour::Colour, paint::Paint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rank {
	Default,
	Vip,
	VipPlus,
	Mvp,
	MvpPlus(Colour),
	MvpPlusPlus(Colour, bool),
	YouTube,
	Mojang,
	Events,
	Mcp,
	Gm,
	Admin,
	Owner,
	Custom(String),
}

macro_rules! mvp_plus {
	($colour: expr) => {
		minecraft_text(concat!("§b[MVP§", $colour, "+§b]"))
	};
}

macro_rules! mvp_plus_plus {
	($colour: expr, $plain: expr) => {
		if $plain {
			minecraft_text(concat!("§b[MVP§", $colour, "++§b]"))
		} else {
			minecraft_text(concat!("§6[MVP§", $colour, "++§6]"))
		}
	};
}

const VIP: [Text; 1] = minecraft_text("§a[VIP]");
const VIP_PLUS: [Text; 3] = minecraft_text("§a[VIP§6+§a]");
const MVP: [Text; 1] = minecraft_text("§b[MVP]");

const MVP_PLUS_BLACK: [Text; 3] = mvp_plus!("0");
const MVP_PLUS_DARK_BLUE: [Text; 3] = mvp_plus!("1");
const MVP_PLUS_DARK_GREEN: [Text; 3] = mvp_plus!("2");
const MVP_PLUS_DARK_AQUA: [Text; 3] = mvp_plus!("3");
const MVP_PLUS_DARK_RED: [Text; 3] = mvp_plus!("4");
const MVP_PLUS_DARK_PURPLE: [Text; 3] = mvp_plus!("5");
const MVP_PLUS_GOLD: [Text; 3] = mvp_plus!("6");
const MVP_PLUS_GRAY: [Text; 3] = mvp_plus!("7");
const MVP_PLUS_DARK_GRAY: [Text; 3] = mvp_plus!("8");
const MVP_PLUS_BLUE: [Text; 3] = mvp_plus!("9");
const MVP_PLUS_GREEN: [Text; 3] = mvp_plus!("a");
const MVP_PLUS_AQUA: [Text; 3] = mvp_plus!("b");
const MVP_PLUS_RED: [Text; 3] = mvp_plus!("c");
const MVP_PLUS_LIGHT_PURPLE: [Text; 3] = mvp_plus!("d");
const MVP_PLUS_YELLOW: [Text; 3] = mvp_plus!("e");
const MVP_PLUS_WHITE: [Text; 3] = mvp_plus!("f");

const MVP_PLUS_PLUS_BLACK: [Text; 3] = mvp_plus_plus!("0", false);
const MVP_PLUS_PLUS_DARK_BLUE: [Text; 3] = mvp_plus_plus!("1", false);
const MVP_PLUS_PLUS_DARK_GREEN: [Text; 3] = mvp_plus_plus!("2", false);
const MVP_PLUS_PLUS_DARK_AQUA: [Text; 3] = mvp_plus_plus!("3", false);
const MVP_PLUS_PLUS_DARK_RED: [Text; 3] = mvp_plus_plus!("4", false);
const MVP_PLUS_PLUS_DARK_PURPLE: [Text; 3] = mvp_plus_plus!("5", false);
const MVP_PLUS_PLUS_GOLD: [Text; 3] = mvp_plus_plus!("6", false);
const MVP_PLUS_PLUS_GRAY: [Text; 3] = mvp_plus_plus!("7", false);
const MVP_PLUS_PLUS_DARK_GRAY: [Text; 3] = mvp_plus_plus!("8", false);
const MVP_PLUS_PLUS_BLUE: [Text; 3] = mvp_plus_plus!("9", false);
const MVP_PLUS_PLUS_GREEN: [Text; 3] = mvp_plus_plus!("a", false);
const MVP_PLUS_PLUS_AQUA: [Text; 3] = mvp_plus_plus!("b", false);
const MVP_PLUS_PLUS_RED: [Text; 3] = mvp_plus_plus!("c", false);
const MVP_PLUS_PLUS_LIGHT_PURPLE: [Text; 3] = mvp_plus_plus!("d", false);
const MVP_PLUS_PLUS_YELLOW: [Text; 3] = mvp_plus_plus!("e", false);
const MVP_PLUS_PLUS_WHITE: [Text; 3] = mvp_plus_plus!("f", false);

const MVP_PLUS_PLUS_BLACK_PLAIN: [Text; 3] = mvp_plus_plus!("0", true);
const MVP_PLUS_PLUS_DARK_BLUE_PLAIN: [Text; 3] = mvp_plus_plus!("1", true);
const MVP_PLUS_PLUS_DARK_GREEN_PLAIN: [Text; 3] = mvp_plus_plus!("2", true);
const MVP_PLUS_PLUS_DARK_AQUA_PLAIN: [Text; 3] = mvp_plus_plus!("3", true);
const MVP_PLUS_PLUS_DARK_RED_PLAIN: [Text; 3] = mvp_plus_plus!("4", true);
const MVP_PLUS_PLUS_DARK_PURPLE_PLAIN: [Text; 3] = mvp_plus_plus!("5", true);
const MVP_PLUS_PLUS_GOLD_PLAIN: [Text; 3] = mvp_plus_plus!("6", true);
const MVP_PLUS_PLUS_GRAY_PLAIN: [Text; 3] = mvp_plus_plus!("7", true);
const MVP_PLUS_PLUS_DARK_GRAY_PLAIN: [Text; 3] = mvp_plus_plus!("8", true);
const MVP_PLUS_PLUS_BLUE_PLAIN: [Text; 3] = mvp_plus_plus!("9", true);
const MVP_PLUS_PLUS_GREEN_PLAIN: [Text; 3] = mvp_plus_plus!("a", true);
const MVP_PLUS_PLUS_AQUA_PLAIN: [Text; 3] = mvp_plus_plus!("b", true);
const MVP_PLUS_PLUS_RED_PLAIN: [Text; 3] = mvp_plus_plus!("c", true);
const MVP_PLUS_PLUS_LIGHT_PURPLE_PLAIN: [Text; 3] = mvp_plus_plus!("d", true);
const MVP_PLUS_PLUS_YELLOW_PLAIN: [Text; 3] = mvp_plus_plus!("e", true);
const MVP_PLUS_PLUS_WHITE_PLAIN: [Text; 3] = mvp_plus_plus!("f", true);

const YOUTUBE: [Text; 3] = minecraft_text("§c[§fYOUTUBE§c]");
const MOJANG: [Text; 1] = minecraft_text("§6[MOJANG]");
const EVENTS: [Text; 1] = minecraft_text("§6[EVENTS]");
const MCP: [Text; 1] = minecraft_text("§c[MCP]");
const GM: [Text; 1] = minecraft_text("§2[GM]");
const ADMIN: [Text; 1] = minecraft_text("§c[ADMIN]");
const OWNER: [Text; 1] = minecraft_text("§c[OWNER]");

fn remove_special_chars(text: &str) -> String {
	let mut result = String::with_capacity(text.len());
	let mut chars = text.chars();

	while let Some(c) = chars.next() {
		if c == '§' {
			chars.next();
		} else {
			result.push(c);
		}
	}

	result
}

impl Rank {
	#[must_use]
	pub fn from_str(
		rank: &str,
		package_rank: Option<&str>,
		colour: Option<Colour>,
		monthly_colour: Option<Colour>,
	) -> Self {
		match rank {
			"VIP" => Self::Vip,
			"VIP_PLUS" => Self::VipPlus,
			"MVP" => Self::Mvp,
			"MVP_PLUS" if package_rank == Some("SUPERSTAR") => Self::MvpPlusPlus(
				colour.unwrap_or(Colour::Red),
				monthly_colour.map_or(false, |c| c == Colour::Aqua),
			),
			"MVP_PLUS" => Self::MvpPlus(colour.unwrap_or(Colour::Red)),
			"YOUTUBER" => Self::YouTube,
			"MOJANG" => Self::Mojang,
			"EVENTS" => Self::Events,
			"MCP" => Self::Mcp,
			"GM" => Self::Gm,
			"ADMIN" => Self::Admin,
			"OWNER" => Self::Owner,
			_ => Self::Default,
		}
	}

	fn get_string_paint(string: &str) -> Paint {
		if let Some(index) = string.rfind(ESCAPE) {
			let char = string.char_indices().rfind(|(i, _)| *i == index + 2);
			let paint = char.and_then(|(_, c)| Paint::try_from(c).ok());

			paint.unwrap_or(Paint::Gray)
		} else {
			Paint::Gray
		}
	}

	#[must_use]
	pub fn get_username_paint(&self) -> Paint {
		match self {
			Self::Default => Paint::Gray,
			Self::Vip | Self::VipPlus => Paint::Green,
			Self::Mvp | Self::MvpPlus(_) | Self::MvpPlusPlus(_, true) => Paint::Aqua,
			Self::MvpPlusPlus(_, false) | Self::Mojang | Self::Events => Paint::Gold,
			Self::Gm => Paint::DarkGreen,
			Self::Admin | Self::Owner | Self::Mcp | Self::YouTube => Paint::Red,
			Self::Custom(prefix) => Self::get_string_paint(prefix),
		}
	}

	#[must_use]
	pub fn as_str(&self) -> Option<Cow<'_, str>> {
		Some(Cow::Borrowed(match self {
			Self::Default => return None,
			Self::Vip => "[VIP]",
			Self::VipPlus => "[VIP+]",
			Self::Mvp => "[MVP]",
			Self::MvpPlus(_) => "[MVP+]",
			Self::MvpPlusPlus(_, _) => "[MVP++]",
			Self::Mojang => "[MOJANG]",
			Self::Events => "[EVENTS]",
			Self::Mcp => "[MCP]",
			Self::Gm => "[GM]",
			Self::Admin => "[ADMIN]",
			Self::Owner => "[OWNER]",
			Self::YouTube => "[YOUTUBE]",
			Self::Custom(prefix) => return Some(Cow::Owned(remove_special_chars(prefix))),
		}))
	}

	/// `None` for `Rank::Custom`
	#[must_use]
	pub fn get_text(&self) -> Option<&'static [Text<'static>]> {
		match self {
			Self::Default => Some(&[]),
			Self::Vip => Some(&VIP),
			Self::VipPlus => Some(&VIP_PLUS),
			Self::Mvp => Some(&MVP),
			Self::MvpPlus(colour) => Some(match colour {
				Colour::Black => &MVP_PLUS_BLACK,
				Colour::DarkBlue => &MVP_PLUS_DARK_BLUE,
				Colour::DarkGreen => &MVP_PLUS_DARK_GREEN,
				Colour::DarkAqua => &MVP_PLUS_DARK_AQUA,
				Colour::DarkRed => &MVP_PLUS_DARK_RED,
				Colour::DarkPurple => &MVP_PLUS_DARK_PURPLE,
				Colour::Gold => &MVP_PLUS_GOLD,
				Colour::Gray => &MVP_PLUS_GRAY,
				Colour::DarkGray => &MVP_PLUS_DARK_GRAY,
				Colour::Blue => &MVP_PLUS_BLUE,
				Colour::Green => &MVP_PLUS_GREEN,
				Colour::Aqua => &MVP_PLUS_AQUA,
				Colour::Red => &MVP_PLUS_RED,
				Colour::LightPurple => &MVP_PLUS_LIGHT_PURPLE,
				Colour::Yellow => &MVP_PLUS_YELLOW,
				Colour::White => &MVP_PLUS_WHITE,
			}),
			Self::MvpPlusPlus(colour, plain) => Some(match (colour, plain) {
				(Colour::Black, false) => &MVP_PLUS_PLUS_BLACK,
				(Colour::DarkBlue, false) => &MVP_PLUS_PLUS_DARK_BLUE,
				(Colour::DarkGreen, false) => &MVP_PLUS_PLUS_DARK_GREEN,
				(Colour::DarkAqua, false) => &MVP_PLUS_PLUS_DARK_AQUA,
				(Colour::DarkRed, false) => &MVP_PLUS_PLUS_DARK_RED,
				(Colour::DarkPurple, false) => &MVP_PLUS_PLUS_DARK_PURPLE,
				(Colour::Gold, false) => &MVP_PLUS_PLUS_GOLD,
				(Colour::Gray, false) => &MVP_PLUS_PLUS_GRAY,
				(Colour::DarkGray, false) => &MVP_PLUS_PLUS_DARK_GRAY,
				(Colour::Blue, false) => &MVP_PLUS_PLUS_BLUE,
				(Colour::Green, false) => &MVP_PLUS_PLUS_GREEN,
				(Colour::Aqua, false) => &MVP_PLUS_PLUS_AQUA,
				(Colour::Red, false) => &MVP_PLUS_PLUS_RED,
				(Colour::LightPurple, false) => &MVP_PLUS_PLUS_LIGHT_PURPLE,
				(Colour::Yellow, false) => &MVP_PLUS_PLUS_YELLOW,
				(Colour::White, false) => &MVP_PLUS_PLUS_WHITE,
				(Colour::Black, true) => &MVP_PLUS_PLUS_BLACK_PLAIN,
				(Colour::DarkBlue, true) => &MVP_PLUS_PLUS_DARK_BLUE_PLAIN,
				(Colour::DarkGreen, true) => &MVP_PLUS_PLUS_DARK_GREEN_PLAIN,
				(Colour::DarkAqua, true) => &MVP_PLUS_PLUS_DARK_AQUA_PLAIN,
				(Colour::DarkRed, true) => &MVP_PLUS_PLUS_DARK_RED_PLAIN,
				(Colour::DarkPurple, true) => &MVP_PLUS_PLUS_DARK_PURPLE_PLAIN,
				(Colour::Gold, true) => &MVP_PLUS_PLUS_GOLD_PLAIN,
				(Colour::Gray, true) => &MVP_PLUS_PLUS_GRAY_PLAIN,
				(Colour::DarkGray, true) => &MVP_PLUS_PLUS_DARK_GRAY_PLAIN,
				(Colour::Blue, true) => &MVP_PLUS_PLUS_BLUE_PLAIN,
				(Colour::Green, true) => &MVP_PLUS_PLUS_GREEN_PLAIN,
				(Colour::Aqua, true) => &MVP_PLUS_PLUS_AQUA_PLAIN,
				(Colour::Red, true) => &MVP_PLUS_PLUS_RED_PLAIN,
				(Colour::LightPurple, true) => &MVP_PLUS_PLUS_LIGHT_PURPLE_PLAIN,
				(Colour::Yellow, true) => &MVP_PLUS_PLUS_YELLOW_PLAIN,
				(Colour::White, true) => &MVP_PLUS_PLUS_WHITE_PLAIN,
			}),
			Self::YouTube => Some(&YOUTUBE),
			Self::Mojang => Some(&MOJANG),
			Self::Events => Some(&EVENTS),
			Self::Mcp => Some(&MCP),
			Self::Gm => Some(&GM),
			Self::Admin => Some(&ADMIN),
			Self::Owner => Some(&OWNER),
			Self::Custom(_) => None,
		}
	}

	#[must_use]
	pub fn as_coloured_str(&self) -> Option<&str> {
		Some(match self {
			Self::Default => return None,
			Self::Vip => "§a[VIP]",
			Self::VipPlus => "§a[VIP§6+§a]",
			Self::Mvp => "§b[MVP]",
			Self::MvpPlus(colour) => match colour {
				Colour::Black => "§b[MVP§0+§b]",
				Colour::DarkBlue => "§b[MVP§1+§b]",
				Colour::DarkGreen => "§b[MVP§2+§b]",
				Colour::DarkAqua => "§b[MVP§3+§b]",
				Colour::DarkRed => "§b[MVP§4+§b]",
				Colour::DarkPurple => "§b[MVP§5+§b]",
				Colour::Gold => "§b[MVP§6+§b]",
				Colour::Gray => "§b[MVP§7+§b]",
				Colour::DarkGray => "§b[MVP§8+§b]",
				Colour::Blue => "§b[MVP§9+§b]",
				Colour::Green => "§b[MVP§a+§b]",
				Colour::Aqua => "§b[MVP§b+§b]",
				Colour::Red => "§b[MVP§c+§b]",
				Colour::LightPurple => "§b[MVP§d+§b]",
				Colour::Yellow => "§b[MVP§e+§b]",
				Colour::White => "§b[MVP§f+§b]",
			},
			Self::MvpPlusPlus(colour, plain) => match (colour, plain) {
				(Colour::Black, false) => "§6[MVP§0++§6]",
				(Colour::DarkBlue, false) => "§6[MVP§1++§6]",
				(Colour::DarkGreen, false) => "§6[MVP§2++§6]",
				(Colour::DarkAqua, false) => "§6[MVP§3++§6]",
				(Colour::DarkRed, false) => "§6[MVP§4++§6]",
				(Colour::DarkPurple, false) => "§6[MVP§5++§6]",
				(Colour::Gold, false) => "§6[MVP§6++§6]",
				(Colour::Gray, false) => "§6[MVP§7++§6]",
				(Colour::DarkGray, false) => "§6[MVP§8++§6]",
				(Colour::Blue, false) => "§6[MVP§9++§6]",
				(Colour::Green, false) => "§6[MVP§a++§6]",
				(Colour::Aqua, false) => "§6[MVP§b++§6]",
				(Colour::Red, false) => "§6[MVP§c++§6]",
				(Colour::LightPurple, false) => "§6[MVP§d++§6]",
				(Colour::Yellow, false) => "§6[MVP§e++§6]",
				(Colour::White, false) => "§6[MVP§f++§6]",
				(Colour::Black, true) => "§b[MVP§0++§b]",
				(Colour::DarkBlue, true) => "§b[MVP§1++§b]",
				(Colour::DarkGreen, true) => "§b[MVP§2++§b]",
				(Colour::DarkAqua, true) => "§b[MVP§3++§b]",
				(Colour::DarkRed, true) => "§b[MVP§4++§b]",
				(Colour::DarkPurple, true) => "§b[MVP§5++§b]",
				(Colour::Gold, true) => "§b[MVP§6++§b]",
				(Colour::Gray, true) => "§b[MVP§7++§b]",
				(Colour::DarkGray, true) => "§b[MVP§8++§b]",
				(Colour::Blue, true) => "§b[MVP§9++§b]",
				(Colour::Green, true) => "§b[MVP§a++§b]",
				(Colour::Aqua, true) => "§b[MVP§b++§b]",
				(Colour::Red, true) => "§b[MVP§c++§b]",
				(Colour::LightPurple, true) => "§b[MVP§d++§b]",
				(Colour::Yellow, true) => "§b[MVP§e++§b]",
				(Colour::White, true) => "§b[MVP§f++§b]",
			},
			Self::YouTube => "§c[§fYOUTUBE§c]",
			Self::Mojang => "§6[MOJANG]",
			Self::Events => "§6[EVENTS]",
			Self::Mcp => "§c[MCP]",
			Self::Gm => "§2[GM]",
			Self::Admin => "§c[ADMIN]",
			Self::Owner => "§c[OWNER]",
			Self::Custom(custom) => custom,
		})
	}
}
