#[macro_export]
macro_rules! generate_history_command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr) => {
		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			mode: Option<$mode>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let (player, mut data, session) = $crate::get_data!(ctx, uuid, username);
			let status =
				$crate::snapshot::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - $duration)?;

			let png: ::std::option::Option<::std::borrow::Cow<[u8]>> =
				if let $crate::snapshot::Status::Found((ref snapshot, _)) = status {
					let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, mode);

					::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
				} else {
					::std::option::Option::None
				};

			let content = match status {
				$crate::snapshot::Status::Found((_, created_at)) => format!(
					"Showing statistics change from <t:{}:f> to <t:{}:f>",
					created_at.timestamp(),
					::chrono::Utc::now().timestamp(),
				),
				$crate::snapshot::Status::Inserted => format!(
					"No previous data found for **{}**, so it has been inserted.\nShowing statistics change from <t:{}:f> to <t:{}:f>",
					$crate::util::escape_username(&player.username),
					::chrono::Utc::now().timestamp(),
					::chrono::Utc::now().timestamp(),
				),
			};

			ctx.send(move |m| {
				if let ::std::option::Option::Some(png) = png {
					m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
						data: png,
						filename: "canvas.png".into(),
					});
				}

				m.content(content)
			})
			.await?;

			Ok(())
		}
	};
}

#[macro_export]
macro_rules! generate_large_history_command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr) => {
		async fn autocomplete_mode<'a>(
			ctx: $crate::Context<'a>,
			partial: &'a str,
		) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
			let partial = partial.to_ascii_lowercase();

			<$game>::autocomplete(ctx, partial).await
		}

		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
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
			let (player, mut data, session) = $crate::get_data!(ctx, uuid, username);
			let status =
				$crate::snapshot::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - $duration)?;

			let png: ::std::option::Option<::std::borrow::Cow<[u8]>> =
				if let $crate::snapshot::Status::Found((ref snapshot, _)) = status {
					let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, mode);

					::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
				} else {
					::std::option::Option::None
				};

			let content = match status {
				$crate::snapshot::Status::Found((_, created_at)) => ::translate::tr_fmt!(
					ctx, "showing-statistics",
					from: ::std::format!("<t:{}:f>", created_at.timestamp()),
					to: ::std::format!("<t:{}:f>", ::chrono::Utc::now().timestamp()),
				),
				$crate::snapshot::Status::Inserted => ::translate::tr_fmt!(
					ctx, "no-previous-statistics",
					name: $crate::util::escape_username(&player.username),
				),
			};

			ctx.send(move |m| {
				if let ::std::option::Option::Some(png) = png {
					m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
						data: png,
						filename: "canvas.png".into(),
					});
				}

				m.content(content)
			})
			.await?;

			Ok(())
		}
	};
}

#[macro_export]
macro_rules! generate_history_commands {
	($fn: ident, $duration: expr) => {
		pub mod $fn {
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
				turkokartracers,
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

			#[poise::command(
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
					"turkokartracers",
					"uhc",
					"vampirez",
					"walls",
					"warlords",
					"woolwars"
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
