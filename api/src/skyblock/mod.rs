pub mod member;
pub mod profile;
pub mod auction;

use std::{str::FromStr, sync::Arc};

use once_cell::sync::Lazy;
use reqwest::{Method, Request, StatusCode, Url};
use serde::Deserialize;
use uuid::Uuid;

use crate::{cache::SKYBLOCK_PROFILE_CACHE, http::HTTP, player::Player, Error};

use self::profile::Profile;

static HYPIXEL_SKYBLOCK_PROFILE_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/skyblock/profile").unwrap());

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
	pub async fn get_skyblock_profile(profile: Uuid) -> Result<Profile, Arc<Error>> {
		SKYBLOCK_PROFILE_CACHE
			.try_get_with_by_ref(&profile, Self::get_skyblock_profile_raw(profile))
			.await
	}

	async fn get_skyblock_profile_raw(profile: Uuid) -> Result<Profile, Error> {
		let url = {
			let mut url = HYPIXEL_SKYBLOCK_PROFILE_ENDPOINT.clone();

			url.set_query(Some(&format!("profile={}", &profile)));
			url
		};

		let request = Request::new(Method::GET, url);
		let response = HTTP.perform_hypixel(request.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::SessionNotFound(profile.to_string()));
		}

		let response = response.json::<Response>().await?;

		response
			.profile
			.ok_or_else(|| Error::SessionNotFound(profile.to_string()))
	}
}
