pub mod auction;
#[cfg(feature = "database")]
pub mod bazaar;
pub mod essence;
pub mod materials;
pub mod member;
pub mod modifier;
pub mod networth;
pub mod pet;
pub mod prestige;
pub mod profile;

use std::{str::FromStr, sync::Arc};

use once_cell::sync::Lazy;
use reqwest::{Method, Request, StatusCode, Url};
use serde::Deserialize;

use crate::{
	http::HTTP,
	player::{stats::sky_block, Player},
	Error,
};

#[cfg(feature = "cache")]
use crate::cache::SKYBLOCK_PROFILE_CACHE;

use self::profile::Profile;

static HYPIXEL_SKYBLOCK_PROFILE_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/v2/skyblock/profile").unwrap());

#[derive(Debug, Deserialize)]
pub enum ProfileName {
	Apple,
	Banana,
	Blueberry,
	Coconut,
	Cucumber,
	Grapes,
	Kiwi,
	Lemon,
	Lime,
	Mango,
	Orange,
	Papaya,
	Pear,
	Peach,
	Pineapple,
	Pomegranate,
	Raspberry,
	Strawberry,
	Tomato,
	Watermelon,
	Zucchini,
	Custom(String),
}

pub const NAMES: &[&str] = &[
	"Apple",
	"Banana",
	"Blueberry",
	"Coconut",
	"Cucumber",
	"Grapes",
	"Kiwi",
	"Lemon",
	"Lime",
	"Mango",
	"Orange",
	"Papaya",
	"Pear",
	"Peach",
	"Pineapple",
	"Pomegranate",
	"Raspberry",
	"Strawberry",
	"Tomato",
	"Watermelon",
	"Zucchini",
];

#[derive(Deserialize)]
pub struct Response {
	pub profile: Option<Profile>,
}

impl Player {
	/// # Errors
	/// Returns [`Error::SessionNotFound`] if the player has no skyblock profile.
	pub async fn get_skyblock_profile(
		profile: &sky_block::Profile,
		username: &str,
	) -> Result<Arc<Profile>, Arc<Error>> {
		#[cfg(feature = "cache")]
		return SKYBLOCK_PROFILE_CACHE
			.try_get_with(
				profile.id,
				Self::get_skyblock_profile_raw(profile, username),
			)
			.await;

		#[cfg(not(feature = "cache"))]
		Self::get_skyblock_profile_raw(profile, username)
			.await
			.map_err(Arc::new)
	}

	async fn get_skyblock_profile_raw(
		profile: &sky_block::Profile,
		username: &str,
	) -> Result<Arc<Profile>, Error> {
		let url = {
			let mut url = HYPIXEL_SKYBLOCK_PROFILE_ENDPOINT.clone();

			url.set_query(Some(&format!("profile={}", &profile.id)));
			url
		};

		let request = Request::new(Method::GET, url);
		let response = HTTP.perform_hypixel(request.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::ProfileNotFound(
				profile.name.clone(),
				username.to_string(),
			));
		}

		let response = response.json::<Response>().await?;

		response.profile.map_or_else(
			|| {
				Err(Error::ProfileNotFound(
					profile.name.clone(),
					username.to_string(),
				))
			},
			|p| Ok(Arc::new(p)),
		)
	}
}
