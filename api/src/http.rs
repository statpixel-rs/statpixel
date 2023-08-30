use once_cell::sync::Lazy;
use reqwest::header::HeaderValue;

use crate::ratelimiter::Ratelimiter;

pub static HTTP: Lazy<Ratelimiter> = Lazy::new(|| {
	#[cfg(any(test, feature = "runtime_env"))]
	dotenvy::dotenv().ok();

	let client = reqwest::Client::builder().build().unwrap();

	#[cfg(not(feature = "runtime_env"))]
	let header = HeaderValue::from_static(dotenvy_macro::dotenv!("HYPIXEL_API_KEY"));

	#[cfg(feature = "runtime_env")]
	let header = HeaderValue::from_str(
		std::env::var("HYPIXEL_API_KEY")
			.expect("HYPIXEL_API_KEY not set")
			.as_str(),
	)
	.expect("HYPIXEL_API_KEY is not a valid header value");

	Ratelimiter::new(client, header)
});
