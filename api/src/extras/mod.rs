pub mod meters;

macro_rules! impl_time_unit {
	($name: ident, $op: tt, $val: expr) => {
		#[derive(Debug, Clone, Copy, Default)]
		pub struct $name(u64);

		impl<'de> ::serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
			where
				D: ::serde::Deserializer<'de>,
			{
				let s: u64 = ::serde::Deserialize::deserialize(deserializer)?;

				Ok($name(s))
			}
		}

		impl ::serde::Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
			where
				S: ::serde::Serializer,
			{
				serializer.serialize_u64(self.0)
			}
		}

		impl ::std::ops::Add for $name {
			type Output = Self;

			fn add(self, rhs: Self) -> Self::Output {
				$name(self.0 + rhs.0)
			}
		}

		impl ::std::convert::From<$name> for u64 {
			fn from(s: $name) -> Self {
				s.0
			}
		}

		impl $crate::canvas::label::ToFormattedLabel for $name {
			fn to_formatted_label(
				&self,
				_ctx: ::translate::Context<'_>,
				_percent: bool,
			) -> ::std::string::String {
				let mut result = ::std::string::String::new();
				let s = self.0 $op $val;

				let days = s / 86_400;
				if days > 0 {
					result.push_str(&format!("{}d ", days));
				}

				let hours = (s % 86_400) / 3_600;
				if hours > 0 {
					result.push_str(&format!("{}h ", hours));
				}

				let minutes = (s % 3_600) / 60;
				if minutes > 0 && days == 0 {
					result.push_str(&format!("{}m ", minutes));
				}

				let seconds = s % 60;
				if (seconds > 0 && days == 0 && hours == 0) || result.is_empty() {
					result.push_str(&format!("{}s", seconds));
				}

				result
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
