#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use chrono::Utc;
use translate::{context, Context, Error};

use crate::util;

macro_rules! generate_command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[allow(clippy::too_many_lines)]
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES"
		)]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			mode: Option<$mode>,
			#[min = 1i64] hours: Option<i64>,
			#[min = 1i64] days: Option<i64>,
			#[min = 1i64] weeks: Option<i64>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let ctx = &context::Context::from_poise(&ctx);

			let uuid = util::parse_uuid(uuid)?;
			let mut duration = ::chrono::Duration::zero();

			if let Some(hours) = hours {
				duration = duration + ::chrono::Duration::hours(hours);
			}

			if let Some(days) = days {
				duration = duration + ::chrono::Duration::days(days);
			}

			if let Some(weeks) = weeks {
				duration = duration + ::chrono::Duration::weeks(weeks);
			}

			if duration.is_zero() {
				duration = ::chrono::Duration::weeks(1);
			}

			super::snapshot::run::command::<$game>(ctx, username, uuid, mode, duration).await
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

		#[allow(clippy::too_many_lines)]
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES"
		)]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			#[min = 1i64] hours: Option<i64>,
			#[min = 1i64] days: Option<i64>,
			#[min = 1i64] weeks: Option<i64>,
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let ctx = &context::Context::from_poise(&ctx);

			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let uuid = util::parse_uuid(uuid)?;
			let mut duration = ::chrono::Duration::zero();

			if let Some(hours) = hours {
				duration = duration + ::chrono::Duration::hours(hours);
			}

			if let Some(days) = days {
				duration = duration + ::chrono::Duration::days(days);
			}

			if let Some(weeks) = weeks {
				duration = duration + ::chrono::Duration::weeks(weeks);
			}

			if duration.is_zero() {
				duration = ::chrono::Duration::weeks(1);
			}

			super::snapshot::run::command::<$game>(ctx, username, uuid, mode, duration).await
		}
	};
}

#[allow(clippy::too_many_lines)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn guild(
	ctx: crate::Context<'_>,
	#[min_length = 3]
	#[max_length = 32]
	#[autocomplete = "crate::commands::autocomplete_guild_name"]
	name: Option<String>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
	#[min = 1i64] hours: Option<i64>,
	#[min = 1i64] days: Option<i64>,
	#[min = 1i64] weeks: Option<i64>,
) -> Result<(), Error> {
	let ctx = &context::Context::from_poise(&ctx);

	let uuid = util::parse_uuid(uuid)?;
	let mut duration = chrono::Duration::zero();

	if let Some(hours) = hours {
		duration = duration + chrono::Duration::hours(hours);
	}

	if let Some(days) = days {
		duration = duration + chrono::Duration::days(days);
	}

	if let Some(weeks) = weeks {
		duration = duration + chrono::Duration::weeks(weeks);
	}

	if duration.is_zero() {
		duration = chrono::Duration::weeks(1);
	}

	super::snapshot::run::guild_command(ctx, name, username, uuid, Utc::now() - duration, None)
		.await
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
		"guild",
	)
)]
#[allow(clippy::unused_async)]
pub async fn from(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
