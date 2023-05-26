use std::borrow::Cow;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use api::canvas::chart;
use chrono::{DateTime, Utc};
use database::schema::snapshot;
use diesel::{ExpressionMethods, QueryDsl};
use minecraft::{paint::Paint, text::Text};
use poise::serenity_prelude::AttachmentType;
use translate::{Context, Error};

macro_rules! generate_large_command {
	($game: ty, $mode: ty, $fn: ident) => {
		async fn autocomplete_mode<'a>(
			ctx: $crate::Context<'a>,
			partial: &'a str,
		) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
			let partial = partial.to_ascii_lowercase();

			<$game>::autocomplete(ctx, partial).await
		}

		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			ctx.defer().await?;

			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let (_format, player, session) = $crate::get_history_data!(ctx, uuid, username);

			player.increase_searches(ctx).await?;

			let snapshots = diesel_async::RunQueryDsl::get_results::<(DateTime<Utc>, Vec<u8>)>(
				snapshot::table
					.filter(snapshot::uuid.eq(player.uuid))
					.order(snapshot::created_at.asc())
					.select((snapshot::created_at, snapshot::data)),
				&mut ctx.data().pool.get().await?,
			)
			.await?;

			let snapshots = {
				let mut snapshots_ = Vec::with_capacity(snapshots.len());

				for (time, data) in snapshots {
					let data = crate::snapshot::user::decode(&data)?;

					snapshots_.push((time, data));
				}

				snapshots_
			};

			let png = {
				let buffer = <$game>::chart(ctx, snapshots, &session, mode)?;

				Cow::Owned(buffer)
			};

			ctx.send(move |m| {
				m.attachment(AttachmentType::Bytes {
					data: png,
					filename: "canvas.png".to_string(),
				})
			})
			.await?;

			Ok(())
		}
	};
}

macro_rules! generate_command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			mode: Option<$mode>,
		) -> ::std::result::Result<(), ::translate::Error> {
			ctx.defer().await?;

			let (_format, player, session) = $crate::get_history_data!(ctx, uuid, username);

			player.increase_searches(ctx).await?;

			let snapshots = diesel_async::RunQueryDsl::get_results::<(DateTime<Utc>, Vec<u8>)>(
				snapshot::table
					.filter(snapshot::uuid.eq(player.uuid))
					.order(snapshot::created_at.asc())
					.select((snapshot::created_at, snapshot::data)),
				&mut ctx.data().pool.get().await?,
			)
			.await?;

			let snapshots = {
				let mut snapshots_ = Vec::with_capacity(snapshots.len());

				for (time, data) in snapshots {
					let data = crate::snapshot::user::decode(&data)?;

					snapshots_.push((time, data));
				}

				snapshots_
			};

			if snapshots.is_empty() {
				return Err(::translate::Error::Custom(format!("`{}` has no snapshots. You can create one with </daily bedwars:1107131762062135366>.", player.username)));
			}

			let png = {
				let buffer = <$game>::chart(ctx, snapshots, &session, mode)?;

				Cow::Owned(buffer)
			};

			ctx.send(move |m| {
				m.attachment(AttachmentType::Bytes {
					data: png,
					filename: "canvas.png".to_string(),
				})
			})
			.await?;

			Ok(())
		}
	};
}

/// Shows the network XP history of a player.
#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
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

	let player = crate::util::get_player_from_input(ctx, uuid, username).await?;

	let snapshots = diesel_async::RunQueryDsl::get_results::<(DateTime<Utc>, Vec<u8>)>(
		snapshot::table
			.filter(snapshot::uuid.eq(player.uuid))
			.order(snapshot::created_at.asc())
			.select((snapshot::created_at, snapshot::data)),
		&mut ctx.data().pool.get().await?,
	)
	.await?;

	let snapshots = {
		let mut snapshots_ = Vec::with_capacity(snapshots.len());

		for (time, data) in snapshots {
			let data = crate::snapshot::user::decode(&data)?;

			snapshots_.push((time, data));
		}

		snapshots_
	};

	let (Some(first), Some(last)) = (snapshots.first(), snapshots.last()) else {
		return Err(::translate::Error::Custom(format!("`{}` has no snapshots. You can create one with </daily bedwars:1107131762062135366>.", player.username)));
	};

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

		Cow::Owned(
			surface
				.image_snapshot()
				.encode_to_data(skia_safe::EncodedImageFormat::PNG)
				.unwrap()
				.to_vec(),
		)
	};

	ctx.send(move |m| {
		m.attachment(AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".to_string(),
		})
	})
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
