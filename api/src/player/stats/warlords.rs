use macros::{Diff, Game, Mode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq, Diff)]
#[game(
	path = "warlords",
	pretty = "§b§lWarlords",
	field(ident = "wins_blue", colour = "blue"),
	field(ident = "wins_red", colour = "red"),
	field(ident = "kills", colour = "gold")
)]
#[serde(default)]
pub struct Warlords {
	#[serde(deserialize_with = "super::from_trunc_f32_to_i32")]
	#[game(label(colour = "gold"))]
	pub coins: i32,
	#[serde(rename = "damage", deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "green"))]
	pub damage_dealt: u32,
	#[serde(
		rename = "damage_taken",
		deserialize_with = "super::from_trunc_f32_to_u32"
	)]
	#[game(label(colour = "blue"))]
	pub damage_taken: u32,
	#[serde(rename = "heal", deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "red"))]
	pub health_regenerated: u32,
	#[game(label(colour = "aqua"))]
	pub hide_prestige: bool,
	#[game(label(tr = "kdr", div = "deaths", colour = "gold"))]
	pub kills: u32,
	pub deaths: u32,
	#[serde(rename = "mvp_count")]
	#[game(label(colour = "yellow"))]
	pub mvps: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub capture_the_flag: CaptureTheFlag,
	#[serde(flatten)]
	#[game(mode())]
	pub donination: Domination,
	#[serde(flatten)]
	#[game(mode())]
	pub team_deathmatch: TeamDeathmatch,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
#[serde(default)]
pub struct CaptureTheFlag {
	#[serde(rename = "wins_capturetheflag_blu")]
	pub wins_blue: u32,
	#[serde(rename = "wins_capturetheflag_red")]
	pub wins_red: u32,
	#[serde(rename = "kills_capturetheflag")]
	pub kills: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
#[serde(default)]
pub struct Domination {
	#[serde(rename = "wins_domination_blu")]
	pub wins_blue: u32,
	#[serde(rename = "wins_domination_red")]
	pub wins_red: u32,
	#[serde(rename = "kills_domination")]
	pub kills: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode, Diff)]
#[serde(default)]
pub struct TeamDeathmatch {
	#[serde(rename = "wins_teamdeathmatch_blu")]
	pub wins_blue: u32,
	#[serde(rename = "wins_teamdeathmatch_red")]
	pub wins_red: u32,
	#[serde(rename = "kills_teamdeathmatch")]
	pub kills: u32,
}