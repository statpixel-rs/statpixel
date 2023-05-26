#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![feature(let_chains)]
#![feature(exclusive_range_pattern)]

use std::num::NonZeroU32;

use api::{key, ratelimit::HYPIXEL_RATELIMIT};
use database::get_pool;
use governor::{Quota, RateLimiter};
use poise::serenity_prelude::GatewayIntents;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use translate::{Context, Data, Error};

mod commands;
mod constants;
mod emoji;
mod format;
mod snapshot;
mod util;

pub use constants::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
	dotenvy::dotenv().ok();

	let (key, remaining) = key::get_data().await.unwrap();

	if remaining != key.limit - 1 {
		warn!(
			"ratelimit-remaining header is at {remaining} (should be {}). this could cause ratelimit issues.",
			key.limit - 1
		);
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
		commands::snapshot::daily::daily(),
		commands::display::display(),
		commands::games::duels(),
		commands::from::from(),
		commands::guild::guild(),
		commands::help::help(),
		commands::history::history(),
		commands::link::link(),
		commands::games::megawalls(),
		commands::snapshot::monthly::monthly(),
		commands::games::murdermystery(),
		commands::games::paintball(),
		commands::games::pit(),
		commands::games::quake(),
		commands::ser::ser(),
		commands::games::skywars(),
		commands::games::smash(),
		commands::games::speeduhc(),
		commands::games::tntgames(),
		commands::games::turbokartracers(),
		commands::games::uhc(),
		commands::unlink::unlink(),
		commands::games::vampirez(),
		commands::games::walls(),
		commands::games::warlords(),
		commands::snapshot::weekly::weekly(),
		commands::games::woolwars(),
	];

	let locale = translate::read_ftl().unwrap();
	locale.apply_translations(&mut commands, false);

	let pool = get_pool(20);

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
		let pool = get_pool(2);

		while let Err(e) = snapshot::user::begin(&pool).await {
			error!(error = ?e, "error in user snapshot update loop");

			tokio::time::sleep(std::time::Duration::from_secs(60)).await;
		}
	});

	tokio::task::spawn(async move {
		let pool = get_pool(2);

		while let Err(e) = snapshot::guild::begin(&pool).await {
			error!(error = ?e, "error in guild snapshot update loop");

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
		poise::Event::GuildCreate { guild, .. } => {
			info!(guild = ?guild.name, "joined guild");
		}
		poise::Event::GuildDelete { incomplete, .. } => {
			info!(guild = ?incomplete.id, "left guild");
		}
		_ => {}
	}

	Ok(())
}
