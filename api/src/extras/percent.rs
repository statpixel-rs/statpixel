use std::borrow::Cow;

use macros::Diff;
use translate::context::Context;

use crate::canvas::label::ToFormatted;

macro_rules! impl_percent {
	($name: ident, $ty: ty) => {
		#[derive(
			bincode::Encode, bincode::Decode, Debug, Clone, Copy, Default, PartialEq, Eq, Diff,
		)]
		pub struct $name(pub $ty);

		impl ToFormatted for $name {
			fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
				let percent = self.0;

				Cow::Owned(format!("{}%", percent.to_formatted_label(ctx)))
			}
		}

		impl std::cmp::PartialOrd for $name {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				self.0.partial_cmp(&other.0)
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
	};
}

impl_percent!(PercentU32, u32);
impl_percent!(PercentU64, u64);
impl_percent!(PercentI32, i32);
impl_percent!(PercentI64, i64);
