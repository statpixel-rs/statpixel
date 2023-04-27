use super::{parse::ESCAPE, MinecraftText};
use crate::{colour::MinecraftColour, minecraft_text, paint::MinecraftPaint};

use std::slice::Iter;

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

const VIP: [MinecraftText; 1] = minecraft_text!("§a[VIP]");
const VIP_PLUS: [MinecraftText; 3] = minecraft_text!("§a[VIP§6+§a]");
const MVP: [MinecraftText; 1] = minecraft_text!("§b[MVP]");

const MVP_PLUS_BLACK: [MinecraftText; 3] = mvp_plus!("0");
const MVP_PLUS_DARK_BLUE: [MinecraftText; 3] = mvp_plus!("1");
const MVP_PLUS_DARK_GREEN: [MinecraftText; 3] = mvp_plus!("2");
const MVP_PLUS_DARK_AQUA: [MinecraftText; 3] = mvp_plus!("3");
const MVP_PLUS_DARK_RED: [MinecraftText; 3] = mvp_plus!("4");
const MVP_PLUS_DARK_PURPLE: [MinecraftText; 3] = mvp_plus!("5");
const MVP_PLUS_GOLD: [MinecraftText; 3] = mvp_plus!("6");
const MVP_PLUS_GRAY: [MinecraftText; 3] = mvp_plus!("7");
const MVP_PLUS_DARK_GRAY: [MinecraftText; 3] = mvp_plus!("8");
const MVP_PLUS_BLUE: [MinecraftText; 3] = mvp_plus!("9");
const MVP_PLUS_GREEN: [MinecraftText; 3] = mvp_plus!("a");
const MVP_PLUS_AQUA: [MinecraftText; 3] = mvp_plus!("b");
const MVP_PLUS_RED: [MinecraftText; 3] = mvp_plus!("c");
const MVP_PLUS_LIGHT_PURPLE: [MinecraftText; 3] = mvp_plus!("d");
const MVP_PLUS_YELLOW: [MinecraftText; 3] = mvp_plus!("e");
const MVP_PLUS_WHITE: [MinecraftText; 3] = mvp_plus!("f");

const MVP_PLUS_PLUS_BLACK: [MinecraftText; 3] = mvp_plus_plus!("0", false);
const MVP_PLUS_PLUS_DARK_BLUE: [MinecraftText; 3] = mvp_plus_plus!("1", false);
const MVP_PLUS_PLUS_DARK_GREEN: [MinecraftText; 3] = mvp_plus_plus!("2", false);
const MVP_PLUS_PLUS_DARK_AQUA: [MinecraftText; 3] = mvp_plus_plus!("3", false);
const MVP_PLUS_PLUS_DARK_RED: [MinecraftText; 3] = mvp_plus_plus!("4", false);
const MVP_PLUS_PLUS_DARK_PURPLE: [MinecraftText; 3] = mvp_plus_plus!("5", false);
const MVP_PLUS_PLUS_GOLD: [MinecraftText; 3] = mvp_plus_plus!("6", false);
const MVP_PLUS_PLUS_GRAY: [MinecraftText; 3] = mvp_plus_plus!("7", false);
const MVP_PLUS_PLUS_DARK_GRAY: [MinecraftText; 3] = mvp_plus_plus!("8", false);
const MVP_PLUS_PLUS_BLUE: [MinecraftText; 3] = mvp_plus_plus!("9", false);
const MVP_PLUS_PLUS_GREEN: [MinecraftText; 3] = mvp_plus_plus!("a", false);
const MVP_PLUS_PLUS_AQUA: [MinecraftText; 3] = mvp_plus_plus!("b", false);
const MVP_PLUS_PLUS_RED: [MinecraftText; 3] = mvp_plus_plus!("c", false);
const MVP_PLUS_PLUS_LIGHT_PURPLE: [MinecraftText; 3] = mvp_plus_plus!("d", false);
const MVP_PLUS_PLUS_YELLOW: [MinecraftText; 3] = mvp_plus_plus!("e", false);
const MVP_PLUS_PLUS_WHITE: [MinecraftText; 3] = mvp_plus_plus!("f", false);

const MVP_PLUS_PLUS_BLACK_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("0", true);
const MVP_PLUS_PLUS_DARK_BLUE_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("1", true);
const MVP_PLUS_PLUS_DARK_GREEN_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("2", true);
const MVP_PLUS_PLUS_DARK_AQUA_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("3", true);
const MVP_PLUS_PLUS_DARK_RED_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("4", true);
const MVP_PLUS_PLUS_DARK_PURPLE_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("5", true);
const MVP_PLUS_PLUS_GOLD_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("6", true);
const MVP_PLUS_PLUS_GRAY_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("7", true);
const MVP_PLUS_PLUS_DARK_GRAY_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("8", true);
const MVP_PLUS_PLUS_BLUE_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("9", true);
const MVP_PLUS_PLUS_GREEN_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("a", true);
const MVP_PLUS_PLUS_AQUA_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("b", true);
const MVP_PLUS_PLUS_RED_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("c", true);
const MVP_PLUS_PLUS_LIGHT_PURPLE_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("d", true);
const MVP_PLUS_PLUS_YELLOW_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("e", true);
const MVP_PLUS_PLUS_WHITE_PLAIN: [MinecraftText; 3] = mvp_plus_plus!("f", true);

const YOUTUBE: [MinecraftText; 3] = minecraft_text!("§c[§fYOUTUBE§c]");
const MOJANG: [MinecraftText; 1] = minecraft_text!("§6[MOJANG]");
const EVENTS: [MinecraftText; 1] = minecraft_text!("§6[EVENTS]");
const MCP: [MinecraftText; 1] = minecraft_text!("§c[MCP]");
const GM: [MinecraftText; 1] = minecraft_text!("§2[GM]");
const ADMIN: [MinecraftText; 1] = minecraft_text!("§c[ADMIN]");
const OWNER: [MinecraftText; 1] = minecraft_text!("§c[OWNER]");

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
			"YOUTUBE" => Self::YouTube,
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
			let char = string.chars().nth(index - 1);
			let paint = char.and_then(|c| MinecraftPaint::try_from(c).ok());

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
	pub fn get_text(&self) -> Option<Iter<MinecraftText<'_>>> {
		match self {
			Self::Default => Some([].iter()),
			Self::Vip => Some(VIP.iter()),
			Self::VipPlus => Some(VIP_PLUS.iter()),
			Self::Mvp => Some(MVP.iter()),
			Self::MvpPlus(colour) => Some(match colour {
				MinecraftColour::Black => MVP_PLUS_BLACK.iter(),
				MinecraftColour::DarkBlue => MVP_PLUS_DARK_BLUE.iter(),
				MinecraftColour::DarkGreen => MVP_PLUS_DARK_GREEN.iter(),
				MinecraftColour::DarkAqua => MVP_PLUS_DARK_AQUA.iter(),
				MinecraftColour::DarkRed => MVP_PLUS_DARK_RED.iter(),
				MinecraftColour::DarkPurple => MVP_PLUS_DARK_PURPLE.iter(),
				MinecraftColour::Gold => MVP_PLUS_GOLD.iter(),
				MinecraftColour::Gray => MVP_PLUS_GRAY.iter(),
				MinecraftColour::DarkGray => MVP_PLUS_DARK_GRAY.iter(),
				MinecraftColour::Blue => MVP_PLUS_BLUE.iter(),
				MinecraftColour::Green => MVP_PLUS_GREEN.iter(),
				MinecraftColour::Aqua => MVP_PLUS_AQUA.iter(),
				MinecraftColour::Red => MVP_PLUS_RED.iter(),
				MinecraftColour::LightPurple => MVP_PLUS_LIGHT_PURPLE.iter(),
				MinecraftColour::Yellow => MVP_PLUS_YELLOW.iter(),
				MinecraftColour::White => MVP_PLUS_WHITE.iter(),
			}),
			Self::MvpPlusPlus(colour, plain) => Some(match (colour, plain) {
				(MinecraftColour::Black, false) => MVP_PLUS_PLUS_BLACK.iter(),
				(MinecraftColour::DarkBlue, false) => MVP_PLUS_PLUS_DARK_BLUE.iter(),
				(MinecraftColour::DarkGreen, false) => MVP_PLUS_PLUS_DARK_GREEN.iter(),
				(MinecraftColour::DarkAqua, false) => MVP_PLUS_PLUS_DARK_AQUA.iter(),
				(MinecraftColour::DarkRed, false) => MVP_PLUS_PLUS_DARK_RED.iter(),
				(MinecraftColour::DarkPurple, false) => MVP_PLUS_PLUS_DARK_PURPLE.iter(),
				(MinecraftColour::Gold, false) => MVP_PLUS_PLUS_GOLD.iter(),
				(MinecraftColour::Gray, false) => MVP_PLUS_PLUS_GRAY.iter(),
				(MinecraftColour::DarkGray, false) => MVP_PLUS_PLUS_DARK_GRAY.iter(),
				(MinecraftColour::Blue, false) => MVP_PLUS_PLUS_BLUE.iter(),
				(MinecraftColour::Green, false) => MVP_PLUS_PLUS_GREEN.iter(),
				(MinecraftColour::Aqua, false) => MVP_PLUS_PLUS_AQUA.iter(),
				(MinecraftColour::Red, false) => MVP_PLUS_PLUS_RED.iter(),
				(MinecraftColour::LightPurple, false) => MVP_PLUS_PLUS_LIGHT_PURPLE.iter(),
				(MinecraftColour::Yellow, false) => MVP_PLUS_PLUS_YELLOW.iter(),
				(MinecraftColour::White, false) => MVP_PLUS_PLUS_WHITE.iter(),
				(MinecraftColour::Black, true) => MVP_PLUS_PLUS_BLACK_PLAIN.iter(),
				(MinecraftColour::DarkBlue, true) => MVP_PLUS_PLUS_DARK_BLUE_PLAIN.iter(),
				(MinecraftColour::DarkGreen, true) => MVP_PLUS_PLUS_DARK_GREEN_PLAIN.iter(),
				(MinecraftColour::DarkAqua, true) => MVP_PLUS_PLUS_DARK_AQUA_PLAIN.iter(),
				(MinecraftColour::DarkRed, true) => MVP_PLUS_PLUS_DARK_RED_PLAIN.iter(),
				(MinecraftColour::DarkPurple, true) => MVP_PLUS_PLUS_DARK_PURPLE_PLAIN.iter(),
				(MinecraftColour::Gold, true) => MVP_PLUS_PLUS_GOLD_PLAIN.iter(),
				(MinecraftColour::Gray, true) => MVP_PLUS_PLUS_GRAY_PLAIN.iter(),
				(MinecraftColour::DarkGray, true) => MVP_PLUS_PLUS_DARK_GRAY_PLAIN.iter(),
				(MinecraftColour::Blue, true) => MVP_PLUS_PLUS_BLUE_PLAIN.iter(),
				(MinecraftColour::Green, true) => MVP_PLUS_PLUS_GREEN_PLAIN.iter(),
				(MinecraftColour::Aqua, true) => MVP_PLUS_PLUS_AQUA_PLAIN.iter(),
				(MinecraftColour::Red, true) => MVP_PLUS_PLUS_RED_PLAIN.iter(),
				(MinecraftColour::LightPurple, true) => MVP_PLUS_PLUS_LIGHT_PURPLE_PLAIN.iter(),
				(MinecraftColour::Yellow, true) => MVP_PLUS_PLUS_YELLOW_PLAIN.iter(),
				(MinecraftColour::White, true) => MVP_PLUS_PLUS_WHITE_PLAIN.iter(),
			}),
			Self::YouTube => Some(YOUTUBE.iter()),
			Self::Mojang => Some(MOJANG.iter()),
			Self::Events => Some(EVENTS.iter()),
			Self::Mcp => Some(MCP.iter()),
			Self::Gm => Some(GM.iter()),
			Self::Admin => Some(ADMIN.iter()),
			Self::Owner => Some(OWNER.iter()),
			Self::Custom(_) => None,
		}
	}
}
