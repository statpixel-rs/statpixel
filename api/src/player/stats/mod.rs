pub mod arcade;
pub mod arena;
pub mod bed_wars;
pub mod blitz_sg;
pub mod build_battle;
pub mod cops_and_crims;
pub mod duels;
pub mod mega_walls;
pub mod murder_mystery;
pub mod paintball;
pub mod pit;
pub mod quake;
pub mod sky_block;
pub mod sky_wars;
pub mod smash_heroes;
pub mod speed_uhc;
pub mod tnt_games;
pub mod turbo_kart_racers;
pub mod uhc;
pub mod vampire_z;
pub mod walls;
pub mod warlords;
pub mod wool_wars;

use serde::{Deserialize, Deserializer};

#[derive(bincode::Encode, bincode::Decode, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub struct Stats {
	pub quake: quake::Quake,
	pub walls: walls::Walls,
	pub paintball: paintball::Paintball,
	#[serde(rename = "HungerGames")]
	pub blitz_sg: blitz_sg::BlitzSg,
	#[serde(rename = "TNTGames")]
	pub tnt_games: tnt_games::TntGames,
	pub vampire_z: vampire_z::VampireZ,
	#[serde(rename = "Walls3")]
	pub mega_walls: mega_walls::MegaWalls,
	pub arcade: arcade::Arcade,
	pub arena: arena::Arena,
	#[serde(rename = "UHC")]
	pub uhc: uhc::Uhc,
	#[serde(rename = "MCGO")]
	pub cops_and_crims: cops_and_crims::CopsAndCrims,
	#[serde(rename = "Battleground")]
	pub warlords: warlords::Warlords,
	#[serde(rename = "SuperSmash")]
	pub smash_heroes: smash_heroes::SmashHeroes,
	#[serde(rename = "GingerBread")]
	pub turbo_kart_racers: turbo_kart_racers::TurboKartRacers,
	pub sky_wars: sky_wars::SkyWars,
	#[serde(rename = "SpeedUHC")]
	pub speed_uhc: speed_uhc::SpeedUhc,
	#[serde(rename = "Bedwars")]
	pub bed_wars: bed_wars::BedWars,
	pub murder_mystery: murder_mystery::MurderMystery,
	#[serde(rename = "BuildBattle")]
	pub build_battle: build_battle::BuildBattle,
	pub duels: duels::Duels,
	pub sky_block: sky_block::SkyBlock,
	pub pit: pit::Outer,
	#[serde(rename = "WoolGames")]
	pub wool_wars: wool_wars::Outer,
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub(crate) fn from_trunc_f32_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f32 = Deserialize::deserialize(deserializer)?;

	Ok(s as u32)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn from_trunc_f32_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f32 = Deserialize::deserialize(deserializer)?;

	Ok(s as i32)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub(crate) fn from_trunc_f32_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f32 = Deserialize::deserialize(deserializer)?;

	Ok(s as u64)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub(crate) fn from_trunc_f64_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f64 = Deserialize::deserialize(deserializer)?;

	Ok(s as u64)
}
