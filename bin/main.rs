#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![feature(let_chains)]
#![feature(exclusive_range_pattern)]

use std::num::NonZeroU32;

use api::{key, ratelimit::HYPIXEL_RATELIMIT};
use database::get_pool;
use governor::{Quota, RateLimiter};
use poise::serenity_prelude::GatewayIntents;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use translate::{Context, Data, Error};

mod commands;
mod constants;
mod snapshot;
mod util;

pub use constants::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
	dotenvy::dotenv().ok();

	let (key, remaining) = key::get_data().await.unwrap();

	if remaining != key.limit - 1 {
		error!(
			"ratelimit-reset header is at {remaining} (should be {}). wait a minute and try again.",
			key.limit - 1
		);

		return;
	}

	HYPIXEL_RATELIMIT
		.set(RateLimiter::direct(Quota::per_minute(
			NonZeroU32::new(key.limit).unwrap(),
		)))
		.unwrap();

	info!(ratelimit_per_min = key.limit, "hypixel api key found");

	let mut commands = vec![
		commands::games::arcade(),
		commands::games::arena(),
		commands::games::bedwars(),
		commands::games::blitz(),
		commands::games::buildbattle(),
		commands::cache::cache(),
		commands::games::copsandcrims(),
		commands::history::daily::daily(),
		commands::display::display(),
		commands::games::duels(),
		commands::link::link(),
		commands::games::megawalls(),
		commands::history::monthly::monthly(),
		commands::games::murdermystery(),
		commands::games::paintball(),
		commands::games::pit(),
		commands::games::quake(),
		commands::ser::ser(),
		commands::games::skywars(),
		commands::games::smash(),
		commands::games::speeduhc(),
		commands::games::tntgames(),
		commands::games::turkokartracers(),
		commands::games::uhc(),
		commands::unlink::unlink(),
		commands::games::vampirez(),
		commands::games::walls(),
		commands::games::warlords(),
		commands::history::weekly::weekly(),
		commands::games::woolwars(),
	];

	let locale = translate::read_ftl().unwrap();
	locale.apply_translations(&mut commands);

	let pool = get_pool(25);

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands,
			event_handler: |ctx, event, framework, user_data| {
				Box::pin(event_handler(ctx, event, framework, user_data))
			},
			..Default::default()
		})
		.token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
		.intents(GatewayIntents::from_bits(0).unwrap())
		.setup(move |ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands)
					.await
					.unwrap();

				Ok(Data { pool, locale })
			})
		});

	tokio::task::spawn(async move {
		let pool = get_pool(5);

		while let Err(e) = snapshot::update::begin(&pool).await {
			error!(error = ?e, "error in snapshot update loop");

			tokio::time::sleep(std::time::Duration::from_secs(60)).await;
		}
	});

	framework.run_autosharded().await.unwrap();
}

async fn event_handler(
	ctx: &poise::serenity_prelude::Context,
	event: &poise::Event<'_>,
	_framework: poise::FrameworkContext<'_, Data, Error>,
	_user_data: &Data,
) -> Result<(), Error> {
	match event {
		poise::Event::Ready { data_about_bot } => {
			info!(user = ?data_about_bot.user.tag(), "logged in");

			ctx.set_activity(poise::serenity_prelude::Activity::watching(format!(
				"Shard #{} | v{VERSION}",
				ctx.shard_id + 1,
			)))
			.await;
		}
		poise::Event::ShardStageUpdate { update } => {
			info!(shard = ?update, "shard stage update");
		}
		_ => {}
	}

	Ok(())
}