use chrono::{DateTime, NaiveDate, Utc};
use minecraft::colour::Colour;
use once_cell::sync::Lazy;
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Deserializer};
use std::{borrow::Cow, str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::{
	cache::{GUILD_DATA_MEMBER_CACHE, GUILD_DATA_NAME_CACHE},
	game::r#type::Type,
	http::HTTP,
	ratelimit::HYPIXEL_RATELIMIT,
	Error,
};

static HYPIXEL_GUILD_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/guild").unwrap());

#[derive(Deserialize, Debug, Clone)]
pub struct Response<'a> {
	pub guild: Guild<'a>,
	pub success: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Guild<'a> {
	pub name: Cow<'a, str>,
	pub coins: u32,
	#[serde(rename = "exp")]
	pub xp: u32,
	#[serde(rename = "created", with = "chrono::serde::ts_milliseconds")]
	pub created_at: DateTime<Utc>,
	pub description: Option<Cow<'a, str>>,
	#[serde(rename = "preferredGames")]
	pub preferred_games: Vec<Type>,
	#[serde(rename = "tagColor", default = "Guild::default_guild_colour")]
	pub tag_colour: Colour,
	pub tag: Option<Cow<'a, str>>,
	#[serde(rename = "guildExpByGameType", deserialize_with = "from_game_xp_map")]
	pub xp_by_game: Vec<(Type, u32)>,
	pub ranks: Vec<Rank<'a>>,
	pub members: Vec<Member<'a>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rank<'a> {
	pub name: Cow<'a, str>,
	pub tag: Option<Cow<'a, str>>,
	pub priority: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Member<'a> {
	pub uuid: Uuid,
	pub rank: Cow<'a, str>,
	#[serde(rename = "joined", with = "chrono::serde::ts_milliseconds")]
	pub joined_at: DateTime<Utc>,
	#[serde(rename = "questParticipation", default)]
	pub quests: u32,
	#[serde(rename = "expHistory", deserialize_with = "from_date_map")]
	pub xp_history: [(NaiveDate, u32); 7],
}

fn from_date_map<'de, D>(deserializer: D) -> Result<[(NaiveDate, u32); 7], D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor([(NaiveDate, u32); 7]);

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = [(NaiveDate, u32); 7];

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a mapping of dates to numbers")
		}

		fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			let mut i = 0;

			while let Some((date, xp)) = map.next_entry()? {
				self.0[i] = (NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(), xp);

				i += 1;
			}

			Ok(self.0)
		}
	}

	deserializer.deserialize_map(Visitor([(NaiveDate::MIN, 0); 7]))
}

fn from_game_xp_map<'de, D>(deserializer: D) -> Result<Vec<(Type, u32)>, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = Vec<(Type, u32)>;

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a mapping of dates to numbers")
		}

		fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			let mut vec = Vec::with_capacity(map.size_hint().unwrap_or(0));

			while let Some(v) = map.next_entry()? {
				vec.push(v);
			}

			Ok(vec)
		}
	}

	deserializer.deserialize_map(Visitor)
}

impl<'a> Guild<'a> {
	#[must_use]
	pub fn default_guild_colour() -> Colour {
		Colour::Gray
	}

	/// # Errors
	/// Returns [`Error::GuildByMemberNotFound`] if the guild could not be found.
	pub async fn from_member_uuid(uuid: Uuid) -> Result<Guild<'static>, Arc<Error>> {
		GUILD_DATA_MEMBER_CACHE
			.try_get_with_by_ref(&uuid, Self::from_member_uuid_raw(uuid))
			.await
	}

	/// # Errors
	/// Returns [`Error::GuildNotFound`] if the guild could not be found.
	pub async fn from_name(name: &str) -> Result<Guild<'static>, Arc<Error>> {
		GUILD_DATA_NAME_CACHE
			.try_get_with_by_ref(name, Self::from_name_raw(name))
			.await
	}

	async fn from_name_raw(name: &str) -> Result<Guild<'static>, Error> {
		let url = {
			let mut url = HYPIXEL_GUILD_API_ENDPOINT.clone();

			url.set_query(Some(&format!("name={}", name)));
			url
		};

		// HYPIXEL_RATELIMIT will always be present, as it is initialized in the main function
		HYPIXEL_RATELIMIT.get().unwrap().until_ready().await;

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::GuildNotFound(name.to_string()));
		}

		let response = response.json::<Response>().await?;

		Ok(response.guild)
	}

	async fn from_member_uuid_raw(uuid: Uuid) -> Result<Guild<'static>, Error> {
		let url = {
			let mut url = HYPIXEL_GUILD_API_ENDPOINT.clone();

			url.set_query(Some(&format!("player={}", uuid)));
			url
		};

		// HYPIXEL_RATELIMIT will always be present, as it is initialized in the main function
		HYPIXEL_RATELIMIT.get().unwrap().until_ready().await;

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::GuildByMemberNotFound(uuid));
		}

		let response = response.json::<Response>().await?;

		Ok(response.guild)
	}
}
