use std::cmp::min;

use skia_safe::Color;

use crate::{colour::Colour, text::ESCAPE};

const TOTAL_LEVEL_XP: [u64; 4] = [500, 1_500, 3_500, 7_000];
const XP_PER_LEVEL: u64 = 5_000;
const XP_PER_PRESTIGE: u64 = 96 * XP_PER_LEVEL + 7_000;

type Format = ((char, char), [char; 5], char);

const LEVEL_FORMAT: [Format; 51] = [
	(('7', '7'), ['7', '7', '7', '7', '7'], '✫'),
	(('f', 'f'), ['f', 'f', 'f', 'f', 'f'], '✫'),
	(('6', '6'), ['6', '6', '6', '6', '6'], '✫'),
	(('b', 'b'), ['b', 'b', 'b', 'b', 'b'], '✫'),
	(('2', '2'), ['2', '2', '2', '2', '2'], '✫'),
	(('3', '3'), ['3', '3', '3', '3', '3'], '✫'),
	(('4', '4'), ['4', '4', '4', '4', '4'], '✫'),
	(('d', 'd'), ['d', 'd', 'd', 'd', 'd'], '✫'),
	(('9', '9'), ['9', '9', '9', '9', '9'], '✫'),
	(('5', '5'), ['5', '5', '5', '5', '5'], '✫'),
	(('c', '5'), ['6', 'e', 'a', 'b', 'd'], '✫'),
	(('7', '7'), ['f', 'f', 'f', 'f', '7'], '✪'),
	(('7', '7'), ['e', 'e', 'e', 'e', '6'], '✪'),
	(('7', '7'), ['b', 'b', 'b', 'b', '3'], '✪'),
	(('7', '7'), ['a', 'a', 'a', 'a', '2'], '✪'),
	(('7', '7'), ['3', '3', '3', '3', '9'], '✪'),
	(('7', '7'), ['c', 'c', 'c', 'c', '4'], '✪'),
	(('7', '7'), ['d', 'd', 'd', 'd', '5'], '✪'),
	(('7', '7'), ['9', '9', '9', '9', '1'], '✪'),
	(('7', '7'), ['5', '5', '5', '5', '8'], '✪'),
	(('8', '8'), ['7', 'f', 'f', '7', '7'], '✪'),
	(('f', '6'), ['f', 'e', 'e', '6', '6'], '⚝'),
	(('6', '3'), ['6', 'f', 'f', 'b', '3'], '⚝'),
	(('5', 'e'), ['5', 'd', 'd', '6', 'e'], '⚝'),
	(('b', '8'), ['b', 'f', 'f', '7', '7'], '⚝'),
	(('f', '2'), ['f', 'a', 'a', '2', '2'], '⚝'),
	(('4', '5'), ['4', 'c', 'c', 'd', 'd'], '⚝'),
	(('e', '8'), ['e', 'f', 'f', '8', '8'], '⚝'),
	(('a', 'e'), ['a', '2', '2', '6', '6'], '⚝'),
	(('b', '1'), ['b', '3', '3', '9', '9'], '⚝'),
	(('e', '4'), ['e', '6', '6', 'c', 'c'], '⚝'),
	(('9', 'e'), ['9', '3', '3', '6', '6'], '✥'),
	(('c', 'c'), ['4', '7', '7', '4', 'c'], '✥'),
	(('9', '4'), ['9', '9', 'd', 'c', 'c'], '✥'),
	(('2', '2'), ['a', 'd', 'd', '5', '5'], '✥'),
	(('c', 'a'), ['c', '4', '4', '2', 'a'], '✥'),
	(('a', '1'), ['a', 'a', 'b', '9', '9'], '✥'),
	(('4', '3'), ['4', 'c', 'd', 'b', '3'], '✥'),
	(('1', '1'), ['1', '9', '5', '5', 'd'], '✥'),
	(('c', '9'), ['c', 'a', 'a', '3', '9'], '✥'),
	(('5', 'e'), ['5', 'c', 'c', '6', '6'], '✥'),
	(('e', '5'), ['e', '6', 'c', 'd', 'd'], '✥'),
	(('1', '7'), ['9', '3', 'b', 'f', '7'], '✥'),
	(('0', '0'), ['5', '8', '8', '5', '5'], '✥'),
	(('2', 'd'), ['2', 'a', 'e', '6', '5'], '✥'),
	(('f', '3'), ['f', 'b', 'b', '3', '3'], '✥'),
	(('3', '5'), ['b', 'e', 'e', '6', 'd'], '✥'),
	(('f', '9'), ['4', 'c', 'c', '9', '1'], '✥'),
	(('5', '3'), ['5', 'c', '6', 'e', 'b'], '✥'),
	(('2', '2'), ['a', 'f', 'f', 'a', 'a'], '✥'),
	(('4', '0'), ['4', '5', '9', '9', '1'], '✥'),
];

#[must_use]
/// # Panics
/// Panics if the format contains invalid `char`s.
pub fn get_colours(level: u64) -> [Color; 2] {
	let prestige = level / 100;
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
pub fn get_level_format(level: u64) -> String {
	let prestige = level / 100;

	let format = LEVEL_FORMAT[min(prestige as usize, LEVEL_FORMAT.len() - 1)];
	let length = (level as f32).log10().ceil() as usize;
	let string = level.to_string();
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

	string.push(format.2);

	string.push(ESCAPE);
	string.push(format.0 .1);
	string.push(']');

	string
}

#[must_use]
pub fn get_level(xp: u64) -> u64 {
	// Level from prestiges, remaining level is from start of a prestige
	let level = 100 * (xp / XP_PER_PRESTIGE);
	let xp = xp % XP_PER_PRESTIGE;

	match xp {
		0..=500 => level,
		501..=1_500 => level + 1,
		1_501..=3_500 => level + 2,
		3_501..=7_000 => level + 3,
		_ => level + 4 + (xp - 7_000) / XP_PER_LEVEL,
	}
}

#[must_use]
pub fn get_xp(level: u64) -> u64 {
	let prestige = level / 100;
	let level = level % 100;

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

	get_xp(level + 1) - get_xp(level)
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
	let next = get_xp(level + 1);

	(xp - base) as f32 / (next - base) as f32
}

#[must_use]
pub fn convert(xp: &u64) -> u64 {
	*xp
}
