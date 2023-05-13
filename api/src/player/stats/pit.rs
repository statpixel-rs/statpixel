use macros::{Diff, Game, Mode};
use minecraft::calc::pit::Level;
use serde::{Deserialize, Serialize};

use crate::minutes;

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
#[serde(default)]
pub struct Outer {
	#[serde(rename = "pit_stats_ptl")]
	pub data: Pit,
	pub profile: Profile,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
#[serde(default)]
pub struct Profile {
	pub cash: f32,
	#[serde(flatten)]
	pub level: Level,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq, Diff)]
#[game(
	path = "pit.data",
	pretty = "§b§lPit",
	xp = "pit.profile.level",
	calc = "minecraft::calc::pit",
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	field(ident = "damage_dealt", colour = "green"),
	field(ident = "damage_taken", colour = "red"),
	field(
		tr = "ddtr",
		ident = "damage_dealt",
		div = "damage_taken",
		colour = "gold"
	),
	field(ident = "bow_damage_dealt", colour = "green"),
	field(ident = "bow_damage_taken", colour = "red"),
	field(
		tr = "bdr",
		ident = "bow_damage_dealt",
		div = "bow_damage_taken",
		colour = "gold"
	),
	field(ident = "contracts_completed", colour = "green"),
	field(ident = "contracts_started", colour = "red"),
	field(
		tr = "cr",
		ident = "contracts_completed",
		div = "contracts_started",
		colour = "gold",
		percent
	),
	label(ident = "cash", path = "pit.profile", colour = "gold")
)]
#[serde(default)]
pub struct Pit {
	#[game(label(colour = "aqua"))]
	pub chat_messages: u32,
	#[serde(rename = "playtime_minutes")]
	#[game(label(colour = "yellow"))]
	pub time_played: minutes::Minutes,
	#[game(label(colour = "gold"))]
	pub cash_earned: u32,
	#[game(label(colour = "blue"))]
	pub soups_drank: u32,
	#[game(label(colour = "light_purple"))]
	pub blocks_broken: u32,
	#[serde(rename = "max_streak")]
	#[game(label(colour = "red"))]
	pub highest_killstreak: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub normal: Normal,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
#[serde(default)]
pub struct Normal {
	pub kills: u32,
	pub deaths: u32,
	#[serde(rename = "melee_damage_dealt")]
	pub damage_dealt: u32,
	#[serde(rename = "melee_damage_received")]
	pub damage_taken: u32,
	#[serde(rename = "bow_damage_dealt")]
	pub bow_damage_dealt: u32,
	#[serde(rename = "bow_damage_received")]
	pub bow_damage_taken: u32,
	pub contracts_completed: u32,
	pub contracts_started: u32,
}
