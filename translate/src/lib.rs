#[cfg(feature = "context")]
pub mod context;
#[cfg(feature = "context")]
pub mod prelude;

#[cfg(feature = "error")]
pub mod error;
#[cfg(feature = "error")]
pub use error::*;

#[cfg(feature = "locale")]
pub mod locale;
#[cfg(feature = "locale")]
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
	use thiserror::Error;

	#[derive(Clone)]
	pub struct Data {
		pub pool: PostgresPool,
		pub locale: Arc<locale::Locale>,
	}

	impl fmt::Debug for Data {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Data").finish()
		}
	}

	#[cfg(feature = "error")]
	pub type Context<'a> = poise::Context<'a, Data, Error>;
}

#[cfg(feature = "data")]
pub use data::*;
