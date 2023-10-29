use serde::Deserialize;

use crate::game::r#type::Type;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Session {
	pub online: bool,
	#[serde(rename = "gameType")]
	pub game_type: Option<Type>,
	#[serde(rename = "mode")]
	pub game_mode: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Status {
	pub session: Session,
}
