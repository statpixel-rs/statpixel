mod bedwars;
mod skywars;

use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Stats {
	#[serde(skip)]
	pub quake: QuakeStats,
	#[serde(skip)]
	pub walls: WallsStats,
	#[serde(skip)]
	pub paintball: PaintballStats,
	#[serde(rename = "HungerGames", skip)]
	pub blitz_sg: BlitzSgStats,
	#[serde(rename = "TNTGames", skip)]
	pub tnt_games: TntGamesStats,
	#[serde(rename = "VampireZ", skip)]
	pub vampire_z: VampireZStats,
	#[serde(rename = "Walls3", skip)]
	pub mega_walls: MegaWallsStats,
	#[serde(skip)]
	pub arcade: ArcadeStats,
	#[serde(skip)]
	pub arena: ArenaStats,
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
	pub sky_wars: skywars::Stats,
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
	pub bed_wars: bedwars::Stats,
	#[serde(skip)]
	pub murder_mystery: MurderMysteryStats,
	#[serde(skip)]
	pub build_battle: BuildBattleStats,
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

#[derive(Deserialize, Default, Debug)]
pub struct QuakeStats;

#[derive(Deserialize, Default, Debug)]
pub struct WallsStats;

#[derive(Deserialize, Default, Debug)]
pub struct PaintballStats;

#[derive(Deserialize, Default, Debug)]
pub struct BlitzSgStats;

#[derive(Deserialize, Default, Debug)]
pub struct TntGamesStats;

#[derive(Deserialize, Default, Debug)]
pub struct VampireZStats;

#[derive(Deserialize, Default, Debug)]
pub struct MegaWallsStats;

#[derive(Deserialize, Default, Debug)]
pub struct ArcadeStats;

#[derive(Deserialize, Default, Debug)]
pub struct ArenaStats;

#[derive(Deserialize, Default, Debug)]
pub struct UhcStats;

#[derive(Deserialize, Default, Debug)]
pub struct CopsAndCrimsStats;

#[derive(Deserialize, Default, Debug)]
pub struct WarlordsStats;

#[derive(Deserialize, Default, Debug)]
pub struct SmashHeroesStats;

#[derive(Deserialize, Default, Debug)]
pub struct TurboKartRacersStats;

#[derive(Deserialize, Default, Debug)]
pub struct HousingStats;

#[derive(Deserialize, Default, Debug)]
pub struct CrazyWallsStats;

#[derive(Deserialize, Default, Debug)]
pub struct SpeedUhcStats;

#[derive(Deserialize, Default, Debug)]
pub struct SkyClashStats;

#[derive(Deserialize, Default, Debug)]
pub struct ClassicStats;

#[derive(Deserialize, Default, Debug)]
pub struct PrototypeStats;

#[derive(Deserialize, Default, Debug)]
pub struct MurderMysteryStats;

#[derive(Deserialize, Default, Debug)]
pub struct BuildBattleStats;

#[derive(Deserialize, Default, Debug)]
pub struct DuelsStats;

#[derive(Deserialize, Default, Debug)]
pub struct SkyBlockStats;

#[derive(Deserialize, Default, Debug)]
pub struct PitStats;

#[derive(Deserialize, Default, Debug)]
pub struct ReplayStats;

#[derive(Deserialize, Default, Debug)]
pub struct SmpStats;

#[derive(Deserialize, Default, Debug)]
pub struct WoolWarsStats;
