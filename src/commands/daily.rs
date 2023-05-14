#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use crate::snapshot::{get_or_insert_snapshot, SnapshotStatus};
use translate::{Context, Error};

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
			let (player, mut data, session) = $crate::get_data!(ctx, uuid, username);
			let status =
				get_or_insert_snapshot(ctx, &player, &data, ::chrono::Utc::now() - ::chrono::Duration::days(1))?;

			let png: ::std::option::Option<::std::borrow::Cow<[u8]>> =
				if let SnapshotStatus::Found((ref snapshot, _)) = status {
					let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, mode);

					::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
				} else {
					::std::option::Option::None
				};

			let content = match status {
				SnapshotStatus::Found((_, created_at)) => format!(
					"Showing statistics change from <t:{}:f> to <t:{}:f>",
					created_at.timestamp(),
					::chrono::Utc::now().timestamp(),
				),
				SnapshotStatus::Inserted => format!(
					"No previous data found for **{}**, so it has been inserted.\nShowing statistics change from <t:{}:f> to <t:{}:f>",
					$crate::util::escape_username(&player.username),
					::chrono::Utc::now().timestamp(),
					::chrono::Utc::now().timestamp(),
				),
			};

			ctx.send(move |m| {
				if let ::std::option::Option::Some(png) = png {
					m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
						data: png,
						filename: "canvas.png".into(),
					});
				}

				m.content(content)
			})
			.await?;

			Ok(())
		}
	};
}

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
			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let (player, mut data, session) = $crate::get_data!(ctx, uuid, username);
			let status =
				get_or_insert_snapshot(ctx, &player, &data, ::chrono::Utc::now() - ::chrono::Duration::days(1))?;

			let png: ::std::option::Option<::std::borrow::Cow<[u8]>> =
				if let SnapshotStatus::Found((ref snapshot, _)) = status {
					let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, mode);

					::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
				} else {
					::std::option::Option::None
				};

			let content = match status {
				SnapshotStatus::Found((_, created_at)) => format!(
					"Showing statistics change from <t:{}:f> to <t:{}:f>",
					created_at.timestamp(),
					::chrono::Utc::now().timestamp(),
				),
				SnapshotStatus::Inserted => format!(
					"No previous data found for **{}**, so it has been inserted.\nShowing statistics change from <t:{}:f> to <t:{}:f>",
					$crate::util::escape_username(&player.username),
					::chrono::Utc::now().timestamp(),
					::chrono::Utc::now().timestamp(),
				),
			};

			ctx.send(move |m| {
				if let ::std::option::Option::Some(png) = png {
					m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
						data: png,
						filename: "canvas.png".into(),
					});
				}

				m.content(content)
			})
			.await?;

			Ok(())
		}
	};
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
	turkokartracers
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
		"turkokartracers",
		"uhc",
		"vampirez",
		"walls",
		"warlords",
		"woolwars"
	)
)]
#[allow(clippy::unused_async)]
pub async fn daily(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
