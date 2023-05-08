#![feature(let_chains)]

use database::get_pool;
use poise::serenity_prelude::GatewayIntents;
use thiserror::Error;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use translate::{Context, Data, Error};

mod commands;
mod constants;
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

	let mut commands = vec![
		commands::link(),
		commands::unlink(),
		commands::display(),
		commands::cache(),
		commands::skywars(),
		commands::bedwars(),
	];

	let locale = translate::read_ftl().unwrap();
	locale.apply_translations(&mut commands);

	let pool = get_pool();
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
