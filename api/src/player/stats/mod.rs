pub mod arcade;
pub mod arena;
pub mod bed_wars;
pub mod blitz_sg;
pub mod build_battle;
pub mod sky_wars;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
#[serde(rename_all = "PascalCase", default)]
pub struct Stats {
	#[serde(skip)]
	pub quake: QuakeStats,
	#[serde(skip)]
	pub walls: WallsStats,
	#[serde(skip)]
	pub paintball: PaintballStats,
	#[serde(rename = "HungerGames")]
	pub blitz_sg: blitz_sg::BlitzSg,
	#[serde(rename = "TNTGames", skip)]
	pub tnt_games: TntGamesStats,
	#[serde(rename = "VampireZ", skip)]
	pub vampire_z: VampireZStats,
	#[serde(rename = "Walls3", skip)]
	pub mega_walls: MegaWallsStats,
	pub arcade: arcade::Arcade,
	pub arena: arena::Arena,
	#[serde(rename = "UHC", skip)]
	pub uhc: UhcStats,
	#[serde(rename = "MCGO", skip)]
	pub cops_and_crims: CopsAndCrimsStats,
	#[serde(rename = "Battleground", skip)]
	pub warlords: WarlordsStats,
	#[serde(rename = "SuperSmash", skip)]
	pub smash_heroes: SmashHeroesStats,
	#[serde(rename = "GingerBread", skip)]
	pub turbo_kart_racers: TurboKartRacersStats,
	#[serde(skip)]
	pub housing: HousingStats,
	pub sky_wars: sky_wars::SkyWars,
	#[serde(rename = "TrueCombat", skip)]
	pub crazy_walls: CrazyWallsStats,
	#[serde(rename = "SpeedUHC", skip)]
	pub speed_uhc: SpeedUhcStats,
	#[serde(skip)]
	pub sky_clash: SkyClashStats,
	#[serde(rename = "Legacy", skip)]
	pub classic: ClassicStats,
	#[serde(skip)]
	pub prototype: PrototypeStats,
	#[serde(rename = "Bedwars")]
	pub bed_wars: bed_wars::BedWars,
	#[serde(skip)]
	pub murder_mystery: MurderMysteryStats,
	#[serde(rename = "BuildBattle")]
	pub build_battle: build_battle::BuildBattle,
	#[serde(skip)]
	pub duels: DuelsStats,
	#[serde(skip)]
	pub sky_block: SkyBlockStats,
	#[serde(skip)]
	pub pit: PitStats,
	#[serde(skip)]
	pub replay: ReplayStats,
	#[serde(rename = "SMP", skip)]
	pub smp: SmpStats,
	#[serde(rename = "WoolGames", skip)]
	pub wool_wars: WoolWarsStats,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct QuakeStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct WallsStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct PaintballStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TntGamesStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct VampireZStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct MegaWallsStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct UhcStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct CopsAndCrimsStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct WarlordsStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct SmashHeroesStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TurboKartRacersStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct HousingStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct CrazyWallsStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct SpeedUhcStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct SkyClashStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ClassicStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct PrototypeStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct MurderMysteryStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DuelsStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct SkyBlockStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct PitStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ReplayStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct SmpStats;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct WoolWarsStats;

pub(crate) fn from_trunc_f32_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f32 = Deserialize::deserialize(deserializer)?;

	Ok(s as u32)
}

pub(crate) fn from_trunc_f32_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f32 = Deserialize::deserialize(deserializer)?;

	Ok(s as u64)
}
