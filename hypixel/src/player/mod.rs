pub mod status;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Game {
	#[serde(rename(deserialize = "date"), with = "chrono::serde::ts_milliseconds")]
	pub started: DateTime<Utc>,
	#[serde(rename = "gameType")]
	pub kind: crate::game::r#type::Type,
	pub mode: crate::game::mode::Mode,
	pub map: Option<String>,
	#[serde(with = "chrono::serde::ts_milliseconds_option", default)]
	pub ended: Option<DateTime<Utc>>,
}
