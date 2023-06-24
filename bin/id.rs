#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use api::id::{GuildMode, Id, Mode, ProjectMode};
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

/// Dispatches a command based on a deserialized `custom_id`
#[allow(clippy::too_many_lines)]
pub async fn map(ctx: &Context<'_>, id: Id) -> Result<(), Error> {
	info!(id = ?id, "dispatching command");

	match id {
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
			Mode::Guild(mode) => match mode {
				GuildMode::General => {
					super::commands::guild::run::general(ctx, None, None, None, Some(uuid)).await
				}
				GuildMode::Member => {
					super::commands::guild::run::members(ctx, None, None, None, Some(uuid)).await
				}
				GuildMode::Top => {
					super::commands::guild::run::top(ctx, None, None, None, None, 30, Some(uuid))
						.await
				}
			},
			_ => Ok(()),
		},
		Id::Snapshot { kind, uuid, from } => match kind {
			Mode::Arcade(mode) => impl_snapshot!(ctx, uuid, from, mode, arcade::Arcade),
			Mode::Arena(mode) => impl_snapshot!(ctx, uuid, from, mode, arena::Arena),
			Mode::BedWars(mode) => impl_snapshot!(ctx, uuid, from, mode, bed_wars::BedWars),
			Mode::BlitzSg(mode) => impl_snapshot!(ctx, uuid, from, mode, blitz_sg::BlitzSg),
			Mode::BuildBattle(mode) => {
				impl_snapshot!(ctx, uuid, from, mode, build_battle::BuildBattle)
			}
			Mode::CopsAndCrims(mode) => {
				impl_snapshot!(ctx, uuid, from, mode, cops_and_crims::CopsAndCrims)
			}
			Mode::Duels(mode) => impl_snapshot!(ctx, uuid, from, mode, duels::Duels),
			Mode::MegaWalls(mode) => impl_snapshot!(ctx, uuid, from, mode, mega_walls::MegaWalls),
			Mode::MurderMystery(mode) => {
				impl_snapshot!(ctx, uuid, from, mode, murder_mystery::MurderMystery)
			}
			Mode::Paintball(mode) => impl_snapshot!(ctx, uuid, from, mode, paintball::Paintball),
			Mode::Pit(mode) => impl_snapshot!(ctx, uuid, from, mode, pit::Pit),
			Mode::Quake(mode) => impl_snapshot!(ctx, uuid, from, mode, quake::Quake),
			Mode::SkyWars(mode) => impl_snapshot!(ctx, uuid, from, mode, sky_wars::SkyWars),
			Mode::SmashHeroes(mode) => {
				impl_snapshot!(ctx, uuid, from, mode, smash_heroes::SmashHeroes)
			}
			Mode::SpeedUhc(mode) => impl_snapshot!(ctx, uuid, from, mode, speed_uhc::SpeedUhc),
			Mode::TntGames(mode) => impl_snapshot!(ctx, uuid, from, mode, tnt_games::TntGames),
			Mode::TurboKartRacers(mode) => {
				impl_snapshot!(ctx, uuid, from, mode, turbo_kart_racers::TurboKartRacers)
			}
			Mode::Uhc(mode) => impl_snapshot!(ctx, uuid, from, mode, uhc::Uhc),
			Mode::VampireZ(mode) => impl_snapshot!(ctx, uuid, from, mode, vampire_z::VampireZ),
			Mode::Walls(mode) => impl_snapshot!(ctx, uuid, from, mode, walls::Walls),
			Mode::Warlords(mode) => impl_snapshot!(ctx, uuid, from, mode, warlords::Warlords),
			Mode::WoolWars(mode) => impl_snapshot!(ctx, uuid, from, mode, wool_wars::WoolWars),
			_ => Ok(()),
		},
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
	}
}
