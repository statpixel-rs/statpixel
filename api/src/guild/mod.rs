pub mod member;

use chrono::{DateTime, Utc};
#[cfg(feature = "database")]
use database::schema::guild_autocomplete;
use derive::Diff;
#[cfg(feature = "database")]
use diesel::ExpressionMethods;
#[cfg(feature = "database")]
use diesel_async::RunQueryDsl;
use hypixel::game::r#type::Type;
use minecraft::colour::Colour;
use once_cell::sync::Lazy;
use reqwest::{Request, StatusCode, Url};
use serde::Deserializer;
use std::{str::FromStr, sync::Arc};
#[cfg(feature = "database")]
use translate::context::Context;
use uuid::Uuid;

use crate::{http::HTTP, xp::Xp, Error, Player};

#[cfg(feature = "cache")]
use crate::cache::*;

use self::member::Member;

pub const VERSION: i16 = 0;

static HYPIXEL_GUILD_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/guild").unwrap());

#[derive(serde::Deserialize)]
pub struct Response {
	pub guild: Option<Guild>,
	pub success: bool,
}

#[derive(serde::Deserialize, bincode::Encode, bincode::Decode, Clone, Diff)]
pub struct Guild {
	#[serde(rename = "_id", deserialize_with = "hex_from_str")]
	pub id: u128,
	pub name: String,
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	pub coins: u32,
	#[serde(rename = "exp")]
	pub xp: u32,
	#[bincode(with_serde)]
	#[serde(rename = "created", with = "chrono::serde::ts_milliseconds")]
	pub created_at: DateTime<Utc>,
	pub description: Option<String>,
	#[serde(rename = "tagColor", default = "Guild::default_guild_colour")]
	pub tag_colour: Colour,
	pub tag: Option<String>,
	#[serde(rename = "guildExpByGameType", deserialize_with = "from_game_xp_map")]
	pub xp_by_game: Vec<(Type, Xp)>,
	#[serde(default, deserialize_with = "from_sorted_ranks")]
	pub ranks: Vec<Rank>,
	#[serde(deserialize_with = "from_sorted_members")]
	pub members: Vec<Member>,
	#[serde(rename = "preferredGames", default)]
	pub preferred_games: Vec<Type>,
}

fn from_sorted_ranks<'de, D>(deserializer: D) -> Result<Vec<Rank>, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = Vec<Rank>;

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a list of ranks, sorted by priority desc")
		}

		fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::SeqAccess<'de>,
		{
			let mut vec: Vec<Rank> = Vec::with_capacity(seq.size_hint().unwrap_or(0));

			while let Some(member) = seq.next_element()? {
				vec.push(member);
			}

			vec.sort_unstable_by_key(|m| std::cmp::Reverse(m.priority));

			Ok(vec)
		}
	}

	deserializer.deserialize_seq(Visitor)
}

fn from_sorted_members<'de, D>(deserializer: D) -> Result<Vec<Member>, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = Vec<Member>;

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a list of members, sorted by xp gained")
		}

		fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::SeqAccess<'de>,
		{
			let mut vec: Vec<Member> = Vec::with_capacity(seq.size_hint().unwrap_or(0));

			while let Some(member) = seq.next_element()? {
				vec.push(member);
			}

			vec.sort_by_cached_key(|m| m.xp_history.iter().map(|h| h.1).sum::<u32>());

			Ok(vec)
		}
	}

	deserializer.deserialize_seq(Visitor)
}

fn hex_from_str<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = serde::Deserialize::deserialize(deserializer)?;

	u128::from_str_radix(s, 16).map_err(serde::de::Error::custom)
}

#[derive(serde::Deserialize, bincode::Encode, bincode::Decode, Debug, Clone)]
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

			vec.sort_unstable_by_key(|g| g.1);

			Ok(vec)
		}
	}

	deserializer.deserialize_map(Visitor)
}

impl Player {
	pub async fn get_guild(&self) -> Option<Arc<Guild>> {
		Guild::from_member_uuid(self.uuid).await.ok()
	}
}

impl Guild {
	#[must_use]
	pub fn default_guild_colour() -> Colour {
		Colour::Gray
	}

	#[allow(clippy::cast_possible_wrap)]
	/// # Errors
	/// Returns an error if the query could not be executed.
	#[cfg(feature = "database")]
	pub async fn increase_searches(&self, ctx: &Context<'_>) -> Result<(), translate::Error> {
		diesel::insert_into(guild_autocomplete::table)
			.values((
				guild_autocomplete::name.eq(&self.name),
				guild_autocomplete::uuid.eq(&Uuid::from_u128(self.id)),
				guild_autocomplete::searches.eq(1),
				guild_autocomplete::xp.eq(self.xp as i32),
			))
			.on_conflict(guild_autocomplete::uuid)
			.do_update()
			.set((
				guild_autocomplete::name.eq(&self.name),
				guild_autocomplete::searches.eq(guild_autocomplete::searches + 1),
				guild_autocomplete::xp.eq(self.xp as i32),
			))
			.execute(&mut ctx.data().pool.get().await?)
			.await?;

		Ok(())
	}

	/// # Errors
	/// Returns [`Error::GuildByMemberUuidNotFound`] if the guild could not be found.
	pub async fn from_member_uuid(uuid: Uuid) -> Result<Arc<Guild>, Arc<Error>> {
		#[cfg(feature = "cache")]
		return crate::cache::GUILD_DATA_MEMBER_CACHE
			.try_get_with_by_ref(&uuid, Self::from_member_uuid_raw(uuid))
			.await;

		#[cfg(not(feature = "cache"))]
		Self::from_member_uuid_raw(uuid).await.map_err(Arc::new)
	}

	/// # Errors
	/// Returns [`Error::GuildNotFound`] if the guild could not be found.
	pub async fn from_name(name: &str) -> Result<Arc<Guild>, Arc<Error>> {
		#[cfg(feature = "cache")]
		return crate::cache::GUILD_DATA_NAME_CACHE
			.try_get_with(name.to_ascii_lowercase(), Self::from_name_raw(name))
			.await;

		#[cfg(not(feature = "cache"))]
		Self::from_name_raw(name).await.map_err(Arc::new)
	}

	/// # Errors
	/// Returns [`Error::GuildNotFound`] if the guild could not be found.
	pub async fn from_uuid(uuid: Uuid) -> Result<Arc<Guild>, Arc<Error>> {
		#[cfg(feature = "cache")]
		return crate::cache::GUILD_DATA_UUID_CACHE
			.try_get_with_by_ref(&uuid, Self::from_uuid_raw(uuid))
			.await;

		#[cfg(not(feature = "cache"))]
		Self::from_uuid_raw(uuid).await.map_err(Arc::new)
	}

	async fn from_uuid_raw(uuid: Uuid) -> Result<Arc<Guild>, Error> {
		let n = uuid.as_u128();
		// format as hex, left-pad with 0s to 24 characters
		let id = format!("{:024x}", n);

		let url = {
			let mut url = HYPIXEL_GUILD_API_ENDPOINT.clone();

			url.set_query(Some(&format!("id={}", id)));
			url
		};

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_hypixel(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::GuildNotFound(id));
		}

		let response = response.json::<Response>().await?;

		if let Some(guild) = response.guild {
			let guild = Arc::new(guild);

			#[cfg(feature = "cache")]
			for member in &guild.members {
				GUILD_DATA_MEMBER_CACHE
					.insert(member.uuid, Arc::clone(&guild))
					.await;
			}

			#[cfg(feature = "cache")]
			GUILD_DATA_NAME_CACHE
				.insert(guild.name.to_ascii_lowercase(), Arc::clone(&guild))
				.await;

			return Ok(guild);
		}

		Err(Error::GuildNotFound(id))
	}

	async fn from_name_raw(name: &str) -> Result<Arc<Guild>, Error> {
		let url = {
			let mut url = HYPIXEL_GUILD_API_ENDPOINT.clone();

			url.set_query(Some(&format!("name={}", name)));
			url
		};

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_hypixel(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::GuildNotFound(name.to_string()));
		}

		let response = response.json::<Response>().await?;

		if let Some(guild) = response.guild {
			let guild = Arc::new(guild);

			#[cfg(feature = "cache")]
			for member in &guild.members {
				GUILD_DATA_MEMBER_CACHE
					.insert(member.uuid, Arc::clone(&guild))
					.await;
			}

			#[cfg(feature = "cache")]
			GUILD_DATA_UUID_CACHE
				.insert(Uuid::from_u128(guild.id), Arc::clone(&guild))
				.await;

			return Ok(guild);
		}

		Err(Error::GuildNotFound(name.to_string()))
	}

	async fn from_member_uuid_raw(uuid: Uuid) -> Result<Arc<Guild>, Error> {
		let url = {
			let mut url = HYPIXEL_GUILD_API_ENDPOINT.clone();

			url.set_query(Some(&format!("player={}", uuid)));
			url
		};

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_hypixel(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::GuildByMemberUuidNotFound(uuid));
		}

		let response = response.json::<Response>().await?;

		if let Some(guild) = response.guild {
			let guild = Arc::new(guild);

			#[cfg(feature = "cache")]
			for member in &guild.members {
				if member.uuid == uuid {
					continue;
				}

				GUILD_DATA_MEMBER_CACHE
					.insert(member.uuid, Arc::clone(&guild))
					.await;
			}

			#[cfg(feature = "cache")]
			GUILD_DATA_UUID_CACHE
				.insert(Uuid::from_u128(guild.id), Arc::clone(&guild))
				.await;

			#[cfg(feature = "cache")]
			GUILD_DATA_NAME_CACHE
				.insert(guild.name.to_ascii_lowercase(), Arc::clone(&guild))
				.await;

			return Ok(guild);
		}

		Err(Error::GuildByMemberUuidNotFound(uuid))
	}

	#[must_use]
	pub fn get_leader(&self) -> Option<&Member> {
		self.members.iter().find(|m| m.is_leader())
	}
}
