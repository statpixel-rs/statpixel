use api::http::HTTP;
use once_cell::sync::Lazy;
use reqwest::{header::HeaderValue, Url};
use serde::Serialize;
use std::sync::atomic;

static URL: Lazy<Url> =
	Lazy::new(|| Url::parse("https://top.gg/api/bots/718687348883193916/stats").unwrap());

#[derive(Debug, Serialize)]
pub struct Payload {
	#[serde(rename = "server_count")]
	pub shards: Vec<u64>,
}

pub async fn post(token: &HeaderValue) -> Result<(), api::Error> {
	let guilds = crate::GUILDS.read().await.len() as u64;
	let shards = crate::SHARDS.load(atomic::Ordering::Relaxed);

	if guilds == 0 || shards == 0 {
		return Ok(());
	}

	let payload = Payload {
		shards: vec![guilds / shards; shards as usize],
	};

	HTTP.post(URL.clone())
		.json(&payload)
		.header(reqwest::header::AUTHORIZATION, token)
		.send()
		.await?;

	Ok(())
}
