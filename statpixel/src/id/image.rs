use std::borrow::Cow;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use api::command::{GuildMode, Id, Mode, ProjectMode, SkyBlockMode};
use chrono::{Duration, Utc};
use translate::context::Context;

macro_rules! impl_root {
	($ctx: expr, $uuid: expr, $mode: expr, $game: ty) => {{
		let (_, data, session, skin, suffix) =
			$crate::commands::get_player_data_session_skin_suffix($ctx, Some($uuid), None)
				.await
				.ok()?;

		Some(
			$crate::commands::games::image::command::<$game>(
				$ctx,
				Some($mode),
				None,
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
	($ctx: expr, $uuid: expr, $mode: expr, $game: ty) => {{
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
				None,
			)
			.await
			.ok()??
			.0,
		)
	}};
}

macro_rules! impl_snapshot {
	($ctx: expr, $uuid: expr, $from: expr, $mode: expr, $game: ty) => {{
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
			None,
		);

		Some(api::canvas::to_png(&mut surface).into())
	}};
}

macro_rules! impl_project {
	($ctx: expr, $uuid: expr, $mode: expr, $kind: expr, $game: ty) => {{
		let (player, session) = $crate::commands::get_player_session($ctx, Some($uuid), None)
			.await
			.ok()?;

		Some(
			$crate::commands::project::image::command::<$game>(
				$ctx,
				&player,
				&session,
				None,
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
pub async fn map(ctx: &Context<'_>, id: Id) -> Option<Cow<'static, [u8]>> {
	match id {
		Id::Builder { shapes, uuid } => {
			let (_, data, session, skin, _) =
				crate::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None)
					.await
					.ok()?;
			let bytes =
				crate::commands::builder::build::build(ctx, &shapes, &data, &session, &skin);

			Some(bytes)
		}
		Id::Root { kind, uuid } => match kind {
			Mode::Arcade(mode) => impl_root!(ctx, uuid, mode, arcade::Arcade),
			Mode::Arena(mode) => impl_root!(ctx, uuid, mode, arena::Arena),
			Mode::BedWars(mode) => impl_root!(ctx, uuid, mode, bed_wars::BedWars),
			Mode::BlitzSg(mode) => impl_root!(ctx, uuid, mode, blitz_sg::BlitzSg),
			Mode::BuildBattle(mode) => impl_root!(ctx, uuid, mode, build_battle::BuildBattle),
			Mode::CopsAndCrims(mode) => impl_root!(ctx, uuid, mode, cops_and_crims::CopsAndCrims),
			Mode::Duels(mode) => impl_root!(ctx, uuid, mode, duels::Duels),
			Mode::MegaWalls(mode) => impl_root!(ctx, uuid, mode, mega_walls::MegaWalls),
			Mode::MurderMystery(mode) => impl_root!(ctx, uuid, mode, murder_mystery::MurderMystery),
			Mode::Paintball(mode) => impl_root!(ctx, uuid, mode, paintball::Paintball),
			Mode::Pit(mode) => impl_root!(ctx, uuid, mode, pit::Pit),
			Mode::Quake(mode) => impl_root!(ctx, uuid, mode, quake::Quake),
			Mode::SkyWars(mode) => impl_root!(ctx, uuid, mode, sky_wars::SkyWars),
			Mode::SmashHeroes(mode) => impl_root!(ctx, uuid, mode, smash_heroes::SmashHeroes),
			Mode::SpeedUhc(mode) => impl_root!(ctx, uuid, mode, speed_uhc::SpeedUhc),
			Mode::TntGames(mode) => impl_root!(ctx, uuid, mode, tnt_games::TntGames),
			Mode::TurboKartRacers(mode) => {
				impl_root!(ctx, uuid, mode, turbo_kart_racers::TurboKartRacers)
			}
			Mode::Uhc(mode) => impl_root!(ctx, uuid, mode, uhc::Uhc),
			Mode::VampireZ(mode) => impl_root!(ctx, uuid, mode, vampire_z::VampireZ),
			Mode::Walls(mode) => impl_root!(ctx, uuid, mode, walls::Walls),
			Mode::Warlords(mode) => impl_root!(ctx, uuid, mode, warlords::Warlords),
			Mode::WoolWars(mode) => impl_root!(ctx, uuid, mode, wool_wars::WoolWars),
			Mode::Guild(mode, limit, nanos, member_id) => match mode {
				GuildMode::General => {
					let guild = crate::commands::get_guild(ctx, None, member_id, None, Some(uuid))
						.await
						.ok()?;

					crate::commands::guild::image::general(ctx, &guild)
						.await
						.ok()
				}
				GuildMode::Member => {
					let (guild, player) =
						crate::commands::get_guild_with_member(ctx, member_id, None)
							.await
							.ok()?;

					crate::commands::guild::image::member(ctx, &guild, &player)
						.await
						.ok()
				}
				GuildMode::Members => {
					let guild = crate::commands::get_guild(ctx, None, member_id, None, Some(uuid))
						.await
						.ok()?;

					crate::commands::guild::image::members(ctx, &guild)
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
						None,
					)
					.await
					.ok()
				}
				SkyBlockMode::Bank => {
					let (_, data) = crate::commands::get_player_data(ctx, Some(uuid), None)
						.await
						.ok()?;

					Some(
						crate::commands::skyblock::image::bank(ctx, &data, None, profile, None)
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
							None,
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
					None,
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
					None,
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
					None,
				))
			}
		},
		Id::Snapshot { kind, uuid, past } => {
			let past = Duration::nanoseconds(past);

			match kind {
				Mode::Arcade(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, arcade::Arcade)
				}
				Mode::Arena(mode) => impl_snapshot!(ctx, uuid, past, mode, arena::Arena),
				Mode::BedWars(mode) => impl_snapshot!(ctx, uuid, past, mode, bed_wars::BedWars),
				Mode::BlitzSg(mode) => impl_snapshot!(ctx, uuid, past, mode, blitz_sg::BlitzSg),
				Mode::BuildBattle(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, build_battle::BuildBattle)
				}
				Mode::CopsAndCrims(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, cops_and_crims::CopsAndCrims)
				}
				Mode::Duels(mode) => impl_snapshot!(ctx, uuid, past, mode, duels::Duels),
				Mode::MegaWalls(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, mega_walls::MegaWalls)
				}
				Mode::MurderMystery(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, murder_mystery::MurderMystery)
				}
				Mode::Paintball(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, paintball::Paintball)
				}
				Mode::Pit(mode) => impl_snapshot!(ctx, uuid, past, mode, pit::Pit),
				Mode::Quake(mode) => impl_snapshot!(ctx, uuid, past, mode, quake::Quake),
				Mode::SkyWars(mode) => impl_snapshot!(ctx, uuid, past, mode, sky_wars::SkyWars),
				Mode::SmashHeroes(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, smash_heroes::SmashHeroes)
				}
				Mode::SpeedUhc(mode) => impl_snapshot!(ctx, uuid, past, mode, speed_uhc::SpeedUhc),
				Mode::TntGames(mode) => impl_snapshot!(ctx, uuid, past, mode, tnt_games::TntGames),
				Mode::TurboKartRacers(mode) => {
					impl_snapshot!(ctx, uuid, past, mode, turbo_kart_racers::TurboKartRacers)
				}
				Mode::Uhc(mode) => impl_snapshot!(ctx, uuid, past, mode, uhc::Uhc),
				Mode::VampireZ(mode) => impl_snapshot!(ctx, uuid, past, mode, vampire_z::VampireZ),
				Mode::Walls(mode) => impl_snapshot!(ctx, uuid, past, mode, walls::Walls),
				Mode::Warlords(mode) => impl_snapshot!(ctx, uuid, past, mode, warlords::Warlords),
				Mode::WoolWars(mode) => impl_snapshot!(ctx, uuid, past, mode, wool_wars::WoolWars),
				_ => None,
			}
		}
		Id::History { kind, uuid } => match kind {
			Mode::Arcade(mode) => impl_history!(ctx, uuid, mode, arcade::Arcade),
			Mode::Arena(mode) => impl_history!(ctx, uuid, mode, arena::Arena),
			Mode::BedWars(mode) => impl_history!(ctx, uuid, mode, bed_wars::BedWars),
			Mode::BlitzSg(mode) => impl_history!(ctx, uuid, mode, blitz_sg::BlitzSg),
			Mode::BuildBattle(mode) => impl_history!(ctx, uuid, mode, build_battle::BuildBattle),
			Mode::CopsAndCrims(mode) => {
				impl_history!(ctx, uuid, mode, cops_and_crims::CopsAndCrims)
			}
			Mode::Duels(mode) => impl_history!(ctx, uuid, mode, duels::Duels),
			Mode::MegaWalls(mode) => impl_history!(ctx, uuid, mode, mega_walls::MegaWalls),
			Mode::MurderMystery(mode) => {
				impl_history!(ctx, uuid, mode, murder_mystery::MurderMystery)
			}
			Mode::Paintball(mode) => impl_history!(ctx, uuid, mode, paintball::Paintball),
			Mode::Pit(mode) => impl_history!(ctx, uuid, mode, pit::Pit),
			Mode::Quake(mode) => impl_history!(ctx, uuid, mode, quake::Quake),
			Mode::SkyWars(mode) => impl_history!(ctx, uuid, mode, sky_wars::SkyWars),
			Mode::SmashHeroes(mode) => impl_history!(ctx, uuid, mode, smash_heroes::SmashHeroes),
			Mode::SpeedUhc(mode) => impl_history!(ctx, uuid, mode, speed_uhc::SpeedUhc),
			Mode::TntGames(mode) => impl_history!(ctx, uuid, mode, tnt_games::TntGames),
			Mode::TurboKartRacers(mode) => {
				impl_history!(ctx, uuid, mode, turbo_kart_racers::TurboKartRacers)
			}
			Mode::Uhc(mode) => impl_history!(ctx, uuid, mode, uhc::Uhc),
			Mode::VampireZ(mode) => impl_history!(ctx, uuid, mode, vampire_z::VampireZ),
			Mode::Walls(mode) => impl_history!(ctx, uuid, mode, walls::Walls),
			Mode::Warlords(mode) => impl_history!(ctx, uuid, mode, warlords::Warlords),
			Mode::WoolWars(mode) => impl_history!(ctx, uuid, mode, wool_wars::WoolWars),
			_ => None,
		},
		Id::Project { kind, uuid } => match kind {
			ProjectMode::Arcade(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, arcade::Arcade)
			}
			ProjectMode::Arena(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, arena::Arena)
			}
			ProjectMode::BedWars(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, bed_wars::BedWars)
			}
			ProjectMode::BlitzSg(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, blitz_sg::BlitzSg)
			}
			ProjectMode::BuildBattle(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, build_battle::BuildBattle)
			}
			ProjectMode::CopsAndCrims(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, cops_and_crims::CopsAndCrims)
			}
			ProjectMode::Duels(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, duels::Duels)
			}
			ProjectMode::MegaWalls(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, mega_walls::MegaWalls)
			}
			ProjectMode::MurderMystery(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, murder_mystery::MurderMystery)
			}
			ProjectMode::Paintball(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, paintball::Paintball)
			}
			ProjectMode::Pit(mode, kind) => impl_project!(ctx, uuid, mode, kind, pit::Pit),
			ProjectMode::Quake(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, quake::Quake)
			}
			ProjectMode::SkyWars(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, sky_wars::SkyWars)
			}
			ProjectMode::SmashHeroes(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, smash_heroes::SmashHeroes)
			}
			ProjectMode::SpeedUhc(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, speed_uhc::SpeedUhc)
			}
			ProjectMode::TntGames(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, tnt_games::TntGames)
			}
			ProjectMode::TurboKartRacers(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, turbo_kart_racers::TurboKartRacers)
			}
			ProjectMode::Uhc(mode, kind) => impl_project!(ctx, uuid, mode, kind, uhc::Uhc),
			ProjectMode::VampireZ(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, vampire_z::VampireZ)
			}
			ProjectMode::Walls(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, walls::Walls)
			}
			ProjectMode::Warlords(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, warlords::Warlords)
			}
			ProjectMode::WoolWars(mode, kind) => {
				impl_project!(ctx, uuid, mode, kind, wool_wars::WoolWars)
			}
			_ => None,
		},
	}
}
