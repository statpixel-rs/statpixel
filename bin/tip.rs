use std::borrow::Cow;

use rand::prelude::SliceRandom;
use translate::{context::Context, tr};

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
	"tip-project",
];

pub fn random<'c>(ctx: &'c Context<'_>) -> Cow<'c, str> {
	tr!(ctx, TIPS.choose(&mut rand::thread_rng()).unwrap())
}
