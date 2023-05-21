use std::{borrow::Cow, ops::SubAssign};

use macros::Diff;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use translate::Context;

use crate::canvas::label::ToFormatted;

#[derive(
	bincode::Encode,
	bincode::Decode,
	Debug,
	Clone,
	Copy,
	Default,
	Diff,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub struct Xp(pub u32);

impl<'de> Deserialize<'de> for Xp {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: u32 = Deserialize::deserialize(deserializer)?;

		Ok(Xp(s))
	}
}

impl Serialize for Xp {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_u32(self.0)
	}
}

impl From<Xp> for u32 {
	fn from(s: Xp) -> Self {
		s.0
	}
}

impl ToFormatted for Xp {
	#[allow(clippy::cast_precision_loss)]
	fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: Context<'c>) -> Cow<'t, str> {
		let xp = self.0;

		Cow::Owned(format!("{} XP", xp.to_formatted_label(ctx)))
	}
}

impl SubAssign for Xp {
	fn sub_assign(&mut self, rhs: Self) {
		self.0 -= rhs.0;
	}
}
