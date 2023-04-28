use minecraft::{colour::MinecraftColour, text::rank::Rank};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerData {
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
	rank_colour: Option<MinecraftColour>,
	#[serde(rename = "monthlyRankColor")]
	monthly_rank_colour: Option<MinecraftColour>,
	pub prefix: Option<String>,
}

impl PlayerData {
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
