use std::sync::Arc;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::{player::Player, Error};

#[derive(Deserialize, Debug, Clone)]
pub struct Member {
	pub uuid: Uuid,
	pub rank: String,
	#[serde(rename = "joined", with = "chrono::serde::ts_milliseconds")]
	pub joined_at: DateTime<Utc>,
	#[serde(rename = "questParticipation", default)]
	pub quests: u32,
	#[serde(rename = "expHistory", deserialize_with = "from_date_map")]
	pub xp_history: [(NaiveDate, u32); 7],
}

impl Member {
	/// # Errors
	/// Returns an error if the player does not exist.
	pub async fn get_player(&self) -> Result<Player, Arc<Error>> {
		Player::from_uuid(&self.uuid).await
	}

	#[must_use]
	pub fn get_player_unchecked(&self) -> Player {
		Player::from_uuid_unchecked(self.uuid)
	}

	#[must_use]
	pub fn is_leader(&self) -> bool {
		self.rank == "Guild Master" || self.rank == "GUILDMASTER"
	}
}

fn from_date_map<'de, D>(deserializer: D) -> Result<[(NaiveDate, u32); 7], D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor([(NaiveDate, u32); 7]);

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = [(NaiveDate, u32); 7];

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a mapping of dates to numbers")
		}

		fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			let mut i = 0;

			while let Some((date, xp)) = map.next_entry()? {
				self.0[i] = (NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(), xp);

				i += 1;
			}

			Ok(self.0)
		}
	}

	deserializer.deserialize_map(Visitor([(NaiveDate::MIN, 0); 7]))
}
