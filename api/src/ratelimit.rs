use std::num::NonZeroU32;

use governor::{
	clock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota, RateLimiter,
};
use once_cell::sync::Lazy;

pub static HYPIXEL_RATELIMIT: Lazy<
	governor::RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
> = Lazy::new(|| RateLimiter::direct(Quota::per_minute(NonZeroU32::new(60).unwrap())));

pub static MOJANG_RATELIMIT: Lazy<
	governor::RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
> = Lazy::new(|| RateLimiter::direct(Quota::per_minute(NonZeroU32::new(60).unwrap())));
