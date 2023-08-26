pub mod run;

use api::player::stats;
use database::schema::session;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;
use translate::{context, tr, tr_fmt};

use crate::{
	snapshot,
	util::{self, success_embed},
	Context, Error,
};

/// Creates a session for a Minecraft account.
#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn create(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> Result<(), Error> {
	let ctx = &context::Context::from_poise(&ctx);
	let uuid = util::parse_uuid(player.as_deref());

	let (player, data) = crate::commands::get_player_data(ctx, uuid, player).await?;

	player.increase_searches(ctx).await?;

	let id = snapshot::user::insert_with_session(ctx, &player, &data).await?;

	ctx.send(
		success_embed(
			tr(ctx, "session-created-title"),
			tr_fmt!(ctx, "session-created", username: data.username.as_str(), id: id.to_string()),
		)
		.content(crate::tip::random(ctx)),
	)
	.await?;

	Ok(())
}

/// Deletes a session for a Minecraft account.
#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn delete(
	ctx: Context<'_>,
	#[max_length = 36]
	#[min_length = 32]
	id: String,
) -> Result<(), Error> {
	let id = util::parse_uuid(Some(&id)).ok_or_else(|| Error::InvalidUuid(id))?;

	let result = diesel::delete(
		session::table
			.filter(session::id.eq(id))
			.filter(session::user_id.eq(ctx.author().id.0.get() as i64)),
	)
	.execute(&mut ctx.data().pool.get().await?)
	.await?;

	let ctx = &context::Context::from_poise(&ctx);

	if result == 0 {
		return Err(Error::SessionNotFound);
	}

	ctx.send(
		poise::CreateReply::new().content(tr_fmt!(ctx, "session-deleted", id: id.to_string())),
	)
	.await?;

	Ok(())
}

/// Lists all sessions for a Minecraft account.
#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn list(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
	page: Option<u32>,
) -> Result<(), Error> {
	let ctx = &context::Context::from_poise(&ctx);
	let page = page.map(|p| p.saturating_sub(1)).unwrap_or_default();

	let player = if let Some(player) = player {
		Some(util::get_player_from_input(ctx, util::parse_uuid(Some(&player)), Some(player)).await?)
	} else {
		None
	};

	run::list(ctx, player, page).await
}

#[allow(clippy::unused_async)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	subcommands("create", "delete", "list")
)]
pub async fn session(_: Context<'_>) -> Result<(), Error> {
	Ok(())
}

macro_rules! command {
	($game: ty, $mode: ty, $fn: ident) => {
		pub mod $fn {
			use super::*;

			#[allow(clippy::too_many_lines)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = "session"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[min_length = 32]
				#[max_length = 36]
				session: ::std::string::String,
				mode: Option<$mode>,
			) -> Result<(), ::translate::Error> {
				let ctx = &context::Context::from_poise(&ctx);
				let uuid = util::parse_uuid(Some(session.as_str()))
					.ok_or_else(|| ::translate::Error::InvalidUuid(session))?;

				let Some(player_uuid) = session::table
					.filter(session::id.eq(uuid))
					.select(session::uuid)
					.get_result::<uuid::Uuid>(&mut ctx.data().pool.get().await?)
					.await
					.optional()?
				else {
					return Err(Error::SessionNotFound);
				};

				crate::commands::snapshot::session::command::<$game>(ctx, uuid, player_uuid, mode)
					.await
			}
		}
	};
}

macro_rules! large_command {
	($game: ty, $mode: ty, $fn: ident) => {
		pub mod $fn {
			use super::*;

			async fn autocomplete_mode<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete(ctx, partial).await
			}

			#[allow(clippy::too_many_lines)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = "session"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[min_length = 32]
				#[max_length = 36]
				session: ::std::string::String,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
				let ctx = &context::Context::from_poise(&ctx);
				let uuid = util::parse_uuid(Some(session.as_str()))
					.ok_or_else(|| ::translate::Error::InvalidUuid(session))?;

				let Some(player_uuid) = session::table
					.filter(session::id.eq(uuid))
					.select(session::uuid)
					.get_result::<uuid::Uuid>(&mut ctx.data().pool.get().await?)
					.await
					.optional()?
				else {
					return Err(Error::SessionNotFound);
				};

				crate::commands::snapshot::session::command::<$game>(ctx, uuid, player_uuid, mode)
					.await
			}
		}
	};
}

large_command!(stats::arcade::Arcade, stats::arcade::ArcadeMode, arcade);
command!(stats::arena::Arena, stats::arena::ArenaMode, arena);
large_command!(
	stats::bed_wars::BedWars,
	stats::bed_wars::BedWarsMode,
	bedwars
);
large_command!(
	stats::blitz_sg::BlitzSg,
	stats::blitz_sg::BlitzSgMode,
	blitz
);
command!(
	stats::build_battle::BuildBattle,
	stats::build_battle::BuildBattleMode,
	buildbattle
);
command!(
	stats::cops_and_crims::CopsAndCrims,
	stats::cops_and_crims::CopsAndCrimsMode,
	copsandcrims
);
large_command!(stats::duels::Duels, stats::duels::DuelsMode, duels);
command!(
	stats::mega_walls::MegaWalls,
	stats::mega_walls::MegaWallsMode,
	megawalls
);
command!(
	stats::murder_mystery::MurderMystery,
	stats::murder_mystery::MurderMysteryMode,
	murdermystery
);
command!(
	stats::paintball::Paintball,
	stats::paintball::PaintballMode,
	paintball
);
command!(stats::pit::Pit, stats::pit::PitMode, pit);
command!(stats::quake::Quake, stats::quake::QuakeMode, quake);
command!(
	stats::sky_wars::SkyWars,
	stats::sky_wars::SkyWarsMode,
	skywars
);
command!(
	stats::smash_heroes::SmashHeroes,
	stats::smash_heroes::SmashHeroesMode,
	smash
);
command!(
	stats::speed_uhc::SpeedUhc,
	stats::speed_uhc::SpeedUhcMode,
	speeduhc
);
command!(
	stats::tnt_games::TntGames,
	stats::tnt_games::TntGamesMode,
	tntgames
);
command!(
	stats::turbo_kart_racers::TurboKartRacers,
	stats::turbo_kart_racers::TurboKartRacersMode,
	turbokartracers
);
command!(stats::uhc::Uhc, stats::uhc::UhcMode, uhc);
command!(
	stats::vampire_z::VampireZ,
	stats::vampire_z::VampireZMode,
	vampirez
);
command!(stats::walls::Walls, stats::walls::WallsMode, walls);
command!(
	stats::warlords::Warlords,
	stats::warlords::WarlordsMode,
	warlords
);
command!(
	stats::wool_wars::WoolWars,
	stats::wool_wars::WoolWarsMode,
	woolwars
);
