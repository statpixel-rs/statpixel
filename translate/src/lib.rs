#[cfg(all(feature = "context", feature = "data"))]
pub mod context;

#[cfg(all(feature = "context", not(feature = "data")))]
pub mod context {
	use std::marker::PhantomData;

	pub struct Context<'c>(PhantomData<&'c ()>);

	impl Context<'_> {
		pub fn new() -> Self {
			Self(PhantomData)
		}
	}
}

#[cfg(all(feature = "locale", feature = "context", feature = "data"))]
pub mod prelude;

#[cfg(feature = "error")]
pub mod error;
#[cfg(feature = "error")]
pub use error::*;

#[cfg(all(feature = "locale", feature = "context", feature = "data"))]
pub mod locale;
#[cfg(all(feature = "locale", feature = "context", feature = "data"))]
pub use locale::*;

#[cfg(feature = "locale")]
pub use fluent;
#[cfg(feature = "error")]
pub use uuid;

#[cfg(all(feature = "data", feature = "locale"))]
mod data {
	use std::fmt;
	use std::sync::Arc;

	use database::PostgresPool;

	#[derive(Clone)]
	pub struct Data {
		pub pool: PostgresPool,
		pub redis: redis::aio::ConnectionManager,
		pub locale: Arc<super::locale::Locale>,
	}

	impl Data {
		pub fn redis(&self) -> redis::aio::ConnectionManager {
			self.redis.clone()
		}
	}

	impl fmt::Debug for Data {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Data").finish()
		}
	}

	#[cfg(feature = "error")]
	pub type Context<'a> = poise::Context<'a, Data, super::error::Error>;
}

#[cfg(feature = "data")]
pub use data::*;
