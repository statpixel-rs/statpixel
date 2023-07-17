mod auth;
mod builder;
mod debug;
mod error;
mod extract;
mod image;
mod metrics;
mod topgg;

use axum::{
	http::header::{AUTHORIZATION, CONTENT_TYPE},
	routing::{get, post},
	Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
	compression::CompressionLayer,
	cors::{Any, CorsLayer},
};

pub type Data = translate::Data;

pub async fn run(data: Data) {
	let app = Router::new()
		.route("/internal/vote", post(topgg::add_vote))
		.route("/internal/debug", get(debug::get))
		.route("/image/:id", get(image::get))
		.route("/metrics", get(metrics::get))
		.route("/auth/login", get(auth::login))
		.route("/auth/me", get(auth::me).patch(auth::update_me))
		.route("/builder/preview", post(builder::preview))
		.route("/builder/test", post(builder::get))
		.layer(ServiceBuilder::new().layer(CompressionLayer::new()))
		.layer(
			CorsLayer::new()
				.allow_methods(Any)
				.allow_headers([AUTHORIZATION, CONTENT_TYPE])
				.allow_origin(
					/*#[cfg(not(debug_assertions))]
					"https://statpixel.xyz"
						.parse::<axum::http::HeaderValue>()
						.unwrap(),
					#[cfg(debug_assertions)]*/
					Any,
				),
		)
		.with_state(Arc::new(data));

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
