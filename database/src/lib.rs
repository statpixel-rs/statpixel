#![warn(clippy::pedantic)]

pub mod extend;
pub mod models;
pub mod schema;

use diesel_async::{
	pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
	AsyncPgConnection,
};

pub type PostgresPool = Pool<AsyncPgConnection>;

#[must_use]
pub fn get_pool(max_size: usize) -> PostgresPool {
	let url = std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL not found");
	let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

	Pool::builder(manager)
		.max_size(max_size)
		.build()
		.expect("failed to create connection pool")
}
