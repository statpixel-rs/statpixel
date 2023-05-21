use serde::Deserialize;

use crate::http::HTTP;

#[derive(Deserialize, Debug)]
pub struct Response {
	pub record: Key,
	pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct Key {
	pub limit: u32,
}

/// # Errors
/// Returns an error if the key does not exist or if there was an error fetching the data.
pub async fn get_data() -> reqwest::Result<(Key, u32)> {
	let response = HTTP
		.get("https://api.hypixel.net/key")
		.send()
		.await?
		.error_for_status()?;

	let remaining = response
		.headers()
		.get("ratelimit-remaining")
		.expect("missing ratelimit-remaining header")
		.to_str()
		.expect("ratelimit-remaining header is not a valid utf-8 string")
		.parse::<u32>()
		.expect("ratelimit-remaining header is not a valid u32");

	let json = response.json::<Response>().await?;

	Ok((json.record, remaining))
}
