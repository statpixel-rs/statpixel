use once_cell::sync::Lazy;
use reqwest::header::HeaderValue;

use crate::ratelimiter::Ratelimiter;

pub static HTTP: Lazy<Ratelimiter> = Lazy::new(|| {
	#[cfg(test)]
	dotenvy::dotenv().ok();

	let client = reqwest::Client::builder().build().unwrap();

	Ratelimiter::new(
		client,
		HeaderValue::from_static(dotenvy_macro::dotenv!("HYPIXEL_API_KEY")),
	)
});
