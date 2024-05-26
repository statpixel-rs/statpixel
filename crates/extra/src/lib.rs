pub mod inverse_bool;
pub mod meters;
pub mod percent;
pub mod time;
pub mod xp;

pub use inverse_bool::*;
pub use meters::*;
pub use percent::*;
pub use time::*;
pub use xp::*;

#[cfg(feature = "redis")]
mod _redis {
	use super::*;

	macro_rules! redis_impl {
		($($ty:ty => $o:ty),*) => {
			$(
				impl redis::ToRedisArgs for $ty {
					fn write_redis_args<W>(&self, out: &mut W)
						where
							W: ?Sized + redis::RedisWrite
					{
						self.0.write_redis_args(out)
					}
				}

				impl redis::FromRedisValue for $ty {
					fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
						Ok(Self(<$o>::from_redis_value(v)?))
					}
				}
			)*
		};
	}

	macro_rules! redis_impl_opt {
		($($ty:ty => $o:ty),*) => {
			$(
				impl redis::ToRedisArgs for $ty {
					fn write_redis_args<W>(&self, out: &mut W)
						where
							W: ?Sized + redis::RedisWrite
					{
						(self.0.unwrap_or_default() as f64).write_redis_args(out)
					}
				}

				impl redis::FromRedisValue for $ty {
					fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
						Ok(Self(Some(f64::from_redis_value(v)? as $o)))
					}
				}
			)*
		};
	}

	redis_impl! {
		Meters => u64,
		PercentU32 => u32,
		PercentU64 => u64,
		PercentI32 => i32,
		PercentI64 => i64,
		milliseconds::Milliseconds => i64,
		seconds::Seconds => i64,
		minutes::Minutes => i64,
		Xp => u32
	}

	redis_impl_opt! {
		milliseconds::MillisecondsOption => i64,
		seconds::SecondsOption => i64,
		minutes::MinutesOption => i64
	}
}
