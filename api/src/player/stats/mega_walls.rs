use macros::Game;

use crate::{meters, minutes};

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default, Game)]
#[game(
	path = "mega_walls",
	pretty = "§3§lMega Walls",
	plain = "Mega Walls",
	field(ident = "wins", colour = "green"),
	field(ident = "losses", colour = "red"),
	field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	field(ident = "final_kills", colour = "green"),
	field(ident = "final_deaths", colour = "red"),
	field(
		tr = "fkdr",
		ident = "final_kills",
		div = "final_deaths",
		colour = "gold"
	),
	label(ident = "time_played", colour = "yellow"),
	label(ident = "distance_walked", colour = "green"),
	label(ident = "distance_fallen", colour = "red"),
	label(ident = "bread_eaten", colour = "aqua"),
	label(ident = "wood_chopped", colour = "yellow"),
	label(ident = "treasures_found", colour = "gold")
)]
#[serde(default)]
pub struct MegaWalls {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub normal: Normal,
	#[serde(flatten)]
	#[game(mode())]
	pub face_off: FaceOff,
	#[serde(flatten)]
	#[game(mode())]
	pub brawl: Brawl,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Normal {
	#[serde(rename = "wins_standard")]
	pub wins: u32,
	#[serde(rename = "losses_standard")]
	pub losses: u32,
	#[serde(rename = "kills_standard")]
	pub kills: u32,
	#[serde(rename = "deaths_standard")]
	pub deaths: u32,
	#[serde(rename = "final_kills_standard")]
	pub final_kills: u32,
	#[serde(rename = "final_deaths_standard")]
	pub final_deaths: u32,
	#[serde(rename = "time_played_standard")]
	pub time_played: minutes::Minutes,
	#[serde(rename = "meters_walked_standard")]
	pub distance_walked: meters::Meters,
	#[serde(rename = "meters_fallen_standard")]
	pub distance_fallen: meters::Meters,
	#[serde(rename = "bread_eaten_standard")]
	pub bread_eaten: u32,
	#[serde(rename = "wood_chopped_standard")]
	pub wood_chopped: u32,
	#[serde(rename = "treasures_found_standard")]
	pub treasures_found: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct FaceOff {
	#[serde(rename = "wins_face_off")]
	pub wins: u32,
	#[serde(rename = "losses_face_off")]
	pub losses: u32,
	#[serde(rename = "kills_face_off")]
	pub kills: u32,
	#[serde(rename = "deaths_face_off")]
	pub deaths: u32,
	#[serde(rename = "final_kills_face_off")]
	pub final_kills: u32,
	#[serde(rename = "final_deaths_face_off")]
	pub final_deaths: u32,
	#[serde(rename = "time_played_face_off")]
	pub time_played: minutes::Minutes,
	#[serde(rename = "meters_walked_face_off")]
	pub distance_walked: meters::Meters,
	#[serde(rename = "meters_fallen_face_off")]
	pub distance_fallen: meters::Meters,
	#[serde(rename = "bread_eaten_face_off")]
	pub bread_eaten: u32,
	#[serde(rename = "wood_chopped_face_off")]
	pub wood_chopped: u32,
	#[serde(rename = "treasures_found_face_off")]
	pub treasures_found: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Brawl {
	#[serde(rename = "wins_gvg")]
	pub wins: u32,
	#[serde(rename = "losses_gvg")]
	pub losses: u32,
	#[serde(rename = "kills_gvg")]
	pub kills: u32,
	#[serde(rename = "deaths_gvg")]
	pub deaths: u32,
	#[serde(rename = "final_kills_gvg")]
	pub final_kills: u32,
	#[serde(rename = "final_deaths_gvg")]
	pub final_deaths: u32,
	#[serde(rename = "time_played_gvg")]
	pub time_played: minutes::Minutes,
	#[serde(rename = "meters_walked_gvg")]
	pub distance_walked: meters::Meters,
	#[serde(rename = "meters_fallen_gvg")]
	pub distance_fallen: meters::Meters,
	#[serde(rename = "bread_eaten_gvg")]
	pub bread_eaten: u32,
	#[serde(rename = "wood_chopped_gvg")]
	pub wood_chopped: u32,
	#[serde(rename = "treasures_found_gvg")]
	pub treasures_found: u32,
}
