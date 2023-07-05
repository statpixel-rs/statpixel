use minecraft::paint::Paint;

use crate::player::stats::*;

#[derive(bitcode::Encode, bitcode::Decode, Debug, Clone, Copy)]
pub enum Location {
	Down,
	DownStart,
	Right,
	RightStart,
}

#[derive(bitcode::Encode, bitcode::Decode, Debug, Clone, Copy)]
pub enum LevelKind {
	BedWars,
	BuildBattle,
	Duels,
	Network,
	Pit,
	SkyWars,
	WoolWars,
}

#[derive(bitcode::Encode, bitcode::Decode, Debug, Clone)]
pub enum Statistic {
	Arcade(arcade::ArcadeKind),
	Arena(arena::ArenaKind),
	BedWars(bed_wars::BedWarsKind),
	BlitzSg(blitz_sg::BlitzSgKind),
	BuildBattle(build_battle::BuildBattleKind),
	CopsAndCrims(cops_and_crims::CopsAndCrimsKind),
	Duels(duels::DuelsKind),
	MegaWalls(mega_walls::MegaWallsKind),
	MurderMystery(murder_mystery::MurderMysteryKind),
	Paintball(paintball::PaintballKind),
	Pit(pit::PitKind),
	Quake(quake::QuakeKind),
	SkyWars(sky_wars::SkyWarsKind),
	SmashHeroes(smash_heroes::SmashHeroesKind),
	SpeedUhc(speed_uhc::SpeedUhcKind),
	TntGames(tnt_games::TntGamesKind),
	TurboKartRacers(turbo_kart_racers::TurboKartRacersKind),
	Uhc(uhc::UhcKind),
	VampireZ(vampire_z::VampireZKind),
	Walls(walls::WallsKind),
	Warlords(warlords::WarlordsKind),
	WoolWars(wool_wars::WoolWarsKind),
}

#[derive(bitcode::Encode, bitcode::Decode, Debug, Clone)]
pub enum ShapeData {
	/// Always the player's username
	Title,
	/// Arbitrary text up to 16 characters,
	Subtitle(String),
	/// One of the player levels
	Level(LevelKind),
	/// The player's skin
	Skin,
	/// A bubble with an arbitrary piece of the user's data
	Bubble(Statistic),
}

impl ShapeData {
	#[must_use]
	pub fn is_subtitle(&self) -> bool {
		matches!(self, Self::Subtitle(..))
	}
}

#[derive(bitcode::Encode, bitcode::Decode, Debug, Clone)]
pub enum ShapeKind {
	Title,
	Subtitle,
	Level,
	Skin,
	Bubble,
}

#[derive(bitcode::Encode, bitcode::Decode, Debug, Clone)]
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
