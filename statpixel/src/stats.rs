use api::http::HTTP;
use once_cell::sync::Lazy;
use reqwest::{header::HeaderValue, Url};
use serde::Serialize;

static URL: Lazy<Url> =
	Lazy::new(|| Url::parse("https://top.gg/api/bots/718687348883193916/stats").unwrap());

#[derive(Debug, Serialize)]
pub struct Payload {
	#[serde(rename = "server_count")]
	pub guilds: u64,
	#[serde(rename = "shard_count")]
	pub shards: u64,
}

pub async fn post(token: &HeaderValue) -> Result<(), api::Error> {
	let payload = Payload {
		guilds: crate::GUILDS.read().await.len() as u64,
		shards: *crate::SHARDS.read().await,
	};

	if payload.shards == 0 || payload.guilds == 0 {
		return Ok(());
	}

	HTTP.post(URL.clone())
		.json(&payload)
		.header(reqwest::header::AUTHORIZATION, token)
		.send()
		.await?;

	Ok(())
}
