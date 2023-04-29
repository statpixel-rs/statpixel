const TOTAL_LEVEL_XP: [u32; 12] = [
	0, 20, 70, 150, 250, 500, 1_000, 2_000, 3_500, 6_000, 10_000, 15_000,
];

pub fn get_level(xp: u32) -> u32 {
	for (i, &x) in TOTAL_LEVEL_XP.iter().enumerate() {
		if x > xp {
			return i as u32;
		}
	}

	let xp = xp - TOTAL_LEVEL_XP[11];

	12 + (xp / 10_000)
}

pub fn get_xp(level: u32) -> u32 {
	if level == 0 {
		0
	} else if level < 12 {
		TOTAL_LEVEL_XP[level as usize - 1]
	} else {
		TOTAL_LEVEL_XP[11] + (level - 12) * 10_000
	}
}

pub fn get_level_xp(xp: u32) -> u32 {
	let level = get_level(xp);

	get_xp(level + 1) - get_xp(level)
}

pub fn get_curr_level_xp(xp: u32) -> u32 {
	xp - get_xp(get_level(xp))
}

pub fn get_level_progress(xp: u32) -> f32 {
	let level = get_level(xp);
	let base = get_xp(level);
	let next = get_xp(level + 1);

	(xp - base) as f32 / (next - base) as f32
}
