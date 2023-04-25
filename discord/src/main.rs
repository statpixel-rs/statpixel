use database::{get_pool, PostgresPool};
use poise::serenity_prelude::GatewayIntents;
use thiserror::Error;

mod canvas;
mod commands;
mod constants;
mod util;

pub use constants::*;

pub struct Data {
	pub pool: PostgresPool,
}

type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("api error")]
	Api(#[from] api::Error),
	#[error("diesel error")]
	Diesel(#[from] diesel::result::Error),
	#[error("database error")]
	Database(#[from] r2d2::Error),
	#[error("framework error")]
	Framework(#[from] poise::serenity_prelude::Error),
	#[error("setup error")]
	Setup,
	#[error("not linked error")]
	NotLinked,
}

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();

	let pool = get_pool();
	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![commands::link(), commands::display(), commands::skywars()],
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
