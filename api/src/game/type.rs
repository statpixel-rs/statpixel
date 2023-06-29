use std::borrow::Cow;

use crate::image::Image;
use crate::include_image;
use minecraft::{text::parse::minecraft_text, text::Text};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(
	Deserialize,
	Serialize,
	bincode::Encode,
	bincode::Decode,
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
)]
#[serde(from = "&str")]
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
	Unknown = 127,
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
			Self::Unknown => "Unknown",
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
			Self::Unknown => "Unknown",
		}
	}

	#[must_use]
	pub fn as_clean_name_lower(&self) -> &'static str {
		match self {
			Self::Limbo => "limbo",
			Self::Lobby => "lobby",
			Self::Quake => "quake",
			Self::Walls => "walls",
			Self::Paintball => "paintball",
			Self::BlitzSg => "blitz survival games",
			Self::TntGames => "tnt games",
			Self::VampireZ => "vampirez",
			Self::MegaWalls => "mega walls",
			Self::Arcade => "arcade",
			Self::Arena => "arena",
			Self::Uhc => "uhc champions",
			Self::CopsAndCrims => "cops and crims",
			Self::Warlords => "warlords",
			Self::SmashHeroes => "smash heroes",
			Self::TurboKartRacers => "turbo kart racers",
			Self::Housing => "housing",
			Self::SkyWars => "skywars",
			Self::CrazyWalls => "crazy walls",
			Self::SpeedUhc => "speed uhc",
			Self::SkyClash => "skyclash",
			Self::Classic => "classic games",
			Self::Prototype => "prototype",
			Self::BedWars => "bed wars",
			Self::MurderMystery => "murder mystery",
			Self::BuildBattle => "build battle",
			Self::Duels => "duels",
			Self::SkyBlock => "skyblock",
			Self::Pit => "pit",
			Self::Replay => "replay",
			Self::Smp => "smp",
			Self::WoolWars => "wool wars",
			Self::Unknown => "unknown",
		}
	}

	#[must_use]
	pub fn try_from_clean_name(value: &str) -> Option<Self> {
		Some(match value {
			"Limbo" => Self::Limbo,
			"Lobby" => Self::Lobby,
			"Quake" => Self::Quake,
			"Walls" => Self::Walls,
			"Paintball" => Self::Paintball,
			"Blitz Survival Games" => Self::BlitzSg,
			"TNT Games" => Self::TntGames,
			"VampireZ" => Self::VampireZ,
			"Mega Walls" => Self::MegaWalls,
			"Arcade" => Self::Arcade,
			"Arena" => Self::Arena,
			"UHC Champions" => Self::Uhc,
			"Cops and Crims" => Self::CopsAndCrims,
			"Warlords" => Self::Warlords,
			"Smash Heroes" => Self::SmashHeroes,
			"Turbo Kart Racers" => Self::TurboKartRacers,
			"Housing" => Self::Housing,
			"SkyWars" => Self::SkyWars,
			"Crazy Walls" => Self::CrazyWalls,
			"Speed UHC" => Self::SpeedUhc,
			"SkyClash" => Self::SkyClash,
			"Classic Games" => Self::Classic,
			"Prototype" => Self::Prototype,
			"Bed Wars" => Self::BedWars,
			"Murder Mystery" => Self::MurderMystery,
			"Build Battle" => Self::BuildBattle,
			"Duels" => Self::Duels,
			"SkyBlock" => Self::SkyBlock,
			"Pit" => Self::Pit,
			"Replay" => Self::Replay,
			"SMP" => Self::Smp,
			"Wool Wars" => Self::WoolWars,
			_ => return None,
		})
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
			Self::Unknown => "Unknown",
		}
	}

	#[must_use]
	#[allow(clippy::too_many_lines)]
	pub fn as_image_bytes(&self) -> Option<&Image> {
		Some(match self {
			Self::Quake => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/quakecraft.png");
				&IMAGE
			}
			Self::Walls => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/walls.png");
				&IMAGE
			}
			Self::Paintball => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/paintball.png");
				&IMAGE
			}
			Self::BlitzSg => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/blitz_sg.png");
				&IMAGE
			}
			Self::TntGames => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/tntgames.png");
				&IMAGE
			}
			Self::VampireZ => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/vampirez.png");
				&IMAGE
			}
			Self::MegaWalls => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/megawalls.png");
				&IMAGE
			}
			Self::Arcade => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/arcade.png");
				&IMAGE
			}
			Self::Arena => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/arena.png");
				&IMAGE
			}
			Self::Uhc => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/uhc.png");
				&IMAGE
			}
			Self::CopsAndCrims => {
				static IMAGE: Lazy<Image> =
					include_image!("../../../assets/games/copsandcrims.png");
				&IMAGE
			}
			Self::Warlords => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/warlords.png");
				&IMAGE
			}
			Self::SmashHeroes => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/smash.png");
				&IMAGE
			}
			Self::TurboKartRacers => {
				static IMAGE: Lazy<Image> =
					include_image!("../../../assets/games/turbokartracers.png");
				&IMAGE
			}
			Self::SkyWars => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/skywars.png");
				&IMAGE
			}
			Self::BedWars => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/bedwars.png");
				&IMAGE
			}
			Self::MurderMystery => {
				static IMAGE: Lazy<Image> =
					include_image!("../../../assets/games/murdermystery.png");
				&IMAGE
			}
			Self::BuildBattle => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/buildbattle.png");
				&IMAGE
			}
			Self::Duels => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/duels.png");
				&IMAGE
			}
			Self::SkyClash => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/skyclash.png");
				&IMAGE
			}
			Self::SpeedUhc => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/speeduhc.png");
				&IMAGE
			}
			Self::Smp => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/smp.png");
				&IMAGE
			}
			Self::CrazyWalls => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/crazywalls.png");
				&IMAGE
			}
			Self::SkyBlock => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/skyblock.png");
				&IMAGE
			}
			Self::Housing => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/housing.png");
				&IMAGE
			}
			Self::Pit => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/thepit.png");
				&IMAGE
			}
			Self::WoolWars => {
				static IMAGE: Lazy<Image> = include_image!("../../../assets/games/woolwars.png");
				&IMAGE
			}
			_ => return None,
		})
	}

	#[allow(clippy::too_many_lines)]
	#[must_use]
	pub fn as_text(&self) -> &'static [Text<'static>] {
		match self {
			Self::Limbo => {
				const TEXT: [Text; 1] = minecraft_text("§4§lLimbo");
				&TEXT
			}
			Self::Lobby => {
				const TEXT: [Text; 1] = minecraft_text("§4§lLobby");
				&TEXT
			}
			Self::Quake => {
				const TEXT: [Text; 2] = minecraft_text("§a§lQuake§2§lcraft");
				&TEXT
			}
			Self::Walls => {
				const TEXT: [Text; 1] = minecraft_text("§e§lThe Walls");
				&TEXT
			}
			Self::Paintball => {
				const TEXT: [Text; 5] = minecraft_text("§f§lPa§e§li§6§lnt§b§lba§3§lll");
				&TEXT
			}
			Self::BlitzSg => {
				const TEXT: [Text; 1] = minecraft_text("§c§lBlitz SG");
				&TEXT
			}
			Self::TntGames => {
				const TEXT: [Text; 2] = minecraft_text("§4§lTNT §c§lGames");
				&TEXT
			}
			Self::VampireZ => {
				const TEXT: [Text; 2] = minecraft_text("§c§lVampire§4§lZ");
				&TEXT
			}
			Self::MegaWalls => {
				const TEXT: [Text; 1] = minecraft_text("§3§lMega Walls");
				&TEXT
			}
			Self::Arcade => {
				const TEXT: [Text; 1] = minecraft_text("§6§lArcade");
				&TEXT
			}
			Self::Arena => {
				const TEXT: [Text; 1] = minecraft_text("§6§lArena Brawl");
				&TEXT
			}
			Self::Uhc => {
				const TEXT: [Text; 2] = minecraft_text("§e§lUHC §6§lChampions");
				&TEXT
			}
			Self::CopsAndCrims => {
				const TEXT: [Text; 3] = minecraft_text("§9§lCops §f§land §c§lCrims");
				&TEXT
			}
			Self::Warlords => {
				const TEXT: [Text; 1] = minecraft_text("§b§lWarlords");
				&TEXT
			}
			Self::SmashHeroes => {
				const TEXT: [Text; 1] = minecraft_text("§e§lSmash Heroes");
				&TEXT
			}
			Self::TurboKartRacers => {
				const TEXT: [Text; 1] = minecraft_text("§a§lKart Racers");
				&TEXT
			}
			Self::Housing => {
				const TEXT: [Text; 1] = minecraft_text("§4§lHousing");
				&TEXT
			}
			Self::SkyWars => {
				const TEXT: [Text; 2] = minecraft_text("§b§lSky §f§lWars");
				&TEXT
			}
			Self::CrazyWalls => {
				const TEXT: [Text; 1] = minecraft_text("§4§lCrazy Walls");
				&TEXT
			}
			Self::SpeedUhc => {
				const TEXT: [Text; 2] = minecraft_text("§e§lSpeed §6§lUHC");
				&TEXT
			}
			Self::SkyClash => {
				const TEXT: [Text; 1] = minecraft_text("§4§lSkyClash");
				&TEXT
			}
			Self::Classic => {
				const TEXT: [Text; 1] = minecraft_text("§4§lClassic Games");
				&TEXT
			}
			Self::Prototype => {
				const TEXT: [Text; 1] = minecraft_text("§4§lPrototype");
				&TEXT
			}
			Self::BedWars => {
				const TEXT: [Text; 2] = minecraft_text("§c§lBed §f§lWars");
				&TEXT
			}
			Self::MurderMystery => {
				const TEXT: [Text; 1] = minecraft_text("§4§lMurder Mystery");
				&TEXT
			}
			Self::BuildBattle => {
				const TEXT: [Text; 1] = minecraft_text("§d§lBuild Battle");
				&TEXT
			}
			Self::Duels => {
				const TEXT: [Text; 1] = minecraft_text("§e§lDuels");
				&TEXT
			}
			Self::SkyBlock => {
				const TEXT: [Text; 2] = minecraft_text("§b§lSky§a§lBlock");
				&TEXT
			}
			Self::Pit => {
				const TEXT: [Text; 1] = minecraft_text("§c§lThe Pit");
				&TEXT
			}
			Self::Replay => {
				const TEXT: [Text; 1] = minecraft_text("§4§lReplay");
				&TEXT
			}
			Self::Smp => {
				const TEXT: [Text; 1] = minecraft_text("§4§lSMP");
				&TEXT
			}
			Self::WoolWars => {
				const TEXT: [Text; 2] = minecraft_text("§f§lWool §7§lWars");
				&TEXT
			}
			Self::Unknown => {
				const TEXT: [Text; 1] = minecraft_text("§f§lUnknown");
				&TEXT
			}
		}
	}
}

impl From<&str> for Type {
	fn from(name: &str) -> Self {
		match name {
			"LIMBO" => Self::Limbo,
			"LOBBY" => Self::Lobby,
			"QUAKECRAFT" => Self::Quake,
			"WALLS" => Self::Walls,
			"PAINTBALL" => Self::Paintball,
			"SURVIVAL_GAMES" => Self::BlitzSg,
			"TNTGAMES" => Self::TntGames,
			"VAMPIREZ" => Self::VampireZ,
			"WALLS3" => Self::MegaWalls,
			"ARCADE" => Self::Arcade,
			"ARENA" => Self::Arena,
			"UHC" => Self::Uhc,
			"MCGO" => Self::CopsAndCrims,
			"BATTLEGROUND" => Self::Warlords,
			"SUPER_SMASH" => Self::SmashHeroes,
			"GINGERBREAD" => Self::TurboKartRacers,
			"HOUSING" => Self::Housing,
			"SKYWARS" => Self::SkyWars,
			"TRUE_COMBAT" => Self::CrazyWalls,
			"SPEED_UHC" => Self::SpeedUhc,
			"SKYCLASH" => Self::SkyClash,
			"LEGACY" => Self::Classic,
			"PROTOTYPE" => Self::Prototype,
			"BEDWARS" => Self::BedWars,
			"MURDER_MYSTERY" => Self::MurderMystery,
			"BUILD_BATTLE" => Self::BuildBattle,
			"DUELS" => Self::Duels,
			"SKYBLOCK" => Self::SkyBlock,
			"PIT" => Self::Pit,
			"REPLAY" => Self::Replay,
			"SMP" => Self::Smp,
			"WOOL_GAMES" => Self::WoolWars,
			_ => Self::Unknown,
		}
	}
}
