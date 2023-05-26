use futures::StreamExt;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use translate::{Context, Error};

macro_rules! generate_command {
	($game: ty, $mode: ty, $fn: ident) => {
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
			#[min = 1i64]
			hours: Option<i64>,
			#[min = 1i64]
			days: Option<i64>,
			#[min = 1i64]
			weeks: Option<i64>,
		) -> ::std::result::Result<(), ::translate::Error> {
			ctx.defer().await?;

			let (format, player, mut data, session) = $crate::get_data!(ctx, uuid, username);

			let mut duration = ::chrono::Duration::min_value();

			if let Some(hours) = hours {
				duration = duration + ::chrono::Duration::hours(hours);
			}

			if let Some(days) = days {
				duration = duration + ::chrono::Duration::days(days);
			}

			if let Some(weeks) = weeks {
				duration = duration + ::chrono::Duration::weeks(weeks);
			}

			if duration.is_zero() {
				duration = ::chrono::Duration::weeks(1);
			}

			let status =
				$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - duration).await?;

			player.increase_searches(ctx).await?;

			let $crate::snapshot::user::Status::Found((ref snapshot, created_at)) = status else {
				let content = ::translate::tr_fmt!(
					ctx, "no-previous-statistics",
					name: $crate::util::escape_username(&player.username),
				);

				ctx.send(move |m| {
					m.content(content)
				})
				.await?;

				return Ok(());
			};

			let content = ::translate::tr_fmt!(
				ctx, "showing-statistics",
				from: ::std::format!("<t:{}:f>", created_at.timestamp()),
				to: ::std::format!("<t:{}:f>", ::chrono::Utc::now().timestamp()),
			);

			match format {
				$crate::format::Display::Image | $crate::format::Display::Compact => {
					let png: ::std::option::Option<::std::borrow::Cow<[u8]>> =
						if let $crate::snapshot::user::Status::Found((ref snapshot, _)) = status {
							let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, mode);

							::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
						} else {
							::std::option::Option::None
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
				}
				$crate::format::Display::Text => {
					let mut embed = <$game>::embed_diff(ctx, &player, snapshot, &mut data, &session);

					embed.colour($crate::EMBED_COLOUR);

					ctx.send(|m| {
						m.content(content);
						m.embeds.push(embed);
						m
					})
					.await?;
				}
			}

			Ok(())
		}
	};
}

macro_rules! generate_large_command {
	($game: ty, $mode: ty, $fn: ident) => {
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
			#[min = 1i64]
			hours: Option<i64>,
			#[min = 1i64]
			days: Option<i64>,
			#[min = 1i64]
			weeks: Option<i64>,
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			ctx.defer().await?;

			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let (format, player, mut data, session) = $crate::get_data!(ctx, uuid, username);

			let mut duration = ::chrono::Duration::min_value();

			if let Some(hours) = hours {
				duration = duration + ::chrono::Duration::hours(hours);
			}

			if let Some(days) = days {
				duration = duration + ::chrono::Duration::days(days);
			}

			if let Some(weeks) = weeks {
				duration = duration + ::chrono::Duration::weeks(weeks);
			}

			if duration.is_zero() {
				duration = ::chrono::Duration::weeks(1);
			}

			let status =
				$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - duration).await?;

			player.increase_searches(ctx).await?;

			let $crate::snapshot::user::Status::Found((ref snapshot, created_at)) = status else {
				let content = ::translate::tr_fmt!(
					ctx, "no-previous-statistics",
					name: $crate::util::escape_username(&player.username),
				);

				ctx.send(move |m| {
					m.content(content)
				})
				.await?;

				return Ok(());
			};

			let content = ::translate::tr_fmt!(
				ctx, "showing-statistics",
				from: ::std::format!("<t:{}:f>", created_at.timestamp()),
				to: ::std::format!("<t:{}:f>", ::chrono::Utc::now().timestamp()),
			);

			match format {
				$crate::format::Display::Image | $crate::format::Display::Compact => {
					let png: ::std::option::Option<::std::borrow::Cow<[u8]>> =
						if let $crate::snapshot::user::Status::Found((ref snapshot, _)) = status {
							let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, mode);

							::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
						} else {
							::std::option::Option::None
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
				}
				$crate::format::Display::Text => {
					let mut embed = <$game>::embed_diff(ctx, &player, snapshot, &mut data, &session);

					embed.colour($crate::EMBED_COLOUR);

					ctx.send(|m| {
						m.content(content);
						m.embeds.push(embed);
						m
					})
					.await?;
				}
			}

			Ok(())
		}
	};
}

macro_rules! generate_guild_command {
	($fn: ident) => {
		#[allow(clippy::too_many_lines)]
		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
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
			#[min = 1i64]
			hours: Option<i64>,
			#[min = 1i64]
			days: Option<i64>,
			#[min = 1i64]
			weeks: Option<i64>,
		) -> ::std::result::Result<(), ::translate::Error> {
			ctx.defer().await?;

			let mut guild = match $crate::util::get_guild_from_input(ctx, ctx.author(), name, uuid, username).await {
				::std::result::Result::Ok(guild) => guild,
				::std::result::Result::Err(::translate::Error::NotLinked) => {
					ctx.send(|m| $crate::util::error_embed(m, ::translate::tr!(ctx, "not-linked"), ::translate::tr!(ctx, "not-linked")))
						.await?;

					return Ok(());
				}
				::std::result::Result::Err(e) => return ::std::result::Result::Err(e),
			};

			let mut duration = ::chrono::Duration::min_value();

			if let Some(hours) = hours {
				duration = duration + ::chrono::Duration::hours(hours);
			}

			if let Some(days) = days {
				duration = duration + ::chrono::Duration::days(days);
			}

			if let Some(weeks) = weeks {
				duration = duration + ::chrono::Duration::weeks(weeks);
			}

			if duration.is_zero() {
				duration = ::chrono::Duration::weeks(1);
			}

			let after = ::chrono::Utc::now() - duration;
			let status = $crate::snapshot::guild::get_or_insert(ctx, &guild, after).await?;
			let guilds = $crate::commands::guild::get_snapshots_multiple_of_weekday(ctx, &guild, after).await?;
			let xp_since = $crate::commands::guild::get_monthly_xp(&guild, &guilds);

			guild.increase_searches(ctx).await?;

			let daily_xp = guild.members
				.iter()
				.map(|g| g.xp_history[1].1)
				.sum::<u32>();

			let weekly_xp = guild.members
				.iter()
				.map(|g| g.xp_history.iter().map(|h| h.1).sum::<u32>())
				.sum::<u32>();

			$crate::commands::guild::apply_member_xp(&mut guild, &guilds);

			let data = if let ::std::option::Option::Some(leader) = guild.get_leader() {
				let player = leader.get_player_unchecked();

				::std::option::Option::Some(player.get_data().await?)
			} else {
				::std::option::Option::None
			};

			guild
				.members
				.sort_by_cached_key(|m| m.xp_history.iter().map(|h| h.1).sum::<u32>());

			let members = futures::stream::iter(
				guild
					.members
					.iter()
					.rev()
					.take(14)
					.map(::api::guild::member::Member::get_player_unchecked)
					.map(::api::player::Player::get_data_owned),
			)
			.buffered(14)
			.filter_map(|r| async { r.ok() })
			.collect::<Vec<_>>()
			.await;

			let png: ::std::option::Option<::std::borrow::Cow<_>> = if let $crate::snapshot::guild::Status::Found((ref snapshot, _)) = status {
				let diff = ::api::canvas::diff::Diff::diff(&guild, snapshot);

				guild.coins = diff.coins;
				guild.xp = diff.xp;
				guild
					.xp_by_game
					.iter_mut()
					.zip(&snapshot.xp_by_game)
					.for_each(|(a, b)| (*a).1 -= b.1);

				let mut surface = ::api::canvas::guild::create_surface();

				if let Some(ref data) = data {
					::api::canvas::guild::leader(&mut surface, data);
				}

				::api::canvas::guild::members(ctx, &mut surface, &guild, members.as_slice());
				::api::canvas::guild::header(&mut surface, &guild);
				::api::canvas::guild::games(ctx, &mut surface, &mut guild);
				::api::canvas::guild::stats_history(ctx, &mut surface, &guild, daily_xp, weekly_xp, xp_since);
				::api::canvas::guild::level(ctx, &mut surface, &guild);
				::api::canvas::guild::preferred_games(&mut surface, &guild);

				::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
			} else {
				::std::option::Option::None
			};

			let content = match status {
				$crate::snapshot::guild::Status::Found((_, created_at)) => ::translate::tr_fmt!(
					ctx, "showing-guild-statistics",
					from: ::std::format!("<t:{}:f>", created_at.timestamp()),
					to: ::std::format!("<t:{}:f>", ::chrono::Utc::now().timestamp()),
				),
				$crate::snapshot::guild::Status::Inserted => ::translate::tr_fmt!(
					ctx, "no-previous-guild-statistics",
					name: guild.name,
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

generate_command!(arcade::Arcade, arcade::ArcadeMode, arcade);
generate_command!(arena::Arena, arena::ArenaMode, arena);
generate_command!(bed_wars::BedWars, bed_wars::BedWarsMode, bedwars);
generate_command!(blitz_sg::BlitzSg, blitz_sg::BlitzSgMode, blitz);
generate_command!(
	build_battle::BuildBattle,
	build_battle::BuildBattleMode,
	buildbattle
);
generate_command!(
	cops_and_crims::CopsAndCrims,
	cops_and_crims::CopsAndCrimsMode,
	copsandcrims
);
generate_large_command!(duels::Duels, duels::DuelsMode, duels);
generate_command!(mega_walls::MegaWalls, mega_walls::MegaWallsMode, megawalls);
generate_command!(
	murder_mystery::MurderMystery,
	murder_mystery::MurderMysteryMode,
	murdermystery
);
generate_command!(paintball::Paintball, paintball::PaintballMode, paintball);
generate_command!(pit::Pit, pit::PitMode, pit);
generate_command!(quake::Quake, quake::QuakeMode, quake);
generate_command!(sky_wars::SkyWars, sky_wars::SkyWarsMode, skywars);
generate_command!(
	smash_heroes::SmashHeroes,
	smash_heroes::SmashHeroesMode,
	smash
);
generate_command!(speed_uhc::SpeedUhc, speed_uhc::SpeedUhcMode, speeduhc);
generate_command!(tnt_games::TntGames, tnt_games::TntGamesMode, tntgames);
generate_command!(
	turbo_kart_racers::TurboKartRacers,
	turbo_kart_racers::TurboKartRacersMode,
	turbokartracers
);
generate_command!(uhc::Uhc, uhc::UhcMode, uhc);
generate_command!(vampire_z::VampireZ, vampire_z::VampireZMode, vampirez);
generate_command!(walls::Walls, walls::WallsMode, walls);
generate_command!(warlords::Warlords, warlords::WarlordsMode, warlords);
generate_command!(wool_wars::WoolWars, wool_wars::WoolWarsMode, woolwars);
generate_guild_command!(guild);

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
		"turbokartracers",
		"uhc",
		"vampirez",
		"walls",
		"warlords",
		"woolwars",
		"guild",
	)
)]
#[allow(clippy::unused_async)]
pub async fn from(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
