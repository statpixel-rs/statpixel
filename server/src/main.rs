mod topgg;

use axum::{routing::post, Router};
use database::{get_pool, PostgresPool};
use std::{net::SocketAddr, sync::Arc};
use topgg::add_vote;

pub struct AppState {
	pub pool: PostgresPool,
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	dotenvy::dotenv().ok();

	let app = Router::new()
		.route("/internal/vote", post(add_vote))
		.with_state(Arc::new(AppState { pool: get_pool(2) }));

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
