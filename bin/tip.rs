use std::borrow::Cow;

use rand::prelude::SliceRandom;
use translate::{tr, Context};

const TIPS: &[&str] = &[
	"tip-background",
	"tip-history",
	"tip-from",
	"tip-leaderboard",
	"tip-skyblock",
	"tip-link",
	"tip-guild",
	"tip-snapshot",
	"tip-display",
	"tip-help",
	"tip-website",
	"tip-support-discord",
];

pub fn random(ctx: Context<'_>) -> Cow<str> {
	tr!(ctx, TIPS.choose(&mut rand::thread_rng()).unwrap())
}
