use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct SkyWarsStats {
	pub coins: u32,

	#[serde(flatten)]
	pub solo_normal: SkyWarsSoloNormalStats,
	#[serde(flatten)]
	pub solo_insane: SkyWarsSoloInsaneStats,
	#[serde(flatten)]
	pub team_normal: SkyWarsTeamNormalStats,
	#[serde(flatten)]
	pub team_insane: SkyWarsTeamInsaneStats,
}

#[derive(Deserialize, Default, Debug)]
pub struct SkyWarsSoloNormalStats {
	#[serde(rename = "losses_solo_normal")]
	pub losses: u32,
	#[serde(rename = "wins_solo_normal")]
	pub wins: u32,
	#[serde(rename = "kills_solo_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_normal")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug)]
pub struct SkyWarsSoloInsaneStats {
	#[serde(rename = "losses_solo_insane")]
	pub losses: u32,
	#[serde(rename = "wins_solo_insane")]
	pub wins: u32,
	#[serde(rename = "kills_solo_insane")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_insane")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug)]
pub struct SkyWarsTeamNormalStats {
	#[serde(rename = "losses_team_normal")]
	pub losses: u32,
	#[serde(rename = "wins_team_normal")]
	pub wins: u32,
	#[serde(rename = "kills_team_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_team_normal")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug)]
pub struct SkyWarsTeamInsaneStats {
	#[serde(rename = "losses_team_insane")]
	pub losses: u32,
	#[serde(rename = "wins_team_insane")]
	pub wins: u32,
	#[serde(rename = "kills_team_insane")]
	pub kills: u32,
	#[serde(rename = "deaths_team_insane")]
	pub deaths: u32,
}
