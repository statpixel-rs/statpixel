use macros::Game;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game)]
#[game(
	path = "arcade",
	pretty = "§b§lArcade",
	field(ident = "wins", colour = "green"),
	field(ident = "games", colour = "red"),
	field(tr = "wr", ident = "wins", div = "games", colour = "gold", percent),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold")
)]
#[serde(default)]
pub struct Arcade {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[serde(rename = "mystery_gifts_obtained")]
	#[game(label(colour = "yellow"))]
	pub mystery_gifts: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub party: Party,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Party {
	#[serde(rename = "rounds_party")]
	pub games: u32,
	#[serde(rename = "wins_party")]
	pub wins: u32,
	#[serde(rename = "kills_party")]
	pub kills: u32,
	#[serde(rename = "deaths_party")]
	pub deaths: u32,
}
