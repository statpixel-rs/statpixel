use std::{
	borrow::Cow,
	ops::{Add, Sub},
};

use serde::Deserializer;
use translate::context::Context;

use crate::canvas::label::ToFormatted;

#[derive(
	bincode::Encode, bincode::Decode, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Meters(pub u64);

impl<'de> serde::Deserialize<'de> for Meters {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: u64 = serde::Deserialize::deserialize(deserializer)?;

		Ok(Meters(s))
	}
}

impl serde::Serialize for Meters {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}

impl Add for Meters {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Meters(self.0 + rhs.0)
	}
}

impl Sub for Meters {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Meters(self.0 - rhs.0)
	}
}

impl From<Meters> for u64 {
	fn from(s: Meters) -> Self {
		s.0
	}
}

#[allow(clippy::cast_possible_truncation)]
impl From<Meters> for u32 {
	fn from(s: Meters) -> Self {
		s.0 as u32
	}
}

#[allow(clippy::cast_precision_loss)]
impl From<Meters> for f64 {
	fn from(s: Meters) -> Self {
		s.0 as f64
	}
}

impl ToFormatted for Meters {
	#[allow(clippy::cast_precision_loss)]
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
		let m = self.0;

		if let 0..1_000 = m {
			return Cow::Owned(format!("{}m", m.to_formatted(ctx)));
		}

		Cow::Owned(format!("{}km", (m as f64 / 1_000.).to_formatted(ctx)))
	}
}
