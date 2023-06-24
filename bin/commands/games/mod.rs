pub mod run;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use crate::util;
use translate::context;

macro_rules! generate_large_command {
	($game: ty, $fn: ident) => {
		async fn autocomplete_mode<'a>(
			ctx: $crate::Context<'a>,
			partial: &'a str,
		) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
			let partial = partial.to_ascii_lowercase();

			<$game>::autocomplete(ctx, partial).await
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
		) -> ::std::result::Result<(), ::translate::Error> {
			let mode: ::std::option::Option<<$game as api::prelude::Game>::Mode> =
				mode.map(|m| m.into());
			let uuid = util::parse_uuid(uuid)?;
			let ctx = &context::Context::from_poise(&ctx);

			run::command::<$game>(ctx, username, uuid, mode).await
		}
	};
}

macro_rules! generate_command {
	($game: ty, $fn: ident) => {
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
			mode: Option<<$game as api::prelude::Game>::Mode>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let uuid = util::parse_uuid(uuid)?;
			let ctx = &context::Context::from_poise(&ctx);

			run::command::<$game>(ctx, username, uuid, mode).await
		}
	};
}

generate_command!(arcade::Arcade, arcade);
generate_command!(arena::Arena, arena);
generate_command!(bed_wars::BedWars, bedwars);
generate_command!(blitz_sg::BlitzSg, blitz);
generate_command!(build_battle::BuildBattle, buildbattle);
generate_command!(cops_and_crims::CopsAndCrims, copsandcrims);
generate_large_command!(duels::Duels, duels);
generate_command!(mega_walls::MegaWalls, megawalls);
generate_command!(murder_mystery::MurderMystery, murdermystery);
generate_command!(paintball::Paintball, paintball);
generate_command!(pit::Pit, pit);
generate_command!(quake::Quake, quake);
generate_command!(sky_wars::SkyWars, skywars);
generate_command!(smash_heroes::SmashHeroes, smash);
generate_command!(speed_uhc::SpeedUhc, speeduhc);
generate_command!(tnt_games::TntGames, tntgames);
generate_command!(turbo_kart_racers::TurboKartRacers, turbokartracers);
generate_command!(uhc::Uhc, uhc);
generate_command!(vampire_z::VampireZ, vampirez);
generate_command!(walls::Walls, walls);
generate_command!(warlords::Warlords, warlords);
generate_command!(wool_wars::WoolWars, woolwars);
