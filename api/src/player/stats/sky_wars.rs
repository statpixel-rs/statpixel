use macros::Game;
use serde::Deserialize;

fn default_level_fmt() -> String {
	"§71".to_string()
}

#[derive(Deserialize, Default, Debug, Clone, Game)]
#[game(
	path = "sky_wars",
	pretty = "§b§lSky§d§lWars",
	calc = "minecraft::calc::skywars",
	field(ident = "wins", colour = "green"),
	field(ident = "losses", colour = "red"),
	field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold")
)]
#[serde(default)]
pub struct SkyWars {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[serde(
		rename = "skywars_chests",
		deserialize_with = "super::from_trunc_f32_to_u32"
	)]
	#[game(label(colour = "yellow"))]
	pub loot_chests: u32,
	#[game(label(colour = "blue"))]
	pub opals: u32,
	#[game(label(colour = "dark_purple"))]
	pub heads: u32,
	#[game(label(colour = "aqua"))]
	pub souls: u32,
	#[serde(rename = "cosmetic_tokens")]
	#[game(label(colour = "dark_green"))]
	pub tokens: u32,
	pub arrows_shot: u32,
	#[game(label(colour = "red", div = "arrows_shot", percent, tr = "bow-accuracy"))]
	pub arrows_hit: u32,
	#[serde(rename = "levelFormatted")]
	#[serde(default = "default_level_fmt")]
	#[game(level)]
	pub level_fmt: String,
	#[serde(
		rename = "skywars_experience",
		deserialize_with = "super::from_trunc_f32_to_u64"
	)]
	#[game(xp)]
	pub xp: u64,

	#[serde(flatten)]
	#[game(mode(hypixel = "solo_normal"))]
	pub solo_normal: SoloNormal,
	#[serde(flatten)]
	#[game(mode(hypixel = "solo_insane"))]
	pub solo_insane: SoloInsane,
	#[serde(flatten)]
	#[game(mode(hypixel = "teams_normal"))]
	pub team_normal: TeamNormal,
	#[serde(flatten)]
	#[game(mode(hypixel = "teams_insane"))]
	pub team_insane: TeamInsane,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SoloNormal {
	#[serde(rename = "losses_solo_normal")]
	pub losses: u32,
	#[serde(rename = "wins_solo_normal")]
	pub wins: u32,
	#[serde(rename = "kills_solo_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_normal")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SoloInsane {
	#[serde(rename = "losses_solo_insane")]
	pub losses: u32,
	#[serde(rename = "wins_solo_insane")]
	pub wins: u32,
	#[serde(rename = "kills_solo_insane")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_insane")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct TeamNormal {
	#[serde(rename = "losses_team_normal")]
	pub losses: u32,
	#[serde(rename = "wins_team_normal")]
	pub wins: u32,
	#[serde(rename = "kills_team_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_team_normal")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct TeamInsane {
	#[serde(rename = "losses_team_insane")]
	pub losses: u32,
	#[serde(rename = "wins_team_insane")]
	pub wins: u32,
	#[serde(rename = "kills_team_insane")]
	pub kills: u32,
	#[serde(rename = "deaths_team_insane")]
	pub deaths: u32,
}
