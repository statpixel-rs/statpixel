#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use chrono::Utc;
use translate::{context, Error};

use crate::util;

macro_rules! command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[allow(clippy::too_many_lines)]
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES",
			rename = "from"
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
			time: String,
		) -> ::std::result::Result<(), ::translate::Error> {
			let ctx = &context::Context::from_poise(&ctx);

			let uuid = util::parse_uuid(uuid.as_deref())?;
			let duration = ::chrono::Duration::from_std(humantime::parse_duration(&time)?).unwrap();

			super::snapshot::run::command::<$game>(ctx, username, uuid, mode, duration).await
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
			) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete(ctx, partial).await
			}

			#[allow(clippy::too_many_lines)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				rename = "from"
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
				time: String,
			) -> ::std::result::Result<(), ::translate::Error> {
				let ctx = &context::Context::from_poise(&ctx);

				let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
				let uuid = util::parse_uuid(uuid.as_deref())?;
				let duration =
					::chrono::Duration::from_std(humantime::parse_duration(&time)?).unwrap();

				crate::commands::snapshot::run::command::<$game>(
					ctx, username, uuid, mode, duration,
				)
				.await
			}
		}
	};
}

#[allow(clippy::too_many_lines)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES",
	rename = "from"
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
	time: String,
) -> Result<(), Error> {
	let ctx = &context::Context::from_poise(&ctx);

	let uuid = util::parse_uuid(uuid.as_deref())?;
	let duration = ::chrono::Duration::from_std(humantime::parse_duration(&time)?).unwrap();

	super::snapshot::run::guild_command(ctx, name, username, uuid, Utc::now() - duration, None)
		.await
}

large_command!(stats::arcade::Arcade, stats::arcade::ArcadeMode, arcade);
command!(arena::Arena, arena::ArenaMode, arena);
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
	build_battle::BuildBattle,
	build_battle::BuildBattleMode,
	buildbattle
);
command!(
	cops_and_crims::CopsAndCrims,
	cops_and_crims::CopsAndCrimsMode,
	copsandcrims
);
large_command!(stats::duels::Duels, stats::duels::DuelsMode, duels);
command!(mega_walls::MegaWalls, mega_walls::MegaWallsMode, megawalls);
command!(
	murder_mystery::MurderMystery,
	murder_mystery::MurderMysteryMode,
	murdermystery
);
command!(paintball::Paintball, paintball::PaintballMode, paintball);
command!(pit::Pit, pit::PitMode, pit);
command!(quake::Quake, quake::QuakeMode, quake);
command!(sky_wars::SkyWars, sky_wars::SkyWarsMode, skywars);
command!(
	smash_heroes::SmashHeroes,
	smash_heroes::SmashHeroesMode,
	smash
);
command!(speed_uhc::SpeedUhc, speed_uhc::SpeedUhcMode, speeduhc);
command!(tnt_games::TntGames, tnt_games::TntGamesMode, tntgames);
command!(
	turbo_kart_racers::TurboKartRacers,
	turbo_kart_racers::TurboKartRacersMode,
	turbokartracers
);
command!(uhc::Uhc, uhc::UhcMode, uhc);
command!(vampire_z::VampireZ, vampire_z::VampireZMode, vampirez);
command!(walls::Walls, walls::WallsMode, walls);
command!(warlords::Warlords, warlords::WarlordsMode, warlords);
command!(wool_wars::WoolWars, wool_wars::WoolWarsMode, woolwars);
