pub mod image;
pub mod run;

use api::player::stats;
use translate::context;

use crate::util;

macro_rules! large_command {
	($game: ty, $mode: ty, $kind: ty, $fn: ident) => {
		pub mod $fn {
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
				required_bot_permissions = "ATTACH_FILES",
				rename = "project"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 16]
				#[autocomplete = "crate::commands::autocomplete_username"]
				username: Option<::std::string::String>,
				#[min_length = 32]
				#[max_length = 36]
				uuid: Option<::std::string::String>,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
				#[autocomplete = "autocomplete_kind"] statistic: Option<u32>,
				#[min = 0.0f64]
				#[max = 1_000_000_000.0f64]
				value: Option<f64>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
				let kind: ::std::option::Option<$kind> = statistic.map(|m| m.into());

				let uuid = util::parse_uuid(uuid.as_deref())?;
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, username, uuid, mode, kind, value).await
			}
		}
	};
}

macro_rules! command {
	($game: ty, $mode: ty, $kind: ty, $fn: ident) => {
		pub mod $fn {
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
				required_bot_permissions = "ATTACH_FILES",
				rename = "project"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 16]
				#[autocomplete = "crate::commands::autocomplete_username"]
				username: Option<::std::string::String>,
				#[min_length = 32]
				#[max_length = 36]
				uuid: Option<::std::string::String>,
				mode: Option<$mode>,
				#[autocomplete = "autocomplete_kind"] statistic: Option<u32>,
				#[min = 0.0f64]
				#[max = 1_000_000_000.0f64]
				value: Option<f64>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let kind: ::std::option::Option<$kind> = statistic.map(|m| m.into());
				let uuid = util::parse_uuid(uuid.as_deref())?;
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, username, uuid, mode, kind, value).await
			}
		}
	};
}

large_command!(
	stats::arcade::Arcade,
	stats::arcade::ArcadeMode,
	stats::arcade::ArcadeKind,
	arcade
);
command!(
	stats::arena::Arena,
	stats::arena::ArenaMode,
	stats::arena::ArenaKind,
	arena
);
large_command!(
	stats::bed_wars::BedWars,
	stats::bed_wars::BedWarsMode,
	stats::bed_wars::BedWarsKind,
	bedwars
);
large_command!(
	stats::blitz_sg::BlitzSg,
	stats::blitz_sg::BlitzSgMode,
	stats::blitz_sg::BlitzSgKind,
	blitz
);
command!(
	stats::build_battle::BuildBattle,
	stats::build_battle::BuildBattleMode,
	stats::build_battle::BuildBattleKind,
	buildbattle
);
command!(
	stats::cops_and_crims::CopsAndCrims,
	stats::cops_and_crims::CopsAndCrimsMode,
	stats::cops_and_crims::CopsAndCrimsKind,
	copsandcrims
);
large_command!(
	stats::duels::Duels,
	stats::duels::DuelsMode,
	stats::duels::DuelsKind,
	duels
);
command!(
	stats::mega_walls::MegaWalls,
	stats::mega_walls::MegaWallsMode,
	stats::mega_walls::MegaWallsKind,
	megawalls
);
command!(
	stats::murder_mystery::MurderMystery,
	stats::murder_mystery::MurderMysteryMode,
	stats::murder_mystery::MurderMysteryKind,
	murdermystery
);
command!(
	stats::paintball::Paintball,
	stats::paintball::PaintballMode,
	stats::paintball::PaintballKind,
	paintball
);
command!(
	stats::pit::Pit,
	stats::pit::PitMode,
	stats::pit::PitKind,
	pit
);
command!(
	stats::quake::Quake,
	stats::quake::QuakeMode,
	stats::quake::QuakeKind,
	quake
);
command!(
	stats::sky_wars::SkyWars,
	stats::sky_wars::SkyWarsMode,
	stats::sky_wars::SkyWarsKind,
	skywars
);
command!(
	stats::smash_heroes::SmashHeroes,
	stats::smash_heroes::SmashHeroesMode,
	stats::smash_heroes::SmashHeroesKind,
	smash
);
command!(
	stats::speed_uhc::SpeedUhc,
	stats::speed_uhc::SpeedUhcMode,
	stats::speed_uhc::SpeedUhcKind,
	speeduhc
);
command!(
	stats::tnt_games::TntGames,
	stats::tnt_games::TntGamesMode,
	stats::tnt_games::TntGamesKind,
	tntgames
);
command!(
	stats::turbo_kart_racers::TurboKartRacers,
	stats::turbo_kart_racers::TurboKartRacersMode,
	stats::turbo_kart_racers::TurboKartRacersKind,
	turbokartracers
);
command!(
	stats::uhc::Uhc,
	stats::uhc::UhcMode,
	stats::uhc::UhcKind,
	uhc
);
command!(
	stats::vampire_z::VampireZ,
	stats::vampire_z::VampireZMode,
	stats::vampire_z::VampireZKind,
	vampirez
);
command!(
	stats::walls::Walls,
	stats::walls::WallsMode,
	stats::walls::WallsKind,
	walls
);
command!(
	stats::warlords::Warlords,
	stats::warlords::WarlordsMode,
	stats::warlords::WarlordsKind,
	warlords
);
command!(
	stats::wool_wars::WoolWars,
	stats::wool_wars::WoolWarsMode,
	stats::wool_wars::WoolWarsKind,
	woolwars
);
