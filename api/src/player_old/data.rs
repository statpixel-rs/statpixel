use chrono::{DateTime, Utc};
use minecraft::colour::Colour;
use serde::Deserialize;
use uuid::Uuid;

use crate::minutes::Minutes;

pub const VERSION: i16 = 14;

#[derive(Deserialize, bincode::Encode, bincode::Decode)]
pub struct Data {
	pub username: String,
	#[bincode(with_serde)]
	pub uuid: Uuid,
	#[serde(skip)]
	pub stats: super::stats::Stats,
	pub(crate) status_rank: Option<String>,
	pub(crate) rank: Option<String>,
	pub(crate) package_rank: Option<String>,
	pub(crate) rank_colour: Option<Colour>,
	pub(crate) monthly_rank_colour: Option<Colour>,
	pub prefix: Option<String>,
	pub xp: u64,
	pub karma: u64,
	#[serde(rename = "firstLogin", with = "chrono::serde::ts_milliseconds_option")]
	#[bincode(with_serde)]
	pub first_login: Option<DateTime<Utc>>,
	#[serde(rename = "lastLogin", with = "chrono::serde::ts_milliseconds_option")]
	#[bincode(with_serde)]
	pub last_login: Option<DateTime<Utc>>,
	pub playtime: Minutes,
	pub rewards: u32,
	pub friend_requests: u32,
	pub challenges: u32,
	pub quests: u32,
	pub gifting: crate::player::data::Gifting,
	pub achivement_points: u32,
	pub language: crate::player::language::Language,
	pub socials: crate::player::socials::Socials,
}

impl From<Data> for crate::player::data::Data {
	fn from(value: Data) -> Self {
		Self {
			username: value.username,
			uuid: value.uuid,
			stats: value.stats.into(),
			status_rank: value.status_rank,
			rank: value.rank,
			package_rank: value.package_rank,
			rank_colour: value.rank_colour,
			monthly_rank_colour: value.monthly_rank_colour,
			prefix: value.prefix,
			xp: value.xp,
			karma: value.karma,
			first_login: value.first_login,
			last_login: value.last_login,
			playtime: value.playtime,
			rewards: value.rewards,
			friend_requests: value.friend_requests,
			challenges: value.challenges,
			quests: value.quests,
			gifting: value.gifting,
			achivement_points: value.achivement_points,
			language: value.language,
			socials: value.socials,
		}
	}
}
