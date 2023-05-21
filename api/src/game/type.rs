use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(
	Deserialize, Serialize, bincode::Encode, bincode::Decode, Debug, Clone, Copy, PartialEq,
)]
#[serde(try_from = "&str")]
pub enum Type {
	Limbo = 0,
	Lobby = 1,
	Quake = 2,
	Walls = 3,
	Paintball = 4,
	BlitzSg = 5,
	TntGames = 6,
	VampireZ = 7,
	MegaWalls = 13,
	Arcade = 14,
	Arena = 17,
	Uhc = 20,
	CopsAndCrims = 21,
	Warlords = 23,
	SmashHeroes = 24,
	TurboKartRacers = 25,
	Housing = 26,
	SkyWars = 51,
	CrazyWalls = 52,
	SpeedUhc = 54,
	SkyClash = 55,
	Classic = 56,
	Prototype = 57,
	BedWars = 58,
	MurderMystery = 59,
	BuildBattle = 60,
	Duels = 61,
	SkyBlock = 63,
	Pit = 64,
	Replay = 65,
	Smp = 67,
	WoolWars = 68,
}

impl Type {
	#[must_use]
	pub fn as_database_name(&self) -> &str {
		match self {
			Self::Limbo => "Limbo",
			Self::Lobby => "Lobby",
			Self::Quake => "Quake",
			Self::Walls => "Walls",
			Self::Paintball => "Paintball",
			Self::BlitzSg => "HungerGames",
			Self::TntGames => "TNTGames",
			Self::VampireZ => "VampireZ",
			Self::MegaWalls => "Walls3",
			Self::Arcade => "Arcade",
			Self::Arena => "Arena",
			Self::Uhc => "UHC",
			Self::CopsAndCrims => "MCGO",
			Self::Warlords => "Battleground",
			Self::SmashHeroes => "SuperSmash",
			Self::TurboKartRacers => "GingerBread",
			Self::Housing => "Housing",
			Self::SkyWars => "SkyWars",
			Self::CrazyWalls => "TrueCombat",
			Self::SpeedUhc => "SpeedUHC",
			Self::SkyClash => "SkyClash",
			Self::Classic => "Legacy",
			Self::Prototype => "Prototype",
			Self::BedWars => "Bedwars",
			Self::MurderMystery => "MurderMystery",
			Self::BuildBattle => "BuildBattle",
			Self::Duels => "Duels",
			Self::SkyBlock => "SkyBlock",
			Self::Pit => "Pit",
			Self::Replay => "Replay",
			Self::Smp => "SMP",
			Self::WoolWars => "WoolGames",
		}
	}

	#[must_use]
	pub fn as_clean_cow(&self) -> Cow<'static, str> {
		Cow::Borrowed(self.as_clean_name())
	}

	#[must_use]
	pub fn as_short_clean_cow(&self) -> Cow<'static, str> {
		Cow::Borrowed(self.as_short_clean_name())
	}

	#[must_use]
	pub fn as_clean_name(&self) -> &'static str {
		match self {
			Self::Limbo => "Limbo",
			Self::Lobby => "Lobby",
			Self::Quake => "Quake",
			Self::Walls => "Walls",
			Self::Paintball => "Paintball",
			Self::BlitzSg => "Blitz Survival Games",
			Self::TntGames => "TNT Games",
			Self::VampireZ => "VampireZ",
			Self::MegaWalls => "Mega Walls",
			Self::Arcade => "Arcade",
			Self::Arena => "Arena",
			Self::Uhc => "UHC Champions",
			Self::CopsAndCrims => "Cops and Crims",
			Self::Warlords => "Warlords",
			Self::SmashHeroes => "Smash Heroes",
			Self::TurboKartRacers => "Turbo Kart Racers",
			Self::Housing => "Housing",
			Self::SkyWars => "SkyWars",
			Self::CrazyWalls => "Crazy Walls",
			Self::SpeedUhc => "Speed UHC",
			Self::SkyClash => "SkyClash",
			Self::Classic => "Classic Games",
			Self::Prototype => "Prototype",
			Self::BedWars => "Bed Wars",
			Self::MurderMystery => "Murder Mystery",
			Self::BuildBattle => "Build Battle",
			Self::Duels => "Duels",
			Self::SkyBlock => "SkyBlock",
			Self::Pit => "Pit",
			Self::Replay => "Replay",
			Self::Smp => "SMP",
			Self::WoolWars => "Wool Wars",
		}
	}

	#[must_use]
	pub fn as_short_clean_name(&self) -> &'static str {
		match self {
			Self::Limbo => "Limbo",
			Self::Lobby => "Lobby",
			Self::Quake => "Quake",
			Self::Walls => "Walls",
			Self::Paintball => "Paintball",
			Self::BlitzSg => "Blitz SG",
			Self::TntGames => "TNT Games",
			Self::VampireZ => "VampireZ",
			Self::MegaWalls => "Mega Walls",
			Self::Arcade => "Arcade",
			Self::Arena => "Arena",
			Self::Uhc => "UHC",
			Self::CopsAndCrims => "CnC",
			Self::Warlords => "Warlords",
			Self::SmashHeroes => "Smash Heroes",
			Self::TurboKartRacers => "TKR",
			Self::Housing => "Housing",
			Self::SkyWars => "SkyWars",
			Self::CrazyWalls => "Crazy Walls",
			Self::SpeedUhc => "Speed UHC",
			Self::SkyClash => "SkyClash",
			Self::Classic => "Classic",
			Self::Prototype => "Prototype",
			Self::BedWars => "Bed Wars",
			Self::MurderMystery => "MM",
			Self::BuildBattle => "BB",
			Self::Duels => "Duels",
			Self::SkyBlock => "SkyBlock",
			Self::Pit => "Pit",
			Self::Replay => "Replay",
			Self::Smp => "SMP",
			Self::WoolWars => "Wool Wars",
		}
	}

	#[must_use]
	pub fn as_image_bytes(&self) -> Option<&'static [u8]> {
		Some(match self {
			Self::Quake => include_bytes!("../../../assets/games/quakecraft.png"),
			Self::Walls => include_bytes!("../../../assets/games/walls.png"),
			Self::Paintball => include_bytes!("../../../assets/games/paintball.png"),
			Self::BlitzSg => include_bytes!("../../../assets/games/blitz_sg.png"),
			Self::TntGames => include_bytes!("../../../assets/games/tntgames.png"),
			Self::VampireZ => include_bytes!("../../../assets/games/vampirez.png"),
			Self::MegaWalls => include_bytes!("../../../assets/games/megawalls.png"),
			Self::Arcade => include_bytes!("../../../assets/games/arcade.png"),
			Self::Arena => include_bytes!("../../../assets/games/arena.png"),
			Self::Uhc => include_bytes!("../../../assets/games/uhc.png"),
			Self::CopsAndCrims => include_bytes!("../../../assets/games/copsandcrims.png"),
			Self::Warlords => include_bytes!("../../../assets/games/warlords.png"),
			Self::SmashHeroes => include_bytes!("../../../assets/games/smash.png"),
			Self::TurboKartRacers => include_bytes!("../../../assets/games/turbokartracers.png"),
			Self::SkyWars => include_bytes!("../../../assets/games/skywars.png"),
			Self::BedWars => include_bytes!("../../../assets/games/bedwars.png"),
			Self::MurderMystery => include_bytes!("../../../assets/games/murdermystery.png"),
			Self::BuildBattle => include_bytes!("../../../assets/games/buildbattle.png"),
			Self::Duels => include_bytes!("../../../assets/games/duels.png"),
			Self::SkyClash => include_bytes!("../../../assets/games/skyclash.png"),
			Self::SpeedUhc => include_bytes!("../../../assets/games/speeduhc.png"),
			Self::Smp => include_bytes!("../../../assets/games/smp.png"),
			Self::CrazyWalls => include_bytes!("../../../assets/games/crazywalls.png"),
			Self::SkyBlock => include_bytes!("../../../assets/games/skyblock.png"),
			Self::Housing => include_bytes!("../../../assets/games/housing.png"),
			Self::Pit => include_bytes!("../../../assets/games/thepit.png"),
			Self::WoolWars => include_bytes!("../../../assets/games/woolwars.png"),
			_ => return None,
		})
	}
}

impl TryFrom<&str> for Type {
	type Error = &'static str;

	fn try_from(name: &str) -> Result<Self, Self::Error> {
		match name {
			"LIMBO" => Ok(Self::Limbo),
			"LOBBY" => Ok(Self::Lobby),
			"QUAKECRAFT" => Ok(Self::Quake),
			"WALLS" => Ok(Self::Walls),
			"PAINTBALL" => Ok(Self::Paintball),
			"SURVIVAL_GAMES" => Ok(Self::BlitzSg),
			"TNTGAMES" => Ok(Self::TntGames),
			"VAMPIREZ" => Ok(Self::VampireZ),
			"WALLS3" => Ok(Self::MegaWalls),
			"ARCADE" => Ok(Self::Arcade),
			"ARENA" => Ok(Self::Arena),
			"UHC" => Ok(Self::Uhc),
			"MCGO" => Ok(Self::CopsAndCrims),
			"BATTLEGROUND" => Ok(Self::Warlords),
			"SUPER_SMASH" => Ok(Self::SmashHeroes),
			"GINGERBREAD" => Ok(Self::TurboKartRacers),
			"HOUSING" => Ok(Self::Housing),
			"SKYWARS" => Ok(Self::SkyWars),
			"TRUE_COMBAT" => Ok(Self::CrazyWalls),
			"SPEED_UHC" => Ok(Self::SpeedUhc),
			"SKYCLASH" => Ok(Self::SkyClash),
			"LEGACY" => Ok(Self::Classic),
			"PROTOTYPE" => Ok(Self::Prototype),
			"BEDWARS" => Ok(Self::BedWars),
			"MURDER_MYSTERY" => Ok(Self::MurderMystery),
			"BUILD_BATTLE" => Ok(Self::BuildBattle),
			"DUELS" => Ok(Self::Duels),
			"SKYBLOCK" => Ok(Self::SkyBlock),
			"PIT" => Ok(Self::Pit),
			"REPLAY" => Ok(Self::Replay),
			"SMP" => Ok(Self::Smp),
			"WOOL_GAMES" => Ok(Self::WoolWars),
			_ => Err("invalid game type"),
		}
	}
}
