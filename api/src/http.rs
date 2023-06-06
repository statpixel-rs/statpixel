use once_cell::sync::Lazy;

use crate::ratelimiter::Ratelimiter;

pub static HTTP: Lazy<Ratelimiter> = Lazy::new(|| {
	#[cfg(test)]
	dotenvy::dotenv().ok();

	let client = reqwest::Client::builder().build().unwrap();

	Ratelimiter::new(
		client,
		std::env::var("HYPIXEL_API_KEY")
			.expect("missing HYPIXEL_API_KEY")
			.parse()
			.expect("failed to parse HYPIXEL_API_KEY"),
	)
});
