use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct BedWarsStats {
	pub coins: u32,
}

#[derive(Deserialize, Default, Debug)]
pub struct BedWarsSoloStats {
	#[serde(rename = "eight_one_deaths_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_one_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_one_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_one_final_kills_bedwars")]
	pub final_kills: u32,
}

#[derive(Deserialize, Default, Debug)]
pub struct BedWarsDoubleStats {
	#[serde(rename = "eight_two_deaths_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_final_kills_bedwars")]
	pub final_kills: u32,
}

#[derive(Deserialize, Default, Debug)]
pub struct BedWarsThreeStats {
	#[serde(rename = "four_three_deaths_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_three_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_three_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_three_final_kills_bedwars")]
	pub final_kills: u32,
}

#[derive(Deserialize, Default, Debug)]
pub struct BedWarsFourStats {
	#[serde(rename = "four_four_deaths_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_final_kills_bedwars")]
	pub final_kills: u32,
}
