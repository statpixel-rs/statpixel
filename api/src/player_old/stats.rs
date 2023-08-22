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
	pub bed_wars: BedWars,
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
			bed_wars: value.bed_wars.into(),
			murder_mystery: value.murder_mystery,
			build_battle: value.build_battle,
			duels: value.duels,
			sky_block: value.sky_block,
			pit: value.pit,
			wool_wars: value.wool_wars,
		}
	}
}

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct BedWars {
	pub coins: u32,
	pub loot_chests: u32,
	pub xp: u64,
	pub win_streak: u32,

	pub solo: bed_wars::Solo,
	pub double: bed_wars::Double,
	pub three: bed_wars::Three,
	pub four: bed_wars::Four,
	pub solo_rush: bed_wars::SoloRush,
	pub double_rush: bed_wars::DoubleRush,
	pub four_rush: bed_wars::FourRush,
	pub solo_ultimate: bed_wars::SoloUltimate,
	pub double_ultimate: bed_wars::DoubleUltimate,
	pub four_ultimate: bed_wars::FourUltimate,
	pub castle: bed_wars::Castle,
	pub double_lucky: bed_wars::DoubleLucky,
	pub four_lucky: bed_wars::FourLucky,
	pub double_voidless: bed_wars::DoubleVoidless,
	pub four_voidless: bed_wars::FourVoidless,
	pub double_armed: bed_wars::DoubleArmed,
	pub four_armed: bed_wars::FourArmed,
	pub double_underworld: bed_wars::DoubleUnderworld,
	pub four_underworld: bed_wars::FourUnderworld,
	pub double_swap: bed_wars::DoubleSwap,
	pub four_swap: bed_wars::FourSwap,
}

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct Practice {
	pub mlg: bed_wars::PracticeMode,
	pub bridging: bed_wars::PracticeMode,
	pub fireball: bed_wars::PracticeMode,
	pub pearl: bed_wars::PracticeMode,
}

impl From<BedWars> for bed_wars::BedWars {
	fn from(value: BedWars) -> Self {
		Self {
			coins: value.coins,
			loot_chests: value.loot_chests,
			xp: value.xp,
			win_streak: value.win_streak,
			solo: value.solo,
			double: value.double,
			three: value.three,
			four: value.four,
			solo_rush: value.solo_rush,
			double_rush: value.double_rush,
			four_rush: value.four_rush,
			solo_ultimate: value.solo_ultimate,
			double_ultimate: value.double_ultimate,
			four_ultimate: value.four_ultimate,
			castle: value.castle,
			double_lucky: value.double_lucky,
			four_lucky: value.four_lucky,
			double_voidless: value.double_voidless,
			four_voidless: value.four_voidless,
			double_armed: value.double_armed,
			four_armed: value.four_armed,
			double_underworld: value.double_underworld,
			four_underworld: value.four_underworld,
			double_swap: value.double_swap,
			four_swap: value.four_swap,
			practice: bed_wars::Practice::default(),
			hotbar: Vec::<bed_wars::HotbarItem>::default(),
			shop: Vec::<bed_wars::ShopItem>::default(),
		}
	}
}
