#[cfg(feature = "game")]
use crate::canvas::diff::DiffLog;
use crate::leaderboard::Leaderboard;

use chrono::{DateTime, Utc};
use extra::minutes::Minutes;
use minecraft::{colour::Colour, text::rank::Rank};
#[cfg(feature = "game")]
use poise::serenity_prelude::CreateEmbed;
use serde::{Deserialize, Deserializer};
#[cfg(feature = "game")]
use translate::context;
use uuid::Uuid;

#[derive(serde::Deserialize, bincode::Encode, bincode::Decode, Default)]
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
	pub language: hypixel::language::Language,
	#[serde(rename(deserialize = "socialMedia"), default, skip_serializing)]
	pub socials: hypixel::socials::Socials,
	#[serde(with = "crate::de::vec_map", default, rename = "parkourCompletions")]
	pub parkour: Vec<(hypixel::game::r#type::Type, Vec<super::parkour::Completion>)>,
}

// Executes the given code, passing in $left and $right on the left and right of each game
macro_rules! execute_for_games {
	([$($left:tt)*], [$($right:tt)*]) => {
		$($left)* $crate::player::stats::arcade::Arcade $($right)*
		$($left)* $crate::player::stats::arena::Arena $($right)*
		$($left)* $crate::player::stats::bed_wars::BedWars $($right)*
		$($left)* $crate::player::stats::blitz_sg::BlitzSg $($right)*
		$($left)* $crate::player::stats::build_battle::BuildBattle $($right)*
		$($left)* $crate::player::stats::cops_and_crims::CopsAndCrims $($right)*
		$($left)* $crate::player::stats::duels::Duels $($right)*
		$($left)* $crate::player::stats::fishing::Fishing $($right)*
		$($left)* $crate::player::stats::mega_walls::MegaWalls $($right)*
		$($left)* $crate::player::stats::murder_mystery::MurderMystery $($right)*
		$($left)* $crate::player::stats::paintball::Paintball $($right)*
		$($left)* $crate::player::stats::pit::Pit $($right)*
		$($left)* $crate::player::stats::quake::Quake $($right)*
		$($left)* $crate::player::stats::sky_wars::SkyWars $($right)*
		$($left)* $crate::player::stats::smash_heroes::SmashHeroes $($right)*
		$($left)* $crate::player::stats::speed_uhc::SpeedUhc $($right)*
		$($left)* $crate::player::stats::tnt_games::TntGames $($right)*
		$($left)* $crate::player::stats::turbo_kart_racers::TurboKartRacers $($right)*
		$($left)* $crate::player::stats::uhc::Uhc $($right)*
		$($left)* $crate::player::stats::vampire_z::VampireZ $($right)*
		$($left)* $crate::player::stats::walls::Walls $($right)*
		$($left)* $crate::player::stats::warlords::Warlords $($right)*
		$($left)* $crate::player::stats::wool_wars::WoolWars $($right)*
	};
}

#[cfg(feature = "game")]
impl DiffLog for Data {
	#[allow(clippy::let_and_return)]
	fn diff_log<'e>(
		data_lhs: &Data,
		data_rhs: &Data,
		ctx: &context::Context<'_>,
		embed: CreateEmbed<'e>,
	) -> Result<CreateEmbed<'e>, CreateEmbed<'e>> {
		let mut is_modified = false;

		execute_for_games!(
			[let embed = match],
			[
			::diff_log(data_lhs, data_rhs, ctx, embed) {
				Ok(embed) => {
					is_modified = true;
					embed
				}
					Err(embed) => embed
				};
			]
		);

		if is_modified {
			Ok(embed)
		} else {
			Err(embed)
		}
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
	pub fn add_to_pipeline(&self, pipeline: &mut redis::Pipeline) {
		execute_for_games!(
			[],
			[::add_to_pipeline(pipeline, self);]
		);
	}

	pub fn leaderboards(ctx: &context::Context<'_>) -> Vec<Leaderboard> {
		let mut leaderboards = Vec::new();

		execute_for_games!([], [::leaderboards(ctx, &mut leaderboards);]);

		leaderboards
	}

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
	#[derive(serde::Deserialize)]
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
	#[derive(serde::Deserialize)]
	struct Quest {
		#[serde(deserialize_with = "vec_len_to_u32", default)]
		completions: u32,
	}

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
