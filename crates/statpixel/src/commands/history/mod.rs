pub mod image;
pub mod run;

use std::borrow::Cow;

use api::player::stats;

use api::canvas::{self, chart};
use chrono::{DateTime, Utc};
use database::schema::snapshot;
use diesel::{ExpressionMethods, QueryDsl};
use minecraft::{paint::Paint, text::Text};
use poise::serenity_prelude::CreateAttachment;
use translate::{context, Context, Error};

use crate::util;

macro_rules! large_command {
	($game: ty, $mode: ty, $fn: ident) => {
		pub mod $fn {
			use super::*;

			async fn autocomplete_mode<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl Iterator<Item = poise::serenity_prelude::AutocompleteChoice<'a>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete(ctx, partial).await
			}

			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = "history"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				player: Option<String>,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
				let uuid = util::parse_uuid(player.as_deref());
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, player, uuid, mode).await
			}
		}
	};
}

macro_rules! command {
	($game: ty, $mode: ty, $fn: ident) => {
		pub mod $fn {
			use super::*;

			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = "history"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				player: Option<String>,
				mode: Option<$mode>,
			) -> Result<(), Error> {
				let uuid = util::parse_uuid(player.as_deref());
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, player, uuid, mode).await
			}
		}
	};
}

/// Shows the network XP history of a player.
#[allow(clippy::too_many_lines)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES",
	rename = "history"
)]
pub async fn network(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> ::std::result::Result<(), ::translate::Error> {
	ctx.defer().await?;

	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);

	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let player = crate::util::get_player_from_input(ctx, uuid, player).await?;

	let snapshots = diesel_async::RunQueryDsl::get_results::<(DateTime<Utc>, Vec<u8>)>(
		snapshot::table
			.filter(snapshot::uuid.eq(player.uuid))
			.order(snapshot::created_at.asc())
			.select((snapshot::created_at, snapshot::data)),
		&mut ctx.connection().await?,
	)
	.await?;

	if snapshots.is_empty() {
		let data = player.get_data(ctx).await?;

		crate::snapshot::user::insert(ctx, &player, &data).await?;

		let content = ::translate::tr_fmt!(
			ctx, "no-previous-statistics",
			name: data.username.as_str(),
		);

		ctx.send(poise::CreateReply::new().content(content)).await?;

		return Ok(());
	}

	let snapshots = {
		let mut snapshots_ = Vec::with_capacity(snapshots.len());

		for (time, data) in snapshots {
			let data = api::snapshot::user::decode(&data)?;

			snapshots_.push((time, data));
		}

		snapshots_
	};

	let first = snapshots.first().unwrap();
	let last = snapshots.last().unwrap();

	let lower = first.1.xp * 15 / 16;
	let upper = last.1.xp * 16 / 15;

	let rank = last.1.get_rank();
	let colour = {
		let colour = rank.get_username_paint();

		if colour == Paint::Gray {
			Paint::White
		} else {
			colour
		}
	};

	let png = {
		let mut buffer = chart::u64::create::<true>(
			ctx,
			family,
			vec![(
				Cow::Borrowed("Network XP"),
				snapshots
					.iter()
					.map(|(time, data)| (*time, data.xp))
					.collect::<Vec<_>>(),
			)],
			first.0..last.0,
			lower..upper,
			Some(colour),
			background,
		)?;
		let mut surface = chart::canvas(&mut buffer)?;

		chart::apply_title(
			ctx,
			family,
			&mut surface,
			&last.1,
			&[Text {
				text: "Network XP",
				paint: Paint::Gold,
				..Default::default()
			}],
			background,
		);
		chart::round_corners(&mut surface);

		Cow::Owned(canvas::to_png(&mut surface))
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
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
command!(
	stats::fishing::Fishing,
	stats::fishing::FishingMode,
	fishing
);
