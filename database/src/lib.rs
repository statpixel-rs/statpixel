#![warn(clippy::pedantic)]

pub mod extend;
pub mod models;
pub mod schema;

use diesel::{
	r2d2::{ConnectionManager, Pool},
	PgConnection,
};

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

#[must_use]
pub fn get_pool() -> PostgresPool {
	let url = std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL not found");
	let manager = ConnectionManager::<PgConnection>::new(url);

	Pool::builder()
		.build(manager)
		.expect("failed to create connection pool")
}
