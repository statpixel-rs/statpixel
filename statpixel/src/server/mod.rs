mod auth;
mod builder;
mod error;
mod extract;
mod image;
mod metrics;
mod topgg;

use axum::{
	error_handling::HandleErrorLayer,
	http::header::{AUTHORIZATION, CONTENT_TYPE},
	routing::{get, post},
	Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower::{BoxError, ServiceBuilder};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
	compression::CompressionLayer,
	cors::{Any, CorsLayer},
};

use self::error::display_error;

pub type Data = translate::Data;

pub async fn run(data: Data) {
	let governor = Box::new(
		GovernorConfigBuilder::default()
			.per_second(2)
			.burst_size(60)
			.use_headers()
			.finish()
			.unwrap(),
	);

	let app = Router::new()
		.route("/internal/vote", post(topgg::add_vote))
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
		.layer(
			ServiceBuilder::new()
				.layer(HandleErrorLayer::new(|e: BoxError| async move {
					display_error(e)
				}))
				.layer(GovernorLayer {
					config: Box::leak(governor),
				}),
		)
		.with_state(Arc::new(data));

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

	axum::Server::bind(&addr)
		.serve(app.into_make_service_with_connect_info::<SocketAddr>())
		.await
		.unwrap();
}
