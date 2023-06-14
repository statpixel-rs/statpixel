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

const BASE: u64 = 10_000;
const GROWTH: f64 = 2_500.;

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
const HALF_GROWTH: u64 = (0.5 * GROWTH) as u64;

#[allow(clippy::cast_precision_loss)]
const REVERSE_PQ_PREFIX: f64 = -(BASE as f64 - 0.5 * GROWTH) / GROWTH;
const REVERSE_CONST: f64 = REVERSE_PQ_PREFIX * REVERSE_PQ_PREFIX;
const GROWTH_DIVIDES_2: f64 = 2. / GROWTH;

#[must_use]
pub fn get_level_format(level: Level) -> String {
	let level = level.0;

	format!("{ESCAPE}6{level}")
}

#[must_use]
pub fn get_colours(_level: Level) -> [Color; 2] {
	[Colour::Gold.into(), Colour::Gold.into()]
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
pub fn get_level(xp: u64) -> Level {
	let xp = xp as f64;

	Level((1. + REVERSE_PQ_PREFIX + (REVERSE_CONST + GROWTH_DIVIDES_2 * xp).sqrt()) as u64)
}

#[must_use]
pub fn get_xp(level: Level) -> u64 {
	(HALF_GROWTH * (level.0 - 2) + BASE) * (level.0 - 1)
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
#[allow(clippy::cast_possible_truncation)]
pub fn get_level_progress(xp: u64) -> f32 {
	let level = get_level(xp);
	let base = get_xp(level);
	let next = get_xp(Level(level.0 + 1));

	((xp - base) as f64 / (next - base) as f64) as f32
}

#[must_use]
pub fn convert(xp: &u64) -> u64 {
	*xp
}
