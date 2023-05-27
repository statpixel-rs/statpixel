use minecraft::colour::Colour;
use serde::Deserialize;

use crate::minutes::Minutes;

pub const VERSION: i16 = 0;

#[derive(Deserialize, bincode::Encode, bincode::Decode, Debug, Clone, PartialEq)]
pub struct Data {
	#[serde(rename = "displayname")]
	pub username: String,
	#[serde(default)]
	pub stats: super::stats::Stats,
	#[serde(rename = "rank")]
	pub(super) status_rank: Option<String>,
	#[serde(rename = "newPackageRank")]
	pub(super) rank: Option<String>,
	#[serde(rename = "monthlyPackageRank")]
	pub(super) package_rank: Option<String>,
	#[serde(rename = "rankPlusColor")]
	pub(super) rank_colour: Option<Colour>,
	#[serde(rename = "monthlyRankColor")]
	pub(super) monthly_rank_colour: Option<Colour>,
	pub prefix: Option<String>,
	#[serde(
		rename = "networkExp",
		deserialize_with = "super::stats::from_trunc_f32_to_u64"
	)]
	pub xp: u64,
}

impl From<Data> for super::data::Data {
	fn from(value: Data) -> Self {
		Self {
			username: value.username,
			stats: value.stats,
			status_rank: value.status_rank,
			rank: value.rank,
			package_rank: value.package_rank,
			rank_colour: value.rank_colour,
			monthly_rank_colour: value.monthly_rank_colour,
			prefix: value.prefix,
			xp: value.xp,
			karma: 0,
			first_login: None,
			last_login: None,
			playtime: Minutes(0),
			rewards: 0,
			friend_requests: 0,
		}
	}
}
