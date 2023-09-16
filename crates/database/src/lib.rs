#![warn(clippy::pedantic)]

pub mod extend;
pub mod models;
pub mod schema;

use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};

pub type PostgresPool = Pool<AsyncPgConnection>;

/// # Panics
/// Panics if a connection cannot be established.
#[cfg(feature = "util")]
#[must_use]
pub fn get_pool(max_size: usize) -> PostgresPool {
	use diesel_async::pooled_connection::AsyncDieselConnectionManager;

	#[cfg(not(feature = "runtime_env"))]
	let url = dotenvy_macro::dotenv!("DATABASE_URL");

	#[cfg(feature = "runtime_env")]
	let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	#[cfg(feature = "runtime_env")]
	let url = url.as_str();

	let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

	Pool::builder(manager)
		.max_size(max_size)
		.build()
		.expect("failed to create connection pool")
}
