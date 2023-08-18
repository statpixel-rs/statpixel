use macros::Game;
use minecraft::colour::Colour;

use crate::meters::Meters;

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default, Game)]
#[game(
	path = "quake",
	pretty = "§a§lQuake§2§lcraft",
	plain = "Quakecraft",
	field(ident = "wins", colour = "green"),
	field(ident = "killstreaks", colour = "red"),
	field(ident = "distance_walked", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	field(ident = "shots_fired", colour = "green"),
	field(ident = "headshots", colour = "red"),
	field(
		tr = "hr",
		ident = "headshots",
		div = "shots_fired",
		colour = "gold",
		percent = "u32"
	)
)]
#[serde(default)]
pub struct Quake {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[game(label(colour = "yellow"), nominal)]
	pub sight: Option<Colour>,
	#[serde(rename = "selectedKillPrefix")]
	#[game(label(colour = "blue"), nominal)]
	pub kill_prefix: Option<Colour>,

	#[serde(flatten)]
	#[game(mode())]
	pub solo: Solo,
	#[serde(flatten)]
	#[game(mode())]
	pub team: Team,
	#[serde(flatten)]
	#[game(mode())]
	pub solo_tournament: SoloTournament,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Solo {
	pub wins: u32,
	pub killstreaks: u32,
	#[serde(rename = "distance_travelled")]
	pub distance_walked: Meters,
	pub kills: u32,
	pub deaths: u32,
	pub shots_fired: u32,
	pub headshots: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Team {
	#[serde(rename = "wins_teams")]
	pub wins: u32,
	#[serde(rename = "killstreaks_teams")]
	pub killstreaks: u32,
	#[serde(rename = "distance_travelled_teams")]
	pub distance_walked: Meters,
	#[serde(rename = "kills_teams")]
	pub kills: u32,
	#[serde(rename = "deaths_teams")]
	pub deaths: u32,
	#[serde(rename = "shots_fired_teams")]
	pub shots_fired: u32,
	#[serde(rename = "headshots_teams")]
	pub headshots: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloTournament {
	#[serde(rename = "wins_solo_tourney")]
	pub wins: u32,
	#[serde(rename = "killstreaks_solo_tourney")]
	pub killstreaks: u32,
	#[serde(rename = "distance_travelled_solo_tourney")]
	pub distance_walked: Meters,
	#[serde(rename = "kills_solo_tourney")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_tourney")]
	pub deaths: u32,
	#[serde(rename = "shots_fired_solo_tourney")]
	pub shots_fired: u32,
	#[serde(rename = "headshots_solo_tourney")]
	pub headshots: u32,
}
