#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, Default, PartialEq)]
pub struct InverseBool(pub bool);

impl<'de> serde::Deserialize<'de> for InverseBool {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s: bool = serde::Deserialize::deserialize(deserializer)?;

		Ok(InverseBool(!s))
	}
}

impl From<InverseBool> for bool {
	fn from(b: InverseBool) -> Self {
		!b.0
	}
}

impl serde::Serialize for InverseBool {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		(!self.0).serialize(serializer)
	}
}

#[cfg(feature = "locale")]
mod locale {
	use super::*;

	use std::borrow::Cow;
	use translate::context::Context;

	impl label::ToFormatted for InverseBool {
		fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
			if self.0 {
				false.to_formatted(ctx)
			} else {
				true.to_formatted(ctx)
			}
		}
	}
}
