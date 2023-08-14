use crate::{player::stats::*, seconds::Seconds};

#[derive(bincode::Encode, bincode::Decode, Default)]
pub struct Stats {
	pub quake: quake::Quake,
	pub walls: walls::Walls,
	pub paintball: paintball::Paintball,
	pub blitz_sg: blitz_sg::BlitzSg,
	pub tnt_games: TntGames,
	pub vampire_z: vampire_z::VampireZ,
	pub mega_walls: mega_walls::MegaWalls,
	pub arcade: arcade::Arcade,
	pub arena: arena::Arena,
	pub uhc: uhc::Uhc,
	pub cops_and_crims: cops_and_crims::CopsAndCrims,
	pub warlords: warlords::Warlords,
	pub smash_heroes: smash_heroes::SmashHeroes,
	pub turbo_kart_racers: turbo_kart_racers::TurboKartRacers,
	pub sky_wars: sky_wars::SkyWars,
	pub speed_uhc: speed_uhc::SpeedUhc,
	pub bed_wars: bed_wars::BedWars,
	pub murder_mystery: murder_mystery::MurderMystery,
	pub build_battle: build_battle::BuildBattle,
	pub duels: duels::Duels,
	pub sky_block: sky_block::SkyBlock,
	pub pit: pit::Outer,
	pub wool_wars: wool_wars::Outer,
}

impl From<Stats> for crate::player::stats::Stats {
	fn from(value: Stats) -> Self {
		Self {
			quake: value.quake,
			walls: value.walls,
			paintball: value.paintball,
			blitz_sg: value.blitz_sg,
			tnt_games: value.tnt_games.into(),
			vampire_z: value.vampire_z,
			mega_walls: value.mega_walls,
			arcade: value.arcade,
			arena: value.arena,
			uhc: value.uhc,
			cops_and_crims: value.cops_and_crims,
			warlords: value.warlords,
			smash_heroes: value.smash_heroes,
			turbo_kart_racers: value.turbo_kart_racers,
			sky_wars: value.sky_wars,
			speed_uhc: value.speed_uhc,
			bed_wars: value.bed_wars,
			murder_mystery: value.murder_mystery,
			build_battle: value.build_battle,
			duels: value.duels,
			sky_block: value.sky_block,
			pit: value.pit,
			wool_wars: value.wool_wars,
		}
	}
}

#[derive(bincode::Encode, bincode::Decode, Default)]
pub struct TntGames {
	pub coins: i32,

	pub tnt_run: tnt_games::TntRun,
	pub pvp_run: tnt_games::PvpRun,
	pub bow_spleef: tnt_games::BowSpleef,
	pub wizard: tnt_games::Wizard,
	pub tnt_tag: tnt_games::TntTag,
}

impl From<TntGames> for tnt_games::TntGames {
	fn from(value: TntGames) -> Self {
		Self {
			coins: value.coins,
			tnt_run: value.tnt_run,
			pvp_run: value.pvp_run,
			bow_spleef: value.bow_spleef,
			wizard: value.wizard,
			tnt_tag: value.tnt_tag,
		}
	}
}

#[derive(bincode::Encode, bincode::Decode, Default)]
pub struct BowSpleef {
	pub wins: u32,
	pub deaths: u32,
	pub tags: Seconds,
}

impl From<BowSpleef> for tnt_games::BowSpleef {
	fn from(value: BowSpleef) -> Self {
		Self {
			wins: value.wins,
			deaths: value.deaths,
			tags: u32::try_from(value.tags.0).unwrap(),
		}
	}
}
