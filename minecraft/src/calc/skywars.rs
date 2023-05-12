use std::cmp::min;

use skia_safe::Color;

use crate::colour::{
	AQUA, BLUE, DARK_AQUA, DARK_BLUE, DARK_GREEN, DARK_PURPLE, DARK_RED, GOLD, GRAY, GREEN,
	LIGHT_PURPLE, RED, WHITE, YELLOW,
};

const TOTAL_LEVEL_XP: [u64; 12] = [
	0, 20, 70, 150, 250, 500, 1_000, 2_000, 3_500, 6_000, 10_000, 15_000,
];

const LEVEL_COLOUR: [[Color; 2]; 31] = [
	[GRAY, GRAY],
	[WHITE, WHITE],
	[GOLD, GOLD],
	[AQUA, AQUA],
	[DARK_GREEN, DARK_GREEN],
	[DARK_AQUA, DARK_AQUA],
	[DARK_RED, DARK_RED],
	[LIGHT_PURPLE, LIGHT_PURPLE],
	[BLUE, BLUE],
	[DARK_PURPLE, DARK_PURPLE],
	[RED, AQUA],
	[WHITE, WHITE],
	[RED, RED],
	[WHITE, WHITE],
	[GOLD, GOLD],
	[BLUE, BLUE],
	[AQUA, AQUA],
	[DARK_AQUA, DARK_AQUA],
	[DARK_AQUA, DARK_AQUA],
	[YELLOW, YELLOW],
	[DARK_BLUE, DARK_BLUE],
	[DARK_RED, DARK_RED],
	[AQUA, AQUA],
	[GRAY, GRAY],
	[DARK_PURPLE, DARK_PURPLE],
	[YELLOW, YELLOW],
	[YELLOW, YELLOW],
	[RED, RED],
	[RED, RED],
	[AQUA, AQUA],
	[RED, GREEN],
];

pub fn get_colours(level: u64) -> [Color; 2] {
	LEVEL_COLOUR[min(level as usize / 5, LEVEL_COLOUR.len() - 1)]
}

pub fn get_level(xp: u64) -> u64 {
	for (i, &x) in TOTAL_LEVEL_XP.iter().enumerate() {
		if x > xp {
			return i as u64;
		}
	}

	let xp = xp - TOTAL_LEVEL_XP[11];

	12 + (xp / 10_000)
}

pub fn get_xp(level: u64) -> u64 {
	if level == 0 {
		0
	} else if level < 12 {
		TOTAL_LEVEL_XP[level as usize - 1]
	} else {
		TOTAL_LEVEL_XP[11] + (level - 12) * 10_000
	}
}

pub fn get_level_xp(xp: u64) -> u64 {
	let level = get_level(xp);

	get_xp(level + 1) - get_xp(level)
}

pub fn get_curr_level_xp(xp: u64) -> u64 {
	xp - get_xp(get_level(xp))
}

pub fn get_level_progress(xp: u64) -> f32 {
	let level = get_level(xp);
	let base = get_xp(level);
	let next = get_xp(level + 1);

	(xp - base) as f32 / (next - base) as f32
}

pub fn convert(xp: &u64) -> u64 {
	*xp
}
