use std::borrow::Cow;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use api::command::{GuildMode, Id, Mode, ProjectMode, SkyBlockMode};
use chrono::{Duration, Utc};
use skia_safe::Color;
use translate::context::Context;

macro_rules! impl_root {
	($ctx: expr, $uuid: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let (_, data, session, skin, suffix) =
			$crate::commands::get_player_data_session_skin_suffix($ctx, Some($uuid), None)
				.await
				.ok()?;

		Some(
			$crate::commands::games::image::command::<$game>(
				$ctx,
				Some($mode),
				$bg,
				&data,
				&session,
				skin.image(),
				suffix.as_deref(),
			)
			.0,
		)
	}};
}

macro_rules! impl_history {
	($ctx: expr, $uuid: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let (player, session) =
			$crate::commands::get_player_username_session($ctx, Some($uuid), None)
				.await
				.ok()?;

		Some(
			$crate::commands::history::image::command::<$game>(
				$ctx,
				Some($mode),
				&player,
				&session,
				$bg,
			)
			.await
			.ok()??
			.0,
		)
	}};
}

macro_rules! impl_compare {
	($ctx: expr, $from: expr, $to: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let (_, data, session, skin, suffix) =
			$crate::commands::get_player_data_session_skin_suffix($ctx, Some($from), None)
				.await
				.ok()?;

		let (_, data_to) = $crate::commands::get_player_data($ctx, Some($to), None)
			.await
			.ok()?;

		let (mut surface, _) = <$game>::canvas_diff(
			$ctx,
			&data_to,
			&mut api::Data::clone(&data),
			&session,
			skin.image(),
			Some($mode),
			suffix.as_deref(),
			$bg,
		);

		Some(api::canvas::to_png(&mut surface).into())
	}};
}

macro_rules! impl_snapshot {
	($ctx: expr, $uuid: expr, $from: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let after = Utc::now() - $from;
		let (player, data, session, skin, suffix) =
			$crate::commands::get_player_data_session_skin_suffix($ctx, Some($uuid), None)
				.await
				.ok()?;
		let (snapshot, _) = $crate::snapshot::user::get_or_insert($ctx, &player, &data, after)
			.await
			.ok()?
			.ok()?;

		let (mut surface, _) = <$game>::canvas_diff(
			$ctx,
			&snapshot,
			&mut api::Data::clone(&data),
			&session,
			skin.image(),
			Some($mode),
			suffix.as_deref(),
			$bg,
		);

		Some(api::canvas::to_png(&mut surface).into())
	}};
}

macro_rules! impl_project {
	($ctx: expr, $uuid: expr, $mode: expr, $kind: expr, $game: ty, $bg: expr) => {{
		let (player, session) = $crate::commands::get_player_session($ctx, Some($uuid), None)
			.await
			.ok()?;

		Some(
			$crate::commands::project::image::command::<$game>(
				$ctx,
				&player,
				&session,
				$bg,
				Some($mode),
				Some($kind),
				None,
			)
			.await
			.ok()??
			.0,
		)
	}};
}

/// Dispatches a command based on a deserialized `custom_id`
#[allow(clippy::too_many_lines)]
pub async fn map(
	ctx: &Context<'_>,
	id: Id,
	background: Option<Color>,
) -> Option<Cow<'static, [u8]>> {
	match id {
		Id::Builder { shapes, uuid, .. } => {
			let (_, data, session, skin, _) =
				crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
					.await
					.ok()?;
			let bytes = crate::commands::builder::build::build(
				ctx, &shapes, &data, &session, &skin, background,
			);

			Some(bytes)
		}
		Id::Root { kind, uuid, .. } => match kind {
			Mode::Arcade(mode) => impl_root!(ctx, uuid, mode, arcade::Arcade, background),
			Mode::Arena(mode) => impl_root!(ctx, uuid, mode, arena::Arena, background),
			Mode::BedWars(mode) => impl_root!(ctx, uuid, mode, bed_wars::BedWars, background),
			Mode::BlitzSg(mode) => impl_root!(ctx, uuid, mode, blitz_sg::BlitzSg, background),
			Mode::BuildBattle(mode) => {
				impl_root!(ctx, uuid, mode, build_battle::BuildBattle, background)
			}
			Mode::CopsAndCrims(mode) => {
				impl_root!(ctx, uuid, mode, cops_and_crims::CopsAndCrims, background)
			}
			Mode::Duels(mode) => impl_root!(ctx, uuid, mode, duels::Duels, background),
			Mode::MegaWalls(mode) => impl_root!(ctx, uuid, mode, mega_walls::MegaWalls, background),
			Mode::MurderMystery(mode) => {
				impl_root!(ctx, uuid, mode, murder_mystery::MurderMystery, background)
			}
			Mode::Paintball(mode) => impl_root!(ctx, uuid, mode, paintball::Paintball, background),
			Mode::Pit(mode) => impl_root!(ctx, uuid, mode, pit::Pit, background),
			Mode::Quake(mode) => impl_root!(ctx, uuid, mode, quake::Quake, background),
			Mode::SkyWars(mode) => impl_root!(ctx, uuid, mode, sky_wars::SkyWars, background),
			Mode::SmashHeroes(mode) => {
				impl_root!(ctx, uuid, mode, smash_heroes::SmashHeroes, background)
			}
			Mode::SpeedUhc(mode) => impl_root!(ctx, uuid, mode, speed_uhc::SpeedUhc, background),
			Mode::TntGames(mode) => impl_root!(ctx, uuid, mode, tnt_games::TntGames, background),
			Mode::TurboKartRacers(mode) => {
				impl_root!(
					ctx,
					uuid,
					mode,
					turbo_kart_racers::TurboKartRacers,
					background
				)
			}
			Mode::Uhc(mode) => impl_root!(ctx, uuid, mode, uhc::Uhc, background),
			Mode::VampireZ(mode) => impl_root!(ctx, uuid, mode, vampire_z::VampireZ, background),
			Mode::Walls(mode) => impl_root!(ctx, uuid, mode, walls::Walls, background),
			Mode::Warlords(mode) => impl_root!(ctx, uuid, mode, warlords::Warlords, background),
			Mode::WoolWars(mode) => impl_root!(ctx, uuid, mode, wool_wars::WoolWars, background),
			Mode::Guild(mode, limit, nanos, member_id) => match mode {
				GuildMode::General => {
					let guild = crate::commands::get_guild(ctx, None, member_id, None, Some(uuid))
						.await
						.ok()?;

					crate::commands::guild::image::general(ctx, &guild, background)
						.await
						.ok()
				}
				GuildMode::Member => {
					let (guild, player) =
						crate::commands::get_guild_with_member(ctx, member_id, None)
							.await
							.ok()?;

					crate::commands::guild::image::member(ctx, &guild, &player, background)
						.await
						.ok()
				}
				GuildMode::Members => {
					let guild = crate::commands::get_guild(ctx, None, member_id, None, Some(uuid))
						.await
						.ok()?;

					crate::commands::guild::image::members(ctx, &guild, background)
						.await
						.ok()
				}
				GuildMode::Top => {
					let guild = crate::commands::get_guild(ctx, None, member_id, None, Some(uuid))
						.await
						.ok()?;

					crate::commands::guild::image::top(
						ctx,
						&guild,
						limit.unwrap_or(30),
						Utc::now() - nanos.map_or(Duration::days(30), Duration::nanoseconds),
						background,
					)
					.await
					.ok()
				}
			},
			Mode::SkyBlock(mode, profile) => match mode {
				SkyBlockMode::Auctions => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					crate::commands::skyblock::image::auctions(
						ctx,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
					)
					.await
					.ok()
				}
				SkyBlockMode::Bank => {
					let (_, data) = crate::commands::get_player_data(ctx, Some(uuid), None)
						.await
						.ok()?;

					Some(
						crate::commands::skyblock::image::bank(
							ctx, &data, background, profile, None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Candy => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::candy(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::EnderChest => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::enderchest(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Equipment => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::equipment(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Fishing => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::fishing(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Inventory => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::inventory(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Networth => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::networth(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Pets => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::pets(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Potions => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::potions(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Profile => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::profile(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Quiver => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::quiver(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Talisman => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::talisman(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Vault => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::vault(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
				SkyBlockMode::Wardrobe => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await
							.ok()?;

					Some(
						crate::commands::skyblock::image::wardrobe(
							ctx,
							&player,
							&data,
							&session,
							skin.image(),
							suffix.as_deref(),
							background,
							profile,
							None,
						)
						.await
						.ok()?
						.0,
					)
				}
			},
			Mode::Network => {
				let (player, data, guild, session, skin, suffix) =
					crate::commands::get_player_data_guild_session_skin_suffix(
						ctx,
						Some(uuid),
						None,
					)
					.await
					.ok()?;

				Some(crate::commands::network::image::network(
					ctx,
					&player,
					guild.as_deref(),
					&data,
					&session,
					suffix.as_deref(),
					skin.image(),
					background,
				))
			}
			Mode::RecentGames => {
				let (_, data, games, session, skin, suffix) =
					crate::commands::get_player_data_games_session_skin_suffix(
						ctx,
						Some(uuid),
						None,
					)
					.await
					.ok()?;

				Some(crate::commands::recent::image::recent(
					ctx,
					&data,
					&games,
					&session,
					skin.image(),
					suffix.as_deref(),
					background,
				))
			}
			Mode::Winstreaks => {
				let (_, data, session, skin, suffix) =
					crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
						.await
						.ok()?;

				Some(crate::commands::winstreaks::image::winstreaks(
					ctx,
					&data,
					&session,
					skin.image(),
					suffix.as_deref(),
					background,
				))
			}
		},
		Id::Snapshot {
			kind, uuid, past, ..
		} => {
			let past = Duration::nanoseconds(past);

			match kind {
				Mode::Arcade(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, arcade::Arcade, background)
				}
				Mode::Arena(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, arena::Arena, background)
				}
				Mode::BedWars(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, bed_wars::BedWars, background)
				}
				Mode::BlitzSg(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, blitz_sg::BlitzSg, background)
				}
				Mode::BuildBattle(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, build_battle::BuildBattle, background)
				}
				Mode::CopsAndCrims(mode) => {
					impl_snapshot!(
						ctx,
						uuid,
						past,
						mode,
						cops_and_crims::CopsAndCrims,
						background
					)
				}
				Mode::Duels(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, duels::Duels, background)
				}
				Mode::MegaWalls(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, mega_walls::MegaWalls, background)
				}
				Mode::MurderMystery(mode) => {
					impl_snapshot!(
						ctx,
						uuid,
						past,
						mode,
						murder_mystery::MurderMystery,
						background
					)
				}
				Mode::Paintball(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, paintball::Paintball, background)
				}
				Mode::Pit(mode) => impl_snapshot!(ctx, uuid, past, mode, pit::Pit, background),
				Mode::Quake(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, quake::Quake, background)
				}
				Mode::SkyWars(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, sky_wars::SkyWars, background)
				}
				Mode::SmashHeroes(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, smash_heroes::SmashHeroes, background)
				}
				Mode::SpeedUhc(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, speed_uhc::SpeedUhc, background)
				}
				Mode::TntGames(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, tnt_games::TntGames, background)
				}
				Mode::TurboKartRacers(mode) => {
					impl_snapshot!(
						ctx,
						uuid,
						past,
						mode,
						turbo_kart_racers::TurboKartRacers,
						background
					)
				}
				Mode::Uhc(mode) => impl_snapshot!(ctx, uuid, past, mode, uhc::Uhc, background),
				Mode::VampireZ(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, vampire_z::VampireZ, background)
				}
				Mode::Walls(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, walls::Walls, background)
				}
				Mode::Warlords(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, warlords::Warlords, background)
				}
				Mode::WoolWars(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, wool_wars::WoolWars, background)
				}
				_ => None,
			}
		}
		Id::History { kind, uuid, .. } => match kind {
			Mode::Arcade(mode) => impl_history!(ctx, uuid, mode, arcade::Arcade, background),
			Mode::Arena(mode) => impl_history!(ctx, uuid, mode, arena::Arena, background),
			Mode::BedWars(mode) => impl_history!(ctx, uuid, mode, bed_wars::BedWars, background),
			Mode::BlitzSg(mode) => impl_history!(ctx, uuid, mode, blitz_sg::BlitzSg, background),
			Mode::BuildBattle(mode) => {
				impl_history!(ctx, uuid, mode, build_battle::BuildBattle, background)
			}
			Mode::CopsAndCrims(mode) => {
				impl_history!(ctx, uuid, mode, cops_and_crims::CopsAndCrims, background)
			}
			Mode::Duels(mode) => impl_history!(ctx, uuid, mode, duels::Duels, background),
			Mode::MegaWalls(mode) => {
				impl_history!(ctx, uuid, mode, mega_walls::MegaWalls, background)
			}
			Mode::MurderMystery(mode) => {
				impl_history!(ctx, uuid, mode, murder_mystery::MurderMystery, background)
			}
			Mode::Paintball(mode) => {
				impl_history!(ctx, uuid, mode, paintball::Paintball, background)
			}
			Mode::Pit(mode) => impl_history!(ctx, uuid, mode, pit::Pit, background),
			Mode::Quake(mode) => impl_history!(ctx, uuid, mode, quake::Quake, background),
			Mode::SkyWars(mode) => impl_history!(ctx, uuid, mode, sky_wars::SkyWars, background),
			Mode::SmashHeroes(mode) => {
				impl_history!(ctx, uuid, mode, smash_heroes::SmashHeroes, background)
			}
			Mode::SpeedUhc(mode) => impl_history!(ctx, uuid, mode, speed_uhc::SpeedUhc, background),
			Mode::TntGames(mode) => impl_history!(ctx, uuid, mode, tnt_games::TntGames, background),
			Mode::TurboKartRacers(mode) => {
				impl_history!(
					ctx,
					uuid,
					mode,
					turbo_kart_racers::TurboKartRacers,
					background
				)
			}
			Mode::Uhc(mode) => impl_history!(ctx, uuid, mode, uhc::Uhc, background),
			Mode::VampireZ(mode) => impl_history!(ctx, uuid, mode, vampire_z::VampireZ, background),
			Mode::Walls(mode) => impl_history!(ctx, uuid, mode, walls::Walls, background),
			Mode::Warlords(mode) => impl_history!(ctx, uuid, mode, warlords::Warlords, background),
			Mode::WoolWars(mode) => impl_history!(ctx, uuid, mode, wool_wars::WoolWars, background),
			_ => None,
		},
		Id::Project { kind, uuid, .. } => match kind {
			ProjectMode::Arcade(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, arcade::Arcade, background)
			}
			ProjectMode::Arena(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, arena::Arena, background)
			}
			ProjectMode::BedWars(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, bed_wars::BedWars, background)
			}
			ProjectMode::BlitzSg(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, blitz_sg::BlitzSg, background)
			}
			ProjectMode::BuildBattle(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, build_battle::BuildBattle, background)
			}
			ProjectMode::CopsAndCrims(mode, kind) => {
				impl_project!(
					ctx,
					uuid,
					mode,
					kind,
					cops_and_crims::CopsAndCrims,
					background
				)
			}
			ProjectMode::Duels(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, duels::Duels, background)
			}
			ProjectMode::MegaWalls(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, mega_walls::MegaWalls, background)
			}
			ProjectMode::MurderMystery(mode, kind) => {
				impl_project!(
					ctx,
					uuid,
					mode,
					kind,
					murder_mystery::MurderMystery,
					background
				)
			}
			ProjectMode::Paintball(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, paintball::Paintball, background)
			}
			ProjectMode::Pit(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, pit::Pit, background)
			}
			ProjectMode::Quake(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, quake::Quake, background)
			}
			ProjectMode::SkyWars(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, sky_wars::SkyWars, background)
			}
			ProjectMode::SmashHeroes(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, smash_heroes::SmashHeroes, background)
			}
			ProjectMode::SpeedUhc(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, speed_uhc::SpeedUhc, background)
			}
			ProjectMode::TntGames(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, tnt_games::TntGames, background)
			}
			ProjectMode::TurboKartRacers(mode, kind) => {
				impl_project!(
					ctx,
					uuid,
					mode,
					kind,
					turbo_kart_racers::TurboKartRacers,
					background
				)
			}
			ProjectMode::Uhc(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, uhc::Uhc, background)
			}
			ProjectMode::VampireZ(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, vampire_z::VampireZ, background)
			}
			ProjectMode::Walls(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, walls::Walls, background)
			}
			ProjectMode::Warlords(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, warlords::Warlords, background)
			}
			ProjectMode::WoolWars(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, wool_wars::WoolWars, background)
			}
			_ => None,
		},
		Id::Compare { kind, from, to, .. } => match kind {
			Mode::Arcade(mode) => impl_compare!(ctx, from, to, mode, arcade::Arcade, background),
			Mode::Arena(mode) => impl_compare!(ctx, from, to, mode, arena::Arena, background),
			Mode::BedWars(mode) => {
				impl_compare!(ctx, from, to, mode, bed_wars::BedWars, background)
			}
			Mode::BlitzSg(mode) => {
				impl_compare!(ctx, from, to, mode, blitz_sg::BlitzSg, background)
			}
			Mode::BuildBattle(mode) => {
				impl_compare!(ctx, from, to, mode, build_battle::BuildBattle, background)
			}
			Mode::CopsAndCrims(mode) => {
				impl_compare!(
					ctx,
					from,
					to,
					mode,
					cops_and_crims::CopsAndCrims,
					background
				)
			}
			Mode::Duels(mode) => impl_compare!(ctx, from, to, mode, duels::Duels, background),
			Mode::MegaWalls(mode) => {
				impl_compare!(ctx, from, to, mode, mega_walls::MegaWalls, background)
			}
			Mode::MurderMystery(mode) => {
				impl_compare!(
					ctx,
					from,
					to,
					mode,
					murder_mystery::MurderMystery,
					background
				)
			}
			Mode::Paintball(mode) => {
				impl_compare!(ctx, from, to, mode, paintball::Paintball, background)
			}
			Mode::Pit(mode) => impl_compare!(ctx, from, to, mode, pit::Pit, background),
			Mode::Quake(mode) => impl_compare!(ctx, from, to, mode, quake::Quake, background),
			Mode::SkyWars(mode) => {
				impl_compare!(ctx, from, to, mode, sky_wars::SkyWars, background)
			}
			Mode::SmashHeroes(mode) => {
				impl_compare!(ctx, from, to, mode, smash_heroes::SmashHeroes, background)
			}
			Mode::SpeedUhc(mode) => {
				impl_compare!(ctx, from, to, mode, speed_uhc::SpeedUhc, background)
			}
			Mode::TntGames(mode) => {
				impl_compare!(ctx, from, to, mode, tnt_games::TntGames, background)
			}
			Mode::TurboKartRacers(mode) => {
				impl_compare!(
					ctx,
					from,
					to,
					mode,
					turbo_kart_racers::TurboKartRacers,
					background
				)
			}
			Mode::Uhc(mode) => impl_compare!(ctx, from, to, mode, uhc::Uhc, background),
			Mode::VampireZ(mode) => {
				impl_compare!(ctx, from, to, mode, vampire_z::VampireZ, background)
			}
			Mode::Walls(mode) => impl_compare!(ctx, from, to, mode, walls::Walls, background),
			Mode::Warlords(mode) => {
				impl_compare!(ctx, from, to, mode, warlords::Warlords, background)
			}
			Mode::WoolWars(mode) => {
				impl_compare!(ctx, from, to, mode, wool_wars::WoolWars, background)
			}
			_ => None,
		},
		Id::Between { .. } => None,
	}
}
