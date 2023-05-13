use macros::{Game, Mode};
use serde::{Deserialize, Serialize};

use crate::seconds;

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq)]
#[game(
	path = "murder_mystery",
	pretty = "§b§lMurder Mystery",
	field(ident = "wins", colour = "green"),
	field(ident = "games", colour = "red"),
	field(tr = "wr", ident = "wins", div = "games", colour = "gold", percent),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold")
)]
#[serde(default)]
pub struct MurderMystery {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[serde(
		rename = "mm_chests",
		deserialize_with = "super::from_trunc_f32_to_u32"
	)]
	#[game(label(colour = "yellow"))]
	pub loot_chests: u32,
	#[serde(rename = "total_time_survived_seconds")]
	#[game(label(colour = "aqua"))]
	pub time_survived: seconds::Seconds,
	#[serde(rename = "murderer_wins")]
	#[game(label(colour = "red"))]
	pub murderer_wins: u32,
	#[serde(rename = "detective_wins")]
	#[game(label(colour = "green"))]
	pub detective_wins: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub assassins: Assassins,
	#[serde(flatten)]
	#[game(mode())]
	pub classic: Classic,
	#[serde(flatten)]
	#[game(mode())]
	pub double_up: DoubleUp,
	#[serde(flatten)]
	#[game(mode())]
	pub infection: Infection,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Assassins {
	#[serde(rename = "wins_MURDER_ASSASSINS")]
	pub wins: u32,
	#[serde(rename = "games_MURDER_ASSASSINS")]
	pub games: u32,
	#[serde(rename = "kills_MURDER_ASSASSINS")]
	pub kills: u32,
	#[serde(rename = "deaths_MURDER_ASSASSINS")]
	pub deaths: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Classic {
	#[serde(rename = "wins_MURDER_CLASSIC")]
	pub wins: u32,
	#[serde(rename = "games_MURDER_CLASSIC")]
	pub games: u32,
	#[serde(rename = "kills_MURDER_CLASSIC")]
	pub kills: u32,
	#[serde(rename = "deaths_MURDER_CLASSIC")]
	pub deaths: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct DoubleUp {
	#[serde(rename = "wins_MURDER_DOUBLE_UP")]
	pub wins: u32,
	#[serde(rename = "games_MURDER_DOUBLE_UP")]
	pub games: u32,
	#[serde(rename = "kills_MURDER_DOUBLE_UP")]
	pub kills: u32,
	#[serde(rename = "deaths_MURDER_DOUBLE_UP")]
	pub deaths: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Infection {
	#[serde(rename = "wins_MURDER_INFECTION")]
	pub wins: u32,
	#[serde(rename = "games_MURDER_INFECTION")]
	pub games: u32,
	#[serde(rename = "kills_MURDER_INFECTION")]
	pub kills: u32,
	#[serde(rename = "deaths_MURDER_INFECTION")]
	pub deaths: u32,
}
