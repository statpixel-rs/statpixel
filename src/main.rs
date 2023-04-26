use database::{get_pool, PostgresPool};
use poise::serenity_prelude::GatewayIntents;
use thiserror::Error;

mod commands;
mod constants;
mod util;

pub use constants::*;

#[derive(Debug)]
pub struct Data {
	pub pool: PostgresPool,
}

type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("An error occurred while fetching data from the internal API.")]
	Api(#[from] api::Error),
	#[error("an error occurred while interacting with Diesel.")]
	Diesel(#[from] diesel::result::Error),
	#[error("An error occurred while interacting with the database.")]
	Database(#[from] r2d2::Error),
	#[error("An internal error occurred.")]
	Framework(#[from] poise::serenity_prelude::Error),
	#[error("An internal error occurred during setup.")]
	Setup,
	#[error("You are not linked to a Minecraft account. Use `/link` to link your account.")]
	NotLinked,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();

	let pool = get_pool();
	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![commands::link(), commands::display(), commands::skywars()],
			event_handler: |ctx, event, framework, user_data| {
				Box::pin(event_handler(ctx, event, framework, user_data))
			},
			..Default::default()
		})
		.token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
		.intents(GatewayIntents::non_privileged())
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands)
					.await
					.map_err(|_| Error::Setup)?;
				Ok(Data { pool })
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
		println!("{} is connected!", data_about_bot.user.name);

		ctx.set_activity(poise::serenity_prelude::Activity::watching(format!(
			"Shard #{} | v{VERSION}",
			ctx.shard_id + 1,
		)))
		.await;
	}

	Ok(())
}
