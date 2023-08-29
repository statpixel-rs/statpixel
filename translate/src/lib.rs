pub mod context;
pub mod prelude;

#[cfg(feature = "error")]
pub mod error;
#[cfg(feature = "error")]
pub use error::*;

#[cfg(feature = "locale")]
pub mod locale;
#[cfg(feature = "locale")]
pub use locale::*;

pub use fluent;
pub use uuid;

#[cfg(feature = "data")]
mod data {
	use std::fmt;

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

	pub type Context<'a> = poise::Context<'a, Data, Error>;
}

#[cfg(feature = "data")]
pub use data::*;
