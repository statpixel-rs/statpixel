use std::ops::Add;

use serde::{Deserialize, Deserializer};

use crate::canvas::label::ToFormattedLabel;

#[derive(Debug, Clone, Copy, Default)]
pub struct Minutes(u64);

impl<'de> Deserialize<'de> for Minutes {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: u64 = Deserialize::deserialize(deserializer)?;

		Ok(Minutes(s))
	}
}

impl serde::Serialize for Minutes {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_u64(self.0)
	}
}

impl Add for Minutes {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Minutes(self.0 + rhs.0)
	}
}

impl From<Minutes> for u64 {
	fn from(s: Minutes) -> Self {
		s.0
	}
}

impl ToFormattedLabel for Minutes {
	fn to_formatted_label(&self, _locale: &num_format::Locale, _percent: bool) -> String {
		// format like 5d 6h 32m 19s, omitting 0 values
		let mut result = String::new();
		let s = self.0 * 60;

		let days = s / 86_400;
		if days > 0 {
			result.push_str(&format!("{}d ", days));
		}

		let hours = (s % 86_400) / 3_600;
		if hours > 0 {
			result.push_str(&format!("{}h ", hours));
		}

		let minutes = (s % 3_600) / 60;
		if minutes > 0 {
			result.push_str(&format!("{}m ", minutes));
		}

		let seconds = s % 60;
		if seconds > 0 || result.is_empty() {
			result.push_str(&format!("{}s", seconds));
		}

		result
	}
}
