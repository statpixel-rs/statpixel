pub mod overall;

use std::cmp::min;

use skia_safe::Color;

use crate::colour::Colour;

#[derive(Clone, Copy)]
pub struct Level(pub u32);

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_lossless)]
impl From<Level> for f64 {
	fn from(value: Level) -> Self {
		value.0 as f64
	}
}

const LEVEL_COLOUR: [Colour; 11] = [
	Colour::DarkGray,
	Colour::White,
	Colour::Gold,
	Colour::DarkAqua,
	Colour::DarkGreen,
	Colour::DarkRed,
	Colour::Red,
	Colour::DarkPurple,
	Colour::Aqua,
	Colour::LightPurple,
	Colour::Blue,
];

const LEVEL_FORMAT: [&str; 101] = [
	"§7None",
	"§8Rookie",
	"§8Rookie II",
	"§8Rookie III",
	"§8Rookie IV",
	"§8Rookie V",
	"§fIron",
	"§fIron II",
	"§fIron III",
	"§fIron IV",
	"§fIron V",
	"§6Gold",
	"§6Gold II",
	"§6Gold III",
	"§6Gold IV",
	"§6Gold V",
	"§3Diamond",
	"§3Diamond II",
	"§3Diamond III",
	"§3Diamond IV",
	"§3Diamond V",
	"§2Master",
	"§2Master II",
	"§2Master III",
	"§2Master IV",
	"§2Master V",
	"§4§lLegend",
	"§4§lLegend II",
	"§4§lLegend III",
	"§4§lLegend IV",
	"§4§lLegend V",
	"§e§lGrandmaster",
	"§e§lGrandmaster II",
	"§e§lGrandmaster III",
	"§e§lGrandmaster IV",
	"§e§lGrandmaster V",
	"§5§lGodlike",
	"§5§lGodlike II",
	"§5§lGodlike III",
	"§5§lGodlike IV",
	"§5§lGodlike V",
	"§b§lCELESTIAL",
	"§b§lCELESTIAL II",
	"§b§lCELESTIAL III",
	"§b§lCELESTIAL IV",
	"§b§lCELESTIAL V",
	"§d§lDIVINE",
	"§d§lDIVINE II",
	"§d§lDIVINE III",
	"§d§lDIVINE IV",
	"§d§lDIVINE V",
	"§6§lASCENDED",
	"§6§lASCENDED II",
	"§6§lASCENDED III",
	"§6§lASCENDED IV",
	"§6§lASCENDED V",
	"§6§lASCENDED VI",
	"§6§lASCENDED VII",
	"§6§lASCENDED VIII",
	"§6§lASCENDED IX",
	"§6§lASCENDED X",
	"§6§lASCENDED XI",
	"§6§lASCENDED XII",
	"§6§lASCENDED XIII",
	"§6§lASCENDED XIV",
	"§6§lASCENDED XV",
	"§6§lASCENDED XVI",
	"§6§lASCENDED XVII",
	"§6§lASCENDED XVIII",
	"§6§lASCENDED XIX",
	"§6§lASCENDED XX",
	"§6§lASCENDED XXI",
	"§6§lASCENDED XXII",
	"§6§lASCENDED XXIII",
	"§6§lASCENDED XXIV",
	"§6§lASCENDED XXV",
	"§6§lASCENDED XXVI",
	"§6§lASCENDED XXVII",
	"§6§lASCENDED XXVIII",
	"§6§lASCENDED XXIX",
	"§6§lASCENDED XXX",
	"§6§lASCENDED XXXI",
	"§6§lASCENDED XXXII",
	"§6§lASCENDED XXXIII",
	"§6§lASCENDED XXXIV",
	"§6§lASCENDED XXXV",
	"§6§lASCENDED XXXVI",
	"§6§lASCENDED XXXVII",
	"§6§lASCENDED XXXVIII",
	"§6§lASCENDED XXXIX",
	"§6§lASCENDED XL",
	"§6§lASCENDED XLI",
	"§6§lASCENDED XLII",
	"§6§lASCENDED XLIII",
	"§6§lASCENDED XLIV",
	"§6§lASCENDED XLV",
	"§6§lASCENDED XLVI",
	"§6§lASCENDED XLVII",
	"§6§lASCENDED XLVIII",
	"§6§lASCENDED XLIX",
	"§6§lASCENDED L",
];

const TOTAL_XP: [u32; 47] = [
	0, 50, 60, 70, 80, 90, 100, 130, 160, 190, 220, 250, 300, 350, 400, 450, 500, 600, 700, 800,
	900, 1_000, 1_200, 1_400, 1_600, 1_800, 2_000, 2_600, 3_200, 3_800, 4_400, 5_000, 6_000, 7_000,
	8_000, 9_000, 10_000, 13_000, 16_000, 19_000, 22_000, 25_000, 30_000, 35_000, 40_000, 45_000,
	50_000,
];
const XP_PER_LEVEL: u32 = 10_000;

#[must_use]
pub fn get_colours(level: Level) -> [Color; 2] {
	let prestige = level.0 / 5;
	let colour = LEVEL_COLOUR[min(prestige as usize, LEVEL_COLOUR.len() - 1)];

	[colour.into(), colour.into()]
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_level_format(level: Level) -> String {
	LEVEL_FORMAT[min(level.0 as usize, LEVEL_FORMAT.len() - 1)].to_string()
}

#[must_use]
pub fn get_level(xp: u32) -> Level {
	Level(match xp {
		0..50 => 0,
		50..60 => 1,
		60..70 => 2,
		70..80 => 3,
		80..90 => 4,
		90..100 => 5,
		100..130 => 6,
		130..160 => 7,
		160..190 => 8,
		190..220 => 9,
		220..250 => 10,
		250..300 => 11,
		300..350 => 12,
		350..400 => 13,
		400..450 => 14,
		450..500 => 15,
		500..600 => 16,
		600..700 => 17,
		700..800 => 18,
		800..900 => 19,
		900..1_000 => 20,
		1_000..1_200 => 21,
		1_200..1_400 => 22,
		1_400..1_600 => 23,
		1_600..1_800 => 24,
		1_800..2_000 => 25,
		2_000..2_600 => 26,
		2_600..3_200 => 27,
		3_200..3_800 => 28,
		3_800..4_400 => 29,
		4_400..5_000 => 30,
		5_000..6_000 => 31,
		6_000..7_000 => 32,
		7_000..8_000 => 33,
		8_000..9_000 => 34,
		9_000..10_000 => 35,
		10_000..13_000 => 36,
		13_000..16_000 => 37,
		16_000..19_000 => 38,
		19_000..22_000 => 39,
		22_000..25_000 => 40,
		25_000..30_000 => 41,
		30_000..35_000 => 42,
		35_000..40_000 => 43,
		40_000..45_000 => 44,
		45_000..50_000 => 45,
		_ => 46 + (xp - 50_000) / XP_PER_LEVEL,
	})
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_xp(level: Level) -> u32 {
	if (level.0 as usize) < TOTAL_XP.len() {
		TOTAL_XP[level.0 as usize]
	} else {
		TOTAL_XP[TOTAL_XP.len() - 1] + (level.0 - TOTAL_XP.len() as u32) * XP_PER_LEVEL
	}
}

#[must_use]
pub fn get_level_xp(xp: u32) -> u32 {
	let level = get_level(xp);

	get_xp(Level(level.0 + 1)) - get_xp(level)
}

#[must_use]
pub fn get_curr_level_xp(xp: u32) -> u32 {
	xp - get_xp(get_level(xp))
}

#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn get_level_progress(xp: u32) -> f32 {
	let level = get_level(xp);
	let base = get_xp(level);
	let next = get_xp(Level(level.0 + 1));

	(xp - base) as f32 / (next - base) as f32
}

#[must_use]
pub fn get_total_xp(xp: u32) -> u32 {
	xp
}

#[must_use]
pub fn convert(xp: &u32) -> u32 {
	*xp
}
