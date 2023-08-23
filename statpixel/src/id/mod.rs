pub mod image;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use api::command::{GuildMode, Id, Mode, ProjectMode, SkyBlockMode};
use chrono::Duration;
use poise::serenity_prelude as serenity;
use tracing::info;
use translate::{context::Context, Error};

macro_rules! impl_root {
	($ctx: expr, $uuid: expr, $mode: expr, $game: ty) => {
		super::commands::games::run::command::<$game>($ctx, None, Some($uuid), Some($mode)).await
	};
}

macro_rules! impl_history {
	($ctx: expr, $uuid: expr, $mode: expr, $game: ty) => {
		super::commands::history::run::command::<$game>($ctx, None, Some($uuid), Some($mode)).await
	};
}

macro_rules! impl_snapshot {
	($ctx: expr, $uuid: expr, $from: expr, $mode: expr, $game: ty) => {
		super::commands::snapshot::run::command::<$game>(
			$ctx,
			None,
			Some($uuid),
			Some($mode),
			$from,
		)
		.await
	};
}

macro_rules! impl_at {
	($ctx: expr, $uuid: expr, $from: expr, $mode: expr, $game: ty) => {
		super::commands::at::run::command::<$game>($ctx, None, Some($uuid), Some($mode), $from)
			.await
	};
}

macro_rules! impl_project {
	($ctx: expr, $uuid: expr, $mode: expr, $kind: expr, $game: ty) => {
		super::commands::project::run::command::<$game>(
			$ctx,
			None,
			Some($uuid),
			Some($mode),
			Some($kind),
			None,
		)
		.await
	};
}

macro_rules! impl_compare {
	($ctx: expr, $lhs: expr, $rhs: expr, $mode: expr, $game: ty) => {
		super::commands::compare::run::command::<$game>(
			$ctx,
			None,
			None,
			Some($mode),
			Some($lhs),
			Some($rhs),
		)
		.await
	};
}

/// Dispatches a command based on a deserialized `custom_id`
#[allow(clippy::too_many_lines)]
pub async fn map(ctx: &Context<'_>, id: Id) -> Result<(), Error> {
	info!(id = ?id, "dispatching command");

	match id {
		Id::Builder { shapes, uuid } => {
			let (_, data, session, skin, _) =
				super::commands::get_player_data_session_skin_suffix(ctx, Some(uuid), None).await?;
			let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;

			let bytes = super::commands::builder::build::build(
				ctx, family, &shapes, &data, &session, &skin, background,
			)?;

			ctx.send(
				poise::CreateReply::new()
					.content(crate::tip::random(ctx))
					.attachment(serenity::CreateAttachment::bytes(bytes, crate::IMAGE_NAME)),
			)
			.await?;

			Ok(())
		}
		Id::Root { kind, uuid } => match kind {
			Mode::BedWarsPractice => {
				super::commands::bedwars::run::practice(ctx, None, Some(uuid)).await
			}
			Mode::BedWarsShop => super::commands::bedwars::run::shop(ctx, None, Some(uuid)).await,
			Mode::BedWarsHotbar => {
				super::commands::bedwars::run::hotbar(ctx, None, Some(uuid)).await
			}
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
					super::commands::guild::run::general(
						ctx,
						None,
						None,
						member_id,
						Some(uuid),
						limit,
						nanos,
					)
					.await
				}
				GuildMode::Member => {
					super::commands::guild::run::member(ctx, None, member_id, limit, nanos).await
				}
				GuildMode::Members => {
					super::commands::guild::run::members(
						ctx,
						None,
						None,
						member_id,
						Some(uuid),
						limit,
						nanos,
					)
					.await
				}
				GuildMode::Top => {
					super::commands::guild::run::top(
						ctx,
						None,
						None,
						member_id,
						nanos.map_or(chrono::Duration::days(30), chrono::Duration::nanoseconds),
						limit.unwrap_or(30),
						Some(uuid),
					)
					.await
				}
			},
			Mode::SkyBlock(mode, profile) => match mode {
				SkyBlockMode::Auctions => {
					super::commands::skyblock::run::auctions(ctx, None, Some(uuid), profile).await
				}
				SkyBlockMode::Bank => {
					super::commands::skyblock::run::bank(ctx, None, None, Some(uuid), profile).await
				}
				SkyBlockMode::Candy => {
					super::commands::skyblock::run::candy(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::EnderChest => {
					super::commands::skyblock::run::enderchest(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Equipment => {
					super::commands::skyblock::run::equipment(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Fishing => {
					super::commands::skyblock::run::fishing(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Inventory => {
					super::commands::skyblock::run::inventory(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Networth => {
					super::commands::skyblock::run::networth(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Pets => {
					super::commands::skyblock::run::pets(ctx, None, None, Some(uuid), profile).await
				}
				SkyBlockMode::Potions => {
					super::commands::skyblock::run::potions(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Profile => {
					super::commands::skyblock::run::profile(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Quiver => {
					super::commands::skyblock::run::quiver(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Talisman => {
					super::commands::skyblock::run::talisman(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Vault => {
					super::commands::skyblock::run::vault(ctx, None, None, Some(uuid), profile)
						.await
				}
				SkyBlockMode::Wardrobe => {
					super::commands::skyblock::run::wardrobe(ctx, None, None, Some(uuid), profile)
						.await
				}
			},
			Mode::Network => super::commands::network::run::network(ctx, None, Some(uuid)).await,
			Mode::RecentGames => super::commands::recent::run::recent(ctx, None, Some(uuid)).await,
			Mode::Winstreaks => {
				super::commands::winstreaks::run::winstreaks(ctx, None, Some(uuid)).await
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
				_ => Ok(()),
			}
		}
		Id::At { kind, uuid, past } => {
			let past = Duration::nanoseconds(past);
			match kind {
				Mode::Arcade(mode) => {
					impl_at!(ctx, uuid, past, mode, arcade::Arcade)
				}
				Mode::Arena(mode) => impl_at!(ctx, uuid, past, mode, arena::Arena),
				Mode::BedWars(mode) => impl_at!(ctx, uuid, past, mode, bed_wars::BedWars),
				Mode::BlitzSg(mode) => impl_at!(ctx, uuid, past, mode, blitz_sg::BlitzSg),
				Mode::BuildBattle(mode) => {
					impl_at!(ctx, uuid, past, mode, build_battle::BuildBattle)
				}
				Mode::CopsAndCrims(mode) => {
					impl_at!(ctx, uuid, past, mode, cops_and_crims::CopsAndCrims)
				}
				Mode::Duels(mode) => impl_at!(ctx, uuid, past, mode, duels::Duels),
				Mode::MegaWalls(mode) => {
					impl_at!(ctx, uuid, past, mode, mega_walls::MegaWalls)
				}
				Mode::MurderMystery(mode) => {
					impl_at!(ctx, uuid, past, mode, murder_mystery::MurderMystery)
				}
				Mode::Paintball(mode) => {
					impl_at!(ctx, uuid, past, mode, paintball::Paintball)
				}
				Mode::Pit(mode) => impl_at!(ctx, uuid, past, mode, pit::Pit),
				Mode::Quake(mode) => impl_at!(ctx, uuid, past, mode, quake::Quake),
				Mode::SkyWars(mode) => impl_at!(ctx, uuid, past, mode, sky_wars::SkyWars),
				Mode::SmashHeroes(mode) => {
					impl_at!(ctx, uuid, past, mode, smash_heroes::SmashHeroes)
				}
				Mode::SpeedUhc(mode) => impl_at!(ctx, uuid, past, mode, speed_uhc::SpeedUhc),
				Mode::TntGames(mode) => impl_at!(ctx, uuid, past, mode, tnt_games::TntGames),
				Mode::TurboKartRacers(mode) => {
					impl_at!(ctx, uuid, past, mode, turbo_kart_racers::TurboKartRacers)
				}
				Mode::Uhc(mode) => impl_at!(ctx, uuid, past, mode, uhc::Uhc),
				Mode::VampireZ(mode) => impl_at!(ctx, uuid, past, mode, vampire_z::VampireZ),
				Mode::Walls(mode) => impl_at!(ctx, uuid, past, mode, walls::Walls),
				Mode::Warlords(mode) => impl_at!(ctx, uuid, past, mode, warlords::Warlords),
				Mode::WoolWars(mode) => impl_at!(ctx, uuid, past, mode, wool_wars::WoolWars),
				_ => Ok(()),
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
			_ => Ok(()),
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
			_ => Ok(()),
		},
		#[allow(unused_variables)]
		Id::Compare {
			kind,
			uuid_lhs,
			uuid_rhs,
			..
		} => match kind {
			Mode::Arcade(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, arcade::Arcade),
			Mode::Arena(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, arena::Arena),
			Mode::BedWars(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, bed_wars::BedWars),
			Mode::BlitzSg(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, blitz_sg::BlitzSg),
			Mode::BuildBattle(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, build_battle::BuildBattle)
			}
			Mode::CopsAndCrims(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, cops_and_crims::CopsAndCrims)
			}
			Mode::Duels(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, duels::Duels),
			Mode::MegaWalls(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, mega_walls::MegaWalls)
			}
			Mode::MurderMystery(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, murder_mystery::MurderMystery)
			}
			Mode::Paintball(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, paintball::Paintball)
			}
			Mode::Pit(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, pit::Pit),
			Mode::Quake(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, quake::Quake),
			Mode::SkyWars(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, sky_wars::SkyWars),
			Mode::SmashHeroes(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, smash_heroes::SmashHeroes)
			}
			Mode::SpeedUhc(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, speed_uhc::SpeedUhc)
			}
			Mode::TntGames(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, tnt_games::TntGames)
			}
			Mode::TurboKartRacers(mode) => {
				impl_compare!(
					ctx,
					uuid_lhs,
					uuid_rhs,
					mode,
					turbo_kart_racers::TurboKartRacers
				)
			}
			Mode::Uhc(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, uhc::Uhc),
			Mode::VampireZ(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, vampire_z::VampireZ)
			}
			Mode::Walls(mode) => impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, walls::Walls),
			Mode::Warlords(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, warlords::Warlords)
			}
			Mode::WoolWars(mode) => {
				impl_compare!(ctx, uuid_lhs, uuid_rhs, mode, wool_wars::WoolWars)
			}
			_ => Ok(()),
		},
		Id::Between { .. } => Ok(()),
	}
}
