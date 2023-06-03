#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct BedWars {
	pub coins: i32,
	pub loot_chests: u32,
	pub xp: u64,
	pub solo: crate::player::stats::bed_wars::Solo,
	pub double: crate::player::stats::bed_wars::Double,
	pub three: crate::player::stats::bed_wars::Three,
	pub four: crate::player::stats::bed_wars::Four,
}

impl From<BedWars> for crate::player::stats::bed_wars::BedWars {
	fn from(value: BedWars) -> Self {
		Self {
			coins: value.coins,
			loot_chests: value.loot_chests,
			xp: value.xp,
			solo: value.solo,
			double: value.double,
			three: value.three,
			four: value.four,
			..Default::default()
		}
	}
}
