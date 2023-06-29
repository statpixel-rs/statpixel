use macros::{Diff, Game, Mode};
use serde::Deserialize;

use crate::seconds::Seconds;

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, Game, PartialEq, Diff,
)]
#[game(
	path = "tnt_games",
	pretty = "§4§lTNT §c§lGames",
	plain = "TNT Games",
	field(ident = "wins", colour = "green")
)]
#[serde(default)]
pub struct TntGames {
	#[serde(deserialize_with = "crate::de::from::f32_to_i32")]
	#[game(label(colour = "gold"))]
	pub coins: i32,

	#[serde(flatten)]
	#[game(mode())]
	pub tnt_run: TntRun,
	#[serde(flatten)]
	#[game(mode())]
	pub pvp_run: PvpRun,
	#[serde(flatten)]
	#[game(mode())]
	pub bow_spleef: BowSpleef,
	#[serde(flatten)]
	#[game(mode())]
	pub wizard: Wizard,
	#[serde(flatten)]
	#[game(mode())]
	pub tnt_tag: TntTag,
}

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, PartialEq, Mode, Diff,
)]
#[serde(default)]
pub struct TntRun {
	#[serde(rename = "wins_tntrun")]
	pub wins: u32,
	#[serde(rename = "deaths_tntrun")]
	#[mode(field(colour = "red"))]
	pub deaths: u32,
	#[serde(rename = "record_tntrun")]
	#[mode(field(colour = "gold"))]
	pub record: Seconds,
}

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, PartialEq, Mode, Diff,
)]
#[mode(field(tr = "kdr", colour = "gold", ident = "kills", div = "deaths"))]
#[serde(default)]
pub struct TntTag {
	#[serde(rename = "wins_tntag")]
	pub wins: u32,
	#[serde(rename = "kills_tntag")]
	#[mode(field(colour = "green"))]
	pub kills: u32,
	#[serde(rename = "deaths_tntag")]
	#[mode(field(colour = "red"))]
	pub deaths: u32,
}

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, PartialEq, Mode, Diff,
)]
#[mode(field(tr = "kdr", colour = "gold", ident = "kills", div = "deaths"))]
#[serde(default)]
pub struct PvpRun {
	#[serde(rename = "wins_pvprun")]
	pub wins: u32,
	#[serde(rename = "new_pvprun_double_jumps")]
	#[mode(field(colour = "red"))]
	pub double_jumps: u32,
	#[serde(rename = "record_pvprun")]
	#[mode(field(colour = "gold"))]
	pub record: Seconds,
	#[serde(rename = "kills_pvprun")]
	#[mode(field(colour = "green"))]
	pub kills: u32,
	#[serde(rename = "deaths_pvprun")]
	#[mode(field(colour = "red"))]
	pub deaths: u32,
}

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, PartialEq, Mode, Diff,
)]
#[serde(default)]
pub struct BowSpleef {
	#[serde(rename = "wins_bowspleef")]
	pub wins: u32,
	#[serde(rename = "deaths_bowspleef")]
	#[mode(field(colour = "red"))]
	pub deaths: u32,
	#[serde(rename = "tags_bowspleef")]
	#[mode(field(colour = "gold"))]
	pub tags: Seconds,
}

#[derive(
	Deserialize, bincode::Decode, bincode::Encode, Default, Debug, Clone, PartialEq, Mode, Diff,
)]
#[mode(field(tr = "kdr", colour = "gold", ident = "kills", div = "deaths"))]
#[serde(default)]
pub struct Wizard {
	#[serde(rename = "wins_capture")]
	pub wins: u32,
	#[serde(rename = "air_time_capture")]
	#[mode(field(colour = "red"))]
	pub air_time: Seconds,
	#[serde(rename = "points_capture")]
	#[mode(field(colour = "gold"))]
	pub points: u32,
	#[serde(rename = "kills_capture")]
	#[mode(field(colour = "green"))]
	pub kills: u32,
	#[serde(rename = "deaths_capture")]
	#[mode(field(colour = "red"))]
	pub deaths: u32,
}
