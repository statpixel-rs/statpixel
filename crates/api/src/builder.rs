use minecraft::paint::Paint;
use serde::{Deserialize, Serialize};

use crate::player::stats::*;

#[derive(Deserialize, Serialize, bitcode::Encode, bitcode::Decode, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Location {
	Down,
	DownStart,
	Right,
	RightStart,
}

#[derive(Deserialize, Serialize, bitcode::Encode, bitcode::Decode, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LevelKind {
	BedWars,
	BuildBattle,
	Duels,
	Network,
	Pit,
	SkyWars,
	WoolWars,
}

#[derive(Deserialize, Serialize, bitcode::Encode, bitcode::Decode, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Statistic {
	Arcade {
		kind: arcade::ArcadeKind,
	},
	Arena {
		kind: arena::ArenaKind,
	},
	BedWars {
		kind: bed_wars::BedWarsKind,
	},
	BlitzSg {
		kind: blitz_sg::BlitzSgKind,
	},
	BuildBattle {
		kind: build_battle::BuildBattleKind,
	},
	CopsAndCrims {
		kind: cops_and_crims::CopsAndCrimsKind,
	},
	Duels {
		kind: duels::DuelsKind,
	},
	MegaWalls {
		kind: mega_walls::MegaWallsKind,
	},
	MurderMystery {
		kind: murder_mystery::MurderMysteryKind,
	},
	Paintball {
		kind: paintball::PaintballKind,
	},
	Pit {
		kind: pit::PitKind,
	},
	Quake {
		kind: quake::QuakeKind,
	},
	SkyWars {
		kind: sky_wars::SkyWarsKind,
	},
	SmashHeroes {
		kind: smash_heroes::SmashHeroesKind,
	},
	SpeedUhc {
		kind: speed_uhc::SpeedUhcKind,
	},
	TntGames {
		kind: tnt_games::TntGamesKind,
	},
	TurboKartRacers {
		kind: turbo_kart_racers::TurboKartRacersKind,
	},
	Uhc {
		kind: uhc::UhcKind,
	},
	VampireZ {
		kind: vampire_z::VampireZKind,
	},
	Walls {
		kind: walls::WallsKind,
	},
	Warlords {
		kind: warlords::WarlordsKind,
	},
	WoolWars {
		kind: wool_wars::WoolWarsKind,
	},
}

#[derive(Deserialize, Serialize, bitcode::Encode, bitcode::Decode, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ShapeData {
	/// Always the player's username
	Title,
	/// Arbitrary text up to 16 characters,
	Subtitle { text: String },
	/// One of the player levels
	Level { kind: LevelKind },
	/// The player's skin
	Skin,
	/// A bubble with an arbitrary piece of the user's data
	Bubble { statistic: Statistic },
}

impl ShapeData {
	#[must_use]
	pub fn is_subtitle(&self) -> bool {
		matches!(self, Self::Subtitle { .. })
	}
}

#[derive(Deserialize, Serialize, bitcode::Encode, bitcode::Decode, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ShapeKind {
	Title,
	Subtitle,
	Level,
	Skin,
	Bubble,
}

#[derive(Deserialize, Serialize, bitcode::Encode, bitcode::Decode, Debug, Clone)]
pub struct Shape {
	pub location: Location,
	pub colour: Paint,
	pub data: ShapeData,
}

#[derive(bitcode::Encode, bitcode::Decode, Debug, Default, Clone)]
pub struct PartialShape {
	pub location: Option<Location>,
	pub kind: Option<ShapeKind>,
	pub colour: Option<Paint>,
	/// This is filled in by the modal after "Add Shape" is clicked
	pub data: Option<ShapeData>,
}

impl PartialShape {
	#[must_use]
	pub fn is_complete(&self) -> bool {
		if let Some(ref kind) = self.kind {
			return self.location.is_some()
				&& (self.colour.is_some()
					|| !matches!(kind, ShapeKind::Bubble | ShapeKind::Subtitle));
		}

		false
	}
}

#[derive(bitcode::Encode, bitcode::Decode, Debug, Default, Clone)]
pub struct State {
	pub shapes: Vec<Shape>,
	pub next: PartialShape,
}

impl State {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub enum Action {
	AddShape,
	Undo,
	Create,
	SetNextPosition,
	SetNextShape,
	SetNextColour,
	SetSubtitleData,
	SetLevelData,
	SetBubbleData,
}

/// The structure of a button's `custom_id`
#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct Id {
	pub action: Action,
	pub state: State,
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn add_shape(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::AddShape,
		state,
	})
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn set_subtitle_data(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::SetSubtitleData,
		state,
	})
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn set_bubble_data(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::SetBubbleData,
		state,
	})
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn set_level_data(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::SetLevelData,
		state,
	})
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn undo(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::Undo,
		state,
	})
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn create(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::Create,
		state,
	})
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn set_next_position(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::SetNextPosition,
		state,
	})
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn set_next_shape(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::SetNextShape,
		state,
	})
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn set_next_colour(state: State) -> crate::Result<String> {
	super::id::builder(Id {
		action: Action::SetNextColour,
		state,
	})
}
