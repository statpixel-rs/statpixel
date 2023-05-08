use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Stats {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	pub coins: u32,
	#[serde(rename = "magical_chest")]
	pub magical_chests: u32,
	#[serde(rename = "keys")]
	pub magical_keys: u32,
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	pub rating: u32,

	#[serde(flatten)]
	pub solo: SoloStats,
	#[serde(flatten)]
	pub double: DoubleStats,
	#[serde(flatten)]
	pub four: FourStats,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SoloStats {
	#[serde(rename = "wins_1v1")]
	pub wins: u32,
	#[serde(rename = "losses_1v1")]
	pub losses: u32,
	#[serde(rename = "kills_1v1")]
	pub kills: u32,
	#[serde(rename = "deaths_1v1")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct DoubleStats {
	#[serde(rename = "wins_2v2")]
	pub wins: u32,
	#[serde(rename = "losses_2v2")]
	pub losses: u32,
	#[serde(rename = "kills_2v2")]
	pub kills: u32,
	#[serde(rename = "deaths_2v2")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct FourStats {
	#[serde(rename = "wins_4v4")]
	pub wins: u32,
	#[serde(rename = "losses_4v4")]
	pub losses: u32,
	#[serde(rename = "kills_4v4")]
	pub kills: u32,
	#[serde(rename = "deaths_4v4")]
	pub deaths: u32,
}
