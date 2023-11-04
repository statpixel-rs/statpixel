mod auth;
mod boosts;
mod builder;
mod error;
mod extract;
mod image;
mod topgg;
mod tracks;
mod vendor;

use axum::{
	error_handling::HandleErrorLayer,
	http::header::{AUTHORIZATION, CONTENT_TYPE},
	routing::{get, post},
	Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower::{BoxError, ServiceBuilder};
use tower_governor::{
	governor::GovernorConfigBuilder, key_extractor::GlobalKeyExtractor, GovernorLayer,
};
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

	let governor_image = Box::new(
		GovernorConfigBuilder::default()
			.per_millisecond(750)
			.burst_size(240)
			.use_headers()
			.key_extractor(GlobalKeyExtractor)
			.finish()
			.unwrap(),
	);

	let image_router = Router::new().route("/image/:id", get(image::get)).layer(
		ServiceBuilder::new()
			.layer(HandleErrorLayer::new(|e: BoxError| async move {
				display_error(e)
			}))
			.layer(GovernorLayer {
				config: Box::leak(governor_image),
			}),
	);

	let app = Router::new()
		.route("/internal/vote", post(topgg::add_vote))
		.route("/auth/login", get(auth::login))
		.route("/auth/me", get(auth::me).patch(auth::update_me))
		.route("/builder/preview", post(builder::preview))
		.route("/builder/test", post(builder::get))
		.route("/tracks", get(tracks::get).delete(tracks::delete))
		.route("/tracks/:uuid", post(tracks::create))
		.route("/boosts", get(boosts::get).delete(boosts::delete))
		.route("/boosts/:guildId", post(boosts::create))
		.layer(
			ServiceBuilder::new()
				.layer(HandleErrorLayer::new(|e: BoxError| async move {
					display_error(e)
				}))
				.layer(GovernorLayer {
					config: Box::leak(governor),
				}),
		)
		// This is done so the global ratelimit layer is only applied to the `/image/:id` route
		.merge(image_router)
		// the /vendor endpoint is not rate-limited, but is instead throttled by
		// only allowing 1 update globally per user, every 15 minutes
		.route("/vendor", post(vendor::post))
		.layer(
			ServiceBuilder::new().layer(CompressionLayer::new()).layer(
				CorsLayer::new()
					.allow_headers([AUTHORIZATION, CONTENT_TYPE])
					.allow_methods(Any)
					.allow_origin(Any),
			),
		)
		.with_state(Arc::new(data));

	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

	axum::Server::bind(&addr)
		.serve(app.into_make_service_with_connect_info::<SocketAddr>())
		.await
		.unwrap();
}
