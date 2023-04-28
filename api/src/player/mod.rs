pub mod data;
pub mod stats;
pub mod status;

use once_cell::sync::Lazy;
use reqwest::{StatusCode, Url};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::{
	cache::{PLAYER_CACHE, PLAYER_DATA_CACHE, PLAYER_SESSION_CACHE},
	http::HTTP,
	ratelimit::{HYPIXEL_RATELIMIT, MOJANG_RATELIMIT},
	Error,
};

use self::status::PlayerStatus;

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
pub struct PlayerResponse {
	pub player: data::PlayerData,
	pub success: bool,
}

#[derive(Deserialize, Clone)]
pub struct MojangResponse {
	pub name: String,
	pub id: Uuid,
}

#[derive(Clone)]
pub struct Player {
	pub uuid: Uuid,
	pub username: String,
}

impl Player {
	pub fn new(uuid: Uuid, username: String) -> Self {
		Self { uuid, username }
	}

	pub async fn from_username(username: &str) -> Result<Player, Arc<Error>> {
		PLAYER_CACHE
			.try_get_with(
				username.to_ascii_lowercase(),
				Self::from_username_raw(username),
			)
			.await
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

	pub async fn get_data(&self) -> Result<data::PlayerData, Arc<Error>> {
		PLAYER_DATA_CACHE
			.try_get_with_by_ref(&self.uuid, self.get_data_raw())
			.await
	}

	async fn get_data_raw(&self) -> Result<data::PlayerData, Error> {
		let url = {
			let mut url = HYPIXEL_PLAYER_API_ENDPOINT.clone();

			url.set_query(Some(&format!("uuid={}", self.uuid)));
			url
		};

		HYPIXEL_RATELIMIT.until_ready().await;

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::PlayerNotFound(self.username.clone()));
		}

		let response = response.json::<PlayerResponse>().await?;

		Ok(response.player)
	}

	pub async fn get_session(&self) -> Result<status::PlayerSession, Arc<Error>> {
		PLAYER_SESSION_CACHE
			.try_get_with_by_ref(&self.uuid, self.get_session_raw())
			.await
	}

	async fn get_session_raw(&self) -> Result<status::PlayerSession, Error> {
		let url = {
			let mut url = HYPIXEL_STATUS_API_ENDPOINT.clone();

			url.set_query(Some(&format!("uuid={}", self.uuid)));
			url
		};

		HYPIXEL_RATELIMIT.until_ready().await;

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::SessionNotFound(self.username.clone()));
		}

		let response = response.json::<PlayerStatus>().await?;

		Ok(response.session)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_player() {
		let uuid = Uuid::new_v4();
		let player = Player::new(uuid, "Notch".to_string());

		assert_eq!(player.uuid, uuid);
	}

	#[tokio::test]
	async fn test_player_from_username() {
		let player = Player::from_username("Notch").await;

		assert!(player.is_ok());
		assert_eq!(
			Uuid::parse_str("069a79f4-44e9-4726-a5be-fca90e38aaf5").unwrap(),
			player.unwrap().uuid
		);
	}

	#[tokio::test]
	async fn test_player_from_uuid() {
		let uuid = Uuid::parse_str("069a79f4-44e9-4726-a5be-fca90e38aaf5").unwrap();
		let player = Player::from_uuid(&uuid).await;

		assert!(player.is_ok());
		assert_eq!("Notch".to_string(), player.unwrap().username);
	}

	#[tokio::test]
	async fn test_player_data() {
		let uuid = Uuid::parse_str("b876ec32-e396-476b-a115-8438d83c67d4").unwrap();
		let player = Player::new(uuid, "Technoblade".to_string());

		assert!(player.get_data().await.is_ok());
	}
}
