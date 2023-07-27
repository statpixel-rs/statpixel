pub mod run;

use translate::context;

macro_rules! generate_history_command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[allow(clippy::too_many_lines)]
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES"
		)]
		pub async fn $fn(
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

			run::command::<$game>(ctx, from, to, mode, None, None).await
		}
	};
}

macro_rules! generate_large_history_command {
	($game: ty, $mode: ty, $fn: ident) => {
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

			run::command::<$game>(ctx, from, to, mode, None, None).await
		}
	};
}

generate_history_command!(
	::api::player::stats::arcade::Arcade,
	::api::player::stats::arcade::ArcadeMode,
	arcade
);
generate_history_command!(
	::api::player::stats::arena::Arena,
	::api::player::stats::arena::ArenaMode,
	arena
);
generate_history_command!(
	::api::player::stats::bed_wars::BedWars,
	::api::player::stats::bed_wars::BedWarsMode,
	bedwars
);
generate_history_command!(
	::api::player::stats::blitz_sg::BlitzSg,
	::api::player::stats::blitz_sg::BlitzSgMode,
	blitz
);
generate_history_command!(
	::api::player::stats::build_battle::BuildBattle,
	::api::player::stats::build_battle::BuildBattleMode,
	buildbattle
);
generate_history_command!(
	::api::player::stats::cops_and_crims::CopsAndCrims,
	::api::player::stats::cops_and_crims::CopsAndCrimsMode,
	copsandcrims
);
generate_large_history_command!(
	::api::player::stats::duels::Duels,
	::api::player::stats::duels::DuelsMode,
	duels
);
generate_history_command!(
	::api::player::stats::mega_walls::MegaWalls,
	::api::player::stats::mega_walls::MegaWallsMode,
	megawalls
);
generate_history_command!(
	::api::player::stats::murder_mystery::MurderMystery,
	::api::player::stats::murder_mystery::MurderMysteryMode,
	murdermystery
);
generate_history_command!(
	::api::player::stats::paintball::Paintball,
	::api::player::stats::paintball::PaintballMode,
	paintball
);
generate_history_command!(
	::api::player::stats::pit::Pit,
	::api::player::stats::pit::PitMode,
	pit
);
generate_history_command!(
	::api::player::stats::quake::Quake,
	::api::player::stats::quake::QuakeMode,
	quake
);
generate_history_command!(
	::api::player::stats::sky_wars::SkyWars,
	::api::player::stats::sky_wars::SkyWarsMode,
	skywars
);
generate_history_command!(
	::api::player::stats::smash_heroes::SmashHeroes,
	::api::player::stats::smash_heroes::SmashHeroesMode,
	smash
);
generate_history_command!(
	::api::player::stats::speed_uhc::SpeedUhc,
	::api::player::stats::speed_uhc::SpeedUhcMode,
	speeduhc
);
generate_history_command!(
	::api::player::stats::tnt_games::TntGames,
	::api::player::stats::tnt_games::TntGamesMode,
	tntgames
);
generate_history_command!(
	::api::player::stats::turbo_kart_racers::TurboKartRacers,
	::api::player::stats::turbo_kart_racers::TurboKartRacersMode,
	turbokartracers
);
generate_history_command!(
	::api::player::stats::uhc::Uhc,
	::api::player::stats::uhc::UhcMode,
	uhc
);
generate_history_command!(
	::api::player::stats::vampire_z::VampireZ,
	::api::player::stats::vampire_z::VampireZMode,
	vampirez
);
generate_history_command!(
	::api::player::stats::walls::Walls,
	::api::player::stats::walls::WallsMode,
	walls
);
generate_history_command!(
	::api::player::stats::warlords::Warlords,
	::api::player::stats::warlords::WarlordsMode,
	warlords
);
generate_history_command!(
	::api::player::stats::wool_wars::WoolWars,
	::api::player::stats::wool_wars::WoolWarsMode,
	woolwars
);

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
	)
)]
#[allow(clippy::unused_async)]
pub async fn compare(_ctx: translate::Context<'_>) -> Result<(), translate::Error> {
	Ok(())
}
