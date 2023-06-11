#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![feature(let_chains)]
#![feature(exclusive_range_pattern)]
#![feature(iter_intersperse)]

use database::{get_pool, schema::usage};
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::GatewayIntents;
use snapshot::user;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use translate::{Context, Data, Error};

mod commands;
mod constants;
mod emoji;
mod format;
mod snapshot;
mod tip;
mod util;

pub use constants::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_os = "linux")]
pub const IMAGE_NAME: &str = "canvas.webp";
#[cfg(not(target_os = "linux"))]
pub const IMAGE_NAME: &str = "canvas.png";

#[tokio::main]
async fn main() {
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
	dotenvy::dotenv().ok();

	let mut commands = vec![
		commands::games::arcade(),
		commands::games::arena(),
		commands::background::background(),
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
		commands::leaderboard::leaderboard(),
		commands::link::link(),
		commands::games::megawalls(),
		commands::snapshot::monthly::monthly(),
		commands::games::murdermystery(),
		commands::network::network(),
		commands::games::paintball(),
		commands::games::pit(),
		commands::games::quake(),
		commands::ser::ser(),
		commands::skyblock::skyblock(),
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

	let pool = user::upgrade::all(get_pool(20)).await.unwrap();

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands,
			event_handler: |ctx, event, framework, user_data| {
				Box::pin(event_handler(ctx, event, framework, user_data))
			},
			pre_command: |ctx| Box::pin(pre_command(ctx)),
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

async fn pre_command(ctx: Context<'_>) {
	let Ok(mut connection) = ctx.data().pool.get().await else {
		return;
	};

	diesel::insert_into(usage::table)
		.values((
			usage::user_id.eq(ctx.author().id.0 as i64),
			usage::command_name.eq(&ctx.command().qualified_name),
			usage::count.eq(1),
		))
		.on_conflict((usage::user_id, usage::command_name))
		.do_update()
		.set(usage::count.eq(usage::count + 1))
		.execute(&mut connection)
		.await
		.ok();
}

async fn event_handler(
	ctx: &poise::serenity_prelude::Context,
	event: &poise::Event<'_>,
	_framework: poise::FrameworkContext<'_, Data, Error>,
	_data: &Data,
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
