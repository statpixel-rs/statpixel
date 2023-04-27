use serde::Deserialize;

use crate::game::r#type::GameType;

#[derive(Deserialize, Debug)]
pub struct PlayerSession {
	pub online: bool,
	#[serde(rename = "gameType")]
	pub game_type: Option<GameType>,
	#[serde(rename = "mode")]
	pub game_mode: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PlayerStatus {
	pub session: PlayerSession,
}
