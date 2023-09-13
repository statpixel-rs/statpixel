use crate::player::stats::*;

// Used as the key to a certain leaderboard
#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub enum Kind {
	Arcade(arcade::ArcadeMode, arcade::ArcadeKind),
	Arena(arena::ArenaMode, arena::ArenaKind),
	BedWars(bed_wars::BedWarsMode, bed_wars::BedWarsKind),
	BlitzSg(blitz_sg::BlitzSgMode, blitz_sg::BlitzSgKind),
	BuildBattle(build_battle::BuildBattleMode, build_battle::BuildBattleKind),
	CopsAndCrims(
		cops_and_crims::CopsAndCrimsMode,
		cops_and_crims::CopsAndCrimsKind,
	),
	Duels(duels::DuelsMode, duels::DuelsKind),
	MegaWalls(mega_walls::MegaWallsMode, mega_walls::MegaWallsKind),
	MurderMystery(
		murder_mystery::MurderMysteryMode,
		murder_mystery::MurderMysteryKind,
	),
	Paintball(paintball::PaintballMode, paintball::PaintballKind),
	Pit(pit::PitMode, pit::PitKind),
	Quake(quake::QuakeMode, quake::QuakeKind),
	SkyWars(sky_wars::SkyWarsMode, sky_wars::SkyWarsKind),
	SmashHeroes(smash_heroes::SmashHeroesMode, smash_heroes::SmashHeroesKind),
	SpeedUhc(speed_uhc::SpeedUhcMode, speed_uhc::SpeedUhcKind),
	TntGames(tnt_games::TntGamesMode, tnt_games::TntGamesKind),
	TurboKartRacers(
		turbo_kart_racers::TurboKartRacersMode,
		turbo_kart_racers::TurboKartRacersKind,
	),
	Uhc(uhc::UhcMode, uhc::UhcKind),
	VampireZ(vampire_z::VampireZMode, vampire_z::VampireZKind),
	Walls(walls::WallsMode, walls::WallsKind),
	Warlords(warlords::WarlordsMode, warlords::WarlordsKind),
	WoolWars(wool_wars::WoolWarsMode, wool_wars::WoolWarsKind),
	Fishing(fishing::FishingMode, fishing::FishingKind),
}

/// # Panics
/// Encoding will never panic
#[must_use]
pub fn encode(id: &Kind) -> Vec<u8> {
	bitcode::encode(&id).unwrap()
}

#[must_use]
pub fn decode(id: &[u8]) -> Option<Kind> {
	bitcode::decode(id).ok()
}

#[derive(Debug)]
pub struct Leaderboard {
	pub kind: Kind,
	pub game: hypixel::game::r#type::Type,
	pub name: String,
	pub display_name: String,
	pub display_name_lower: String,
}
