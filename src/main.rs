#![feature(let_chains)]

use std::sync::Arc;

use database::{get_pool, PostgresPool};
use poise::serenity_prelude::GatewayIntents;
use thiserror::Error;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod commands;
mod constants;
mod locale;
mod util;

pub use constants::*;

pub struct Data {
	pub pool: PostgresPool,
	pub locale: locale::Locale,
}

type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Api(#[from] Arc<api::Error>),
	#[error("An error occurred while interacting with Diesel.")]
	Diesel(#[from] diesel::result::Error),
	#[error("An error occurred while interacting with the database.")]
	Database(#[from] r2d2::Error),
	#[error("An internal error occurred.")]
	Framework(#[from] poise::serenity_prelude::Error),
	#[error("An internal error occurred during setup.")]
	Setup,
	#[error("You are not linked to a Minecraft account. Use `/link` to link your account.")]
	NotLinked,
	#[error("An error occurred while negotiating game mode.")]
	GameMode,
	#[error("The uuid `{0}` is invalid.")]
	InvalidUuid(String),
	#[error("The username `{0}` is invalid.")]
	InvalidUsername(String),
	#[error("An error occurred while handling io.")]
	Io(#[from] std::io::Error),
}

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
		commands::skywars(),
		commands::cache(),
	];

	let locale = locale::read_ftl().unwrap();
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
		.intents(GatewayIntents::non_privileged())
		.setup(move |ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands)
					.await
					.map_err(|_| Error::Setup)?;
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
	if let poise::Event::Ready { data_about_bot } = event {
		info!("{} is connected!", data_about_bot.user.name);

		ctx.set_activity(poise::serenity_prelude::Activity::watching(format!(
			"Shard #{} | v{VERSION}",
			ctx.shard_id + 1,
		)))
		.await;
	}

	Ok(())
}
