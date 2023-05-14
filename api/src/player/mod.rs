pub mod data;
pub mod stats;
pub mod status;

use database::schema::autocomplete;
use diesel::{ExpressionMethods, RunQueryDsl};
use once_cell::sync::Lazy;
use reqwest::{StatusCode, Url};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};
use translate::Context;
use uuid::Uuid;

use crate::{
	cache::{PLAYER_CACHE, PLAYER_DATA_CACHE, PLAYER_SESSION_CACHE},
	http::HTTP,
	ratelimit::{HYPIXEL_RATELIMIT, MOJANG_RATELIMIT},
	Error,
};

use self::status::Status;

static HYPIXEL_PLAYER_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/player").unwrap());

static HYPIXEL_STATUS_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/status").unwrap());

static MOJANG_USERNAME_TO_UUID_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.mojang.com/users/profiles/minecraft/").unwrap());

static MOJANG_UUID_TO_USERNAME_API_ENDPOINT: Lazy<Url> = Lazy::new(|| {
	Url::from_str("https://sessionserver.mojang.com/session/minecraft/profile/").unwrap()
});

#[derive(Deserialize, Debug)]
pub struct Response {
	pub player: data::Data,
	pub success: bool,
}

#[derive(Deserialize, Clone)]
pub struct MojangResponse {
	pub name: String,
	pub id: Uuid,
}

#[derive(Clone, Debug)]
pub struct Player {
	pub uuid: Uuid,
	pub username: String,
}

impl Player {
	#[must_use]
	pub fn new(uuid: Uuid, username: String) -> Self {
		Self { uuid, username }
	}

	/// # Errors
	/// Returns an error if there is an issue with the database.
	pub fn increase_searches(&self, ctx: Context<'_>) -> Result<(), translate::Error> {
		diesel::insert_into(autocomplete::table)
			.values((
				autocomplete::name.eq(&self.username),
				autocomplete::uuid.eq(&self.uuid),
				autocomplete::searches.eq(1),
			))
			.on_conflict(autocomplete::uuid)
			.do_update()
			.set((
				autocomplete::name.eq(&self.username),
				autocomplete::searches.eq(autocomplete::searches + 1),
			))
			.execute(&mut ctx.data().pool.get()?)?;

		Ok(())
	}

	/// # Errors
	/// Returns an error if the username does not exist or if their data is invalid.
	pub async fn from_username(username: &str) -> Result<Player, Arc<Error>> {
		let player = PLAYER_CACHE
			.try_get_with(
				username.to_ascii_lowercase(),
				Self::from_username_raw(username),
			)
			.await?;

		Ok(player)
	}

	async fn from_username_raw(username: &str) -> Result<Player, Error> {
		let url = MOJANG_USERNAME_TO_UUID_API_ENDPOINT.join(username).unwrap();

		MOJANG_RATELIMIT.until_ready().await;

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::UsernameNotFound(username.to_string()));
		}

		let response = response.json::<MojangResponse>().await?;
		let player = Self::new(response.id, response.name);

		// Also add the player with their uuid to the cache
		PLAYER_CACHE
			.insert(response.id.to_string(), player.clone())
			.await;

		Ok(player)
	}

	/// # Errors
	/// Returns an error if the uuid does not exist or if their data is invalid.
	pub async fn from_uuid(uuid: &Uuid) -> Result<Player, Arc<Error>> {
		PLAYER_CACHE
			.try_get_with(uuid.to_string(), Self::from_uuid_raw(uuid))
			.await
	}

	async fn from_uuid_raw(uuid: &Uuid) -> Result<Player, Error> {
		let url = MOJANG_UUID_TO_USERNAME_API_ENDPOINT
			.join(&uuid.to_string())
			.unwrap();

		MOJANG_RATELIMIT.until_ready().await;

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::UuidNotFound(*uuid));
		}

		let response = response.json::<MojangResponse>().await?;
		let lower = response.name.to_ascii_lowercase();
		let player = Self::new(response.id, response.name);

		// Also add the player to the cache with the lower-case username
		PLAYER_CACHE.insert(lower, player.clone()).await;

		Ok(player)
	}

	/// # Errors
	/// Returns an error if the player does not have a profile or if their data is invalid.
	pub async fn get_data(&self) -> Result<data::Data, Arc<Error>> {
		PLAYER_DATA_CACHE
			.try_get_with_by_ref(&self.uuid, self.get_data_raw())
			.await
	}

	async fn get_data_raw(&self) -> Result<data::Data, Error> {
		let url = {
			let mut url = HYPIXEL_PLAYER_API_ENDPOINT.clone();

			url.set_query(Some(&format!("uuid={}", self.uuid)));
			url
		};

		// HYPIXEL_RATELIMIT will always be present, as it is initialized in the main function
		unsafe {
			HYPIXEL_RATELIMIT.get_unchecked().until_ready().await;
		}

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::PlayerNotFound(self.username.clone()));
		}

		let response = response.json::<Response>().await?;

		Ok(response.player)
	}

	/// # Errors
	/// Returns an error if the player does not have a profile or if their data is invalid.
	pub async fn get_session(&self) -> Result<status::Session, Arc<Error>> {
		PLAYER_SESSION_CACHE
			.try_get_with_by_ref(&self.uuid, self.get_session_raw())
			.await
	}

	async fn get_session_raw(&self) -> Result<status::Session, Error> {
		let url = {
			let mut url = HYPIXEL_STATUS_API_ENDPOINT.clone();

			url.set_query(Some(&format!("uuid={}", self.uuid)));
			url
		};

		// HYPIXEL_RATELIMIT will always be present, as it is initialized in the main function
		unsafe {
			HYPIXEL_RATELIMIT.get_unchecked().until_ready().await;
		}

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::SessionNotFound(self.username.clone()));
		}

		let response = response.json::<Status>().await?;

		Ok(response.session)
	}
}

#[cfg(test)]
mod tests {
	use std::{assert_matches::assert_matches, num::NonZeroU32};

	use governor::{Quota, RateLimiter};

	use super::*;
	use crate::key;

	async fn set_up_key() {
		if HYPIXEL_RATELIMIT.get().is_some() {
			return;
		}

		let (key, _) = key::get_data().await.unwrap();

		HYPIXEL_RATELIMIT
			.set(RateLimiter::direct(Quota::per_minute(
				NonZeroU32::new(key.limit).unwrap(),
			)))
			.ok();
	}

	#[test]
	fn test_player() {
		let uuid = Uuid::new_v4();
		let player = Player::new(uuid, "Notch".to_string());

		assert_eq!(player.uuid, uuid);
		assert_eq!(player.username, "Notch".to_string());
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
		assert_eq!(player.unwrap().username, "Notch".to_string());
	}

	#[tokio::test]
	async fn test_player_data() {
		set_up_key().await;

		let uuid = Uuid::parse_str("b876ec32-e396-476b-a115-8438d83c67d4").unwrap();
		let player = Player::new(uuid, "Technoblade".to_string());

		assert_matches!(player.get_data().await, Ok(_));
	}
}
