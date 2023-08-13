use macros::{Diff, Game};
use serde::Deserialize;

use crate::inverse_bool;

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, Game, PartialEq, Diff,
)]
#[game(
	path = "vampire_z",
	pretty = "§c§lVampire§4§lZ",
	plain = "VampireZ",
	field(ident = "human_wins", colour = "green"),
	field(ident = "vampire_wins", colour = "red"),
	field(ident = "zombie_kills", colour = "gold"),
	field(ident = "human_kills", colour = "green"),
	field(ident = "human_deaths", colour = "red"),
	field(
		tr = "hkdr",
		ident = "human_kills",
		div = "human_deaths",
		colour = "gold"
	),
	field(ident = "vampire_kills", colour = "green"),
	field(ident = "vampire_deaths", colour = "red"),
	field(
		tr = "vkdr",
		ident = "vampire_kills",
		div = "vampire_deaths",
		colour = "gold"
	)
)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
pub struct VampireZ {
	#[serde(deserialize_with = "crate::de::from::f32_to_i32")]
	#[game(label(colour = "gold"))]
	pub coins: i32,
	#[game(label(colour = "red"), nominal)]
	pub blood: bool,
	#[serde(rename = "no_starting_compass")]
	#[game(label(colour = "yellow"), nominal)]
	pub starting_compass: inverse_bool::InverseBool,
	#[serde(rename = "no_starting_gear")]
	#[game(label(colour = "blue"), nominal)]
	pub starting_gear: inverse_bool::InverseBool,
	#[serde(rename = "combatTracker")]
	#[game(label(colour = "green"), nominal)]
	pub tracker: bool,
	#[serde(rename = "updated_stats")]
	#[game(label(colour = "red"), nominal)]
	pub updated: bool,
	#[serde(rename = "using_old_vamp")]
	#[game(label(colour = "aqua"), nominal)]
	pub old_vampire: bool,

	#[serde(flatten)]
	#[game(mode())]
	pub normal: Normal,
}

#[derive(Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, PartialEq, Diff)]
#[serde(default)]
pub struct Normal {
	pub human_wins: u32,
	pub vampire_wins: u32,
	pub zombie_kills: u32,
	pub human_kills: u32,
	pub human_deaths: u32,
	pub vampire_kills: u32,
	pub vampire_deaths: u32,
}
