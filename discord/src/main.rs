use database::{get_pool, PostgresPool};
use poise::serenity_prelude::GatewayIntents;

mod commands;
mod constants;

pub use constants::*;

pub struct Data {
	pub pool: PostgresPool,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();

	let pool = get_pool();
	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![commands::link(), commands::display()],
			..Default::default()
		})
		.token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
		.intents(GatewayIntents::non_privileged())
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(Data { pool })
			})
		});

	framework.run().await.unwrap();
}
