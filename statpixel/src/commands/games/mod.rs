pub mod image;
pub mod run;

use std::convert::Into;

#[allow(clippy::wildcard_imports)]
use api::player::stats;

use crate::util;
use translate::{context, Context, Error};

#[rustfmt::skip]
macro_rules! large_command {
	($game: ty, $fn: ident, $name: literal) => {
		pub mod $fn {
			use super::*;
			use crate::commands::from::$fn::command as from;
			use crate::commands::at::$fn::command as at;
			use crate::commands::history::$fn::command as history;
			use crate::commands::project::$fn::command as project;
			use crate::commands::session::$fn::command as session;
			use crate::commands::snapshot::daily::$fn::command as daily;
			use crate::commands::snapshot::monthly::$fn::command as monthly;
			use crate::commands::snapshot::weekly::$fn::command as weekly;
			use crate::commands::compare::$fn::command as compare;

			async fn autocomplete_mode<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl Iterator<Item = poise::AutocompleteChoice<u32>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete(ctx, partial).await
			}

			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES"
			)]
			async fn general(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				player: Option<String>,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
			) -> Result<(), Error> {
				let mode: ::std::option::Option<<$game as api::canvas::prelude::Game>::Mode> =
					mode.map(Into::into);
				let uuid = util::parse_uuid(player.as_deref());
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, player, uuid, mode).await
			}

			#[allow(clippy::unused_async)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				subcommands("general", "from", "daily", "weekly", "monthly", "history", "project", "compare", "at", "session"),
				rename = $name
			)]
			pub async fn parent(_ctx: Context<'_>) -> Result<(), Error> {
				Ok(())
			}
		}
	};
}

#[rustfmt::skip]
macro_rules! command {
	($game: ty, $fn: ident, $name: literal) => {
		pub mod $fn {
			use super::*;
			use crate::commands::from::$fn::command as from;
			use crate::commands::at::$fn::command as at;
			use crate::commands::history::$fn::command as history;
			use crate::commands::project::$fn::command as project;
			use crate::commands::session::$fn::command as session;
			use crate::commands::snapshot::daily::$fn::command as daily;
			use crate::commands::snapshot::monthly::$fn::command as monthly;
			use crate::commands::snapshot::weekly::$fn::command as weekly;
			use crate::commands::compare::$fn::command as compare;

			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES"
			)]
			async fn general(
				ctx: $crate::Context<'_>,
				#[max_length = 36]
				#[autocomplete = "crate::commands::autocomplete_username"]
				player: Option<String>,
				mode: Option<<$game as api::canvas::prelude::Game>::Mode>,
			) -> Result<(), Error> {
				let uuid = util::parse_uuid(player.as_deref());
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, player, uuid, mode).await
			}

			#[allow(clippy::unused_async)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				subcommands("general", "from", "daily", "weekly", "monthly", "history", "project", "compare", "at", "session"),
				rename = $name
			)]
			pub async fn parent(_ctx: Context<'_>) -> Result<(), Error> {
				Ok(())
			}
		}
	};
}

large_command!(stats::arcade::Arcade, arcade, "arcade");
command!(stats::arena::Arena, arena, "arena");
large_command!(stats::blitz_sg::BlitzSg, blitz, "blitz");
command!(stats::build_battle::BuildBattle, buildbattle, "buildbattle");
command!(
	stats::cops_and_crims::CopsAndCrims,
	copsandcrims,
	"copsandcrims"
);
large_command!(stats::duels::Duels, duels, "duels");
command!(stats::mega_walls::MegaWalls, megawalls, "megawalls");
command!(
	stats::murder_mystery::MurderMystery,
	murdermystery,
	"murdermystery"
);
command!(stats::paintball::Paintball, paintball, "paintball");
command!(stats::pit::Pit, pit, "pit");
command!(stats::quake::Quake, quake, "quake");
command!(stats::sky_wars::SkyWars, skywars, "skywars");
command!(stats::smash_heroes::SmashHeroes, smash, "smash");
command!(stats::speed_uhc::SpeedUhc, speeduhc, "speeduhc");
command!(stats::tnt_games::TntGames, tntgames, "tntgames");
command!(
	stats::turbo_kart_racers::TurboKartRacers,
	turbokartracers,
	"turbokartracers"
);
command!(stats::uhc::Uhc, uhc, "uhc");
command!(stats::vampire_z::VampireZ, vampirez, "vampirez");
command!(stats::walls::Walls, walls, "walls");
command!(stats::warlords::Warlords, warlords, "warlords");
command!(stats::wool_wars::WoolWars, woolwars, "woolwars");
command!(stats::fishing::Fishing, fishing, "fishing");

pub mod bedwars {
	use super::*;
	use crate::commands::at::bedwars::command as at;
	use crate::commands::bedwars::{hotbar, practice, shop};
	use crate::commands::compare::bedwars::command as compare;
	use crate::commands::from::bedwars::command as from;
	use crate::commands::history::bedwars::command as history;
	use crate::commands::project::bedwars::command as project;
	use crate::commands::session::bedwars::command as session;
	use crate::commands::snapshot::daily::bedwars::command as daily;
	use crate::commands::snapshot::monthly::bedwars::command as monthly;
	use crate::commands::snapshot::weekly::bedwars::command as weekly;

	async fn autocomplete_mode<'a>(
		ctx: crate::Context<'a>,
		partial: &'a str,
	) -> impl Iterator<Item = poise::AutocompleteChoice<u32>> + 'a {
		let partial = partial.to_ascii_lowercase();

		stats::bed_wars::BedWars::autocomplete(ctx, partial).await
	}

	#[poise::command(
		on_error = "crate::util::error_handler",
		slash_command,
		required_bot_permissions = "ATTACH_FILES"
	)]
	async fn general(
		ctx: Context<'_>,
		#[max_length = 36]
		#[autocomplete = "crate::commands::autocomplete_username"]
		player: Option<String>,
		#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
	) -> ::std::result::Result<(), ::translate::Error> {
		let mode: Option<<stats::bed_wars::BedWars as api::canvas::prelude::Game>::Mode> =
			mode.map(Into::into);
		let uuid = util::parse_uuid(player.as_deref());
		let ctx = &context::Context::from_poise(&ctx);

		run::command::<stats::bed_wars::BedWars>(ctx, player, uuid, mode).await
	}

	#[allow(clippy::unused_async)]
	#[poise::command(
		on_error = "crate::util::error_handler",
		slash_command,
		required_bot_permissions = "ATTACH_FILES",
		subcommands(
			"general", "from", "daily", "weekly", "monthly", "history", "project", "compare",
			"hotbar", "shop", "practice", "at", "session"
		),
		rename = "bedwars"
	)]
	pub async fn parent(_ctx: Context<'_>) -> Result<(), Error> {
		Ok(())
	}
}
