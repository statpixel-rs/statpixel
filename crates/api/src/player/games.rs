use std::{str::FromStr, sync::Arc, time::Duration};

use moka::future::{Cache, CacheBuilder};
use once_cell::sync::Lazy;
use reqwest::{Request, Url};
use serde::Deserialize;
use uuid::Uuid;

use crate::http::HTTP;

pub use hypixel::player::Game;

pub static GAMES_CACHE: Lazy<Cache<Uuid, Arc<Vec<Game>>>> = Lazy::new(|| {
	CacheBuilder::new(10_000)
		.time_to_live(Duration::from_secs(60 * 5))
		.build()
});

static HYPIXEL_RECENT_GAMES_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/v2/recentgames").unwrap());

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Response {
	pub games: Vec<Game>,
}

impl super::Player {
	/// # Errors
	/// Returns an error if the response could not be decoded.
	pub async fn get_games(&self) -> Result<Arc<Vec<Game>>, Arc<crate::Error>> {
		GAMES_CACHE
			.try_get_with(self.uuid, self.get_games_raw())
			.await
	}

	async fn get_games_raw(&self) -> Result<Arc<Vec<Game>>, crate::Error> {
		let url = {
			let mut url = HYPIXEL_RECENT_GAMES_API_ENDPOINT.clone();

			url.set_query(Some(&format!("uuid={}", self.uuid)));
			url
		};

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_hypixel(req.into()).await?;
		let response = response.json::<Response>().await?;

		Ok(Arc::new(response.games))
	}
}
