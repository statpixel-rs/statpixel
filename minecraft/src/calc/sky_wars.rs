use std::cmp::min;

use skia_safe::Color;

use crate::{
	colour::{
		AQUA, BLUE, DARK_AQUA, DARK_BLUE, DARK_GREEN, DARK_PURPLE, DARK_RED, GOLD, GRAY, GREEN,
		LIGHT_PURPLE, RED, WHITE, YELLOW,
	},
	ESCAPE,
};

#[derive(Clone, Copy)]
pub struct Level(pub u64);

#[allow(clippy::cast_precision_loss)]
impl From<Level> for f64 {
	fn from(value: Level) -> Self {
		value.0 as f64
	}
}

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

const LEVEL_FORMAT: [((char, char), [char; 3], &str); 31] = [
	(('7', '7'), ['7', '7', '7'], "§7⋆"),       // Default
	(('f', 'f'), ['f', 'f', 'f'], "§f✙"),       // Iron
	(('6', '6'), ['6', '6', '6'], "§6❤"),       // Gold
	(('b', 'b'), ['b', 'b', 'b'], "§b☠"),       // Diamond
	(('2', '2'), ['2', '2', '2'], "§2✦"),       // Emerald
	(('3', '3'), ['3', '3', '3'], "§3✌"),       // Sapphire
	(('4', '4'), ['4', '4', '4'], "§4❦"),       // Ruby
	(('d', 'd'), ['d', 'd', 'd'], "§d✵"),       // Crystal
	(('9', '9'), ['9', '9', '9'], "§9❣"),       // Opal
	(('5', '5'), ['5', '5', '5'], "§5☯"),       // Amethyst
	(('c', 'b'), ['6', 'e', 'a'], "§b✺"),       // Rainbow
	(('7', '7'), ['f', 'f', 'f'], "§f✈"),       // First Class
	(('4', '4'), ['c', 'c', 'c'], "§c⚰"),       // Assassin
	(('c', 'c'), ['f', 'f', 'f'], "§f✠"),       // Veteran
	(('e', 'e'), ['6', '6', '6'], "§6♕"),       // God Like
	(('f', 'f'), ['9', '9', '9'], "§9⚡"),      // Warrior
	(('f', 'f'), ['b', 'b', 'b'], "§b⁂"),       // Captain
	(('f', 'f'), ['3', '3', '3'], "§3✰"),       // Soldier
	(('a', 'a'), ['3', '3', '3'], "§3⁑"),       // Infantry
	(('c', 'c'), ['e', 'e', 'e'], "§e☢"),       // Sergeant
	(('9', '9'), ['1', '1', '1'], "§1✥"),       // Lieutenant
	(('6', '6'), ['4', '4', '4'], "§4♝"),       // Admiral
	(('1', '1'), ['b', 'b', 'b'], "§b♆"),       // General
	(('8', '8'), ['7', '7', '7'], "§7☁"),       // Villain
	(('d', 'd'), ['5', '5', '5'], "§5⍟"),       // Skilled
	(('f', 'f'), ['e', 'e', 'e'], "§e♗"),       // Sneaky
	(('c', 'c'), ['e', 'e', 'e'], "§e♔"),       // Overlord
	(('6', '6'), ['c', 'c', 'c'], "§c♞"),       // War Chief
	(('a', 'a'), ['c', 'c', 'c'], "§c✏"),       // Warlock
	(('a', 'a'), ['b', 'b', 'b'], "§b❈"),       // Emperor
	(('c', 'c'), ['6', 'e', 'a'], "§bಠ§d_§5ಠ"), // Mythic
];

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
pub fn get_level_format(level: Level) -> String {
	let prestige = level.0 / 5;

	let format = LEVEL_FORMAT[min(prestige as usize, LEVEL_FORMAT.len() - 1)];
	let length = (level.0 as f32).log10().ceil() as usize;
	let string = level.0.to_string();
	let chars = string.chars();

	// char, escape, colour, + 3 for each bracket, + star if it exists
	let mut string = String::with_capacity(length * 3 + 6 + 1);

	string.push(ESCAPE);
	string.push(format.0 .0);
	string.push('[');

	for (i, c) in chars.enumerate() {
		string.push(ESCAPE);
		string.push(format.1[i]);
		string.push(c);
	}

	string.push(ESCAPE);
	string.push_str(format.2);

	string.push(ESCAPE);
	string.push(format.0 .1);
	string.push(']');

	string
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_colours(level: Level) -> [Color; 2] {
	LEVEL_COLOUR[min(level.0 as usize / 5, LEVEL_COLOUR.len() - 1)]
}

#[must_use]
pub fn get_level(xp: u64) -> Level {
	for (i, &x) in TOTAL_LEVEL_XP.iter().enumerate() {
		if x > xp {
			return Level(i as u64);
		}
	}

	let xp = xp - TOTAL_LEVEL_XP[11];

	Level(12 + (xp / 10_000))
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_xp(level: Level) -> u64 {
	if level.0 == 0 {
		0
	} else if level.0 < 12 {
		TOTAL_LEVEL_XP[level.0 as usize - 1]
	} else {
		TOTAL_LEVEL_XP[11] + (level.0 - 12) * 10_000
	}
}

#[must_use]
pub fn get_level_xp(xp: u64) -> u64 {
	let level = get_level(xp);

	get_xp(Level(level.0 + 1)) - get_xp(level)
}

#[must_use]
pub fn get_curr_level_xp(xp: u64) -> u64 {
	xp - get_xp(get_level(xp))
}

#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn get_level_progress(xp: u64) -> f32 {
	let level = get_level(xp);
	let base = get_xp(level);
	let next = get_xp(Level(level.0 + 1));

	(xp - base) as f32 / (next - base) as f32
}

#[must_use]
pub fn get_total_xp(xp: u64) -> u64 {
	xp
}

#[must_use]
pub fn convert(xp: &u64) -> u64 {
	*xp
}
