pub mod inverse_bool;
pub mod meters;

macro_rules! impl_time_unit {
	($name: ident, $op: tt, $val: expr) => {
		#[derive(bincode::Decode, bincode::Encode, Debug, Clone, Copy, Default, PartialEq, ::macros::Diff)]
		pub struct $name(i64);

		impl<'de> ::serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
			where
				D: ::serde::Deserializer<'de>,
			{
				let s: i64 = ::serde::Deserialize::deserialize(deserializer)?;

				Ok($name(s))
			}
		}

		impl ::std::ops::Add for $name {
			type Output = Self;

			fn add(self, rhs: Self) -> Self::Output {
				$name(self.0 + rhs.0)
			}
		}

		impl ::std::convert::From<$name> for i64 {
			fn from(s: $name) -> Self {
				s.0
			}
		}

		impl $crate::canvas::label::ToFormatted for $name {
			fn to_formatted_label<'t, 'c: 't>(
				&'t self,
				_ctx: ::translate::Context<'c>,
				_percent: bool,
			) -> ::std::borrow::Cow<'t, str> {
				let mut result = ::std::string::String::with_capacity(3);
				let (s, neg) = {
					let s = self.0 $op $val;

					if s < 0 {
						(-s, true)
					} else {
						(s, false)
					}
				};

				let days = s / 86_400;
				if days > 0 {
					if neg {
						result.push('-');
					}

					result.push_str(&format!("{}d ", days));
				}

				let hours = (s % 86_400) / 3_600;
				if hours > 0 {
					if neg && days == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}h ", hours));
				}

				let minutes = (s % 3_600) / 60;
				if minutes > 0 && days == 0 {
					if neg && days == 0 && hours == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}m ", minutes));
				}

				let seconds = s % 60;
				if (seconds > 0 && days == 0 && hours == 0) || result.is_empty() {
					if seconds > 0 && neg && days == 0 && hours == 0 && minutes == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}s", seconds));
				}

				::std::borrow::Cow::Owned(result)
			}
		}
	};
}

pub mod milliseconds {
	impl_time_unit!(Milliseconds, /, 1_000);
}

pub mod minutes {
	impl_time_unit!(Minutes, *, 60);
}

pub mod seconds {
	impl_time_unit!(Seconds, *, 1);
}
