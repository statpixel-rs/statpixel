use macros::{Game, Mode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq)]
#[serde(default)]
#[game(
	path = "arena",
	pretty = "§6§lArena Brawl",
	field(ident = "wins", colour = "green"),
	field(ident = "losses", colour = "red"),
	field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold")
)]
pub struct Arena {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[serde(rename = "magical_chest")]
	#[game(label(colour = "dark_purple"))]
	pub magical_chests: u32,
	#[serde(rename = "keys")]
	#[game(label(colour = "aqua"))]
	pub magical_keys: u32,
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "green"))]
	pub rating: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub solo: Solo,
	#[serde(flatten)]
	#[game(mode())]
	pub double: Double,
	#[serde(flatten)]
	#[game(mode())]
	pub four: Four,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Solo {
	#[serde(rename = "wins_1v1")]
	pub wins: u32,
	#[serde(rename = "losses_1v1")]
	pub losses: u32,
	#[serde(rename = "kills_1v1")]
	pub kills: u32,
	#[serde(rename = "deaths_1v1")]
	pub deaths: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Double {
	#[serde(rename = "wins_2v2")]
	pub wins: u32,
	#[serde(rename = "losses_2v2")]
	pub losses: u32,
	#[serde(rename = "kills_2v2")]
	pub kills: u32,
	#[serde(rename = "deaths_2v2")]
	pub deaths: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Four {
	#[serde(rename = "wins_4v4")]
	pub wins: u32,
	#[serde(rename = "losses_4v4")]
	pub losses: u32,
	#[serde(rename = "kills_4v4")]
	pub kills: u32,
	#[serde(rename = "deaths_4v4")]
	pub deaths: u32,
}
