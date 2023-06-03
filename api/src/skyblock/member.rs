use serde::Deserialize;

use crate::player::stats::from_trunc_f64_to_u64;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Member {
	pub fairy_exchanges: u32,
	pub fairy_souls_collected: u32,
	pub fishing_treasure_caught: u32,
	#[serde(deserialize_with = "from_trunc_f64_to_u64")]
	pub coin_purse: u64,
	#[serde(
		deserialize_with = "crate::player::data::vec_len_to_u32",
		rename = "visited_zones"
	)]
	pub zones_visited: u32,
	#[serde(
		deserialize_with = "crate::player::data::vec_len_to_u32",
		rename = "crafted_generators"
	)]
	pub generators_crafted: u32,
	pub stats: Stats,
	#[serde(flatten)]
	pub skills: Skills,
	pub dungeons: Dungeons,
	pub leveling: Leveling,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Leveling {
	#[serde(rename = "experience")]
	pub xp: u32,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Dungeons {
	#[serde(rename = "dungeon_types")]
	pub types: Types,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Types {
	pub catacombs: Catacombs,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Catacombs {
	#[serde(deserialize_with = "from_trunc_f64_to_u64")]
	pub experience: u64,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Stats {
	#[serde(deserialize_with = "from_trunc_f64_to_u64")]
	pub highest_critical_damage: u64,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Skills {
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_farming"
	)]
	pub farming: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_mining"
	)]
	pub mining: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_combat"
	)]
	pub combat: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_foraging"
	)]
	pub foraging: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_fishing"
	)]
	pub fishing: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_enchanting"
	)]
	pub enchanting: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_alchemy"
	)]
	pub alchemy: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_taming"
	)]
	pub taming: u64,
	// Dungeons is in `dungeons.types.catacombs.experience`
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_carpentry"
	)]
	pub carpentry: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_runecrafting"
	)]
	pub runecrafting: u64,
	#[serde(
		deserialize_with = "from_trunc_f64_to_u64",
		rename = "experience_skill_social2"
	)]
	pub social: u64,
}
