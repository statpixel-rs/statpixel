use chrono::{DateTime, Utc};
use minecraft::colour::Colour;
use serde::Deserialize;

use crate::minutes::Minutes;

pub const VERSION: i16 = 9;

#[derive(Deserialize, bincode::Encode, bincode::Decode)]
pub struct Data {
	pub username: String,
	#[serde(skip)]
	pub stats: crate::player::stats::Stats,
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
}

impl From<Data> for crate::player::data::Data {
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
			karma: value.karma,
			first_login: value.first_login,
			last_login: value.last_login,
			playtime: value.playtime,
			rewards: value.rewards,
			friend_requests: value.friend_requests,
			challenges: 0,
			quests: 0,
			gifting: crate::player::data::Gifting::default(),
			achivement_points: 0,
			language: crate::player::language::Language::default(),
		}
	}
}
