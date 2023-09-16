use crate::player::stats::*;

#[derive(bincode::Encode, bincode::Decode, Default)]
pub struct Stats {
	pub quake: quake::Quake,
	pub walls: walls::Walls,
	pub paintball: paintball::Paintball,
	pub blitz_sg: blitz_sg::BlitzSg,
	pub tnt_games: tnt_games::TntGames,
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
			tnt_games: value.tnt_games,
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
			fishing: fishing::Outer::default(),
		}
	}
}
