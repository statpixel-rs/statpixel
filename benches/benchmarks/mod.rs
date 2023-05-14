pub mod player_data_compress;
pub mod player_data_de_bson;
pub mod player_data_de_json;
pub mod player_data_decompress;
pub mod player_data_se_bson;

use std::num::NonZeroU32;

use api::{key, ratelimit::HYPIXEL_RATELIMIT};
use governor::{Quota, RateLimiter};

async fn set_up_key() {
	if HYPIXEL_RATELIMIT.get().is_some() {
		return;
	}

	dotenvy::dotenv().ok();

	let (key, _) = key::get_data().await.unwrap();

	HYPIXEL_RATELIMIT
		.set(RateLimiter::direct(Quota::per_minute(
			NonZeroU32::new(key.limit).unwrap(),
		)))
		.ok();
}
