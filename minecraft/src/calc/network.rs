use skia_safe::Color;

use crate::{colour::Colour, text::parse::ESCAPE};

const BASE: u64 = 10_000;
const GROWTH: f32 = 2_500.;

const HALF_GROWTH: u64 = (0.5 * GROWTH) as u64;

const REVERSE_PQ_PREFIX: f32 = -(BASE as f32 - 0.5 * GROWTH) / GROWTH;
const REVERSE_CONST: f32 = REVERSE_PQ_PREFIX * REVERSE_PQ_PREFIX;
const GROWTH_DIVIDES_2: f32 = 2. / GROWTH;

pub fn get_level_format(level: u64) -> String {
	format!("{ESCAPE}6{level}")
}

pub fn get_colours(_level: u64) -> [Color; 2] {
	[Colour::Gold.into(), Colour::Gold.into()]
}

pub fn get_level(xp: u64) -> u64 {
	let xp = xp as f32;

	(1. + REVERSE_PQ_PREFIX + (REVERSE_CONST + GROWTH_DIVIDES_2 * xp).sqrt()) as u64
}

pub fn get_xp(level: u64) -> u64 {
	(HALF_GROWTH * (level - 2) + BASE) * (level - 1)
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
