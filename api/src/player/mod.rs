pub mod data;
pub mod stats;

use once_cell::sync::Lazy;
use reqwest::{StatusCode, Url};
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

use crate::{http::HTTP, Error};

static HYPIXEL_PLAYER_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/player").unwrap());

static MOJANG_USERNAME_TO_UUID_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.mojang.com/users/profiles/minecraft/").unwrap());

static MOJANG_UUID_TO_USERNAME_API_ENDPOINT: Lazy<Url> = Lazy::new(|| {
	Url::from_str("https://sessionserver.mojang.com/session/minecraft/profile/").unwrap()
});

#[derive(Deserialize)]
pub struct PlayerResponse {
	pub player: data::PlayerData,
	pub success: bool,
}

#[derive(Deserialize)]
pub struct MojangResponse {
	pub name: String,
	pub id: Uuid,
}

pub struct Player {
	pub uuid: Uuid,
	pub username: String,
}

impl Player {
	pub fn new(uuid: Uuid, username: String) -> Self {
		Self { uuid, username }
	}

	pub async fn from_username(username: &str) -> Result<Player, Error> {
		let url = MOJANG_USERNAME_TO_UUID_API_ENDPOINT.join(username).unwrap();
		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::NotFound);
		}

		let response = response.json::<MojangResponse>().await?;

		Ok(Self::new(response.id, response.name))
	}

	pub async fn from_uuid(uuid: &Uuid) -> Result<Player, Error> {
		let url = MOJANG_UUID_TO_USERNAME_API_ENDPOINT
			.join(&uuid.to_string())
			.unwrap();

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::NotFound);
		}

		let response = response.json::<MojangResponse>().await?;

		Ok(Self::new(response.id, response.name))
	}

	pub async fn get_data(&self) -> Result<data::PlayerData, Error> {
		let mut url = HYPIXEL_PLAYER_API_ENDPOINT.clone();

		url.set_query(Some(&format!("uuid={}", self.uuid)));

		let response = HTTP.get(url).send().await?;

		if response.status() != StatusCode::OK {
			return Err(Error::NotFound);
		}

		let response = response.json::<PlayerResponse>().await?;

		Ok(response.player)
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
