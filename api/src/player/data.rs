use chrono::{DateTime, Utc};
use minecraft::{colour::Colour, text::rank::Rank};
use serde::{Deserialize, Deserializer};

use crate::minutes::Minutes;

#[derive(Deserialize, bincode::Encode, bincode::Decode, Debug, Clone, PartialEq, Default)]
#[serde(default)]
pub struct Data {
	#[serde(rename = "displayname")]
	pub username: String,
	#[serde(default)]
	pub stats: super::stats::Stats,
	#[serde(rename = "rank")]
	pub(crate) status_rank: Option<String>,
	#[serde(rename = "newPackageRank")]
	pub(crate) rank: Option<String>,
	#[serde(rename = "monthlyPackageRank")]
	pub(crate) package_rank: Option<String>,
	#[serde(rename = "rankPlusColor")]
	pub(crate) rank_colour: Option<Colour>,
	#[serde(rename = "monthlyRankColor")]
	pub(crate) monthly_rank_colour: Option<Colour>,
	pub prefix: Option<String>,
	#[serde(
		rename = "networkExp",
		deserialize_with = "super::stats::from_trunc_f32_to_u64",
		default
	)]
	pub xp: u64,
	#[serde(rename = "networkLevel", default)]
	pub karma: u64,
	#[serde(rename = "firstLogin", with = "chrono::serde::ts_milliseconds_option")]
	#[bincode(with_serde)]
	pub first_login: Option<DateTime<Utc>>,
	#[serde(rename = "lastLogin", with = "chrono::serde::ts_milliseconds_option")]
	#[bincode(with_serde)]
	pub last_login: Option<DateTime<Utc>>,
	#[serde(rename = "timePlaying", default)]
	pub playtime: Minutes,
	#[serde(rename = "totalRewards", default)]
	pub rewards: u32,
	#[serde(deserialize_with = "vec_len_to_u32", default)]
	pub friend_requests: u32,
}

impl Data {
	#[must_use]
	pub fn get_rank(&self) -> Rank {
		if let Some(prefix) = self.prefix.as_ref() {
			return Rank::Custom(prefix.to_string());
		}

		if let Some(rank) = self.status_rank.as_ref() {
			return Rank::from_str(
				rank,
				self.package_rank.as_deref(),
				self.rank_colour,
				self.monthly_rank_colour,
			);
		}

		if let Some(rank) = self.rank.as_ref() {
			return Rank::from_str(
				rank,
				self.package_rank.as_deref(),
				self.rank_colour,
				self.monthly_rank_colour,
			);
		}

		Rank::Default
	}
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn vec_len_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Vec<&str> = Deserialize::deserialize(deserializer)?;

	Ok(s.len() as u32)
}
