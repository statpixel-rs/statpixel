mod image;
mod topgg;

use axum::{
	routing::{get, post},
	Router,
};
use std::{net::SocketAddr, sync::Arc};
use topgg::add_vote;

pub type Data = translate::Data;

pub async fn run(data: Data) {
	let app = Router::new()
		.route("/internal/vote", post(add_vote))
		.route("/image/:id", get(image::get))
		.with_state(Arc::new(data));

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
