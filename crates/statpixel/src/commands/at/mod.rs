pub mod run;

use api::player::stats;

use chrono::Utc;
use translate::{context, Error};

use crate::util;

macro_rules! command {
	($game: ty, $mode: ty, $fn: ident) => {
		pub mod $fn {
			use super::*;

			#[allow(clippy::too_many_lines)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = "at"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				player: Option<String>,
				mode: Option<$mode>,
				time: String,
			) -> ::std::result::Result<(), ::translate::Error> {
				let ctx = &context::Context::from_poise(&ctx);

				let uuid = util::parse_uuid(player.as_deref());
				let duration =
					chrono::Duration::from_std(humantime::parse_duration(&time)?).unwrap();

				run::command::<$game>(ctx, player, uuid, mode, duration).await
			}
		}
	};
}

macro_rules! large_command {
	($game: ty, $mode: ty, $fn: ident) => {
		pub mod $fn {
			use super::*;
			use api::player::stats;

			async fn autocomplete_mode<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl Iterator<Item = poise::AutocompleteChoice<u32>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete(ctx, partial).await
			}

			#[allow(clippy::too_many_lines)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = "at"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				player: Option<String>,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
				time: String,
			) -> ::std::result::Result<(), ::translate::Error> {
				let ctx = &context::Context::from_poise(&ctx);

				let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
				let uuid = util::parse_uuid(player.as_deref());
				let duration =
					::chrono::Duration::from_std(humantime::parse_duration(&time)?).unwrap();

				run::command::<$game>(ctx, player, uuid, mode, duration).await
			}
		}
	};
}

#[allow(clippy::too_many_lines)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES",
	rename = "at"
)]
pub async fn guild(
	ctx: crate::Context<'_>,
	#[min_length = 3]
	#[max_length = 32]
	#[autocomplete = "crate::commands::autocomplete_guild_name"]
	name: Option<String>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
	time: String,
) -> Result<(), Error> {
	let ctx = &context::Context::from_poise(&ctx);

	let uuid = util::parse_uuid(player.as_deref());
	let duration = ::chrono::Duration::from_std(humantime::parse_duration(&time)?).unwrap();

	run::guild_command(ctx, name, player, uuid, Utc::now() - duration, None).await
}

large_command!(stats::arcade::Arcade, stats::arcade::ArcadeMode, arcade);
command!(stats::arena::Arena, stats::arena::ArenaMode, arena);
large_command!(
	stats::bed_wars::BedWars,
	stats::bed_wars::BedWarsMode,
	bedwars
);
large_command!(
	stats::blitz_sg::BlitzSg,
	stats::blitz_sg::BlitzSgMode,
	blitz
);
command!(
	stats::build_battle::BuildBattle,
	stats::build_battle::BuildBattleMode,
	buildbattle
);
command!(
	stats::cops_and_crims::CopsAndCrims,
	stats::cops_and_crims::CopsAndCrimsMode,
	copsandcrims
);
large_command!(stats::duels::Duels, stats::duels::DuelsMode, duels);
command!(
	stats::mega_walls::MegaWalls,
	stats::mega_walls::MegaWallsMode,
	megawalls
);
command!(
	stats::murder_mystery::MurderMystery,
	stats::murder_mystery::MurderMysteryMode,
	murdermystery
);
command!(
	stats::paintball::Paintball,
	stats::paintball::PaintballMode,
	paintball
);
command!(stats::pit::Pit, stats::pit::PitMode, pit);
command!(stats::quake::Quake, stats::quake::QuakeMode, quake);
command!(
	stats::sky_wars::SkyWars,
	stats::sky_wars::SkyWarsMode,
	skywars
);
command!(
	stats::smash_heroes::SmashHeroes,
	stats::smash_heroes::SmashHeroesMode,
	smash
);
command!(
	stats::speed_uhc::SpeedUhc,
	stats::speed_uhc::SpeedUhcMode,
	speeduhc
);
command!(
	stats::tnt_games::TntGames,
	stats::tnt_games::TntGamesMode,
	tntgames
);
command!(
	stats::turbo_kart_racers::TurboKartRacers,
	stats::turbo_kart_racers::TurboKartRacersMode,
	turbokartracers
);
command!(stats::uhc::Uhc, stats::uhc::UhcMode, uhc);
command!(
	stats::vampire_z::VampireZ,
	stats::vampire_z::VampireZMode,
	vampirez
);
command!(stats::walls::Walls, stats::walls::WallsMode, walls);
command!(
	stats::warlords::Warlords,
	stats::warlords::WarlordsMode,
	warlords
);
command!(
	stats::wool_wars::WoolWars,
	stats::wool_wars::WoolWarsMode,
	woolwars
);
command!(
	stats::fishing::Fishing,
	stats::fishing::FishingMode,
	fishing
);
