use macros::{Game, Mode};
use minecraft::colour::Colour;
use serde::{Deserialize, Serialize};

use crate::seconds;

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq)]
#[game(
	path = "paintball",
	pretty = "§b§lPaintball",
	field(ident = "wins", colour = "green"),
	field(ident = "kill_prefix", colour = "red"),
	field(ident = "show_kill_prefix", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	field(ident = "shots_fired", colour = "green"),
	field(ident = "killstreaks", colour = "red"),
	field(ident = "forcefield_time", colour = "gold")
)]
#[serde(default)]
pub struct Paintball {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[game(label(colour = "aqua"))]
	pub adrenaline: u8,
	#[game(label(colour = "red"))]
	pub endurance: u8,
	#[game(label(colour = "yellow"))]
	pub fortune: u8,
	#[game(label(colour = "green"))]
	pub godfather: u8,
	#[game(label(colour = "light_purple"))]
	pub superluck: u8,
	#[game(label(colour = "dark_purple"))]
	pub transfusion: u8,

	#[serde(flatten)]
	#[game(mode())]
	pub normal: Normal,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Normal {
	pub wins: u32,
	#[serde(rename = "selectedKillPrefix")]
	pub kill_prefix: Colour,
	#[serde(rename = "showKillPrefix")]
	pub show_kill_prefix: bool,
	pub kills: u32,
	pub deaths: u32,
	pub killstreaks: u32,
	#[serde(rename = "forcefieldTime")]
	pub forcefield_time: seconds::Seconds,
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	pub shots_fired: u32,
}
