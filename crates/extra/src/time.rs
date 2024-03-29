macro_rules! impl_time_unit {
	($name: ident, $op: tt, $val: expr) => {
		#[derive(bincode::Decode, bincode::Encode, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
		pub struct $name(pub i64);

		impl<'de> ::serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
			where
				D: ::serde::Deserializer<'de>,
			{
				let s: i64 = ::serde::Deserialize::deserialize(deserializer)?;

				Ok($name(s))
			}
		}

		impl ::serde::Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
			where
				S: ::serde::Serializer,
			{
				serializer.serialize_i64(self.0)
			}
		}

		impl ::std::ops::Add for $name {
			type Output = Self;

			fn add(self, rhs: Self) -> Self::Output {
				$name(self.0 + rhs.0)
			}
		}

		impl ::std::ops::Sub for $name {
			type Output = Self;

			fn sub(self, rhs: Self) -> Self::Output {
				$name(self.0 - rhs.0)
			}
		}

		impl ::std::convert::From<$name> for i64 {
			fn from(s: $name) -> Self {
				s.0
			}
		}

		impl ::std::convert::From<$name> for u32 {
			fn from(s: $name) -> Self {
				s.0 as u32
			}
		}

		impl ::std::convert::From<$name> for f64 {
			fn from(s: $name) -> Self {
				s.0 as f64
			}
		}

		#[cfg(feature = "locale")]
		impl label::ToFormatted for $name {
			fn to_formatted<'t, 'c: 't>(
				&'t self,
				_ctx: &::translate::context::Context<'c>,
			) -> ::std::borrow::Cow<'t, str> {
				let mut result = ::std::string::String::with_capacity(3);
				let (ms, neg) = {
					let ms = self.0 $op $val;

					if ms < 0 {
						(-ms, true)
					} else {
						(ms, false)
					}
				};

				let days = ms / 86_400_000;
				if days > 0 {
					if neg {
						result.push('-');
					}

					result.push_str(&format!("{}d ", days));
				}

				let hours = (ms % 86_400_000) / 3_600_000;
				if hours > 0 {
					if neg && days == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}h ", hours));
				}

				let minutes = (ms % 3_600_000) / 60_000;
				if minutes > 0 && days == 0 {
					if neg && days == 0 && hours == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}m ", minutes));
				}

				let seconds = ms % 60_000 / 1_000;
				if seconds > 0 && days == 0 && hours == 0 {
					if seconds > 0 && neg && days == 0 && hours == 0 && minutes == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}s ", seconds));
				}

				let milliseconds = ms % 1_000;
				if (milliseconds > 0 && days == 0 && hours == 0 && minutes == 0) || result.is_empty() {
					if milliseconds > 0
						&& neg && days == 0 && hours == 0
						&& minutes == 0 && seconds == 0
					{
						result.push('-');
					}

					result.push_str(&format!("{}ms", milliseconds));
				}

				::std::borrow::Cow::Owned(result)
			}
		}
	};
}

macro_rules! impl_time_unit_opt {
	($name: ident, $op: tt, $val: expr) => {
		#[derive(
			bincode::Decode,
			bincode::Encode,
			Clone,
			Copy,
			Default,
			PartialEq,
			Eq,
			Debug,
		)]
		pub struct $name(pub Option<i64>);

		impl<'de> ::serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
			where
				D: ::serde::Deserializer<'de>,
			{
				let s: Option<i64> = ::serde::Deserialize::deserialize(deserializer)?;

				Ok($name(s))
			}
		}

		impl ::serde::Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
			where
				S: ::serde::Serializer,
			{
				serializer.serialize_i64(self.0.unwrap_or_default())
			}
		}

		impl ::std::ops::Add for $name {
			type Output = Self;

			fn add(self, rhs: Self) -> Self::Output {
				$name(Some((self.0.unwrap_or_default() + rhs.0.unwrap_or_default()).abs()))
			}
		}

		impl ::std::ops::Sub for $name {
			type Output = Self;

			fn sub(self, rhs: Self) -> Self::Output {
				$name(Some((self.0.unwrap_or_default() - rhs.0.unwrap_or_default()).abs()))
			}
		}

		impl ::std::convert::From<$name> for i64 {
			fn from(s: $name) -> Self {
				s.0.unwrap_or_default()
			}
		}

		impl ::std::convert::From<$name> for u32 {
			fn from(s: $name) -> Self {
				s.0.unwrap_or_default() as u32
			}
		}

		impl ::std::convert::From<$name> for f64 {
			fn from(s: $name) -> Self {
				s.0.unwrap_or_default() as f64
			}
		}

		impl PartialOrd for $name {
			fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}

		impl Ord for $name {
			fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
				match (self.0, other.0) {
					(None, None) => ::std::cmp::Ordering::Equal,
					(None, Some(_)) => ::std::cmp::Ordering::Less,
					(Some(_), None) => ::std::cmp::Ordering::Greater,
					(Some(a), Some(b)) => a.cmp(&b),
				}
			}
		}

		#[cfg(feature = "locale")]
		impl label::ToFormatted for $name {
			fn to_formatted<'t, 'c: 't>(
				&'t self,
				ctx: &'c ::translate::context::Context<'c>,
			) -> ::std::borrow::Cow<'t, str> {
				let mut result = ::std::string::String::with_capacity(3);
				let Some(value) = self.0 else {
					return ::translate::tr(ctx, "none");
				};

				let (ms, neg) = {
					let ms = value $op $val;

					if ms < 0 {
						(-ms, true)
					} else {
						(ms, false)
					}
				};

				let days = ms / 86_400_000;
				if days > 0 {
					if neg {
						result.push('-');
					}

					result.push_str(&format!("{}d ", days));
				}

				let hours = (ms % 86_400_000) / 3_600_000;
				if hours > 0 {
					if neg && days == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}h ", hours));
				}

				let minutes = (ms % 3_600_000) / 60_000;
				if minutes > 0 && days == 0 {
					if neg && days == 0 && hours == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}m ", minutes));
				}

				let seconds = ms % 60_000 / 1_000;
				if seconds > 0 && days == 0 && hours == 0 {
					if seconds > 0 && neg && days == 0 && hours == 0 && minutes == 0 {
						result.push('-');
					}

					result.push_str(&format!("{}s ", seconds));
				}

				let milliseconds = ms % 1_000;
				if (milliseconds > 0 && days == 0 && hours == 0 && minutes == 0) || result.is_empty() {
					if milliseconds > 0
						&& neg && days == 0 && hours == 0
						&& minutes == 0 && seconds == 0
					{
						result.push('-');
					}

					result.push_str(&format!("{}ms", milliseconds));
				}

				::std::borrow::Cow::Owned(result)
			}
		}
	};
}

pub mod milliseconds {
	impl_time_unit!(Milliseconds, *, 1);
	impl_time_unit_opt!(MillisecondsOption, *, 1);
}

pub mod seconds {
	impl_time_unit!(Seconds, *, 1_000);
	impl_time_unit_opt!(SecondsOption, *, 1_000);
}

pub mod minutes {
	impl_time_unit!(Minutes, *, 60 * 1_000);
	impl_time_unit_opt!(MinutesOption, *, 60 * 1_000);
}
