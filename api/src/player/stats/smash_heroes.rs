use macros::{Diff, Game, Mode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq, Diff)]
#[game(
	path = "smash_heroes",
	pretty = "§b§lSmash Heroes",
	field(ident = "wins", colour = "green"),
	field(ident = "losses", colour = "red"),
	field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	field(ident = "smasher", colour = "green"),
	field(ident = "smashed", colour = "red"),
	field(tr = "ssr", ident = "smasher", div = "smashed", colour = "gold")
)]
#[serde(default)]
pub struct SmashHeroes {
	#[serde(deserialize_with = "super::from_trunc_f32_to_i32")]
	#[game(label(colour = "gold"))]
	pub coins: i32,
	#[game(label(colour = "yellow"))]
	pub win_streak: u32,
	#[serde(rename = "smash_level_total")]
	#[game(label(colour = "blue"))]
	pub level: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub solo: Solo,
	#[serde(flatten)]
	#[game(mode())]
	pub double: Double,
	#[serde(flatten)]
	#[game(mode())]
	pub team: Team,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
#[serde(default)]
pub struct Solo {
	#[serde(rename = "wins_normal")]
	pub wins: u32,
	#[serde(rename = "losses_normal")]
	pub losses: u32,
	#[serde(rename = "kills_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_normal")]
	pub deaths: u32,
	#[serde(rename = "smasher_normal")]
	pub smasher: u32,
	#[serde(rename = "smashed_normal")]
	pub smashed: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
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
	#[serde(rename = "smasher_2v2")]
	pub smasher: u32,
	#[serde(rename = "smashed_2v2")]
	pub smashed: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
#[serde(default)]
pub struct Team {
	#[serde(rename = "wins_teams")]
	pub wins: u32,
	#[serde(rename = "losses_teams")]
	pub losses: u32,
	#[serde(rename = "kills_teams")]
	pub kills: u32,
	#[serde(rename = "deaths_teams")]
	pub deaths: u32,
	#[serde(rename = "smasher_teams")]
	pub smasher: u32,
	#[serde(rename = "smashed_teams")]
	pub smashed: u32,
}
