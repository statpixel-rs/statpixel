use std::borrow::Cow;

use label::ToFormatted;
use translate::context::Context;

macro_rules! impl_percent {
	($name: ident, $ty: ty) => {
		#[derive(bincode::Encode, bincode::Decode, Clone, Copy, Default, PartialEq, Eq)]
		pub struct $name(pub $ty);

		impl ToFormatted for $name {
			fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
				let percent = self.0;

				Cow::Owned(format!("{}%", percent.to_formatted(ctx)))
			}
		}

		impl std::cmp::PartialOrd for $name {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				Some(self.0.cmp(&other.0))
			}
		}

		impl std::cmp::Ord for $name {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				self.0.cmp(&other.0)
			}
		}

		impl std::ops::Add for $name {
			type Output = Self;

			fn add(self, rhs: Self) -> Self::Output {
				Self(self.0 + rhs.0)
			}
		}

		impl std::ops::Sub for $name {
			type Output = Self;

			fn sub(self, rhs: Self) -> Self::Output {
				Self(self.0 - rhs.0)
			}
		}

		impl From<$name> for u32 {
			fn from(percent: $name) -> Self {
				percent.0 as u32
			}
		}

		impl From<$name> for f64 {
			fn from(percent: $name) -> Self {
				percent.0 as f64
			}
		}
	};
}

impl_percent!(PercentU32, u32);
impl_percent!(PercentU64, u64);
impl_percent!(PercentI32, i32);
impl_percent!(PercentI64, i64);
