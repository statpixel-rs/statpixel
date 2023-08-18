use chrono::{DateTime, Utc};
use minecraft::{colour::Colour, text::rank::Rank};
use poise::serenity_prelude::Embed;
use serde::{Deserialize, Deserializer};
use translate::context;
use uuid::Uuid;

use crate::{canvas::diff::DiffLog, minutes::Minutes};

#[derive(serde::Deserialize, serde::Serialize, bincode::Encode, bincode::Decode, Default)]
#[serde(default)]
pub struct Data {
	#[serde(rename(deserialize = "displayname"))]
	pub username: String,
	#[bincode(with_serde)]
	pub uuid: Uuid,
	#[serde(default)]
	pub stats: super::stats::Stats,
	#[serde(rename = "rank")]
	pub(crate) status_rank: Option<String>,
	#[serde(rename = "newPackageRank")]
	pub(crate) rank: Option<String>,
	#[serde(rename(deserialize = "monthlyPackageRank"))]
	pub(crate) package_rank: Option<String>,
	#[serde(rename(deserialize = "rankPlusColor"))]
	pub(crate) rank_colour: Option<Colour>,
	#[serde(rename(deserialize = "monthlyRankColor"))]
	pub(crate) monthly_rank_colour: Option<Colour>,
	pub prefix: Option<String>,
	#[serde(
		rename(deserialize = "networkExp"),
		deserialize_with = "crate::de::from::f64_to_u64",
		default
	)]
	pub xp: u64,
	#[serde(default, deserialize_with = "crate::de::from::f64_to_u64")]
	pub karma: u64,
	#[serde(
		rename(deserialize = "firstLogin"),
		with = "chrono::serde::ts_milliseconds_option"
	)]
	#[bincode(with_serde)]
	pub first_login: Option<DateTime<Utc>>,
	#[serde(
		rename(deserialize = "lastLogin"),
		with = "chrono::serde::ts_milliseconds_option"
	)]
	#[bincode(with_serde)]
	pub last_login: Option<DateTime<Utc>>,
	#[serde(rename(deserialize = "timePlaying"), default)]
	pub playtime: Minutes,
	#[serde(rename(deserialize = "totalRewards"), default)]
	pub rewards: u32,
	#[serde(deserialize_with = "vec_len_to_u32", default)]
	pub friend_requests: u32,
	#[serde(deserialize_with = "from_challenges", default)]
	pub challenges: u32,
	#[serde(deserialize_with = "from_quests", default)]
	pub quests: u32,
	#[serde(rename(deserialize = "giftingMeta"), default)]
	pub gifting: Gifting,
	#[serde(rename(deserialize = "achievementPoints"))]
	pub achivement_points: u32,
	#[serde(rename(deserialize = "userLanguage"), default)]
	pub language: super::language::Language,
	#[serde(rename(deserialize = "socialMedia"), default, skip_serializing)]
	pub socials: super::socials::Socials,
}

impl DiffLog for Data {
	fn diff_log(
		data_lhs: &Data,
		data_rhs: &Data,
		ctx: &context::Context<'_>,
		embed: Embed,
	) -> Embed {
		use super::stats::*;

		let embed = arcade::Arcade::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = arena::Arena::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = bed_wars::BedWars::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = blitz_sg::BlitzSg::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = build_battle::BuildBattle::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = cops_and_crims::CopsAndCrims::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = duels::Duels::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = mega_walls::MegaWalls::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = murder_mystery::MurderMystery::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = paintball::Paintball::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = pit::Pit::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = quake::Quake::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = sky_wars::SkyWars::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = smash_heroes::SmashHeroes::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = speed_uhc::SpeedUhc::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = tnt_games::TntGames::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = turbo_kart_racers::TurboKartRacers::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = uhc::Uhc::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = vampire_z::VampireZ::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = walls::Walls::diff_log(data_lhs, data_rhs, ctx, embed);
		let embed = warlords::Warlords::diff_log(data_lhs, data_rhs, ctx, embed);
		wool_wars::WoolWars::diff_log(data_lhs, data_rhs, ctx, embed)
	}
}

#[derive(
	serde::Deserialize,
	serde::Serialize,
	bincode::Encode,
	bincode::Decode,
	Debug,
	Clone,
	PartialEq,
	Default,
)]
#[serde(default)]
pub struct Gifting {
	#[serde(rename = "giftsGiven")]
	pub gifts_given: u32,
	#[serde(rename = "ranksGiven")]
	pub ranks_given: u32,
}

impl Data {
	#[must_use]
	pub fn placeholder() -> Self {
		Self {
			username: "StatPixel".to_string(),
			rank: Some("MVP_PLUS".to_string()),
			package_rank: Some("MVP_PLUS_PLUS".to_string()),
			monthly_rank_colour: Some(Colour::Gold),
			..Self::default()
		}
	}

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
	struct V;

	impl<'de> serde::de::Visitor<'de> for V {
		type Value = u32;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			formatter.write_str("a number or a sequence")
		}

		fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			v.try_into().map_err(serde::de::Error::custom)
		}

		fn visit_seq<A>(self, map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::SeqAccess<'de>,
		{
			let s: Vec<serde::de::IgnoredAny> =
				Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(map))?;

			s.len().try_into().map_err(serde::de::Error::custom)
		}
	}

	deserializer.deserialize_any(V)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn from_challenges<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	#[derive(Deserialize)]
	struct Challenges {
		#[serde(with = "crate::de::vec_map_no_key")]
		all_time: Vec<u32>,
	}

	struct V;

	impl<'de> serde::de::Visitor<'de> for V {
		type Value = u32;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			formatter.write_str("a number or object with an `all_time` field")
		}

		fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			v.try_into().map_err(serde::de::Error::custom)
		}

		fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			let c: Challenges =
				Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;

			Ok(c.all_time.into_iter().sum::<u32>())
		}
	}

	deserializer.deserialize_any(V)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn from_quests<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	#[derive(Deserialize)]
	struct Quest {
		#[serde(deserialize_with = "vec_len_to_u32", default)]
		completions: u32,
	}

	#[derive(Deserialize)]
	struct Quests(#[serde(with = "crate::de::vec_map_no_key")] Vec<Quest>);

	struct V;

	impl<'de> serde::de::Visitor<'de> for V {
		type Value = u32;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			formatter.write_str("a number or map")
		}

		fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			v.try_into().map_err(serde::de::Error::custom)
		}

		fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			let q: Vec<Quest> = crate::de::vec_map_no_key::deserialize(
				serde::de::value::MapAccessDeserializer::new(map),
			)?;

			Ok(q.into_iter().map(|q| q.completions).sum::<u32>())
		}
	}

	deserializer.deserialize_any(V)
}
