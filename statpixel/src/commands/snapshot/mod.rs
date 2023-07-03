pub mod run;

use chrono::Utc;
use translate::context;

use crate::util;

macro_rules! generate_history_command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr) => {
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
		) -> Result<(), ::translate::Error> {
			let uuid = util::parse_uuid(uuid)?;
			let ctx = &context::Context::from_poise(&ctx);

			run::command::<$game>(ctx, username, uuid, mode, $duration).await
		}
	};
}

macro_rules! generate_large_history_command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr) => {
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
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let uuid = util::parse_uuid(uuid)?;
			let ctx = &context::Context::from_poise(&ctx);

			run::command::<$game>(ctx, username, uuid, mode, $duration).await
		}
	};
}

macro_rules! generate_guild_history_command {
	($fn: ident, $duration: expr) => {
		#[allow(clippy::too_many_lines)]
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES"
		)]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[min_length = 3]
			#[max_length = 32]
			#[autocomplete = "crate::commands::autocomplete_guild_name"]
			name: Option<::std::string::String>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
		) -> Result<(), ::translate::Error> {
			let uuid = util::parse_uuid(uuid)?;
			let ctx = &context::Context::from_poise(&ctx);

			run::guild_command(ctx, name, username, uuid, Utc::now() - $duration, None).await
		}
	};
}

#[macro_export]
macro_rules! generate_history_commands {
	($fn: ident, $duration: expr) => {
		pub mod $fn {
			use super::*;

			generate_history_command!(
				::api::player::stats::arcade::Arcade,
				::api::player::stats::arcade::ArcadeMode,
				arcade,
				$duration
			);
			generate_history_command!(
				::api::player::stats::arena::Arena,
				::api::player::stats::arena::ArenaMode,
				arena,
				$duration
			);
			generate_history_command!(
				::api::player::stats::bed_wars::BedWars,
				::api::player::stats::bed_wars::BedWarsMode,
				bedwars,
				$duration
			);
			generate_history_command!(
				::api::player::stats::blitz_sg::BlitzSg,
				::api::player::stats::blitz_sg::BlitzSgMode,
				blitz,
				$duration
			);
			generate_history_command!(
				::api::player::stats::build_battle::BuildBattle,
				::api::player::stats::build_battle::BuildBattleMode,
				buildbattle,
				$duration
			);
			generate_history_command!(
				::api::player::stats::cops_and_crims::CopsAndCrims,
				::api::player::stats::cops_and_crims::CopsAndCrimsMode,
				copsandcrims,
				$duration
			);
			generate_large_history_command!(
				::api::player::stats::duels::Duels,
				::api::player::stats::duels::DuelsMode,
				duels,
				$duration
			);
			generate_history_command!(
				::api::player::stats::mega_walls::MegaWalls,
				::api::player::stats::mega_walls::MegaWallsMode,
				megawalls,
				$duration
			);
			generate_history_command!(
				::api::player::stats::murder_mystery::MurderMystery,
				::api::player::stats::murder_mystery::MurderMysteryMode,
				murdermystery,
				$duration
			);
			generate_history_command!(
				::api::player::stats::paintball::Paintball,
				::api::player::stats::paintball::PaintballMode,
				paintball,
				$duration
			);
			generate_history_command!(
				::api::player::stats::pit::Pit,
				::api::player::stats::pit::PitMode,
				pit,
				$duration
			);
			generate_history_command!(
				::api::player::stats::quake::Quake,
				::api::player::stats::quake::QuakeMode,
				quake,
				$duration
			);
			generate_history_command!(
				::api::player::stats::sky_wars::SkyWars,
				::api::player::stats::sky_wars::SkyWarsMode,
				skywars,
				$duration
			);
			generate_history_command!(
				::api::player::stats::smash_heroes::SmashHeroes,
				::api::player::stats::smash_heroes::SmashHeroesMode,
				smash,
				$duration
			);
			generate_history_command!(
				::api::player::stats::speed_uhc::SpeedUhc,
				::api::player::stats::speed_uhc::SpeedUhcMode,
				speeduhc,
				$duration
			);
			generate_history_command!(
				::api::player::stats::tnt_games::TntGames,
				::api::player::stats::tnt_games::TntGamesMode,
				tntgames,
				$duration
			);
			generate_history_command!(
				::api::player::stats::turbo_kart_racers::TurboKartRacers,
				::api::player::stats::turbo_kart_racers::TurboKartRacersMode,
				turbokartracers,
				$duration
			);
			generate_history_command!(
				::api::player::stats::uhc::Uhc,
				::api::player::stats::uhc::UhcMode,
				uhc,
				$duration
			);
			generate_history_command!(
				::api::player::stats::vampire_z::VampireZ,
				::api::player::stats::vampire_z::VampireZMode,
				vampirez,
				$duration
			);
			generate_history_command!(
				::api::player::stats::walls::Walls,
				::api::player::stats::walls::WallsMode,
				walls,
				$duration
			);
			generate_history_command!(
				::api::player::stats::warlords::Warlords,
				::api::player::stats::warlords::WarlordsMode,
				warlords,
				$duration
			);
			generate_history_command!(
				::api::player::stats::wool_wars::WoolWars,
				::api::player::stats::wool_wars::WoolWarsMode,
				woolwars,
				$duration
			);
			generate_guild_history_command!(guild, $duration);

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
					"guild"
				)
			)]
			#[allow(clippy::unused_async)]
			pub async fn $fn(
				_ctx: ::translate::Context<'_>,
			) -> ::std::result::Result<(), ::translate::Error> {
				::std::result::Result::Ok(())
			}
		}
	};
}

generate_history_commands!(daily, ::chrono::Duration::days(1));
generate_history_commands!(weekly, ::chrono::Duration::weeks(1));
generate_history_commands!(monthly, ::chrono::Duration::days(30));
