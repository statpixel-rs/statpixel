pub mod run;
pub mod session;

use api::player::stats;
use chrono::Utc;
use translate::context;

use crate::util;

#[rustfmt::skip]
macro_rules! command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr, $name: literal) => {
		pub mod $fn {
			use super::*;

			#[allow(clippy::too_many_lines)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = $name
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				player: Option<String>,
				mode: Option<$mode>,
			) -> Result<(), ::translate::Error> {
				let uuid = util::parse_uuid(player.as_deref());
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, player, uuid, mode, $duration).await
			}
		}
	};
}

#[rustfmt::skip]
macro_rules! large_command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr, $name: literal) => {
		pub mod $fn {
			use super::*;

			async fn autocomplete_mode<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl Iterator<Item = poise::serenity_prelude::AutocompleteChoice<'a>> + 'a {
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
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				player: Option<String>,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
				let uuid = util::parse_uuid(player.as_deref());
				let ctx = &context::Context::from_poise(&ctx);
	
				run::command::<$game>(ctx, player, uuid, mode, $duration).await
			}
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
			#[max_length = 36]
			#[autocomplete = "crate::commands::autocomplete_username"]
			player: Option<String>,
		) -> Result<(), ::translate::Error> {
			let uuid = util::parse_uuid(player.as_deref());
			let ctx = &context::Context::from_poise(&ctx);

			run::guild_command(ctx, name, player, uuid, Utc::now() - $duration, None).await
		}
	};
}

#[macro_export]
macro_rules! commands {
	($fn: ident, $duration: expr, $name: literal) => {
		pub mod $fn {
			use super::*;

			large_command!(
				stats::arcade::Arcade,
				stats::arcade::ArcadeMode,
				arcade,
				$duration,
				$name
			);
			command!(
				stats::arena::Arena,
				stats::arena::ArenaMode,
				arena,
				$duration,
				$name
			);
			large_command!(
				stats::bed_wars::BedWars,
				stats::bed_wars::BedWarsMode,
				bedwars,
				$duration,
				$name
			);
			large_command!(
				stats::blitz_sg::BlitzSg,
				stats::blitz_sg::BlitzSgMode,
				blitz,
				$duration,
				$name
			);
			command!(
				stats::build_battle::BuildBattle,
				stats::build_battle::BuildBattleMode,
				buildbattle,
				$duration,
				$name
			);
			command!(
				stats::cops_and_crims::CopsAndCrims,
				stats::cops_and_crims::CopsAndCrimsMode,
				copsandcrims,
				$duration,
				$name
			);
			large_command!(
				stats::duels::Duels,
				stats::duels::DuelsMode,
				duels,
				$duration,
				$name
			);
			command!(
				stats::mega_walls::MegaWalls,
				stats::mega_walls::MegaWallsMode,
				megawalls,
				$duration,
				$name
			);
			command!(
				stats::murder_mystery::MurderMystery,
				stats::murder_mystery::MurderMysteryMode,
				murdermystery,
				$duration,
				$name
			);
			command!(
				stats::paintball::Paintball,
				stats::paintball::PaintballMode,
				paintball,
				$duration,
				$name
			);
			command!(stats::pit::Pit, stats::pit::PitMode, pit, $duration, $name);
			command!(
				stats::quake::Quake,
				stats::quake::QuakeMode,
				quake,
				$duration,
				$name
			);
			command!(
				stats::sky_wars::SkyWars,
				stats::sky_wars::SkyWarsMode,
				skywars,
				$duration,
				$name
			);
			command!(
				stats::smash_heroes::SmashHeroes,
				stats::smash_heroes::SmashHeroesMode,
				smash,
				$duration,
				$name
			);
			command!(
				stats::speed_uhc::SpeedUhc,
				stats::speed_uhc::SpeedUhcMode,
				speeduhc,
				$duration,
				$name
			);
			command!(
				stats::tnt_games::TntGames,
				stats::tnt_games::TntGamesMode,
				tntgames,
				$duration,
				$name
			);
			command!(
				stats::turbo_kart_racers::TurboKartRacers,
				stats::turbo_kart_racers::TurboKartRacersMode,
				turbokartracers,
				$duration,
				$name
			);
			command!(stats::uhc::Uhc, stats::uhc::UhcMode, uhc, $duration, $name);
			command!(
				stats::vampire_z::VampireZ,
				stats::vampire_z::VampireZMode,
				vampirez,
				$duration,
				$name
			);
			command!(
				stats::walls::Walls,
				stats::walls::WallsMode,
				walls,
				$duration,
				$name
			);
			command!(
				stats::warlords::Warlords,
				stats::warlords::WarlordsMode,
				warlords,
				$duration,
				$name
			);
			command!(
				stats::wool_wars::WoolWars,
				stats::wool_wars::WoolWarsMode,
				woolwars,
				$duration,
				$name
			);
			command!(
				stats::fishing::Fishing,
				stats::fishing::FishingMode,
				fishing,
				$duration,
				$name
			);
			guild_command!(guild, $duration, $name);
		}
	};
}

commands!(daily, ::chrono::Duration::try_days(1).unwrap(), "daily");
commands!(weekly, ::chrono::Duration::try_weeks(1).unwrap(), "weekly");
commands!(
	monthly,
	::chrono::Duration::try_days(30).unwrap(),
	"monthly"
);
