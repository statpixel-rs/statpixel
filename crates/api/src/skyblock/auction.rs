use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use reqwest::{Method, Request, Url};
use serde::Deserialize;
use uuid::Uuid;

use crate::{http::HTTP, player::Player, Error};

#[cfg(feature = "cache")]
use crate::cache::SKYBLOCK_AUCTION_CACHE;

#[derive(Deserialize, Clone)]
pub struct Auction {
	#[serde(rename = "auctioneer")]
	pub user: Uuid,
	#[serde(with = "chrono::serde::ts_milliseconds")]
	pub start: DateTime<Utc>,
	#[serde(with = "chrono::serde::ts_milliseconds")]
	pub end: DateTime<Utc>,
	#[serde(rename = "item_bytes")]
	pub item: Item,
	pub starting_bid: u64,
	#[serde(rename = "highest_bid_amount")]
	pub highest_bid: u64,
}

#[derive(Deserialize, Clone)]
pub struct Item {
	#[serde(
		deserialize_with = "crate::nbt::item::name_from_gzipped_base64",
		rename = "data"
	)]
	pub name: String,
}

#[derive(Deserialize)]
pub struct Response {
	pub auctions: Vec<Auction>,
}

static HYPIXEL_AUCTION_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/skyblock/auction").unwrap());

impl Player {
	/// # Errors
	/// Returns an error if the player cannot be found
	pub async fn get_auctions(&self) -> Result<Arc<Vec<Auction>>, Arc<Error>> {
		#[cfg(feature = "cache")]
		return SKYBLOCK_AUCTION_CACHE
			.try_get_with_by_ref(&self.uuid, self.get_auctions_raw())
			.await;

		#[cfg(not(feature = "cache"))]
		return self.get_auctions_raw().await.map_err(Arc::new);
	}

	async fn get_auctions_raw(&self) -> Result<Arc<Vec<Auction>>, Error> {
		let url = {
			let mut url = HYPIXEL_AUCTION_API_ENDPOINT.clone();

			url.set_query(Some(&format!("player={}", self.uuid.as_simple())));
			url
		};

		let request = Request::new(Method::GET, url);
		let response = HTTP
			.perform_hypixel(request.into())
			.await?
			.json::<Response>()
			.await?;

		Ok(Arc::new(response.auctions))
	}
}
