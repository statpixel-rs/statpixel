use macros::{Game, Mode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq)]
#[game(
	path = "quake",
	pretty = "§b§lQuake",
	field(ident = "wins", colour = "green"),
	field(ident = "losses", colour = "red"),
	field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold")
)]
#[serde(default)]
pub struct Quake {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[game(label(colour = "yellow"))]
	pub sight: Option<Colour>,
	#[serde(rename = "selectedKillPrefix")]
	#[game(label(colour = "blue"))]
	pub kill_prefix: Option<Colour>,

	#[serde(flatten)]
	#[game(mode())]
	pub solo: Solo,
	#[serde(flatten)]
	#[game(mode())]
	pub solo: Team,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Solo {
	#[serde(rename = "uhc_duel_wins")]
	pub wins: u32,
	#[serde(rename = "wins_party")]
	pub losses: u32,
	#[serde(rename = "kills_party")]
	pub kills: u32,
	#[serde(rename = "deaths_party")]
	pub deaths: u32,
}
