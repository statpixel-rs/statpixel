use std::cmp::min;

use skia_safe::Color;

use crate::{colour::Colour, text::ESCAPE};

#[derive(Clone, Copy)]
pub struct Level(pub u64);

#[allow(clippy::cast_precision_loss)]
impl From<Level> for f64 {
	fn from(value: Level) -> Self {
		value.0 as f64
	}
}

const TOTAL_LEVEL_XP: [u64; 4] = [500, 1_500, 3_500, 7_000];
const XP_PER_LEVEL: u64 = 5_000;
const XP_PER_PRESTIGE: u64 = 96 * XP_PER_LEVEL + 7_000;

type Format = ((char, char), [char; 5], char);

const LEVEL_FORMAT: [Format; 51] = [
	(('7', '7'), ['7', '7', '7', '7', '7'], '✫'), // 0
	(('f', 'f'), ['f', 'f', 'f', 'f', 'f'], '✫'), // 100
	(('6', '6'), ['6', '6', '6', '6', '6'], '✫'), // 200
	(('b', 'b'), ['b', 'b', 'b', 'b', 'b'], '✫'), // 300
	(('2', '2'), ['2', '2', '2', '2', '2'], '✫'), // 400
	(('3', '3'), ['3', '3', '3', '3', '3'], '✫'), // 500
	(('4', '4'), ['4', '4', '4', '4', '4'], '✫'), // 600
	(('d', 'd'), ['d', 'd', 'd', 'd', 'd'], '✫'), // 700
	(('9', '9'), ['9', '9', '9', '9', '9'], '✫'), // 800
	(('5', '5'), ['5', '5', '5', '5', '5'], '✫'), // 900
	(('c', '5'), ['6', 'e', 'a', 'b', 'd'], '✫'), // 1000
	(('7', '7'), ['f', 'f', 'f', 'f', '7'], '✪'), // 1100
	(('7', '7'), ['e', 'e', 'e', 'e', '6'], '✪'), // 1200
	(('7', '7'), ['b', 'b', 'b', 'b', '3'], '✪'), // 1300
	(('7', '7'), ['a', 'a', 'a', 'a', '2'], '✪'), // 1400
	(('7', '7'), ['3', '3', '3', '3', '9'], '✪'), // 1500
	(('7', '7'), ['c', 'c', 'c', 'c', '4'], '✪'), // 1600
	(('7', '7'), ['d', 'd', 'd', 'd', '5'], '✪'), // 1700
	(('7', '7'), ['9', '9', '9', '9', '1'], '✪'), // 1800
	(('7', '7'), ['5', '5', '5', '5', '8'], '✪'), // 1900
	(('8', '8'), ['7', 'f', 'f', '7', '7'], '✪'), // 2000
	(('f', '6'), ['f', 'e', 'e', '6', '6'], '⚝'), // 2100
	(('6', '3'), ['6', 'f', 'f', 'b', '3'], '⚝'), // 2200
	(('5', 'e'), ['5', 'd', 'd', '6', 'e'], '⚝'), // 2300
	(('b', '8'), ['b', 'f', 'f', '7', '7'], '⚝'), // 2400
	(('f', '2'), ['f', 'a', 'a', '2', '2'], '⚝'), // 2500
	(('4', '5'), ['4', 'c', 'c', 'd', 'd'], '⚝'), // 2600
	(('e', '8'), ['e', 'f', 'f', '8', '8'], '⚝'), // 2700
	(('a', 'e'), ['a', '2', '2', '6', '6'], '⚝'), // 2800
	(('b', '1'), ['b', '3', '3', '9', '9'], '⚝'), // 2900
	(('e', '4'), ['e', '6', '6', 'c', 'c'], '⚝'), // 3000
	(('9', 'e'), ['9', '3', '3', '6', '6'], '✥'), // 3100
	(('c', 'c'), ['4', '7', '7', '4', 'c'], '✥'), // 3200
	(('9', '4'), ['9', '9', 'd', 'c', 'c'], '✥'), // 3300
	(('2', '2'), ['a', 'd', 'd', '5', '5'], '✥'), // 3400
	(('c', 'a'), ['c', '4', '4', '2', 'a'], '✥'), // 3500
	(('a', '1'), ['a', 'a', 'b', '9', '9'], '✥'), // 3600
	(('4', '3'), ['4', 'c', 'd', 'b', '3'], '✥'), // 3700
	(('1', '1'), ['1', '9', '5', '5', 'd'], '✥'), // 3800
	(('c', '9'), ['c', 'a', 'a', '3', '9'], '✥'), // 3900
	(('5', 'e'), ['5', 'c', 'c', '6', '6'], '✥'), // 4000
	(('e', '5'), ['e', '6', 'c', 'd', 'd'], '✥'), // 4100
	(('1', '7'), ['9', '3', 'b', 'f', '7'], '✥'), // 4200
	(('0', '0'), ['5', '8', '8', '5', '5'], '✥'), // 4300
	(('2', 'd'), ['2', 'a', 'e', '6', '5'], '✥'), // 4400
	(('f', '3'), ['f', 'b', 'b', '3', '3'], '✥'), // 4500
	(('3', '5'), ['b', 'e', 'e', '6', 'd'], '✥'), // 4600
	(('f', '9'), ['4', 'c', 'c', '9', '1'], '✥'), // 4700
	(('5', '3'), ['5', 'c', '6', 'e', 'b'], '✥'), // 4800
	(('2', '2'), ['a', 'f', 'f', 'a', 'a'], '✥'), // 4900
	(('4', '0'), ['4', '5', '9', '9', '1'], '✥'), // 5000
];

#[must_use]
/// # Panics
/// Panics if the format contains invalid `char`s.
pub fn get_colours(level: Level) -> [Color; 2] {
	let prestige = level.0 / 100;
	let format = LEVEL_FORMAT[min(prestige as usize, LEVEL_FORMAT.len() - 1)];

	[
		Colour::try_from(format.1[0]).unwrap().into(),
		Colour::try_from(format.1[4]).unwrap().into(),
	]
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
pub fn get_level_format(level: Level) -> String {
	let prestige = level.0 / 100;

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
	string.push(format.1[4]);
	string.push(format.2);

	string.push(ESCAPE);
	string.push(format.0 .1);
	string.push(']');

	string
}

#[must_use]
pub fn get_level(xp: u64) -> Level {
	// Level from prestiges, remaining level is from start of a prestige
	let level = 100 * (xp / XP_PER_PRESTIGE);
	let xp = xp % XP_PER_PRESTIGE;

	Level(match xp {
		0..=500 => level,
		501..=1_500 => level + 1,
		1_501..=3_500 => level + 2,
		3_501..=7_000 => level + 3,
		_ => level + 4 + (xp - 7_000) / XP_PER_LEVEL,
	})
}

#[must_use]
pub fn get_xp(level: Level) -> u64 {
	let prestige = level.0 / 100;
	let level = level.0 % 100;

	let mut xp = prestige * XP_PER_PRESTIGE;

	if level > 0 {
		xp += TOTAL_LEVEL_XP[min(level as usize - 1, TOTAL_LEVEL_XP.len() - 1)];
	}

	if level >= 5 {
		xp += (level - 4) * XP_PER_LEVEL;
	}

	xp
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
