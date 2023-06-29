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

	pub solo_normal: Data,
	pub solo_insane: Data,
	pub team_normal: Data,
	pub team_insane: Data,
	pub mega_double: Data,
	pub mega_normal: Data,
	pub ranked: Data,
}

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct Data {
	pub losses: u32,
	pub wins: u32,
	pub kills: u32,
	pub deaths: u32,
}

impl From<Data> for sky_wars::SoloNormal {
	fn from(value: Data) -> Self {
		Self {
			losses: value.losses,
			wins: value.wins,
			kills: value.kills,
			deaths: value.deaths,
			..Default::default()
		}
	}
}

impl From<Data> for sky_wars::SoloInsane {
	fn from(value: Data) -> Self {
		Self {
			losses: value.losses,
			wins: value.wins,
			kills: value.kills,
			deaths: value.deaths,
		}
	}
}

impl From<Data> for sky_wars::TeamNormal {
	fn from(value: Data) -> Self {
		Self {
			losses: value.losses,
			wins: value.wins,
			kills: value.kills,
			deaths: value.deaths,
			..Default::default()
		}
	}
}

impl From<Data> for sky_wars::TeamInsane {
	fn from(value: Data) -> Self {
		Self {
			losses: value.losses,
			wins: value.wins,
			kills: value.kills,
			deaths: value.deaths,
		}
	}
}

impl From<Data> for sky_wars::MegaDouble {
	fn from(value: Data) -> Self {
		Self {
			losses: value.losses,
			wins: value.wins,
			kills: value.kills,
			deaths: value.deaths,
			..Default::default()
		}
	}
}

impl From<Data> for sky_wars::MegaNormal {
	fn from(value: Data) -> Self {
		Self {
			losses: value.losses,
			wins: value.wins,
			kills: value.kills,
			deaths: value.deaths,
			..Default::default()
		}
	}
}

impl From<Data> for sky_wars::Ranked {
	fn from(value: Data) -> Self {
		Self {
			losses: value.losses,
			wins: value.wins,
			kills: value.kills,
			deaths: value.deaths,
			..Default::default()
		}
	}
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
			level_fmt: value.level_fmt,
			xp: value.xp,
			solo_normal: value.solo_normal.into(),
			solo_insane: value.solo_insane.into(),
			team_normal: value.team_normal.into(),
			team_insane: value.team_insane.into(),
			mega_double: value.mega_double.into(),
			mega_normal: value.mega_normal.into(),
			ranked: value.ranked.into(),
			..Default::default()
		}
	}
}
