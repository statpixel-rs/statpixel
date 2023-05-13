use macros::{Game, Mode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq)]
#[game(
	path = "build_battle",
	pretty = "§b§lBuild Battle",
	calc = "minecraft::calc::build_battle",
	field(ident = "wins", colour = "green")
)]
#[serde(default)]
pub struct BuildBattle {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[game(label(colour = "yellow"), xp)]
	pub score: u32,
	#[serde(rename = "solo_most_points")]
	#[game(label(colour = "light_purple"))]
	pub most_points_solo: u32,
	#[serde(rename = "teams_most_points")]
	#[game(label(colour = "dark_purple"))]
	pub most_points_teams: u32,
	#[serde(rename = "total_votes")]
	#[game(label(colour = "aqua"))]
	pub votes: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub solo_normal: SoloNormal,
	#[serde(flatten)]
	#[game(mode())]
	pub solo_pro: SoloPro,
	#[serde(flatten)]
	#[game(mode())]
	pub teams_normal: TeamNormal,
	#[serde(flatten)]
	#[game(mode())]
	pub guess_the_build: GuessTheBuild,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct SoloNormal {
	#[serde(rename = "wins_solo_normal")]
	pub wins: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct SoloPro {
	#[serde(rename = "wins_solo_pro")]
	pub wins: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct TeamNormal {
	#[serde(rename = "wins_teams_normal")]
	pub wins: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct GuessTheBuild {
	#[serde(rename = "wins_guess_the_build")]
	pub wins: u32,
}
