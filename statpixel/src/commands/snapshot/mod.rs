pub mod run;

use chrono::Utc;
use translate::context;

use crate::util;

#[rustfmt::skip]
macro_rules! command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr, $name: literal) => {
		#[allow(clippy::too_many_lines)]
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES",
			rename = $name
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
			let uuid = util::parse_uuid(uuid.as_deref())?;
			let ctx = &context::Context::from_poise(&ctx);

			run::command::<$game>(ctx, username, uuid, mode, $duration).await
		}
	};
}

#[rustfmt::skip]
macro_rules! large_command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr, $name: literal) => {
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
			required_bot_permissions = "ATTACH_FILES",
			rename = $name
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
			let uuid = util::parse_uuid(uuid.as_deref())?;
			let ctx = &context::Context::from_poise(&ctx);

			run::command::<$game>(ctx, username, uuid, mode, $duration).await
		}
	};
}

#[rustfmt::skip]
macro_rules! guild_command {
	($fn: ident, $duration: expr, $name: literal) => {
		#[allow(clippy::too_many_lines)]
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES",
			rename = $name
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
			let uuid = util::parse_uuid(uuid.as_deref())?;
			let ctx = &context::Context::from_poise(&ctx);

			run::guild_command(ctx, name, username, uuid, Utc::now() - $duration, None).await
		}
	};
}

#[macro_export]
macro_rules! commands {
	($fn: ident, $duration: expr, $name: literal) => {
		pub mod $fn {
			use super::*;

			command!(
				::api::player::stats::arcade::Arcade,
				::api::player::stats::arcade::ArcadeMode,
				arcade,
				$duration,
				$name
			);
			command!(
				::api::player::stats::arena::Arena,
				::api::player::stats::arena::ArenaMode,
				arena,
				$duration,
				$name
			);
			command!(
				::api::player::stats::bed_wars::BedWars,
				::api::player::stats::bed_wars::BedWarsMode,
				bedwars,
				$duration,
				$name
			);
			command!(
				::api::player::stats::blitz_sg::BlitzSg,
				::api::player::stats::blitz_sg::BlitzSgMode,
				blitz,
				$duration,
				$name
			);
			command!(
				::api::player::stats::build_battle::BuildBattle,
				::api::player::stats::build_battle::BuildBattleMode,
				buildbattle,
				$duration,
				$name
			);
			command!(
				::api::player::stats::cops_and_crims::CopsAndCrims,
				::api::player::stats::cops_and_crims::CopsAndCrimsMode,
				copsandcrims,
				$duration,
				$name
			);
			large_command!(
				::api::player::stats::duels::Duels,
				::api::player::stats::duels::DuelsMode,
				duels,
				$duration,
				$name
			);
			command!(
				::api::player::stats::mega_walls::MegaWalls,
				::api::player::stats::mega_walls::MegaWallsMode,
				megawalls,
				$duration,
				$name
			);
			command!(
				::api::player::stats::murder_mystery::MurderMystery,
				::api::player::stats::murder_mystery::MurderMysteryMode,
				murdermystery,
				$duration,
				$name
			);
			command!(
				::api::player::stats::paintball::Paintball,
				::api::player::stats::paintball::PaintballMode,
				paintball,
				$duration,
				$name
			);
			command!(
				::api::player::stats::pit::Pit,
				::api::player::stats::pit::PitMode,
				pit,
				$duration,
				$name
			);
			command!(
				::api::player::stats::quake::Quake,
				::api::player::stats::quake::QuakeMode,
				quake,
				$duration,
				$name
			);
			command!(
				::api::player::stats::sky_wars::SkyWars,
				::api::player::stats::sky_wars::SkyWarsMode,
				skywars,
				$duration,
				$name
			);
			command!(
				::api::player::stats::smash_heroes::SmashHeroes,
				::api::player::stats::smash_heroes::SmashHeroesMode,
				smash,
				$duration,
				$name
			);
			command!(
				::api::player::stats::speed_uhc::SpeedUhc,
				::api::player::stats::speed_uhc::SpeedUhcMode,
				speeduhc,
				$duration,
				$name
			);
			command!(
				::api::player::stats::tnt_games::TntGames,
				::api::player::stats::tnt_games::TntGamesMode,
				tntgames,
				$duration,
				$name
			);
			command!(
				::api::player::stats::turbo_kart_racers::TurboKartRacers,
				::api::player::stats::turbo_kart_racers::TurboKartRacersMode,
				turbokartracers,
				$duration,
				$name
			);
			command!(
				::api::player::stats::uhc::Uhc,
				::api::player::stats::uhc::UhcMode,
				uhc,
				$duration,
				$name
			);
			command!(
				::api::player::stats::vampire_z::VampireZ,
				::api::player::stats::vampire_z::VampireZMode,
				vampirez,
				$duration,
				$name
			);
			command!(
				::api::player::stats::walls::Walls,
				::api::player::stats::walls::WallsMode,
				walls,
				$duration,
				$name
			);
			command!(
				::api::player::stats::warlords::Warlords,
				::api::player::stats::warlords::WarlordsMode,
				warlords,
				$duration,
				$name
			);
			command!(
				::api::player::stats::wool_wars::WoolWars,
				::api::player::stats::wool_wars::WoolWarsMode,
				woolwars,
				$duration,
				$name
			);
			guild_command!(guild, $duration, $name);
		}
	};
}

commands!(daily, ::chrono::Duration::days(1), "daily");
commands!(weekly, ::chrono::Duration::weeks(1), "weekly");
commands!(monthly, ::chrono::Duration::days(30), "monthly");
