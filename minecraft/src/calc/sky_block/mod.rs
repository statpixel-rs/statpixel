use std::cmp::min;

use skia_safe::Color;

use crate::{colour::Colour, text::ESCAPE};

pub mod skills;

const LEVEL_FORMAT: [Colour; 10] = [
	Colour::Gray,
	Colour::White,
	Colour::Green,
	Colour::Blue,
	Colour::DarkPurple,
	Colour::Gold,
	Colour::LightPurple,
	Colour::Aqua,
	Colour::Red,
	Colour::DarkRed,
];
const XP_PER_LEVEL: u32 = 100;

#[must_use]
/// # Panics
/// Panics if the format contains invalid `char`s.
pub fn get_colours(level: u32) -> [Color; 2] {
	let prestige = level / 40;
	let colour = LEVEL_FORMAT[min(prestige as usize, LEVEL_FORMAT.len() - 1)];

	[colour.into(), colour.into()]
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
pub fn get_level_format(level: u32) -> String {
	let prestige = level / 40;
	let colour = LEVEL_FORMAT[min(prestige as usize, LEVEL_FORMAT.len() - 1)];
	let level = level.to_string();

	let mut string = String::with_capacity(level.len() + 3 * 2 + 2);

	string.push(ESCAPE);
	string.push(Colour::DarkGray.into());
	string.push('[');

	string.push(ESCAPE);
	string.push(colour.into());
	string.push_str(&level);

	string.push(ESCAPE);
	string.push(Colour::DarkGray.into());
	string.push(']');

	string
}

#[must_use]
pub fn get_level(xp: u32) -> u32 {
	xp / XP_PER_LEVEL
}

#[must_use]
pub fn get_xp(level: u32) -> u32 {
	level * XP_PER_LEVEL
}

#[must_use]
pub fn get_level_xp(xp: u32) -> u32 {
	let level = get_level(xp);

	get_xp(level + 1) - get_xp(level)
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
	let next = get_xp(level + 1);

	(xp - base) as f32 / (next - base) as f32
}
