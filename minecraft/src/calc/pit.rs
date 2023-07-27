use std::cmp::min;

use serde::Deserialize;
use skia_safe::Color;

use crate::{colour::Colour, text::ESCAPE};

#[derive(bincode::Decode, bincode::Encode, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Prestige {
	#[serde(rename = "xp_on_prestige")]
	pub xp: u64,
}

#[derive(bincode::Encode, bincode::Decode, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Level {
	pub xp: u64,
	pub prestiges: Vec<Prestige>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelSimple {
	pub xp: u64,
	pub prestige: usize,
}

#[derive(Clone, Copy)]
pub struct LevelData(pub usize, pub u64);

impl From<&Level> for LevelSimple {
	fn from(level: &Level) -> Self {
		Self {
			xp: level.xp,
			prestige: level.prestiges.len(),
		}
	}
}

#[allow(clippy::cast_precision_loss)]
impl From<LevelData> for f64 {
	fn from(value: LevelData) -> Self {
		value.0 as f64 * 120. + value.1 as f64
	}
}

const ROMAN: [&str; 50] = [
	"I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII", "XIV", "XV",
	"XVI", "XVII", "XVIII", "XIX", "XX", "XXI", "XXII", "XXIII", "XXIV", "XXV", "XXVI", "XXVII",
	"XXVIII", "XXIX", "XXX", "XXXI", "XXXII", "XXXIII", "XXXIV", "XXXV", "XXXVI", "XXXVII",
	"XXXVIII", "XXXIX", "XL", "XLI", "XLII", "XLIII", "XLIV", "XLV", "XLVI", "XLVII", "XLVIII",
	"XLIX", "L",
];

const LEVEL_COLOUR: [&str; 13] = [
	"§7", "§9", "§3", "§2", "§a", "§e", "§6§l", "§c§l", "§4§l", "§5§l", "§d§l", "§f§l", "§b§l",
];

const PRESTIGE_COLOUR: [&str; 12] = [
	"§7", "§9", "§e", "§6", "§c", "§5", "§d", "§f", "§b", "§3", "§1", "§0",
];

const PRESTIGES: [(f64, u64); 37] = [
	(1., 0),
	(1.1, 65_950),
	(1.2, 138_495),
	(1.3, 217_635),
	(1.4, 303_370),
	(1.5, 395_700),
	(1.75, 494_625),
	(2., 610_037),
	(2.5, 741_937),
	(3., 906_812),
	(4., 1_104_662),
	(5., 1_368_462),
	(6., 1_698_212),
	(7., 2_093_912),
	(8., 2_555_562),
	(9., 3_083_162),
	(10., 3_676_712),
	(12., 4_336_212),
	(14., 5_127_612),
	(16., 6_050_912),
	(18., 7_106_112),
	(20., 8_293_212),
	(24., 9_612_212),
	(28., 11_195_012),
	(32., 13_041_612),
	(36., 15_152_012),
	(40., 17_526_212),
	(45., 20_164_212),
	(50., 23_131_962),
	(75., 26_429_462),
	(100., 31_375_712),
	(101., 37_970_712),
	(101., 44_631_662),
	(101., 51_292_612),
	(101., 57_953_562),
	(101., 64_614_512),
	(101., 71_275_462),
];

fn get_level_data(xp: u64) -> u64 {
	match xp {
		..150 => xp / 15,
		150..450 => (xp - 150) / 30 + 10,
		450..950 => (xp - 450) / 50 + 20,
		950..1_700 => (xp - 950) / 75 + 30,
		1_700..2_950 => (xp - 1_700) / 125 + 40,
		2_950..5_950 => (xp - 2_950) / 300 + 50,
		5_950..11_950 => (xp - 5_950) / 600 + 60,
		11_950..19_950 => (xp - 11_950) / 800 + 70,
		19_950..28_950 => (xp - 19_950) / 900 + 80,
		28_950..38_950 => (xp - 28_950) / 1_000 + 90,
		38_950..50_950 => (xp - 38_950) / 1_200 + 100,
		_ => min((xp - 50_950) / 1_500 + 110, 120),
	}
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_level_format(LevelData(prestige, level): LevelData) -> String {
	let bracket = PRESTIGE_COLOUR[if prestige == 0 {
		0
	} else {
		min(prestige / 5 + 1, PRESTIGE_COLOUR.len())
	}];
	let number = LEVEL_COLOUR[level as usize / 10];
	let roman = if prestige == 0 {
		String::new()
	} else {
		format!("{}{ESCAPE}{bracket}-", ROMAN[prestige - 1])
	};

	format!("{bracket}[{ESCAPE}e{roman}{number}{level}{bracket}]")
}

#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn get_xp(LevelData(prestige, level): LevelData) -> u64 {
	let (mul, base) = PRESTIGES[min(prestige, PRESTIGES.len() - 1)];

	(level as f64 * mul) as u64 + base
}

#[must_use]
pub fn get_colours(LevelData(prestige, _level): LevelData) -> [Color; 2] {
	let colour = match prestige {
		0 => Colour::Gray,
		1..5 => Colour::Blue,
		5..10 => Colour::Yellow,
		10..15 => Colour::Gold,
		15..20 => Colour::Red,
		20..25 => Colour::DarkPurple,
		25..30 => Colour::LightPurple,
		30..35 => Colour::White,
		35..40 => Colour::Aqua,
		40..45 => Colour::DarkBlue,
		_ => Colour::Black,
	};

	[colour.into(), colour.into()]
}

#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn get_level_progress(level: LevelSimple) -> f32 {
	let div = get_level_xp(level);

	get_curr_level_xp(level) as f32 / if div == 0 { 1. } else { div as f32 }
}

#[must_use]
pub fn get_curr_level_xp(level: LevelSimple) -> u64 {
	let prestige = level.prestige;
	let (_, base) = PRESTIGES[min(prestige, PRESTIGES.len() - 1)];

	level.xp - base
}

#[must_use]
pub fn get_level_xp(level: LevelSimple) -> u64 {
	let prestige = level.prestige;
	let (_, curr_base) = PRESTIGES[min(prestige, PRESTIGES.len() - 1)];
	let (_, base) = PRESTIGES[min(prestige + 1, PRESTIGES.len() - 1)];

	base - curr_base
}

#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn get_level(level: LevelSimple) -> LevelData {
	let prestige = level.prestige;
	let (mul, base) = PRESTIGES[min(prestige, PRESTIGES.len() - 1)];

	let xp = ((level.xp - base) as f64 / mul) as u64;
	let level = get_level_data(xp);

	LevelData(prestige, level)
}

#[must_use]
pub fn get_total_xp(level: LevelSimple) -> u64 {
	level.xp
}

#[must_use]
pub fn convert(xp: &Level) -> LevelSimple {
	xp.into()
}
