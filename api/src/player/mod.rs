mod data;

pub use data::*;

use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::{http::HTTP, Error};

const HYPIXEL_PLAYER_API_ENDPOINT: &str = "https://api.hypixel.net/player";
const MOJANG_USERNAME_TO_UUID_API_ENDPOINT: &str =
	"https://api.mojang.com/users/profiles/minecraft";
const MOJANG_UUID_TO_USERNAME_API_ENDPOINT: &str =
	"https://sessionserver.mojang.com/session/minecraft/profile";

#[derive(Deserialize)]
pub struct PlayerResponse {
	pub player: PlayerData,
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

#[derive(Deserialize, Debug)]
pub struct PlayerData {
	#[serde(rename = "displayname")]
	pub display_name: String,
	#[serde(default)]
	pub stats: Stats,
}

impl Player {
	pub fn new(uuid: Uuid, username: String) -> Self {
		Self { uuid, username }
	}

	pub async fn from_username(username: &str) -> Result<Player, Error> {
		let response = HTTP
			.get(format!("{MOJANG_USERNAME_TO_UUID_API_ENDPOINT}/{username}"))
			.send()
			.await?;

		if response.status() != StatusCode::OK {
			return Err(Error::NotFound);
		}

		let response = response.json::<MojangResponse>().await?;

		Ok(Self::new(response.id, response.name))
	}

	pub async fn from_uuid(uuid: &Uuid) -> Result<Player, Error> {
		let response = HTTP
			.get(format!("{MOJANG_UUID_TO_USERNAME_API_ENDPOINT}/{uuid}"))
			.send()
			.await?;

		if response.status() != StatusCode::OK {
			return Err(Error::NotFound);
		}

		let response = response.json::<MojangResponse>().await?;

		Ok(Self::new(response.id, response.name))
	}

	pub async fn get_data(&self) -> Result<PlayerData, Error> {
		let response = HTTP
			.get(HYPIXEL_PLAYER_API_ENDPOINT)
			.query(&[("uuid", self.uuid)])
			.send()
			.await?;

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
	async fn test_player_data() {
		let uuid = Uuid::parse_str("b876ec32-e396-476b-a115-8438d83c67d4").unwrap();
		let player = Player::from_uuid(&uuid).await;

		assert!(player.is_ok());

		let player = player.unwrap();

		assert_eq!("Technoblade".to_string(), player.username);
		assert!(player.get_data().await.is_ok());
	}
}
