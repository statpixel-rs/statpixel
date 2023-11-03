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
	pub arcade: Arcade,
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
	pub fishing: fishing::Outer,
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
			arcade: value.arcade.into(),
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
			fishing: value.fishing,
		}
	}
}

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct Arcade {
	pub coins: u32,
	pub mystery_gifts: u32,
	pub party: arcade::Party,
	pub santa_says: arcade::SantaSays,
	pub simon_says: arcade::SimonSays,
	pub mini_walls: arcade::MiniWalls,
	pub soccer: arcade::Soccer,
	pub one_in_the_quiver: arcade::OneInTheQuiver,
	pub ender_spleef: arcade::EnderSpleef,
	pub farm_hunt: arcade::FarmHunt,
	pub dragon_wars: arcade::DragonWars,
	pub blocking_dead: arcade::BlockingDead,
	pub zombies: arcade::Zombies,
	pub zombies_bad_blood: arcade::ZombiesBadBlood,
	pub zombies_dead_end: arcade::ZombiesDeadEnd,
	pub pixel_painters: arcade::PixelPainters,
	pub hole_in_the_wall: arcade::HoleInTheWall,
	pub throw_out: arcade::ThrowOut,
	pub easter_simulator: arcade::EasterSimulator,
	pub scuba_simulator: arcade::ScubaSimulator,
	pub halloween_simulator: arcade::HalloweenSimulator,
	pub grinch_simulator: arcade::GrinchSimulator,
	// pub dropper: Dropper,
	// pub pixel_party: PixelParty,
}

impl From<Arcade> for arcade::Arcade {
	fn from(value: Arcade) -> Self {
		Self {
			coins: value.coins,
			mystery_gifts: value.mystery_gifts,
			party: value.party,
			santa_says: value.santa_says,
			simon_says: value.simon_says,
			mini_walls: value.mini_walls,
			soccer: value.soccer,
			one_in_the_quiver: value.one_in_the_quiver,
			ender_spleef: value.ender_spleef,
			farm_hunt: value.farm_hunt,
			dragon_wars: value.dragon_wars,
			blocking_dead: value.blocking_dead,
			zombies: value.zombies,
			zombies_bad_blood: value.zombies_bad_blood,
			zombies_dead_end: value.zombies_dead_end,
			pixel_painters: value.pixel_painters,
			hole_in_the_wall: value.hole_in_the_wall,
			throw_out: value.throw_out,
			easter_simulator: value.easter_simulator,
			scuba_simulator: value.scuba_simulator,
			halloween_simulator: value.halloween_simulator,
			grinch_simulator: value.grinch_simulator,
			dropper: arcade::Dropper::default(),
			pixel_party: arcade::PixelParty::default(),
		}
	}
}
