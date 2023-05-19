pub mod member;

use chrono::{DateTime, Utc};
use minecraft::colour::Colour;
use once_cell::sync::Lazy;
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Deserializer};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::{
	cache::{GUILD_DATA_MEMBER_CACHE, GUILD_DATA_NAME_CACHE},
	game::r#type::Type,
	http::HTTP,
	ratelimit::HYPIXEL_RATELIMIT,
	xp::Xp,
	Error,
};

use self::member::Member;

static HYPIXEL_GUILD_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/guild").unwrap());

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
	pub guild: Option<Guild>,
	pub success: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Guild {
	pub name: String,
	pub coins: u32,
	#[serde(rename = "exp")]
	pub xp: u32,
	#[serde(rename = "created", with = "chrono::serde::ts_milliseconds")]
	pub created_at: DateTime<Utc>,
	pub description: Option<String>,
	#[serde(rename = "tagColor", default = "Guild::default_guild_colour")]
	pub tag_colour: Colour,
	pub tag: Option<String>,
	#[serde(rename = "guildExpByGameType", deserialize_with = "from_game_xp_map")]
	pub xp_by_game: Vec<(Type, Xp)>,
	pub ranks: Vec<Rank>,
	pub members: Vec<Member>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rank {
	pub name: String,
	pub tag: Option<String>,
	pub priority: u8,
}

fn from_game_xp_map<'de, D>(deserializer: D) -> Result<Vec<(Type, Xp)>, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = Vec<(Type, Xp)>;

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a mapping of dates to numbers")
		}

		fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			let mut vec = Vec::with_capacity(map.size_hint().unwrap_or(0));

			while let Some((game, xp)) = map.next_entry()? {
				vec.push((game, Xp(xp)));
			}

			Ok(vec)
		}
	}

	deserializer.deserialize_map(Visitor)
}

impl Guild {
	#[must_use]
	pub fn default_guild_colour() -> Colour {
		Colour::Gray
	}

	/// # Errors
	/// Returns [`Error::GuildByMemberNotFound`] if the guild could not be found.
	pub async fn from_member_uuid(uuid: Uuid) -> Result<Guild, Arc<Error>> {
		GUILD_DATA_MEMBER_CACHE
			.try_get_with_by_ref(&uuid, Self::from_member_uuid_raw(uuid))
			.await
	}

	/// # Errors
	/// Returns [`Error::GuildNotFound`] if the guild could not be found.
	pub async fn from_name(name: &str) -> Result<Guild, Arc<Error>> {
		GUILD_DATA_NAME_CACHE
			.try_get_with_by_ref(name, Self::from_name_raw(name))
			.await
	}

	async fn from_name_raw(name: &str) -> Result<Guild, Error> {
		let url = {
			let mut url = HYPIXEL_GUILD_API_ENDPOINT.clone();

			url.set_query(Some(&format!("name={}", name)));
			url
		};

		// HYPIXEL_RATELIMIT will always be present, as it is initialized in the main function
		HYPIXEL_RATELIMIT.get().unwrap().until_ready().await;

		tracing::info!("Requesting guild data for {}", name);

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::GuildNotFound(name.to_string()));
		}

		let response = response.json::<Response>().await?;

		response
			.guild
			.ok_or_else(|| Error::GuildNotFound(name.to_string()))
	}

	async fn from_member_uuid_raw(uuid: Uuid) -> Result<Guild, Error> {
		let url = {
			let mut url = HYPIXEL_GUILD_API_ENDPOINT.clone();

			url.set_query(Some(&format!("player={}", uuid)));
			url
		};

		// HYPIXEL_RATELIMIT will always be present, as it is initialized in the main function
		HYPIXEL_RATELIMIT.get().unwrap().until_ready().await;

		tracing::info!("Requesting guild data for {}", uuid);

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::GuildByMemberNotFound(uuid));
		}

		let response = response.json::<Response>().await?;

		response
			.guild
			.ok_or_else(|| Error::GuildByMemberNotFound(uuid))
	}

	#[must_use]
	pub fn get_leader(&self) -> Option<&Member> {
		self.members.iter().find(|m| m.is_leader())
	}
}
