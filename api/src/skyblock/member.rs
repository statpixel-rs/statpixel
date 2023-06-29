use serde::Deserialize;

use super::{essence::Essence, pet::Pet};
use crate::nbt::inventory::Inventory;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Member {
	pub fairy_exchanges: u32,
	pub fairy_souls_collected: u32,
	pub fishing_treasure_caught: u32,
	#[serde(deserialize_with = "crate::de::from::f64_to_u64")]
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
	#[serde(flatten)]
	pub essence: Essence,
	#[serde(with = "crate::de::vec_map", rename = "sacks_counts")]
	pub sacks: Vec<(String, u32)>,
	pub dungeons: Dungeons,
	pub leveling: Leveling,
	#[serde(
		deserialize_with = "crate::nbt::from_data_opt",
		rename = "inv_contents"
	)]
	pub inventory: Option<Inventory<true>>,
	#[serde(
		deserialize_with = "crate::nbt::from_data_opt",
		rename = "ender_chest_contents"
	)]
	pub ender_chest: Option<Inventory>,
	#[serde(deserialize_with = "crate::nbt::from_data_opt")]
	pub talisman_bag: Option<Inventory>,
	#[serde(deserialize_with = "crate::nbt::from_data_opt")]
	pub quiver: Option<Inventory>,
	#[serde(deserialize_with = "crate::nbt::from_data_opt")]
	pub fishing_bag: Option<Inventory>,
	#[serde(deserialize_with = "crate::nbt::from_data_opt")]
	pub potion_bag: Option<Inventory>,
	#[serde(
		deserialize_with = "crate::nbt::from_data_opt",
		rename = "equippment_contents"
	)]
	pub equipment: Option<Inventory>,
	#[serde(
		deserialize_with = "crate::nbt::from_data_opt",
		rename = "wardrobe_contents"
	)]
	pub wardrobe: Option<Inventory>,
	#[serde(
		deserialize_with = "crate::nbt::from_data_opt",
		rename = "candy_inventory_contents"
	)]
	pub candy: Option<Inventory>,
	#[serde(
		deserialize_with = "crate::nbt::from_data_opt",
		rename = "personal_vault_contents"
	)]
	pub vault: Option<Inventory>,
	pub pets: Option<Vec<Pet>>,
	#[serde(deserialize_with = "crate::nbt::from_data_opt", rename = "inv_armor")]
	pub armour: Option<Inventory>,
	#[serde(
		with = "crate::de::vec_map_inventory",
		rename = "backpack_contents",
		default
	)]
	pub backpack: Vec<Inventory>,
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
	#[serde(deserialize_with = "crate::de::from::f64_to_u64")]
	pub experience: u64,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Stats {
	#[serde(deserialize_with = "crate::de::from::f64_to_u64")]
	pub highest_critical_damage: u64,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Skills {
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_farming"
	)]
	pub farming: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_mining"
	)]
	pub mining: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_combat"
	)]
	pub combat: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_foraging"
	)]
	pub foraging: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_fishing"
	)]
	pub fishing: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_enchanting"
	)]
	pub enchanting: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_alchemy"
	)]
	pub alchemy: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_taming"
	)]
	pub taming: u64,
	// Dungeons is in `dungeons.types.catacombs.experience`
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_carpentry"
	)]
	pub carpentry: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_runecrafting"
	)]
	pub runecrafting: u64,
	#[serde(
		deserialize_with = "crate::de::from::f64_to_u64",
		rename = "experience_skill_social2"
	)]
	pub social: u64,
}
