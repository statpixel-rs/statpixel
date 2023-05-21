use std::borrow::Cow;

use macros::Diff;
use translate::Context;

use crate::canvas::label::ToFormatted;

macro_rules! impl_percent {
	($name: ident, $ty: ty) => {
		#[derive(
			bincode::Encode, bincode::Decode, Debug, Clone, Copy, Default, PartialEq, Diff,
		)]
		pub struct $name(pub $ty);

		impl ToFormatted for $name {
			fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: Context<'c>) -> Cow<'t, str> {
				let percent = self.0;

				Cow::Owned(format!("{}%", percent.to_formatted_label(ctx)))
			}
		}
	};
}

impl_percent!(PercentU32, u32);
impl_percent!(PercentU64, u64);
impl_percent!(PercentI32, i32);
impl_percent!(PercentI64, i64);
