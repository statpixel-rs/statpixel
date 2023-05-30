use std::time::Duration;

use moka::future::{Cache, CacheBuilder};
use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::{
	guild::Guild,
	player::{data::Data, status::Session, Player},
	skyblock::{self, profile::Profile},
};

pub static PLAYER_DATA_CACHE: Lazy<Cache<Uuid, Data>> = Lazy::new(|| {
	CacheBuilder::new(100_000)
		.time_to_idle(Duration::from_secs(60 * 10))
		.time_to_live(Duration::from_secs(60 * 30))
		.build()
});

pub static GUILD_DATA_MEMBER_CACHE: Lazy<Cache<Uuid, Guild>> = Lazy::new(|| {
	CacheBuilder::new(100_000)
		.time_to_idle(Duration::from_secs(60 * 10))
		.time_to_live(Duration::from_secs(60 * 30))
		.build()
});

pub static GUILD_DATA_NAME_CACHE: Lazy<Cache<String, Guild>> = Lazy::new(|| {
	CacheBuilder::new(100_000)
		.time_to_idle(Duration::from_secs(60 * 10))
		.time_to_live(Duration::from_secs(60 * 30))
		.build()
});

pub static GUILD_DATA_UUID_CACHE: Lazy<Cache<Uuid, Guild>> = Lazy::new(|| {
	CacheBuilder::new(100_000)
		.time_to_idle(Duration::from_secs(60 * 10))
		.time_to_live(Duration::from_secs(60 * 30))
		.build()
});

pub static PLAYER_SESSION_CACHE: Lazy<Cache<Uuid, Session>> = Lazy::new(|| {
	CacheBuilder::new(100_000)
		.time_to_live(Duration::from_secs(60 * 5))
		.build()
});

// lower-case username or uuid to Player
pub static PLAYER_CACHE: Lazy<Cache<String, Player>> = Lazy::new(|| {
	CacheBuilder::new(1_000_000)
		.time_to_live(Duration::from_secs(60 * 60 * 24))
		.build()
});

// profile uuid to profile
pub static SKYBLOCK_PROFILE_CACHE: Lazy<Cache<Uuid, Profile>> = Lazy::new(|| {
	CacheBuilder::new(50_000)
		.time_to_live(Duration::from_secs(60 * 30))
		.build()
});

pub static SKYBLOCK_AUCTION_CACHE: Lazy<Cache<Uuid, Vec<skyblock::auction::Auction>>> =
	Lazy::new(|| {
		CacheBuilder::new(50_000)
			.time_to_live(Duration::from_secs(60 * 30))
			.build()
	});
