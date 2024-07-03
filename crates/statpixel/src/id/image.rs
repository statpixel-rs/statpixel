use std::borrow::Cow;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use api::{
	canvas::prelude::Game,
	command::{GuildMode, Id, Mode, ProjectMode, SkyBlockMode},
};
use chrono::{Duration, Utc};
use minecraft::style::Family;
use skia_safe::Color;
use translate::context::Context;

use crate::Error;

macro_rules! impl_root {
	($ctx: expr, $family: expr, $uuid: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let (_, data, session, skin, suffix) =
			$crate::commands::get_player_data_session_skin_suffix($ctx, Some($uuid), None).await?;

		Ok($crate::commands::games::image::command::<$game>(
			$ctx,
			$family,
			Some($mode),
			$bg,
			&data,
			&session,
			skin.image(),
			suffix.as_deref(),
		)
		.0)
	}};
}

macro_rules! impl_history {
	($ctx: expr, $family: expr, $uuid: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let (player, session) =
			$crate::commands::get_player_username_session($ctx, Some($uuid), None).await?;

		Ok($crate::commands::history::image::command::<$game>(
			$ctx,
			$family,
			Some($mode),
			&player,
			&session,
			$bg,
		)
		.await?
		.ok_or($crate::Error::NotImplemented)?
		.0)
	}};
}

macro_rules! impl_compare {
	($ctx: expr, $family: expr, $lhs: expr, $rhs: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let (_, data_lhs, session, skin, suffix) =
			$crate::commands::get_player_data_session_skin_suffix($ctx, Some($lhs), None).await?;

		let (_, data_rhs) = $crate::commands::get_player_data($ctx, Some($rhs), None).await?;

		let (mut surface, _) = <$game>::canvas_diff(
			$ctx,
			$family,
			&data_lhs,
			&data_rhs,
			&session,
			skin.image(),
			Some($mode),
			suffix.as_deref(),
			$bg,
			false,
		);

		Ok(api::canvas::to_png(&mut surface).into())
	}};
}

macro_rules! impl_snapshot {
	($ctx: expr, $family: expr, $uuid: expr, $from: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let after = Utc::now() - $from;
		let (player, data_rhs, session, skin, suffix) =
			$crate::commands::get_player_data_session_skin_suffix($ctx, Some($uuid), None).await?;
		let (data_lhs, _) = $crate::snapshot::user::get_or_insert($ctx, &player, &data_rhs, after)
			.await?
			.ok()
			.ok_or($crate::Error::NotImplemented)?;

		let (mut surface, _) = <$game>::canvas_diff(
			$ctx,
			$family,
			&data_lhs,
			&data_rhs,
			&session,
			skin.image(),
			Some($mode),
			suffix.as_deref(),
			$bg,
			false,
		);

		Ok(api::canvas::to_png(&mut surface).into())
	}};
}

macro_rules! impl_at {
	($ctx: expr, $family: expr, $uuid: expr, $from: expr, $mode: expr, $game: ty, $bg: expr) => {{
		let after = Utc::now() - $from;
		let (player, data_rhs, session, skin, suffix) =
			$crate::commands::get_player_data_session_skin_suffix($ctx, Some($uuid), None).await?;
		let (data_rhs, _) = $crate::snapshot::user::get_or_insert($ctx, &player, &data_rhs, after)
			.await?
			.ok()
			.ok_or($crate::Error::NotImplemented)?;

		let (mut surface, _) = <$game>::canvas(
			$ctx,
			$family,
			&data_rhs,
			&session,
			skin.image(),
			Some($mode),
			suffix.as_deref(),
			$bg,
		);

		Ok(api::canvas::to_png(&mut surface).into())
	}};
}

macro_rules! impl_project {
	($ctx: expr, $family: expr, $uuid: expr, $mode: expr, $kind: expr, $game: ty, $bg: expr) => {{
		let (player, session) =
			$crate::commands::get_player_session($ctx, Some($uuid), None).await?;

		Ok($crate::commands::project::image::command::<$game>(
			$ctx,
			$family,
			&player,
			&session,
			$bg,
			Some($mode),
			Some($kind),
			None,
		)
		.await?
		.ok_or($crate::Error::NotImplemented)?
		.0)
	}};
}

/// Dispatches a command based on a deserialized `custom_id`
#[allow(clippy::too_many_lines)]
pub async fn map(
	ctx: &Context<'_>,
	id: Id,
	background: Option<Color>,
	family: Family,
) -> Result<Cow<'static, [u8]>, Error> {
	match id {
		Id::Leaderboard {
			board,
			input,
			filter,
			order,
		} => {
			let leaderboard = crate::commands::leaderboard::LEADERBOARDS
				.get(board as usize)
				.ok_or(Error::NotImplemented)?;

			Ok(crate::commands::leaderboard::image::command(
				ctx,
				leaderboard,
				input,
				filter,
				order,
				family,
				background,
			)
			.await?
			.0)
		}
		Id::Between { .. } | Id::Session { .. } | Id::SessionPage { .. } => {
			Err(crate::Error::NotImplemented)
		}
		Id::Builder { shapes, uuid } => {
			let (_, data, session, skin, _) =
				crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None).await?;
			let bytes = crate::commands::builder::build::build(
				ctx, family, &shapes, &data, &session, &skin, background,
			)?;

			Ok(bytes)
		}
		Id::Root { kind, uuid } => match kind {
			Mode::Parkour => {
				let (_, data, session, skin, suffix) =
					crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
						.await?;
				let bytes = crate::commands::parkour::image::parkour(
					ctx,
					family,
					background,
					&data,
					&session,
					skin.image(),
					suffix.as_deref(),
				);

				Ok(bytes)
			}
			Mode::BedWarsPractice => {
				let (_, data, session, skin, suffix) =
					crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
						.await?;
				let bytes = crate::commands::bedwars::image::practice(
					ctx,
					family,
					background,
					&data,
					&session,
					skin.image(),
					suffix.as_deref(),
				);

				Ok(bytes)
			}
			Mode::BedWarsShop => {
				let (_, data, session, skin, suffix) =
					crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
						.await?;
				let bytes = crate::commands::bedwars::image::shop(
					ctx,
					family,
					background,
					&data,
					&session,
					skin.image(),
					suffix.as_deref(),
				);

				Ok(bytes)
			}
			Mode::BedWarsHotbar => {
				let (_, data, session, skin, suffix) =
					crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
						.await?;
				let bytes = crate::commands::bedwars::image::hotbar(
					ctx,
					family,
					background,
					&data,
					&session,
					skin.image(),
					suffix.as_deref(),
				);

				Ok(bytes)
			}
			Mode::Arcade(mode) => impl_root!(ctx, family, uuid, mode, arcade::Arcade, background),
			Mode::Arena(mode) => impl_root!(ctx, family, uuid, mode, arena::Arena, background),
			Mode::BedWars(mode) => {
				impl_root!(ctx, family, uuid, mode, bed_wars::BedWars, background)
			}
			Mode::BlitzSg(mode) => {
				impl_root!(ctx, family, uuid, mode, blitz_sg::BlitzSg, background)
			}
			Mode::BuildBattle(mode) => {
				impl_root!(
					ctx,
					family,
					uuid,
					mode,
					build_battle::BuildBattle,
					background
				)
			}
			Mode::CopsAndCrims(mode) => {
				impl_root!(
					ctx,
					family,
					uuid,
					mode,
					cops_and_crims::CopsAndCrims,
					background
				)
			}
			Mode::Duels(mode) => impl_root!(ctx, family, uuid, mode, duels::Duels, background),
			Mode::MegaWalls(mode) => {
				impl_root!(ctx, family, uuid, mode, mega_walls::MegaWalls, background)
			}
			Mode::MurderMystery(mode) => {
				impl_root!(
					ctx,
					family,
					uuid,
					mode,
					murder_mystery::MurderMystery,
					background
				)
			}
			Mode::Paintball(mode) => {
				impl_root!(ctx, family, uuid, mode, paintball::Paintball, background)
			}
			Mode::Pit(mode) => impl_root!(ctx, family, uuid, mode, pit::Pit, background),
			Mode::Quake(mode) => impl_root!(ctx, family, uuid, mode, quake::Quake, background),
			Mode::SkyWars(mode) => {
				impl_root!(ctx, family, uuid, mode, sky_wars::SkyWars, background)
			}
			Mode::SmashHeroes(mode) => {
				impl_root!(
					ctx,
					family,
					uuid,
					mode,
					smash_heroes::SmashHeroes,
					background
				)
			}
			Mode::SpeedUhc(mode) => {
				impl_root!(ctx, family, uuid, mode, speed_uhc::SpeedUhc, background)
			}
			Mode::TntGames(mode) => {
				impl_root!(ctx, family, uuid, mode, tnt_games::TntGames, background)
			}
			Mode::TurboKartRacers(mode) => {
				impl_root!(
					ctx,
					family,
					uuid,
					mode,
					turbo_kart_racers::TurboKartRacers,
					background
				)
			}
			Mode::Uhc(mode) => impl_root!(ctx, family, uuid, mode, uhc::Uhc, background),
			Mode::VampireZ(mode) => {
				impl_root!(ctx, family, uuid, mode, vampire_z::VampireZ, background)
			}
			Mode::Walls(mode) => impl_root!(ctx, family, uuid, mode, walls::Walls, background),
			Mode::Warlords(mode) => {
				impl_root!(ctx, family, uuid, mode, warlords::Warlords, background)
			}
			Mode::WoolWars(mode) => {
				impl_root!(ctx, family, uuid, mode, wool_wars::WoolWars, background)
			}
			Mode::Fishing(mode) => {
				impl_root!(ctx, family, uuid, mode, fishing::Fishing, background)
			}
			Mode::Guild(mode, limit, nanos, member_id) => match mode {
				GuildMode::General => {
					let guild =
						crate::commands::get_guild(ctx, None, member_id, None, Some(uuid)).await?;

					crate::commands::guild::image::general(ctx, family, &guild, background).await
				}
				GuildMode::Member => {
					let (guild, player) =
						crate::commands::get_guild_with_member(ctx, member_id, None).await?;

					crate::commands::guild::image::member(ctx, family, &guild, &player, background)
						.await
				}
				GuildMode::Members => {
					let guild =
						crate::commands::get_guild(ctx, None, member_id, None, Some(uuid)).await?;

					crate::commands::guild::image::members(ctx, family, &guild, background).await
				}
				GuildMode::Top => {
					let guild =
						crate::commands::get_guild(ctx, None, member_id, None, Some(uuid)).await?;

					crate::commands::guild::image::top(
						ctx,
						family,
						&guild,
						limit.unwrap_or(30),
						Utc::now()
							- nanos.map_or(Duration::try_days(30).unwrap(), Duration::nanoseconds),
						background,
					)
					.await
				}
			},
			Mode::SkyBlock(mode, profile) => match mode {
				SkyBlockMode::Auctions => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					crate::commands::skyblock::image::auctions(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
					)
					.await
				}
				SkyBlockMode::Bank => {
					let (_, data) = crate::commands::get_player_data(ctx, Some(uuid), None).await?;

					Ok(crate::commands::skyblock::image::bank(
						ctx, family, &data, background, profile, None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Candy => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::candy(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::EnderChest => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::enderchest(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Equipment => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::equipment(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Fishing => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::fishing(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Inventory => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::inventory(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Networth => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::networth(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Pets => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::pets(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Potions => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::potions(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Profile => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::profile(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Quiver => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::quiver(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Talisman => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::talisman(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Vault => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::vault(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
				SkyBlockMode::Wardrobe => {
					let (player, data, session, skin, suffix) =
						crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
							.await?;

					Ok(crate::commands::skyblock::image::wardrobe(
						ctx,
						family,
						&player,
						&data,
						&session,
						skin.image(),
						suffix.as_deref(),
						background,
						profile,
						None,
					)
					.await?
					.0)
				}
			},
			Mode::Network => {
				let (player, data, guild, session, skin, suffix) =
					crate::commands::get_player_data_guild_session_skin_suffix(
						ctx,
						Some(uuid),
						None,
					)
					.await?;

				Ok(crate::commands::network::image::network(
					ctx,
					family,
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
					.await?;

				Ok(crate::commands::recent::image::recent(
					ctx,
					family,
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
						.await?;

				Ok(crate::commands::winstreaks::image::winstreaks(
					ctx,
					family,
					&data,
					&session,
					skin.image(),
					suffix.as_deref(),
					background,
				))
			}
		},
		Id::Snapshot { kind, uuid, past } => {
			let past = Duration::nanoseconds(past);

			match kind {
				Mode::Arcade(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, arcade::Arcade, background)
				}
				Mode::Arena(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, arena::Arena, background)
				}
				Mode::BedWars(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, bed_wars::BedWars, background)
				}
				Mode::BlitzSg(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, blitz_sg::BlitzSg, background)
				}
				Mode::BuildBattle(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						build_battle::BuildBattle,
						background
					)
				}
				Mode::CopsAndCrims(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						cops_and_crims::CopsAndCrims,
						background
					)
				}
				Mode::Duels(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, duels::Duels, background)
				}
				Mode::MegaWalls(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						mega_walls::MegaWalls,
						background
					)
				}
				Mode::MurderMystery(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						murder_mystery::MurderMystery,
						background
					)
				}
				Mode::Paintball(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						paintball::Paintball,
						background
					)
				}
				Mode::Pit(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, pit::Pit, background)
				}
				Mode::Quake(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, quake::Quake, background)
				}
				Mode::SkyWars(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, sky_wars::SkyWars, background)
				}
				Mode::SmashHeroes(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						smash_heroes::SmashHeroes,
						background
					)
				}
				Mode::SpeedUhc(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						speed_uhc::SpeedUhc,
						background
					)
				}
				Mode::TntGames(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						tnt_games::TntGames,
						background
					)
				}
				Mode::TurboKartRacers(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						turbo_kart_racers::TurboKartRacers,
						background
					)
				}
				Mode::Uhc(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, uhc::Uhc, background)
				}
				Mode::VampireZ(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						vampire_z::VampireZ,
						background
					)
				}
				Mode::Walls(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, walls::Walls, background)
				}
				Mode::Warlords(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						warlords::Warlords,
						background
					)
				}
				Mode::WoolWars(mode) => {
					impl_snapshot!(
						ctx,
						family,
						uuid,
						past,
						mode,
						wool_wars::WoolWars,
						background
					)
				}
				Mode::Fishing(mode) => {
					impl_snapshot!(ctx, family, uuid, past, mode, fishing::Fishing, background)
				}
				Mode::RecentGames
				| Mode::Winstreaks
				| Mode::SkyBlock(..)
				| Mode::Guild(..)
				| Mode::Network
				| Mode::BedWarsShop
				| Mode::BedWarsPractice
				| Mode::BedWarsHotbar
				| Mode::Parkour => Err(crate::Error::NotImplemented),
			}
		}
		Id::At { kind, uuid, past } => {
			let past = Duration::nanoseconds(past);

			match kind {
				Mode::Arcade(mode) => {
					impl_at!(ctx, family, uuid, past, mode, arcade::Arcade, background)
				}
				Mode::Arena(mode) => {
					impl_at!(ctx, family, uuid, past, mode, arena::Arena, background)
				}
				Mode::BedWars(mode) => {
					impl_at!(ctx, family, uuid, past, mode, bed_wars::BedWars, background)
				}
				Mode::BlitzSg(mode) => {
					impl_at!(ctx, family, uuid, past, mode, blitz_sg::BlitzSg, background)
				}
				Mode::BuildBattle(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						build_battle::BuildBattle,
						background
					)
				}
				Mode::CopsAndCrims(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						cops_and_crims::CopsAndCrims,
						background
					)
				}
				Mode::Duels(mode) => {
					impl_at!(ctx, family, uuid, past, mode, duels::Duels, background)
				}
				Mode::MegaWalls(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						mega_walls::MegaWalls,
						background
					)
				}
				Mode::MurderMystery(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						murder_mystery::MurderMystery,
						background
					)
				}
				Mode::Paintball(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						paintball::Paintball,
						background
					)
				}
				Mode::Pit(mode) => {
					impl_at!(ctx, family, uuid, past, mode, pit::Pit, background)
				}
				Mode::Quake(mode) => {
					impl_at!(ctx, family, uuid, past, mode, quake::Quake, background)
				}
				Mode::SkyWars(mode) => {
					impl_at!(ctx, family, uuid, past, mode, sky_wars::SkyWars, background)
				}
				Mode::SmashHeroes(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						smash_heroes::SmashHeroes,
						background
					)
				}
				Mode::SpeedUhc(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						speed_uhc::SpeedUhc,
						background
					)
				}
				Mode::TntGames(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						tnt_games::TntGames,
						background
					)
				}
				Mode::TurboKartRacers(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						turbo_kart_racers::TurboKartRacers,
						background
					)
				}
				Mode::Uhc(mode) => {
					impl_at!(ctx, family, uuid, past, mode, uhc::Uhc, background)
				}
				Mode::VampireZ(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						vampire_z::VampireZ,
						background
					)
				}
				Mode::Walls(mode) => {
					impl_at!(ctx, family, uuid, past, mode, walls::Walls, background)
				}
				Mode::Warlords(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						warlords::Warlords,
						background
					)
				}
				Mode::WoolWars(mode) => {
					impl_at!(
						ctx,
						family,
						uuid,
						past,
						mode,
						wool_wars::WoolWars,
						background
					)
				}
				Mode::Fishing(mode) => {
					impl_at!(ctx, family, uuid, past, mode, fishing::Fishing, background)
				}
				Mode::RecentGames
				| Mode::Winstreaks
				| Mode::SkyBlock(..)
				| Mode::Guild(..)
				| Mode::Network
				| Mode::BedWarsShop
				| Mode::BedWarsPractice
				| Mode::BedWarsHotbar
				| Mode::Parkour => Err(crate::Error::NotImplemented),
			}
		}
		Id::History { kind, uuid } => match kind {
			Mode::Arcade(mode) => {
				impl_history!(ctx, family, uuid, mode, arcade::Arcade, background)
			}
			Mode::Arena(mode) => impl_history!(ctx, family, uuid, mode, arena::Arena, background),
			Mode::BedWars(mode) => {
				impl_history!(ctx, family, uuid, mode, bed_wars::BedWars, background)
			}
			Mode::BlitzSg(mode) => {
				impl_history!(ctx, family, uuid, mode, blitz_sg::BlitzSg, background)
			}
			Mode::BuildBattle(mode) => {
				impl_history!(
					ctx,
					family,
					uuid,
					mode,
					build_battle::BuildBattle,
					background
				)
			}
			Mode::CopsAndCrims(mode) => {
				impl_history!(
					ctx,
					family,
					uuid,
					mode,
					cops_and_crims::CopsAndCrims,
					background
				)
			}
			Mode::Duels(mode) => impl_history!(ctx, family, uuid, mode, duels::Duels, background),
			Mode::MegaWalls(mode) => {
				impl_history!(ctx, family, uuid, mode, mega_walls::MegaWalls, background)
			}
			Mode::MurderMystery(mode) => {
				impl_history!(
					ctx,
					family,
					uuid,
					mode,
					murder_mystery::MurderMystery,
					background
				)
			}
			Mode::Paintball(mode) => {
				impl_history!(ctx, family, uuid, mode, paintball::Paintball, background)
			}
			Mode::Pit(mode) => impl_history!(ctx, family, uuid, mode, pit::Pit, background),
			Mode::Quake(mode) => impl_history!(ctx, family, uuid, mode, quake::Quake, background),
			Mode::SkyWars(mode) => {
				impl_history!(ctx, family, uuid, mode, sky_wars::SkyWars, background)
			}
			Mode::SmashHeroes(mode) => {
				impl_history!(
					ctx,
					family,
					uuid,
					mode,
					smash_heroes::SmashHeroes,
					background
				)
			}
			Mode::SpeedUhc(mode) => {
				impl_history!(ctx, family, uuid, mode, speed_uhc::SpeedUhc, background)
			}
			Mode::TntGames(mode) => {
				impl_history!(ctx, family, uuid, mode, tnt_games::TntGames, background)
			}
			Mode::TurboKartRacers(mode) => {
				impl_history!(
					ctx,
					family,
					uuid,
					mode,
					turbo_kart_racers::TurboKartRacers,
					background
				)
			}
			Mode::Uhc(mode) => impl_history!(ctx, family, uuid, mode, uhc::Uhc, background),
			Mode::VampireZ(mode) => {
				impl_history!(ctx, family, uuid, mode, vampire_z::VampireZ, background)
			}
			Mode::Walls(mode) => impl_history!(ctx, family, uuid, mode, walls::Walls, background),
			Mode::Warlords(mode) => {
				impl_history!(ctx, family, uuid, mode, warlords::Warlords, background)
			}
			Mode::WoolWars(mode) => {
				impl_history!(ctx, family, uuid, mode, wool_wars::WoolWars, background)
			}
			Mode::Fishing(mode) => {
				impl_history!(ctx, family, uuid, mode, fishing::Fishing, background)
			}
			Mode::RecentGames
			| Mode::Winstreaks
			| Mode::SkyBlock(..)
			| Mode::Guild(..)
			| Mode::Network
			| Mode::BedWarsShop
			| Mode::BedWarsPractice
			| Mode::BedWarsHotbar
			| Mode::Parkour => Err(crate::Error::NotImplemented),
		},
		Id::Project { kind, uuid } => match kind {
			ProjectMode::Arcade(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, arcade::Arcade, background)
			}
			ProjectMode::Arena(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, arena::Arena, background)
			}
			ProjectMode::BedWars(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, bed_wars::BedWars, background)
			}
			ProjectMode::BlitzSg(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, blitz_sg::BlitzSg, background)
			}
			ProjectMode::BuildBattle(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					build_battle::BuildBattle,
					background
				)
			}
			ProjectMode::CopsAndCrims(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					cops_and_crims::CopsAndCrims,
					background
				)
			}
			ProjectMode::Duels(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, duels::Duels, background)
			}
			ProjectMode::MegaWalls(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					mega_walls::MegaWalls,
					background
				)
			}
			ProjectMode::MurderMystery(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					murder_mystery::MurderMystery,
					background
				)
			}
			ProjectMode::Paintball(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					paintball::Paintball,
					background
				)
			}
			ProjectMode::Pit(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, pit::Pit, background)
			}
			ProjectMode::Quake(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, quake::Quake, background)
			}
			ProjectMode::SkyWars(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, sky_wars::SkyWars, background)
			}
			ProjectMode::SmashHeroes(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					smash_heroes::SmashHeroes,
					background
				)
			}
			ProjectMode::SpeedUhc(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					speed_uhc::SpeedUhc,
					background
				)
			}
			ProjectMode::TntGames(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					tnt_games::TntGames,
					background
				)
			}
			ProjectMode::TurboKartRacers(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					turbo_kart_racers::TurboKartRacers,
					background
				)
			}
			ProjectMode::Uhc(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, uhc::Uhc, background)
			}
			ProjectMode::VampireZ(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					vampire_z::VampireZ,
					background
				)
			}
			ProjectMode::Walls(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, walls::Walls, background)
			}
			ProjectMode::Warlords(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					warlords::Warlords,
					background
				)
			}
			ProjectMode::WoolWars(mode, kind) => {
				impl_project!(
					ctx,
					family,
					uuid,
					mode,
					kind,
					wool_wars::WoolWars,
					background
				)
			}
			ProjectMode::Fishing(mode, kind) => {
				impl_project!(ctx, family, uuid, mode, kind, fishing::Fishing, background)
			}
			ProjectMode::SkyBlock(..) | ProjectMode::Guild(..) => Err(crate::Error::NotImplemented),
		},
		Id::Compare {
			kind,
			uuid_lhs,
			uuid_rhs,
			..
		} => match kind {
			Mode::Arcade(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					arcade::Arcade,
					background
				)
			}
			Mode::Arena(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					arena::Arena,
					background
				)
			}
			Mode::BedWars(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					bed_wars::BedWars,
					background
				)
			}
			Mode::BlitzSg(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					blitz_sg::BlitzSg,
					background
				)
			}
			Mode::BuildBattle(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					build_battle::BuildBattle,
					background
				)
			}
			Mode::CopsAndCrims(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					cops_and_crims::CopsAndCrims,
					background
				)
			}
			Mode::Duels(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					duels::Duels,
					background
				)
			}
			Mode::MegaWalls(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					mega_walls::MegaWalls,
					background
				)
			}
			Mode::MurderMystery(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					murder_mystery::MurderMystery,
					background
				)
			}
			Mode::Paintball(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					paintball::Paintball,
					background
				)
			}
			Mode::Pit(mode) => {
				impl_compare!(ctx, family, uuid_lhs, uuid_rhs, mode, pit::Pit, background)
			}
			Mode::Quake(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					quake::Quake,
					background
				)
			}
			Mode::SkyWars(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					sky_wars::SkyWars,
					background
				)
			}
			Mode::SmashHeroes(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					smash_heroes::SmashHeroes,
					background
				)
			}
			Mode::SpeedUhc(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					speed_uhc::SpeedUhc,
					background
				)
			}
			Mode::TntGames(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					tnt_games::TntGames,
					background
				)
			}
			Mode::TurboKartRacers(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					turbo_kart_racers::TurboKartRacers,
					background
				)
			}
			Mode::Uhc(mode) => {
				impl_compare!(ctx, family, uuid_lhs, uuid_rhs, mode, uhc::Uhc, background)
			}
			Mode::VampireZ(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					vampire_z::VampireZ,
					background
				)
			}
			Mode::Walls(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					walls::Walls,
					background
				)
			}
			Mode::Warlords(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					warlords::Warlords,
					background
				)
			}
			Mode::WoolWars(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					wool_wars::WoolWars,
					background
				)
			}
			Mode::Fishing(mode) => {
				impl_compare!(
					ctx,
					family,
					uuid_lhs,
					uuid_rhs,
					mode,
					fishing::Fishing,
					background
				)
			}
			Mode::RecentGames
			| Mode::Winstreaks
			| Mode::SkyBlock(..)
			| Mode::Guild(..)
			| Mode::Network
			| Mode::BedWarsShop
			| Mode::BedWarsPractice
			| Mode::BedWarsHotbar
			| Mode::Parkour => Err(crate::Error::NotImplemented),
		},
	}
}
