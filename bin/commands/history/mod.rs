pub mod run;

use std::borrow::Cow;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use api::canvas::{self, chart};
use chrono::{DateTime, Utc};
use database::schema::snapshot;
use diesel::{ExpressionMethods, QueryDsl};
use minecraft::{paint::Paint, text::Text};
use poise::serenity_prelude::CreateAttachment;
use translate::{context, Context, Error};

use crate::util;

macro_rules! generate_large_command {
	($game: ty, $mode: ty, $fn: ident) => {
		async fn autocomplete_mode<'a>(
			ctx: $crate::Context<'a>,
			partial: &'a str,
		) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
			let partial = partial.to_ascii_lowercase();

			<$game>::autocomplete(ctx, partial).await
		}

		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES"
		)]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<String>,
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let uuid = util::parse_uuid(uuid)?;
			let ctx = &context::Context::from_poise(&ctx);

			run::command::<$game>(ctx, username, uuid, mode).await
		}
	};
}

macro_rules! generate_command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES"
		)]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<String>,
			mode: Option<$mode>,
		) -> Result<(), Error> {
			let uuid = util::parse_uuid(uuid)?;
			let ctx = &context::Context::from_poise(&ctx);

			run::command::<$game>(ctx, username, uuid, mode).await
		}
	};
}

/// Shows the network XP history of a player.
#[allow(clippy::too_many_lines)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
async fn network(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<::std::string::String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<::std::string::String>,
) -> ::std::result::Result<(), ::translate::Error> {
	ctx.defer().await?;

	let uuid = util::parse_uuid(uuid)?;
	let ctx = &context::Context::from_poise(&ctx);
	let player = crate::util::get_player_from_input(ctx, uuid, username).await?;

	let snapshots = diesel_async::RunQueryDsl::get_results::<(DateTime<Utc>, Vec<u8>)>(
		snapshot::table
			.filter(snapshot::uuid.eq(player.uuid))
			.order(snapshot::created_at.asc())
			.select((snapshot::created_at, snapshot::data)),
		&mut ctx.data().pool.get().await?,
	)
	.await?;

	if snapshots.is_empty() {
		let data = player.get_data().await?;

		crate::snapshot::user::insert(ctx, &player, &data).await?;

		let content = ::translate::tr_fmt!(
			ctx, "no-previous-statistics",
			name: crate::util::escape_username(&data.username),
		);

		ctx.send(poise::CreateReply::new().content(content)).await?;

		return Ok(());
	}

	let snapshots = {
		let mut snapshots_ = Vec::with_capacity(snapshots.len());

		for (time, data) in snapshots {
			let data = crate::snapshot::user::decode(&data)?;

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
		let mut buffer = chart::u64::create(
			ctx,
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
		)?;
		let mut surface = chart::canvas(&mut buffer)?;

		chart::apply_title(
			ctx,
			&mut surface,
			&last.1,
			&[Text {
				text: "Network XP",
				paint: Paint::Gold,
				..Default::default()
			}],
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

generate_command!(arcade::Arcade, arcade::ArcadeMode, arcade);
generate_command!(arena::Arena, arena::ArenaMode, arena);
generate_command!(bed_wars::BedWars, bed_wars::BedWarsMode, bedwars);
generate_command!(blitz_sg::BlitzSg, blitz_sg::BlitzSgMode, blitz);
generate_command!(
	build_battle::BuildBattle,
	build_battle::BuildBattleMode,
	buildbattle
);
generate_command!(
	cops_and_crims::CopsAndCrims,
	cops_and_crims::CopsAndCrimsMode,
	copsandcrims
);
generate_large_command!(duels::Duels, duels::DuelsMode, duels);
generate_command!(mega_walls::MegaWalls, mega_walls::MegaWallsMode, megawalls);
generate_command!(
	murder_mystery::MurderMystery,
	murder_mystery::MurderMysteryMode,
	murdermystery
);
generate_command!(paintball::Paintball, paintball::PaintballMode, paintball);
generate_command!(pit::Pit, pit::PitMode, pit);
generate_command!(quake::Quake, quake::QuakeMode, quake);
generate_command!(sky_wars::SkyWars, sky_wars::SkyWarsMode, skywars);
generate_command!(
	smash_heroes::SmashHeroes,
	smash_heroes::SmashHeroesMode,
	smash
);
generate_command!(speed_uhc::SpeedUhc, speed_uhc::SpeedUhcMode, speeduhc);
generate_command!(tnt_games::TntGames, tnt_games::TntGamesMode, tntgames);
generate_command!(
	turbo_kart_racers::TurboKartRacers,
	turbo_kart_racers::TurboKartRacersMode,
	turbokartracers
);
generate_command!(uhc::Uhc, uhc::UhcMode, uhc);
generate_command!(vampire_z::VampireZ, vampire_z::VampireZMode, vampirez);
generate_command!(walls::Walls, walls::WallsMode, walls);
generate_command!(warlords::Warlords, warlords::WarlordsMode, warlords);
generate_command!(wool_wars::WoolWars, wool_wars::WoolWarsMode, woolwars);

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	subcommands(
		"arcade",
		"arena",
		"bedwars",
		"blitz",
		"buildbattle",
		"copsandcrims",
		"duels",
		"megawalls",
		"murdermystery",
		"paintball",
		"pit",
		"quake",
		"skywars",
		"smash",
		"speeduhc",
		"tntgames",
		"turbokartracers",
		"uhc",
		"vampirez",
		"walls",
		"warlords",
		"woolwars",
		"network"
	)
)]
#[allow(clippy::unused_async)]
pub async fn history(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
