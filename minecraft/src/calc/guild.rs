use std::cmp::min;

pub const TOTAL_LEVEL_XP: [u32; 14] = [
	100_000, 250_000, 500_000, 1_000_000, 1_750_000, 2_750_000, 4_000_000, 5_500_000, 7_500_000,
	10_000_000, 12_500_000, 15_000_000, 17_500_000, 20_000_000,
];

pub const XP_PER_LEVEL: u32 = 3_000_000;

#[must_use]
pub fn get_level(xp: u32) -> u32 {
	match xp {
		0..100_000 => 1,
		100_000..250_000 => 2,
		250_000..500_000 => 3,
		500_000..1_000_000 => 4,
		1_000_000..1_750_000 => 5,
		1_750_000..2_750_000 => 6,
		2_750_000..4_000_000 => 7,
		4_000_000..5_500_000 => 8,
		5_500_000..7_500_000 => 9,
		7_500_000..10_000_000 => 10,
		10_000_000..12_500_000 => 11,
		12_500_000..15_000_000 => 12,
		15_000_000..17_500_000 => 13,
		17_500_000..20_000_000 => 14,
		_ => 14 + (xp - 20_000_000) / XP_PER_LEVEL,
	}
}

#[must_use]
pub fn get_xp(level: u32) -> u32 {
	let mut xp = 0;

	if level > 1 {
		xp += TOTAL_LEVEL_XP[min(level as usize - 2, TOTAL_LEVEL_XP.len() - 1)];
	}

	if level >= 15 {
		xp += (level - 14) * XP_PER_LEVEL;
	}

	xp
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
