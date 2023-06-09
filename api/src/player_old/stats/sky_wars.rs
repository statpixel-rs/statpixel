use crate::player::stats::sky_wars;

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct SkyWars {
	pub coins: i32,
	pub loot_chests: u32,
	pub opals: u32,
	pub heads: u32,
	pub souls: u32,
	pub tokens: u32,
	pub arrows_shot: u32,
	pub arrows_hit: u32,
	pub level_fmt: String,
	pub xp: u64,

	pub solo_normal: sky_wars::SoloNormal,
	pub solo_insane: sky_wars::SoloInsane,
	pub team_normal: sky_wars::TeamNormal,
	pub team_insane: sky_wars::TeamInsane,
}

impl From<SkyWars> for crate::player::stats::sky_wars::SkyWars {
	fn from(value: SkyWars) -> Self {
		Self {
			coins: value.coins,
			loot_chests: value.loot_chests,
			opals: value.opals,
			heads: value.heads,
			souls: value.souls,
			tokens: value.tokens,
			arrows_shot: value.arrows_shot,
			arrows_hit: value.arrows_hit,
			level_fmt: value.level_fmt,
			xp: value.xp,
			solo_normal: value.solo_normal,
			solo_insane: value.solo_insane,
			team_normal: value.team_normal,
			team_insane: value.team_insane,
			..Default::default()
		}
	}
}
