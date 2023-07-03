use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use super::member::Member;

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Profile {
	#[serde(rename = "profile_id")]
	pub id: Uuid,
	pub members: HashMap<Uuid, Member>,
	pub banking: Banking,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Banking {
	#[serde(deserialize_with = "crate::de::from::f64_to_u64")]
	pub balance: u64,
	pub transactions: Vec<Transaction>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Transaction {
	#[serde(deserialize_with = "crate::de::from::f64_to_u64")]
	pub amount: u64,
	#[serde(with = "chrono::serde::ts_milliseconds")]
	pub timestamp: DateTime<Utc>,
	pub action: TransactionAction,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(try_from = "&str")]
pub enum TransactionAction {
	#[default]
	Deposit,
	Withdraw,
}

impl TryFrom<&str> for TransactionAction {
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"DEPOSIT" => Ok(Self::Deposit),
			"WITHDRAW" => Ok(Self::Withdraw),
			_ => Err("invalid transaction action"),
		}
	}
}
