use std::{borrow::Cow, ops::Add};

use macros::Diff;
use serde::{Deserialize, Deserializer};
use translate::Context;

use crate::canvas::label::ToFormatted;

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, Default, PartialEq, Diff)]
pub struct Meters(pub u64);

impl<'de> Deserialize<'de> for Meters {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: u64 = Deserialize::deserialize(deserializer)?;

		Ok(Meters(s))
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

impl ToFormatted for Meters {
	#[allow(clippy::cast_precision_loss)]
	fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: Context<'c>) -> Cow<'t, str> {
		let m = self.0;

		if let 0..1_000 = m {
			return Cow::Owned(format!("{}m", m.to_formatted_label(ctx)));
		}

		Cow::Owned(format!("{}km", (m as f64 / 1_000.).to_formatted_label(ctx)))
	}
}
