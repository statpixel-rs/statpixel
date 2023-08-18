use std::borrow::Cow;

use rand::prelude::SliceRandom;
use translate::{prelude::GetLocale, tr};

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
	"tip-winstreak",
	"tip-recent",
	"tip-bazaar",
	"tip-boost",
	"tip-track",
	"tip-builder",
	"tip-dashboard",
];

pub fn random(ctx: &impl GetLocale) -> Cow<str> {
	tr(ctx, TIPS.choose(&mut rand::thread_rng()).unwrap())
}
