use std::ops::Add;

use serde::{Deserialize, Deserializer};

use crate::canvas::label::ToFormattedLabel;

#[derive(Debug, Clone, Copy, Default)]
pub struct Meters(u64);

impl<'de> Deserialize<'de> for Meters {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: u64 = Deserialize::deserialize(deserializer)?;

		Ok(Meters(s))
	}
}

impl serde::Serialize for Meters {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_u64(self.0)
	}
}

impl Add for Meters {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Meters(self.0 + rhs.0)
	}
}

impl From<Meters> for u64 {
	fn from(s: Meters) -> Self {
		s.0
	}
}

impl ToFormattedLabel for Meters {
	fn to_formatted_label(&self, locale: &num_format::Locale, _percent: bool) -> String {
		let m = self.0;

		match m {
			0..1_000 => format!("{}m", m.to_formatted_label(locale, false)),
			_ => format!(
				"{}km",
				(m as f32 / 1_000.).to_formatted_label(locale, false)
			),
		}
	}
}
