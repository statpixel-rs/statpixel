use once_cell::sync::Lazy;

pub static HTTP: Lazy<reqwest::Client> = Lazy::new(|| {
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

	reqwest::Client::builder()
		.default_headers(headers)
		.build()
		.unwrap()
});
