use super::{parse::ESCAPE, Text};
use crate::{colour::MinecraftColour, minecraft_text, paint::MinecraftPaint};

pub enum Rank {
	Default,
	Vip,
	VipPlus,
	Mvp,
	MvpPlus(MinecraftColour),
	MvpPlusPlus(MinecraftColour, bool),
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
		minecraft_text!(concat!("§b[MVP§", $colour, "+§b]"))
	};
}

macro_rules! mvp_plus_plus {
	($colour: expr, $plain: expr) => {
		if $plain {
			minecraft_text!(concat!("§b[MVP§", $colour, "++§b]"))
		} else {
			minecraft_text!(concat!("§6[MVP§", $colour, "++§6]"))
		}
	};
}

const VIP: [Text; 1] = minecraft_text!("§a[VIP]");
const VIP_PLUS: [Text; 3] = minecraft_text!("§a[VIP§6+§a]");
const MVP: [Text; 1] = minecraft_text!("§b[MVP]");

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

const YOUTUBE: [Text; 3] = minecraft_text!("§c[§fYOUTUBE§c]");
const MOJANG: [Text; 1] = minecraft_text!("§6[MOJANG]");
const EVENTS: [Text; 1] = minecraft_text!("§6[EVENTS]");
const MCP: [Text; 1] = minecraft_text!("§c[MCP]");
const GM: [Text; 1] = minecraft_text!("§2[GM]");
const ADMIN: [Text; 1] = minecraft_text!("§c[ADMIN]");
const OWNER: [Text; 1] = minecraft_text!("§c[OWNER]");

impl Rank {
	pub fn from_str(
		rank: &str,
		package_rank: Option<&str>,
		colour: Option<MinecraftColour>,
		monthly_colour: Option<MinecraftColour>,
	) -> Self {
		match rank {
			"VIP" => Self::Vip,
			"VIP_PLUS" => Self::VipPlus,
			"MVP" => Self::Mvp,
			"MVP_PLUS" if package_rank == Some("SUPERSTAR") => Self::MvpPlusPlus(
				colour.unwrap_or(MinecraftColour::Red),
				monthly_colour
					.map(|c| c == MinecraftColour::Aqua)
					.unwrap_or(false),
			),
			"MVP_PLUS" => Self::MvpPlus(colour.unwrap_or(MinecraftColour::Red)),
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

	fn get_string_paint(&self, string: &str) -> MinecraftPaint {
		if let Some(index) = string.rfind(ESCAPE) {
			let char = string.char_indices().rfind(|(i, _)| *i == index + 2);
			let paint = char.and_then(|(_, c)| MinecraftPaint::try_from(c).ok());

			paint.unwrap_or(MinecraftPaint::Gray)
		} else {
			MinecraftPaint::Gray
		}
	}

	pub fn get_username_paint(&self) -> MinecraftPaint {
		match self {
			Self::Default => MinecraftPaint::Gray,
			Self::Vip | Self::VipPlus => MinecraftPaint::Green,
			Self::Mvp | Self::MvpPlus(_) => MinecraftPaint::Aqua,
			Self::MvpPlusPlus(_, true) => MinecraftPaint::Aqua,
			Self::MvpPlusPlus(_, false) => MinecraftPaint::Gold,
			Self::YouTube => MinecraftPaint::Red,
			Self::Mojang => MinecraftPaint::Gold,
			Self::Events => MinecraftPaint::Gold,
			Self::Mcp => MinecraftPaint::Red,
			Self::Gm => MinecraftPaint::DarkGreen,
			Self::Admin | Self::Owner => MinecraftPaint::Red,
			Self::Custom(prefix) => self.get_string_paint(prefix),
		}
	}

	/// `None` for Rank::Custom
	pub fn get_text(&self) -> Option<&[Text<'_>]> {
		match self {
			Self::Default => Some(&[]),
			Self::Vip => Some(&VIP),
			Self::VipPlus => Some(&VIP_PLUS),
			Self::Mvp => Some(&MVP),
			Self::MvpPlus(colour) => Some(match colour {
				MinecraftColour::Black => &MVP_PLUS_BLACK,
				MinecraftColour::DarkBlue => &MVP_PLUS_DARK_BLUE,
				MinecraftColour::DarkGreen => &MVP_PLUS_DARK_GREEN,
				MinecraftColour::DarkAqua => &MVP_PLUS_DARK_AQUA,
				MinecraftColour::DarkRed => &MVP_PLUS_DARK_RED,
				MinecraftColour::DarkPurple => &MVP_PLUS_DARK_PURPLE,
				MinecraftColour::Gold => &MVP_PLUS_GOLD,
				MinecraftColour::Gray => &MVP_PLUS_GRAY,
				MinecraftColour::DarkGray => &MVP_PLUS_DARK_GRAY,
				MinecraftColour::Blue => &MVP_PLUS_BLUE,
				MinecraftColour::Green => &MVP_PLUS_GREEN,
				MinecraftColour::Aqua => &MVP_PLUS_AQUA,
				MinecraftColour::Red => &MVP_PLUS_RED,
				MinecraftColour::LightPurple => &MVP_PLUS_LIGHT_PURPLE,
				MinecraftColour::Yellow => &MVP_PLUS_YELLOW,
				MinecraftColour::White => &MVP_PLUS_WHITE,
			}),
			Self::MvpPlusPlus(colour, plain) => Some(match (colour, plain) {
				(MinecraftColour::Black, false) => &MVP_PLUS_PLUS_BLACK,
				(MinecraftColour::DarkBlue, false) => &MVP_PLUS_PLUS_DARK_BLUE,
				(MinecraftColour::DarkGreen, false) => &MVP_PLUS_PLUS_DARK_GREEN,
				(MinecraftColour::DarkAqua, false) => &MVP_PLUS_PLUS_DARK_AQUA,
				(MinecraftColour::DarkRed, false) => &MVP_PLUS_PLUS_DARK_RED,
				(MinecraftColour::DarkPurple, false) => &MVP_PLUS_PLUS_DARK_PURPLE,
				(MinecraftColour::Gold, false) => &MVP_PLUS_PLUS_GOLD,
				(MinecraftColour::Gray, false) => &MVP_PLUS_PLUS_GRAY,
				(MinecraftColour::DarkGray, false) => &MVP_PLUS_PLUS_DARK_GRAY,
				(MinecraftColour::Blue, false) => &MVP_PLUS_PLUS_BLUE,
				(MinecraftColour::Green, false) => &MVP_PLUS_PLUS_GREEN,
				(MinecraftColour::Aqua, false) => &MVP_PLUS_PLUS_AQUA,
				(MinecraftColour::Red, false) => &MVP_PLUS_PLUS_RED,
				(MinecraftColour::LightPurple, false) => &MVP_PLUS_PLUS_LIGHT_PURPLE,
				(MinecraftColour::Yellow, false) => &MVP_PLUS_PLUS_YELLOW,
				(MinecraftColour::White, false) => &MVP_PLUS_PLUS_WHITE,
				(MinecraftColour::Black, true) => &MVP_PLUS_PLUS_BLACK_PLAIN,
				(MinecraftColour::DarkBlue, true) => &MVP_PLUS_PLUS_DARK_BLUE_PLAIN,
				(MinecraftColour::DarkGreen, true) => &MVP_PLUS_PLUS_DARK_GREEN_PLAIN,
				(MinecraftColour::DarkAqua, true) => &MVP_PLUS_PLUS_DARK_AQUA_PLAIN,
				(MinecraftColour::DarkRed, true) => &MVP_PLUS_PLUS_DARK_RED_PLAIN,
				(MinecraftColour::DarkPurple, true) => &MVP_PLUS_PLUS_DARK_PURPLE_PLAIN,
				(MinecraftColour::Gold, true) => &MVP_PLUS_PLUS_GOLD_PLAIN,
				(MinecraftColour::Gray, true) => &MVP_PLUS_PLUS_GRAY_PLAIN,
				(MinecraftColour::DarkGray, true) => &MVP_PLUS_PLUS_DARK_GRAY_PLAIN,
				(MinecraftColour::Blue, true) => &MVP_PLUS_PLUS_BLUE_PLAIN,
				(MinecraftColour::Green, true) => &MVP_PLUS_PLUS_GREEN_PLAIN,
				(MinecraftColour::Aqua, true) => &MVP_PLUS_PLUS_AQUA_PLAIN,
				(MinecraftColour::Red, true) => &MVP_PLUS_PLUS_RED_PLAIN,
				(MinecraftColour::LightPurple, true) => &MVP_PLUS_PLUS_LIGHT_PURPLE_PLAIN,
				(MinecraftColour::Yellow, true) => &MVP_PLUS_PLUS_YELLOW_PLAIN,
				(MinecraftColour::White, true) => &MVP_PLUS_PLUS_WHITE_PLAIN,
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
}
