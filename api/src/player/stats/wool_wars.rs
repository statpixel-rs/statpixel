use macros::{Diff, Game, Mode};
use serde::Deserialize;

#[derive(bincode::Decode, bincode::Encode, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Progression {
	#[serde(rename = "available_layers")]
	pub layers: u32,
	#[serde(
		rename = "experience",
		deserialize_with = "crate::de::from::f64_to_u64"
	)]
	pub xp: u64,
}

#[derive(bincode::Decode, bincode::Encode, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Outer {
	#[serde(rename = "wool_wars")]
	pub inner: Inner,
	#[serde(deserialize_with = "crate::de::from::f32_to_i32")]
	pub coins: i32,
	pub progression: Progression,
}

#[derive(bincode::Decode, bincode::Encode, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Inner {
	pub stats: WoolWars,
}

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, Game, PartialEq, Diff,
)]
#[game(
	path = "wool_wars.inner.stats",
	pretty = "§f§lWool §7§lWars",
	plain = "Wool Wars",
	calc = "minecraft::calc::wool_wars",
	field(ident = "wins", colour = "green"),
	field(ident = "games", colour = "red"),
	field(
		tr = "wr",
		ident = "wins",
		div = "games",
		colour = "gold",
		percent = "u32"
	),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	field(ident = "assists", colour = "green"),
	field(ident = "powerups_collected", colour = "red"),
	field(ident = "wool_placed", colour = "gold"),
	label(ident = "coins", path = "wool_wars", colour = "gold"),
	label(ident = "layers", path = "wool_wars.progression", colour = "blue"),
	xp = "wool_wars.progression.xp"
)]
#[serde(default)]
pub struct WoolWars {
	#[game(label(colour = "aqua"))]
	pub blocks_broken: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub normal: Normal,
}

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, PartialEq, Mode, Diff,
)]
#[serde(default)]
pub struct Normal {
	pub wins: u32,
	#[serde(rename = "games_played")]
	pub games: u32,
	pub kills: u32,
	pub deaths: u32,
	pub assists: u32,
	#[serde(rename = "powerups_gotten")]
	pub powerups_collected: u32,
	pub wool_placed: u32,
}
