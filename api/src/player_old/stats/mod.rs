mod arena;
mod bed_wars;
mod sky_wars;
mod warlords;

use crate::player::stats;

#[derive(bincode::Encode, bincode::Decode, Default)]
pub struct Stats {
	pub quake: stats::quake::Quake,
	pub walls: stats::walls::Walls,
	pub paintball: stats::paintball::Paintball,
	pub blitz_sg: stats::blitz_sg::BlitzSg,
	pub tnt_games: stats::tnt_games::TntGames,
	pub vampire_z: stats::vampire_z::VampireZ,
	pub mega_walls: stats::mega_walls::MegaWalls,
	pub arcade: stats::arcade::Arcade,
	pub arena: arena::Arena,
	pub uhc: stats::uhc::Uhc,
	pub cops_and_crims: stats::cops_and_crims::CopsAndCrims,
	pub warlords: warlords::Warlords,
	pub smash_heroes: stats::smash_heroes::SmashHeroes,
	pub turbo_kart_racers: stats::turbo_kart_racers::TurboKartRacers,
	pub sky_wars: sky_wars::SkyWars,
	pub speed_uhc: stats::speed_uhc::SpeedUhc,
	pub bed_wars: bed_wars::BedWars,
	pub murder_mystery: stats::murder_mystery::MurderMystery,
	pub build_battle: stats::build_battle::BuildBattle,
	pub duels: stats::duels::Duels,
	pub sky_block: stats::sky_block::SkyBlock,
	pub pit: stats::pit::Outer,
	pub wool_wars: stats::wool_wars::Outer,
}

impl From<Stats> for crate::player::stats::Stats {
	fn from(value: Stats) -> Self {
		Self {
			quake: value.quake,
			walls: value.walls,
			paintball: value.paintball,
			blitz_sg: value.blitz_sg,
			tnt_games: value.tnt_games,
			vampire_z: value.vampire_z,
			mega_walls: value.mega_walls,
			arcade: value.arcade,
			arena: value.arena.into(),
			uhc: value.uhc,
			cops_and_crims: value.cops_and_crims,
			warlords: value.warlords.into(),
			smash_heroes: value.smash_heroes,
			turbo_kart_racers: value.turbo_kart_racers,
			sky_wars: value.sky_wars.into(),
			speed_uhc: value.speed_uhc,
			bed_wars: value.bed_wars.into(),
			murder_mystery: value.murder_mystery,
			build_battle: value.build_battle,
			duels: value.duels,
			pit: value.pit,
			wool_wars: value.wool_wars,
			sky_block: value.sky_block,
		}
	}
}
