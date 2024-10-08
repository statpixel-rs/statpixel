pub mod data;
pub mod games;
pub mod parkour;
pub mod stats;

pub use hypixel::player::status;

#[cfg(feature = "database")]
use database::schema::{autocomplete, snapshot, user};
#[cfg(feature = "database")]
use diesel::{ExpressionMethods, QueryDsl};
#[cfg(feature = "database")]
use diesel_async::RunQueryDsl;
use once_cell::sync::Lazy;
use reqwest::{Request, RequestBuilder, StatusCode, Url};
use serde::Deserialize;
use std::{borrow::Cow, str::FromStr, sync::Arc, time::Duration};
use tracing::error;
use translate::context;
use uuid::Uuid;

use crate::{http::HTTP, Error};

#[cfg(feature = "cache")]
use crate::cache::{PLAYER_CACHE, PLAYER_DATA_CACHE, PLAYER_SESSION_CACHE};

use self::status::Status;

pub const VERSION: i16 = 20;
pub static DEFAULT_SKIN: Lazy<image::Image> =
	image::include_image!("../../../../assets/skins/steve.png");

static HYPIXEL_PLAYER_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/v2/player").unwrap());

static HYPIXEL_STATUS_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/v2/status").unwrap());

static MOJANG_USERNAME_TO_UUID_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.mojang.com/users/profiles/minecraft/").unwrap());

static MINETOOLS_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.minetools.eu/uuid/").unwrap());

static MOJANG_UUID_TO_USERNAME_API_ENDPOINT: Lazy<Url> = Lazy::new(|| {
	Url::from_str("https://sessionserver.mojang.com/session/minecraft/profile/").unwrap()
});

#[derive(Deserialize)]
pub struct Response {
	pub player: Option<data::Data>,
	pub success: bool,
}

#[derive(Deserialize, Clone)]
pub struct MojangResponse {
	pub name: String,
	pub id: Uuid,
}

#[derive(Deserialize, Clone)]
pub struct MineToolsResponse {
	pub name: Option<String>,
	pub id: Option<Uuid>,
}

#[derive(Clone, Debug)]
pub struct Player {
	pub uuid: Uuid,
	pub username: Option<String>,
	pub session: Option<(Uuid, i64)>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SkinDataResponse {
	pub properties: Vec<SkinData>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SkinData {
	#[serde(deserialize_with = "crate::de::base64::json")]
	pub value: DecodedSkin,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DecodedSkin {
	pub textures: DecodedTexture,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DecodedTexture {
	#[serde(rename = "SKIN")]
	pub skin: Skin,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Skin {
	pub url: String,
	pub metadata: Option<Metadata>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Metadata {
	pub model: String,
}

impl Player {
	#[must_use]
	pub fn new(uuid: Uuid, username: Option<String>) -> Self {
		Self {
			uuid,
			username,
			session: None,
		}
	}

	/// Creates a new player from a uuid without any validation.
	/// The username and session will be [`None`].
	#[must_use]
	pub fn from_uuid_unchecked(uuid: Uuid) -> Self {
		Self {
			uuid,
			username: None,
			session: None,
		}
	}

	/// # Errors
	/// Returns an error if there is an issue with the database.
	#[cfg(feature = "database")]
	pub async fn increase_searches(
		&self,
		ctx: &context::Context<'_>,
	) -> Result<(), translate::Error> {
		if let Some(ref username) = self.username {
			diesel::insert_into(autocomplete::table)
				.values((
					autocomplete::name.eq(username),
					autocomplete::id.eq(&self.uuid),
					autocomplete::searches.eq(1),
				))
				.on_conflict(autocomplete::id)
				.do_update()
				.set((
					autocomplete::name.eq(username),
					autocomplete::searches.eq(autocomplete::searches + 1),
				))
				.execute(&mut ctx.connection().await?)
				.await?;
		} else {
			diesel::update(autocomplete::table)
				.filter(autocomplete::id.eq(&self.uuid))
				.set((autocomplete::searches.eq(autocomplete::searches + 1),))
				.execute(&mut ctx.connection().await?)
				.await?;
		}

		Ok(())
	}

	#[cfg(feature = "database")]
	pub async fn get_suffix(&self, ctx: &context::Context<'_>) -> Option<String> {
		let Ok(mut connnection) = ctx.connection().await else {
			return None;
		};

		user::table
			.filter(user::uuid.eq(&self.uuid))
			.select(user::suffix)
			.first::<Option<String>>(&mut connnection)
			.await
			.ok()
			.flatten()
	}

	/// # Errors
	/// Returns an error if the username does not exist or if their data is invalid.
	pub async fn from_username(username: &str) -> Result<Player, Arc<Error>> {
		#[cfg(feature = "cache")]
		let player = PLAYER_CACHE
			.try_get_with(
				username.to_ascii_lowercase(),
				Self::from_username_raw(username),
			)
			.await?;

		#[cfg(not(feature = "cache"))]
		let player = Self::from_username_raw(username).await?;

		Ok(player)
	}

	async fn from_username_raw(username: &str) -> Result<Player, Error> {
		let url = MINETOOLS_API_ENDPOINT.join(username).unwrap();
		let req = Request::new(reqwest::Method::GET, url);
		let req = RequestBuilder::from_parts(HTTP.client.clone(), req)
			.timeout(Duration::from_millis(1_000))
			.build()?;

		if let Ok(response) = HTTP.perform_bare(req).await
			&& response.status() != StatusCode::SERVICE_UNAVAILABLE
		{
			if response.status() != StatusCode::OK {
				return Err(Error::UsernameNotFound(username.to_string()));
			}

			let response = response.json::<MineToolsResponse>().await?;

			if let Some((id, name)) = response.id.zip(response.name) {
				let player = Self::new(id, Some(name));

				#[cfg(feature = "cache")]
				PLAYER_CACHE.insert(id.to_string(), player.clone()).await;

				return Ok(player);
			}
		}

		// If the id or name was not present in the above response, the username
		// may or may not be invalid. Therefore, we still need to check with Mojang.
		let url = MOJANG_USERNAME_TO_UUID_API_ENDPOINT.join(username).unwrap();
		let req = Request::new(reqwest::Method::GET, url);

		let response = HTTP.perform_mojang(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::UsernameNotFound(username.to_string()));
		}

		let response = response.json::<MojangResponse>().await?;
		let player = Self::new(response.id, Some(response.name));

		// Also add the player with their uuid to the cache
		#[cfg(feature = "cache")]
		PLAYER_CACHE
			.insert(response.id.to_string(), player.clone())
			.await;

		Ok(player)
	}

	/// # Errors
	/// Returns an error if the uuid does not exist or if their data is invalid.
	pub async fn from_uuid(uuid: &Uuid) -> Result<Player, Arc<Error>> {
		#[cfg(feature = "cache")]
		return PLAYER_CACHE
			.try_get_with(uuid.to_string(), Self::from_uuid_raw(uuid))
			.await;

		#[cfg(not(feature = "cache"))]
		Self::from_uuid_raw(uuid).await.map_err(Arc::new)
	}

	async fn from_uuid_raw(uuid: &Uuid) -> Result<Player, Error> {
		let url = MINETOOLS_API_ENDPOINT.join(&uuid.to_string()).unwrap();
		let req = Request::new(reqwest::Method::GET, url);
		let req = RequestBuilder::from_parts(HTTP.client.clone(), req)
			.timeout(Duration::from_millis(1_000))
			.build()?;

		if let Ok(response) = HTTP.perform_bare(req).await
			&& response.status() != StatusCode::SERVICE_UNAVAILABLE
		{
			if response.status() != StatusCode::OK {
				return Err(Error::UuidNotFound(*uuid));
			}

			let response = response.json::<MineToolsResponse>().await?;

			if let Some((id, name)) = response.id.zip(response.name) {
				let player = Self::new(id, Some(name));

				#[cfg(feature = "cache")]
				PLAYER_CACHE.insert(id.to_string(), player.clone()).await;

				return Ok(player);
			}
		}

		let url = MOJANG_UUID_TO_USERNAME_API_ENDPOINT
			.join(&uuid.to_string())
			.unwrap();
		let req = Request::new(reqwest::Method::GET, url);

		let response = HTTP.perform_mojang(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::UuidNotFound(*uuid));
		}

		let response = response.json::<MojangResponse>().await?;
		#[cfg(feature = "cache")]
		let lower = response.name.to_ascii_lowercase();
		let player = Self::new(response.id, Some(response.name));

		// Also add the player to the cache with the lower-case username
		#[cfg(feature = "cache")]
		PLAYER_CACHE.insert(lower, player.clone()).await;

		Ok(player)
	}

	/// # Errors
	/// Returns an error if the player does not have a profile or if their data is invalid.
	pub async fn get_data(
		&self,
		ctx: &context::Context<'_>,
	) -> Result<Arc<data::Data>, Arc<Error>> {
		#[cfg(feature = "cache")]
		return Box::pin(PLAYER_DATA_CACHE.try_get_with_by_ref(
			self.session.as_ref().map_or(&self.uuid, |s| &s.0),
			self.get_data_raw(ctx),
		))
		.await;

		#[cfg(not(feature = "cache"))]
		self.get_data_raw(ctx).await.map_err(Arc::new)
	}

	/// # Errors
	/// Returns an error if the player does not have a profile or if their data is invalid.
	pub async fn get_data_owned(
		self,
		ctx: &context::Context<'_>,
	) -> Result<Arc<data::Data>, Arc<Error>> {
		#[cfg(feature = "cache")]
		return Box::pin(PLAYER_DATA_CACHE.try_get_with(
			self.session.map_or(self.uuid, |s| s.0),
			self.get_data_raw(ctx),
		))
		.await;

		#[cfg(not(feature = "cache"))]
		self.get_data_raw(ctx).await.map_err(Arc::new)
	}

	#[allow(unused_variables)]
	async fn get_data_raw(&self, ctx: &context::Context<'_>) -> Result<Arc<data::Data>, Error> {
		let player = if let Some((_, snapshot_id)) = self.session {
			#[cfg(feature = "database")]
			let result = {
				let data = snapshot::table
					.filter(snapshot::id.eq(snapshot_id))
					.select(snapshot::data)
					.first::<Vec<u8>>(&mut ctx.connection().await?)
					.await?;

				crate::snapshot::user::decode(data.as_slice())
			};

			#[cfg(not(feature = "database"))]
			let result = Err(Error::UuidNotFound(self.uuid));

			result?
		} else {
			let url = {
				let mut url = HYPIXEL_PLAYER_API_ENDPOINT.clone();

				url.set_query(Some(&format!("uuid={}", self.uuid)));
				url
			};

			let req = Request::new(reqwest::Method::GET, url);
			let response = HTTP
				.perform_hypixel(req.into())
				.await
				.map_err(Error::from)?;

			if response.status() != StatusCode::OK {
				return Err(Error::PlayerNotFound(
					self.username
						.as_ref()
						.map_or_else(|| self.uuid.to_string(), std::clone::Clone::clone),
				));
			}

			let response = match response.json::<Response>().await {
				Ok(response) => response,
				Err(err) => {
					error!("Failed to deserialize {} data: {}", self.uuid, err);

					return Err(err.into());
				}
			};

			let Some(player) = response.player else {
				return Err(Error::PlayerNotFound(
					self.username
						.as_ref()
						.map_or_else(|| self.uuid.to_string(), std::clone::Clone::clone),
				));
			};

			player
		};

		#[cfg(feature = "redis")]
		self.set_display_str(ctx, &player).await?;

		#[cfg(feature = "database")]
		self.update_leaderboard(ctx, &player).await?;

		#[cfg(feature = "database")]
		if !ctx.is_automated() {
			self.update_activity(ctx).await?;
		}

		Ok(Arc::new(player))
	}

	#[must_use]
	pub fn get_head_url(&self) -> String {
		format!("https://visage.surgeplay.com/head/64/{}?y=72.5", self.uuid)
	}

	#[must_use]
	pub fn get_body_url(&self) -> String {
		format!("https://visage.surgeplay.com/full/{}?y=20", self.uuid)
	}

	/// # Panics
	/// Will not panic.
	pub async fn get_skin(&self) -> Cow<'static, image::Image<'static>> {
		let url = MOJANG_UUID_TO_USERNAME_API_ENDPOINT
			.join(&self.uuid.to_string())
			.unwrap();

		let response = HTTP
			.get(url)
			.timeout(Duration::from_millis(1_000))
			.send()
			.await;

		let resp = match response {
			Ok(response) if response.status() == StatusCode::OK => {
				match response.json::<SkinDataResponse>().await {
					Ok(response) => {
						let Some((url, is_slim)) = response.properties.get(0).map(|p| {
							(
								p.value.textures.skin.url.as_str(),
								p.value
									.textures
									.skin
									.metadata
									.as_ref()
									.is_some_and(|m| m.model == "slim"),
							)
						}) else {
							return Cow::Borrowed(&*DEFAULT_SKIN);
						};

						let skin = skin_renderer::render_skin(
							if is_slim {
								skin_renderer::SkinKind::Slim
							} else {
								skin_renderer::SkinKind::Classic
							},
							Some(url),
						)
						.await;

						if let Ok(skin) = skin {
							Cow::Owned(image::from_bytes_copy(skin.as_slice()).unwrap())
						} else {
							Cow::Borrowed(&*DEFAULT_SKIN)
						}
					}
					Err(_) => Cow::Borrowed(&*DEFAULT_SKIN),
				}
			}
			_ => Cow::Borrowed(&*DEFAULT_SKIN),
		};

		resp
	}

	/// # Errors
	/// Returns an error if the player does not have a profile or if their data is invalid.
	pub async fn get_session(&self) -> Result<Arc<status::Session>, Arc<Error>> {
		#[cfg(feature = "cache")]
		return PLAYER_SESSION_CACHE
			.try_get_with_by_ref(&self.uuid, self.get_session_raw())
			.await;

		#[cfg(not(feature = "cache"))]
		self.get_session_raw().await.map_err(Arc::new)
	}

	async fn get_session_raw(&self) -> Result<Arc<status::Session>, Error> {
		let url = {
			let mut url = HYPIXEL_STATUS_API_ENDPOINT.clone();

			url.set_query(Some(&format!("uuid={}", self.uuid)));
			url
		};

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_hypixel(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::SnapshotNotFound(
				self.username
					.as_ref()
					.map_or_else(|| self.uuid.to_string(), std::clone::Clone::clone),
			));
		}

		let response = response.json::<Status>().await?;

		Ok(Arc::new(response.session))
	}
}

#[cfg(test)]
mod tests {
	use std::assert_matches::assert_matches;

	use super::*;

	#[test]
	fn test_player() {
		let uuid = Uuid::new_v4();
		let player = Player::new(uuid, Some("Notch".to_string()));

		assert_eq!(player.uuid, uuid);
		assert_eq!(player.username, Some("Notch".to_string()));
	}

	#[tokio::test]
	async fn test_player_from_username() {
		let player = Player::from_username("Notch").await;

		assert_matches!(player, Ok(_));
		assert_eq!(
			player.unwrap().uuid,
			Uuid::parse_str("069a79f4-44e9-4726-a5be-fca90e38aaf5").unwrap(),
		);
	}

	#[tokio::test]
	async fn test_player_from_uuid() {
		let uuid = Uuid::parse_str("069a79f4-44e9-4726-a5be-fca90e38aaf5").unwrap();
		let player = Player::from_uuid(&uuid).await;

		assert_matches!(player, Ok(_));
		assert_eq!(player.unwrap().username, Some("Notch".to_string()));
	}
}
