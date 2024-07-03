pub mod run;

use api::player::stats;
use translate::context;

macro_rules! command {
	($game: ty, $mode: ty, $fn: ident) => {
		pub mod $fn {
			use super::*;

			#[allow(clippy::too_many_lines)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = "compare"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				from: Option<::std::string::String>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				to: Option<::std::string::String>,
				mode: Option<$mode>,
			) -> Result<(), ::translate::Error> {
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, to, from, mode, None, None, true).await
			}
		}
	};
}

macro_rules! large_command {
	($game: ty, $mode: ty, $fn: ident) => {
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
				rename = "compare"
			)]
			pub async fn command(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				from: Option<::std::string::String>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				to: Option<::std::string::String>,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, to, from, mode, None, None, true).await
			}
		}
	};
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
