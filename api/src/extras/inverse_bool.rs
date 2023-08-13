use std::borrow::Cow;

use serde::{Deserialize, Deserializer};
use translate::context::Context;

use crate::canvas::{diff::Diff, label::ToFormatted};

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, Default, PartialEq)]
pub struct InverseBool(pub bool);

impl<'de> Deserialize<'de> for InverseBool {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: bool = Deserialize::deserialize(deserializer)?;

		Ok(InverseBool(!s))
	}
}

impl ToFormatted for InverseBool {
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
		if self.0 {
			false.to_formatted(ctx)
		} else {
			true.to_formatted(ctx)
		}
	}
}

impl Diff for InverseBool {
	fn diff(&self, other: &Self) -> Self {
		Self(self.0.diff(&other.0))
	}
}
