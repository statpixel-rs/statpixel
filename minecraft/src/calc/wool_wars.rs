use std::cmp::min;

use skia_safe::Color;

use crate::{colour::Colour, text::ESCAPE};

const TOTAL_LEVEL_XP: [u64; 4] = [1_000, 3_000, 6_000, 10_000];
const XP_PER_LEVEL: u64 = 5_000;
const XP_PER_PRESTIGE: u64 = 96 * XP_PER_LEVEL + 10_000;

type Format = ((char, char), [char; 5], char);

const LEVEL_FORMAT: [Format; 11] = [
	(('7', '7'), ['7', '7', '7', '7', '7'], '❤'),
	(('f', 'f'), ['f', 'f', 'f', 'f', 'f'], '✙'),
	(('c', 'c'), ['c', 'c', 'c', 'c', 'c'], '✫'),
	(('6', '6'), ['6', '6', '6', '6', '6'], '✈'),
	(('e', 'e'), ['e', 'e', 'e', 'e', 'e'], '✠'),
	(('a', 'a'), ['a', 'a', 'a', 'a', 'a'], '♔'),
	(('3', '3'), ['3', '3', '3', '3', '3'], '⚡'),
	(('5', '5'), ['5', '5', '5', '5', '5'], '☢'),
	(('d', 'd'), ['d', 'd', 'd', 'd', 'd'], '✏'),
	(('c', 'c'), ['c', 'c', 'c', 'c', 'c'], '☯'),
	(('0', '0'), ['f', 'f', 'f', 'f', 'f'], '⚛'),
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
	let length = (level as f32).log10().ceil() as u8;
	let string = level.to_string();
	let chars = string.chars();

	// char, escape, colour, + 3 for each bracket, + star
	let mut string = String::with_capacity((length * 3 + 6 + 1) as usize);

	string.push(ESCAPE);
	string.push(format.0 .0);
	string.push('[');

	for (i, c) in chars.enumerate() {
		string.push(ESCAPE);
		string.push(format.1[i]);
		string.push(c);
	}

	string.push(ESCAPE);
	string.push(format.1[string.len() % 5]);
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
		0..1_000 => level + 1,
		1_000..3_000 => level + 2,
		3_000..6_000 => level + 3,
		6_000..10_000 => level + 4,
		_ => level + 5 + (xp - 10_000) / XP_PER_LEVEL,
	}
}

#[must_use]
pub fn get_xp(level: u64) -> u64 {
	let prestige = level / 100;
	let level = level % 100;

	let mut xp = prestige * XP_PER_PRESTIGE;

	if level > 1 {
		xp += TOTAL_LEVEL_XP[min(level as usize - 2, TOTAL_LEVEL_XP.len() - 1)];
	}

	if level >= 6 {
		xp += (level - 5) * XP_PER_LEVEL;
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
