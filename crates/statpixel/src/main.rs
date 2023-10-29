#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::wildcard_imports)]
#![feature(let_chains)]
#![feature(exclusive_range_pattern)]
#![feature(iter_intersperse)]
#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::sync::Arc;

pub use api::command::Id;
use api::skyblock;
use database::{
	get_pool,
	models::MetricKind,
	schema::{metric, usage},
};
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use once_cell::sync::Lazy;
use poise::serenity_prelude::{
	self as serenity, ConnectionStage, FullEvent, GatewayIntents, Interaction,
};
#[cfg(not(debug_assertions))]
use reqwest::header::HeaderValue;
use snapshot::user;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use translate::{context, Context, Data, Error};

mod commands;
mod constants;
mod emoji;
mod format;
mod id;
mod server;
mod snapshot;
#[cfg(not(debug_assertions))]
mod stats;
mod tip;
mod util;

pub use constants::*;
use util::deprecated_interaction;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub static GUILDS: Lazy<tokio::sync::RwLock<HashSet<u64>>> = Lazy::new(Default::default);
pub static SHARDS: Lazy<tokio::sync::RwLock<u64>> = Lazy::new(Default::default);

#[cfg(target_os = "linux")]
pub const IMAGE_NAME: &str = "statpixel.png";
#[cfg(not(target_os = "linux"))]
pub const IMAGE_NAME: &str = "statpixel.png";

pub static DATA: std::sync::OnceLock<Arc<translate::Data>> = std::sync::OnceLock::new();

async fn create_redis_manager() -> redis::aio::ConnectionManager {
	#[cfg(not(feature = "runtime_env"))]
	let url = dotenvy_macro::dotenv!("REDIS_URL");

	#[cfg(feature = "runtime_env")]
	let url = std::env::var("REDIS_URL").expect("REDIS_URL not set");

	#[cfg(feature = "runtime_env")]
	let url = url.as_str();

	redis::aio::ConnectionManager::new(
		redis::Client::open(url).expect("failed to connect to Redis"),
	)
	.await
	.expect("failed to create Redis connection manager")
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() {
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

	#[cfg(feature = "runtime_env")]
	dotenvy::dotenv().ok();

	let mut commands = vec![
		// These commands are registered per-server, so we can handle the "base" separately
		commands::games::arcade::basic(),
		commands::games::arena::basic(),
		commands::games::bedwars::basic(),
		commands::games::blitz::basic(),
		commands::games::buildbattle::basic(),
		commands::games::copsandcrims::basic(),
		commands::games::duels::basic(),
		commands::games::fishing::basic(),
		commands::games::megawalls::basic(),
		commands::games::murdermystery::basic(),
		commands::games::paintball::basic(),
		commands::games::pit::basic(),
		commands::games::quake::basic(),
		commands::games::skywars::basic(),
		commands::games::smash::basic(),
		commands::games::speeduhc::basic(),
		commands::games::tntgames::basic(),
		commands::games::turbokartracers::basic(),
		commands::games::uhc::basic(),
		commands::games::vampirez::basic(),
		commands::games::walls::basic(),
		commands::games::warlords::basic(),
		commands::games::woolwars::basic(),
		//
		commands::about::about(),
		commands::games::arcade::parent(),
		commands::games::arena::parent(),
		commands::background::background(),
		commands::games::bedwars::parent(),
		commands::builder::builder(),
		commands::games::blitz::parent(),
		commands::boost::boost(),
		commands::games::buildbattle::parent(),
		commands::games::copsandcrims::parent(),
		commands::execute::execute(),
		commands::display::display(),
		commands::games::duels::parent(),
		commands::games::fishing::parent(),
		commands::guild::guild(),
		commands::help::help(),
		commands::leaderboard::leaderboard(),
		commands::link::link(),
		commands::games::megawalls::parent(),
		commands::games::murdermystery::parent(),
		commands::network::network(),
		commands::games::paintball::parent(),
		commands::games::pit::parent(),
		commands::games::quake::parent(),
		commands::recent::recent(),
		commands::session::session(),
		commands::skyblock::skyblock(),
		commands::games::skywars::parent(),
		commands::games::smash::parent(),
		commands::games::speeduhc::parent(),
		commands::track::track(),
		commands::games::tntgames::parent(),
		commands::games::turbokartracers::parent(),
		commands::games::uhc::parent(),
		commands::unlink::unlink(),
		commands::games::vampirez::parent(),
		commands::games::walls::parent(),
		commands::games::warlords::parent(),
		commands::winstreaks::winstreaks(),
		commands::games::woolwars::parent(),
		commands::parkour::parkour(),
	];

	let locale = translate::read_ftl().unwrap();
	locale.apply_translations(&mut commands, None);

	let pool = user::upgrade::all(get_pool(20)).await.unwrap();
	let data = Data {
		pool,
		locale: Arc::new(locale),
		redis: create_redis_manager().await,
	};

	DATA.set(Arc::new(data.clone())).unwrap();

	let framework = poise::Framework::new(
		poise::FrameworkOptions {
			commands,
			listener: |event, framework, user_data| {
				Box::pin(event_handler(event, framework, user_data))
			},
			pre_command: |ctx| Box::pin(pre_command(ctx)),
			..Default::default()
		},
		{
			let data = data.clone();

			move |ctx, _ready, framework| {
				Box::pin(async move {
					serenity::Command::set_global_commands(
						&ctx.http,
						poise::builtins::create_application_commands(
							&framework.options().commands[translate::GAMES..],
						),
					)
					.await
					.unwrap();

					Ok(data)
				})
			}
		},
	);

	#[cfg(not(feature = "runtime_env"))]
	let token = dotenvy_macro::dotenv!("DISCORD_TOKEN");

	#[cfg(feature = "runtime_env")]
	let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
	#[cfg(feature = "runtime_env")]
	let token = token.as_str();

	let mut client = serenity::Client::builder(token, GatewayIntents::GUILDS)
		.framework(framework)
		.await
		.unwrap();

	tokio::task::spawn({
		let http = Arc::clone(&client.http);
		let data = data.clone();

		async move {
			let ctx = context::Context::automated(&data);
			let pool = get_pool(2);

			while let Err(e) = snapshot::user::begin(&pool, &ctx, &http).await {
				error!(error = ?e, "error in user snapshot update loop");

				tokio::time::sleep(std::time::Duration::from_secs(60)).await;
			}
		}
	});

	tokio::task::spawn(async move {
		let pool = get_pool(2);

		while let Err(e) = snapshot::guild::begin(&pool).await {
			error!(error = ?e, "error in guild snapshot update loop");

			tokio::time::sleep(std::time::Duration::from_secs(60)).await;
		}
	});

	tokio::task::spawn(async move {
		let pool = get_pool(3);
		let mut identifiers = skyblock::bazaar::get_all_item_identifiers(&pool).await;

		loop {
			if let Err(e) = skyblock::bazaar::update(&pool, &mut identifiers).await {
				error!(error = ?e, "error in bazaar update loop");
			}

			tokio::time::sleep(std::time::Duration::from_secs(60)).await;
		}
	});

	tokio::task::spawn(async move {
		server::run(data).await;
	});

	#[cfg(not(debug_assertions))]
	tokio::task::spawn(async move {
		info!("starting topgg stats loop");

		#[cfg(not(feature = "runtime_env"))]
		let token = dotenvy_macro::dotenv!("TOPGG_TOKEN");

		#[cfg(feature = "runtime_env")]
		let token = std::env::var("TOPGG_TOKEN").expect("TOPGG_TOKEN not set");
		#[cfg(feature = "runtime_env")]
		let token = token.as_str();

		let token = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();

		loop {
			if let Err(e) = stats::post(&token).await {
				error!(error = ?e, "error in topgg stats loop");
			}

			tokio::time::sleep(std::time::Duration::from_secs(60)).await;
		}
	});

	client.start_autosharded().await.unwrap();
}

async fn pre_command(ctx: Context<'_>) {
	let Ok(mut connection) = context::Context::from_poise(&ctx).connection().await else {
		return;
	};

	diesel::insert_into(usage::table)
		.values((
			usage::user_id.eq(ctx.author().id.get() as i64),
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

#[allow(clippy::too_many_lines)]
async fn event_handler(
	event: &FullEvent,
	framework: poise::FrameworkContext<'_, Data, Error>,
	data: &Data,
) -> Result<(), Error> {
	match event {
		FullEvent::Ready {
			ctx,
			data_about_bot: ready,
		} => {
			info!(user = ?ready.user.tag(), guilds = ready.guilds.len(), "logged in");

			GUILDS
				.write()
				.await
				.extend(ready.guilds.iter().map(|g| g.id.get()));

			for guild in &ready.guilds {
				guild
					.id
					.set_commands(
						&ctx.http,
						poise::builtins::create_application_commands(
							&framework.options().commands[..translate::GAMES],
						),
					)
					.await
					.ok();
			}

			ctx.set_activity(Some(serenity::ActivityData {
				name: format!("statpixel.xyz | v{VERSION}"),
				kind: serenity::ActivityType::Custom,
				state: Some(format!("statpixel.xyz | v{VERSION}")),
				url: None,
			}));
		}
		FullEvent::InteractionCreate {
			ctx,
			interaction: Interaction::Modal(interaction),
		} => {
			let Some(id) = api::id::decode(&interaction.data.custom_id) else {
				let ctx = context::Context::from_modal(ctx, data, interaction);
				return Ok(ctx.send(deprecated_interaction(&ctx)).await?);
			};

			match id {
				api::id::Id::Builder(id) => {
					if let Err(e) =
						commands::builder::modal_handler(ctx, interaction, data, id).await
					{
						let ctx = context::Context::from_modal(ctx, data, interaction);
						util::error(&ctx, e).await;
					}
				}
				api::id::Id::Command(..) => return Ok(()),
			}
		}
		FullEvent::InteractionCreate {
			ctx,
			interaction: Interaction::Component(interaction),
		} => {
			let ctx = context::Context::from_component(ctx, data, interaction);
			let values = match &interaction.data.kind {
				serenity::ComponentInteractionDataKind::StringSelect { ref values } => values,
				serenity::ComponentInteractionDataKind::Button => {
					let Some(id) = api::id::decode(&interaction.data.custom_id) else {
						return Ok(ctx.send(deprecated_interaction(&ctx)).await?);
					};

					return match id {
						api::id::Id::Builder(id) => {
							commands::builder::handler(&ctx, interaction, id).await
						}
						api::id::Id::Command(id) => {
							if let Err(e) = crate::id::map(&ctx, id).await {
								util::error(&ctx, e).await;
							}

							Ok(())
						}
					};
				}
				_ => return Ok(()),
			};

			if interaction.data.custom_id == "select" {
				let Some(id) = values.get(0).and_then(|p| api::id::decode(p)) else {
					return Ok(ctx.send(deprecated_interaction(&ctx)).await?);
				};

				if let api::id::Id::Command(id) = id {
					if let Err(e) = crate::id::map(&ctx, id).await {
						util::error(&ctx, e).await;
					}
				}
			} else {
				let Some(id) = api::id::decode(&interaction.data.custom_id) else {
					return Ok(ctx.send(deprecated_interaction(&ctx)).await?);
				};

				if let api::id::Id::Builder(id) = id {
					if let Err(e) = commands::builder::handler(&ctx, interaction, id).await {
						util::error(&ctx, e).await;
					}
				}
			}
		}
		FullEvent::GuildCreate { ctx, guild, .. } => {
			if GUILDS.write().await.insert(guild.id.get()) && tracing::enabled!(Level::INFO) {
				let guilds = GUILDS.read().await.len();

				info!(guilds = guilds, "guild count");

				diesel::insert_into(metric::table)
					.values((
						metric::discord_id.eq(guild.id.get() as i64),
						metric::kind.eq(i16::from(MetricKind::GuildJoin)),
					))
					.execute(&mut data.pool.get().await?)
					.await
					.ok();

				info!(name = guild.name, "joined guild");

				guild
					.set_commands(
						&ctx.http,
						poise::builtins::create_application_commands(
							&framework.options().commands[..translate::GAMES],
						),
					)
					.await
					.ok();
			}
		}
		FullEvent::GuildDelete {
			incomplete: guild, ..
		} => {
			if GUILDS.write().await.remove(&guild.id.get()) && tracing::enabled!(Level::INFO) {
				let guilds = GUILDS.read().await.len();

				info!(guilds = guilds, "guild count");

				diesel::insert_into(metric::table)
					.values((
						metric::discord_id.eq(guild.id.get() as i64),
						metric::kind.eq(i16::from(MetricKind::GuildLeave)),
					))
					.execute(&mut data.pool.get().await?)
					.await
					.ok();
			}
		}
		FullEvent::ShardStageUpdate { event, .. } => {
			if event.new == ConnectionStage::Connected {
				*SHARDS.write().await += 1;
			} else if event.old == ConnectionStage::Connected {
				*SHARDS.write().await -= 1;
			}
		}
		_ => {}
	}

	Ok(())
}
