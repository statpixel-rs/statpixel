#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![feature(let_chains)]
#![feature(exclusive_range_pattern)]
#![feature(iter_intersperse)]

pub use api::id::Id;
use api::skyblock;
use database::{get_pool, schema::usage};
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::{self as serenity, FullEvent, GatewayIntents, Interaction};
use snapshot::user;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use translate::{context, Context, Data, Error};

mod commands;
mod constants;
mod emoji;
mod format;
mod id;
mod snapshot;
mod tip;
mod util;

pub use constants::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_os = "linux")]
pub const IMAGE_NAME: &str = "statpixel.png";
#[cfg(not(target_os = "linux"))]
pub const IMAGE_NAME: &str = "statpixel.png";

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() {
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
	dotenvy::dotenv().ok();

	let mut commands = vec![
		commands::about::about(),
		commands::games::arcade(),
		commands::games::arena(),
		commands::background::background(),
		commands::games::bedwars(),
		commands::games::blitz(),
		commands::games::buildbattle(),
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
		commands::project::project(),
		commands::games::quake(),
		commands::recent::recent(),
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
		commands::winstreaks::winstreaks(),
		commands::games::woolwars(),
	];

	let locale = translate::read_ftl().unwrap();
	locale.apply_translations(&mut commands, false);

	let pool = user::upgrade::all(get_pool(20)).await.unwrap();

	let framework = poise::Framework::new(
		poise::FrameworkOptions {
			commands,
			listener: |event, framework, user_data| {
				Box::pin(event_handler(event, framework, user_data))
			},
			pre_command: |ctx| Box::pin(pre_command(ctx)),
			..Default::default()
		},
		move |ctx, _ready, framework| {
			Box::pin(async move {
				serenity::Command::set_global_commands(
					&ctx.http,
					poise::builtins::create_application_commands(&framework.options().commands),
				)
				.await
				.unwrap();

				Ok(Data {
					pool,
					locale,
					guilds: tokio::sync::RwLock::default(),
				})
			})
		},
	);

	let mut client = serenity::Client::builder(
		std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"),
		GatewayIntents::GUILDS,
	)
	.framework(framework)
	.await
	.unwrap();

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

	client.start_autosharded().await.unwrap();
}

async fn pre_command(ctx: Context<'_>) {
	let Ok(mut connection) = ctx.data().pool.get().await else {
		return;
	};

	diesel::insert_into(usage::table)
		.values((
			usage::user_id.eq(ctx.author().id.0.get() as i64),
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
	event: &FullEvent,
	_framework: poise::FrameworkContext<'_, Data, Error>,
	data: &Data,
) -> Result<(), Error> {
	match event {
		FullEvent::Ready {
			ctx,
			data_about_bot: ready,
		} => {
			info!(user = ?ready.user.tag(), guilds = ready.guilds.len(), "logged in");

			data.guilds
				.write()
				.await
				.extend(ready.guilds.iter().map(|g| g.id.0.get()));

			ctx.set_activity(Some(serenity::ActivityData {
				name: format!("Shard #{} | v{VERSION}", ctx.shard_id.0 + 1),
				kind: serenity::ActivityType::Watching,
				url: None,
			}));
		}
		FullEvent::InteractionCreate {
			ctx,
			interaction: Interaction::Component(interaction),
		} => {
			let serenity::ComponentInteractionDataKind::StringSelect { ref values } =
				interaction.data.kind
			else {
				return Ok(());
			};

			let Some(id) = values.get(0).and_then(|p| api::id::decode(p)) else {
				return Ok(());
			};

			let ctx = context::Context::from_component(ctx, data, interaction);

			if let Err(e) = crate::id::map(&ctx, id).await {
				util::error(&ctx, e).await;
			};
		}
		FullEvent::GuildCreate { guild, .. } => {
			if data.guilds.write().await.insert(guild.id.0.get()) && tracing::enabled!(Level::INFO)
			{
				let guilds = data.guilds.read().await.len();

				info!(guilds = guilds, "guild count");
			}
		}
		FullEvent::GuildDelete {
			incomplete: guild, ..
		} => {
			if data.guilds.write().await.remove(&guild.id.0.get()) && tracing::enabled!(Level::INFO)
			{
				let guilds = data.guilds.read().await.len();

				info!(guilds = guilds, "guild count");
			}
		}
		_ => {}
	}

	Ok(())
}
