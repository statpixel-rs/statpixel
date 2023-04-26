use http_cache_reqwest::{CacheMode, HttpCache, MokaManager};
use moka::future::CacheBuilder;
use once_cell::sync::Lazy;
use reqwest_middleware::ClientBuilder;
use std::time::Duration;

pub static HTTP: Lazy<reqwest_middleware::ClientWithMiddleware> = Lazy::new(|| {
	let mut headers = reqwest::header::HeaderMap::new();

	#[cfg(test)]
	dotenvy::dotenv().ok();

	headers.insert(
		"API-Key",
		std::env::var("HYPIXEL_API_KEY")
			.expect("missing HYPIXEL_API_KEY")
			.parse()
			.expect("failed to parse HYPIXEL_API_KEY"),
	);

	ClientBuilder::new(
		reqwest::Client::builder()
			.default_headers(headers)
			.build()
			.unwrap(),
	)
	.with(http_cache_reqwest::Cache(HttpCache {
		mode: CacheMode::Default,
		manager: MokaManager::new(
			CacheBuilder::new(1_000)
				.time_to_idle(Duration::from_secs(60 * 5))
				.time_to_live(Duration::from_secs(60 * 30))
				.build(),
		),
		options: None,
	}))
	.build()
});
