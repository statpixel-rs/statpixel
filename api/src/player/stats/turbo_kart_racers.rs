use macros::{Diff, Game, Mode};
use serde::Deserialize;

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, Game, PartialEq, Diff,
)]
#[game(
	path = "turbo_kart_racers",
	pretty = "§a§lKart Racers",
	plain = "Kart Racers",
	field(ident = "bronze_trophies", colour = "red"),
	field(ident = "silver_trophies", colour = "gray"),
	field(ident = "gold_trophies", colour = "gold")
)]
#[serde(default)]
pub struct TurboKartRacers {
	#[serde(deserialize_with = "crate::de::from::f32_to_i32")]
	#[game(label(colour = "gold"))]
	pub coins: i32,
	#[game(label(colour = "aqua"))]
	pub box_pickups: u32,
	#[serde(rename = "coins_picked_up")]
	#[game(label(colour = "blue"))]
	pub coin_pickups: u32,
	#[game(label(colour = "yellow"))]
	pub grand_prix: bool,
	#[serde(rename = "show_win_prefix")]
	#[game(label(colour = "light_purple"))]
	pub show_prefix: bool,
	#[game(label(colour = "green"))]
	pub wins: u32,
	#[game(label(colour = "red"))]
	pub laps_completed: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub normal: Normal,
}

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, PartialEq, Mode, Diff,
)]
#[serde(default)]
pub struct Normal {
	#[serde(rename = "bronze_trophy")]
	pub bronze_trophies: u32,
	#[serde(rename = "silver_trophy")]
	pub silver_trophies: u32,
	#[serde(rename = "gold_trophy")]
	pub gold_trophies: u32,
}
