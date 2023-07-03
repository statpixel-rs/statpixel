pub mod image;
pub mod run;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use translate::{context, Context, Error};

use crate::util;

macro_rules! generate_large_command {
	($m: ident, $game: ty, $mode: ty, $kind: ty, $fn: ident) => {
		pub mod $m {
			use super::*;

			async fn autocomplete_mode<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete(ctx, partial).await
			}

			async fn autocomplete_kind<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete_kind(ctx, partial).await
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
				username: Option<::std::string::String>,
				#[min_length = 32]
				#[max_length = 36]
				uuid: Option<::std::string::String>,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
				#[autocomplete = "autocomplete_kind"] kind: Option<u32>,
				#[min = 0.0f64]
				#[max = 1_000_000_000.0f64]
				value: Option<f64>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
				let kind: ::std::option::Option<$kind> = kind.map(|m| m.into());

				let uuid = util::parse_uuid(uuid)?;
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, username, uuid, mode, kind, value).await
			}
		}
	};
}

macro_rules! generate_command {
	($m: ident, $game: ty, $mode: ty, $kind: ty, $fn: ident) => {
		pub mod $m {
			use super::*;

			async fn autocomplete_kind<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete_kind(ctx, partial).await
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
				username: Option<::std::string::String>,
				#[min_length = 32]
				#[max_length = 36]
				uuid: Option<::std::string::String>,
				mode: Option<$mode>,
				#[autocomplete = "autocomplete_kind"] kind: Option<u32>,
				#[min = 0.0f64]
				#[max = 1_000_000_000.0f64]
				value: Option<f64>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let kind: ::std::option::Option<$kind> = kind.map(|m| m.into());
				let uuid = util::parse_uuid(uuid)?;
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, username, uuid, mode, kind, value).await
			}
		}
	};
}

generate_command!(
	arcade_command,
	arcade::Arcade,
	arcade::ArcadeMode,
	arcade::ArcadeKind,
	arcade
);
generate_command!(
	arena_command,
	arena::Arena,
	arena::ArenaMode,
	arena::ArenaKind,
	arena
);
generate_command!(
	bed_wars_command,
	bed_wars::BedWars,
	bed_wars::BedWarsMode,
	bed_wars::BedWarsKind,
	bedwars
);
generate_command!(
	blitz_sg_command,
	blitz_sg::BlitzSg,
	blitz_sg::BlitzSgMode,
	blitz_sg::BlitzSgKind,
	blitz
);
generate_command!(
	build_battle_command,
	build_battle::BuildBattle,
	build_battle::BuildBattleMode,
	build_battle::BuildBattleKind,
	buildbattle
);
generate_command!(
	cops_and_crims_command,
	cops_and_crims::CopsAndCrims,
	cops_and_crims::CopsAndCrimsMode,
	cops_and_crims::CopsAndCrimsKind,
	copsandcrims
);
generate_large_command!(
	duels_command,
	duels::Duels,
	duels::DuelsMode,
	duels::DuelsKind,
	duels
);
generate_command!(
	mega_walls_command,
	mega_walls::MegaWalls,
	mega_walls::MegaWallsMode,
	mega_walls::MegaWallsKind,
	megawalls
);
generate_command!(
	murder_mystery_command,
	murder_mystery::MurderMystery,
	murder_mystery::MurderMysteryMode,
	murder_mystery::MurderMysteryKind,
	murdermystery
);
generate_command!(
	paintball_command,
	paintball::Paintball,
	paintball::PaintballMode,
	paintball::PaintballKind,
	paintball
);
generate_command!(pit_command, pit::Pit, pit::PitMode, pit::PitKind, pit);
generate_command!(
	quake_command,
	quake::Quake,
	quake::QuakeMode,
	quake::QuakeKind,
	quake
);
generate_command!(
	sky_wars_command,
	sky_wars::SkyWars,
	sky_wars::SkyWarsMode,
	sky_wars::SkyWarsKind,
	skywars
);
generate_command!(
	smash_heroes_command,
	smash_heroes::SmashHeroes,
	smash_heroes::SmashHeroesMode,
	smash_heroes::SmashHeroesKind,
	smash
);
generate_command!(
	speed_uhc_command,
	speed_uhc::SpeedUhc,
	speed_uhc::SpeedUhcMode,
	speed_uhc::SpeedUhcKind,
	speeduhc
);
generate_command!(
	tnt_games_command,
	tnt_games::TntGames,
	tnt_games::TntGamesMode,
	tnt_games::TntGamesKind,
	tntgames
);
generate_command!(
	turbo_kart_racers_command,
	turbo_kart_racers::TurboKartRacers,
	turbo_kart_racers::TurboKartRacersMode,
	turbo_kart_racers::TurboKartRacersKind,
	turbokartracers
);
generate_command!(uhc_command, uhc::Uhc, uhc::UhcMode, uhc::UhcKind, uhc);
generate_command!(
	vampire_z_command,
	vampire_z::VampireZ,
	vampire_z::VampireZMode,
	vampire_z::VampireZKind,
	vampirez
);
generate_command!(
	walls_command,
	walls::Walls,
	walls::WallsMode,
	walls::WallsKind,
	walls
);
generate_command!(
	warlords_command,
	warlords::Warlords,
	warlords::WarlordsMode,
	warlords::WarlordsKind,
	warlords
);
generate_command!(
	wool_wars_command,
	wool_wars::WoolWars,
	wool_wars::WoolWarsMode,
	wool_wars::WoolWarsKind,
	woolwars
);

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	subcommands(
		"arcade_command::arcade",
		"arena_command::arena",
		"bed_wars_command::bedwars",
		"blitz_sg_command::blitz",
		"build_battle_command::buildbattle",
		"cops_and_crims_command::copsandcrims",
		"duels_command::duels",
		"mega_walls_command::megawalls",
		"murder_mystery_command::murdermystery",
		"paintball_command::paintball",
		"pit_command::pit",
		"quake_command::quake",
		"sky_wars_command::skywars",
		"smash_heroes_command::smash",
		"speed_uhc_command::speeduhc",
		"tnt_games_command::tntgames",
		"turbo_kart_racers_command::turbokartracers",
		"uhc_command::uhc",
		"vampire_z_command::vampirez",
		"walls_command::walls",
		"warlords_command::warlords",
		"wool_wars_command::woolwars",
	)
)]
#[allow(clippy::unused_async)]
pub async fn project(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
