use minecraft::{colour::Colour, text::rank::Rank};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Data {
	#[serde(rename = "displayname")]
	pub username: String,
	#[serde(default)]
	pub stats: super::stats::Stats,
	#[serde(rename = "rank")]
	status_rank: Option<String>,
	#[serde(rename = "newPackageRank")]
	rank: Option<String>,
	#[serde(rename = "monthlyPackageRank")]
	package_rank: Option<String>,
	#[serde(rename = "rankPlusColor")]
	rank_colour: Option<Colour>,
	#[serde(rename = "monthlyRankColor")]
	monthly_rank_colour: Option<Colour>,
	pub prefix: Option<String>,
	#[serde(
		rename = "networkExp",
		deserialize_with = "super::stats::from_trunc_f32_to_u64"
	)]
	pub xp: u64,
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
