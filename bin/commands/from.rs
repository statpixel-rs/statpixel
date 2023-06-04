use futures::StreamExt;

#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

use translate::{Context, Error};

macro_rules! generate_command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[allow(clippy::too_many_lines)]
		#[poise::command(on_error = "crate::util::error_handler", slash_command, required_bot_permissions = "ATTACH_FILES")]
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
			let format = $crate::util::get_format_from_input(ctx).await;

			let mut duration = ::chrono::Duration::zero();

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

			match format {
				$crate::format::Display::Image | $crate::format::Display::Compact => {
					let (player, mut data, session, skin, suffix) = $crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;
					let ctx_id = ctx.id();
					let status =
						$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - duration).await?;

					player.increase_searches(ctx).await?;

					let $crate::snapshot::user::Status::Found((ref snapshot, created_at)) = status else {
						let content = ::translate::tr_fmt!(
							ctx, "no-previous-statistics",
							name: $crate::util::escape_username(&data.username),
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

					let png: ::std::borrow::Cow<[u8]> = {
						let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, skin.as_ref(), mode, suffix.as_deref());

						::api::canvas::to_png(&mut surface).into()
					};

					ctx.send(move |m| {
						m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
							data: png,
							filename: "canvas.png".into(),
						});
						m.components = Some(<$mode>::as_components(ctx));
						m.content(content)
					})
					.await?;

					while let Some(press) =
						::poise::serenity_prelude::CollectComponentInteraction::new(ctx)
							.filter(move |press| press.data.custom_id.eq(&ctx_id.to_string()))
							.timeout(std::time::Duration::from_secs(60 * 5))
							.await
					{
						let mode = &press.data.values.first().unwrap();
						let mode = <$mode>::from_u8_str(mode.as_str());

						let (mut data, session, skin, suffix) = $crate::commands::get_from_player_data_session_skin_suffix(ctx, &player).await?;

						let content = ::translate::tr_fmt!(
							ctx, "showing-statistics",
							from: ::std::format!("<t:{}:f>", created_at.timestamp()),
							to: ::std::format!("<t:{}:f>", ::chrono::Utc::now().timestamp()),
						);

						let png: ::std::borrow::Cow<[u8]> = {
							let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, skin.as_ref(), Some(mode), suffix.as_deref());

							::api::canvas::to_png(&mut surface).into()
						};

						press
							.create_interaction_response(ctx, |b| {
								b.kind(::poise::serenity_prelude::InteractionResponseType::UpdateMessage).interaction_response_data(|m| {
									m.files([::poise::serenity_prelude::AttachmentType::Bytes {
										data: png,
										filename: "canvas.png".into(),
									}]);
									m.set_components(<$mode>::as_components(ctx));
									m.content(content);
									m
								})
							})
							.await?;
					}
				}
				$crate::format::Display::Text => {
					let (player, mut data) = $crate::commands::get_player_data(ctx, uuid, username).await?;
					let status =
						$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - duration).await?;

					player.increase_searches(ctx).await?;

					let $crate::snapshot::user::Status::Found((ref snapshot, created_at)) = status else {
						let content = ::translate::tr_fmt!(
							ctx, "no-previous-statistics",
							name: $crate::util::escape_username(&data.username),
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

					let mut embed = <$game>::embed_diff(ctx, &player, snapshot, &mut data);

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

		#[allow(clippy::too_many_lines)]
		#[poise::command(on_error = "crate::util::error_handler", slash_command, required_bot_permissions = "ATTACH_FILES")]
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
			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let format = $crate::util::get_format_from_input(ctx).await;

			let mut duration = ::chrono::Duration::zero();

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

			match format {
				$crate::format::Display::Image | $crate::format::Display::Compact => {
					let (player, mut data, session, skin, suffix) = $crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;
					let ctx_id = ctx.id();
					let status =
						$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - duration).await?;

					player.increase_searches(ctx).await?;

					let $crate::snapshot::user::Status::Found((ref snapshot, created_at)) = status else {
						let content = ::translate::tr_fmt!(
							ctx, "no-previous-statistics",
							name: $crate::util::escape_username(player.username.as_deref().unwrap()),
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

					let png: ::std::borrow::Cow<[u8]> = {
						let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, skin.as_ref(), mode, suffix.as_deref());

						::api::canvas::to_png(&mut surface).into()
					};

					ctx.send(move |m| {
						m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
							data: png,
							filename: "canvas.png".into(),
						});
						m.components = Some(<$mode>::as_components(ctx));
						m.content(content)
					})
					.await?;

					while let Some(press) =
						::poise::serenity_prelude::CollectComponentInteraction::new(ctx)
							.filter(move |press| press.data.custom_id.eq(&ctx_id.to_string()))
							.timeout(std::time::Duration::from_secs(60 * 5))
							.await
					{
						let mode = &press.data.values.first().unwrap();
						let mode = <$mode>::from_u8_str(mode.as_str());

						let (mut data, session, skin, suffix) = $crate::commands::get_from_player_data_session_skin_suffix(ctx, &player).await?;

						let content = ::translate::tr_fmt!(
							ctx, "showing-statistics",
							from: ::std::format!("<t:{}:f>", created_at.timestamp()),
							to: ::std::format!("<t:{}:f>", ::chrono::Utc::now().timestamp()),
						);

						let png: ::std::borrow::Cow<[u8]> = {
							let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, skin.as_ref(), Some(mode), suffix.as_deref());

							::api::canvas::to_png(&mut surface).into()
						};

						press
							.create_interaction_response(ctx, |b| {
								b.kind(::poise::serenity_prelude::InteractionResponseType::UpdateMessage).interaction_response_data(|m| {
									m.files([::poise::serenity_prelude::AttachmentType::Bytes {
										data: png,
										filename: "canvas.png".into(),
									}]);
									m.set_components(<$mode>::as_components(ctx));
									m.content(content);
									m
								})
							})
							.await?;
					}
				}
				$crate::format::Display::Text => {
					let (player, mut data) = $crate::commands::get_player_data(ctx, uuid, username).await?;
					let status =
						$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - duration).await?;

					player.increase_searches(ctx).await?;

					let $crate::snapshot::user::Status::Found((ref snapshot, created_at)) = status else {
						let content = ::translate::tr_fmt!(
							ctx, "no-previous-statistics",
							name: $crate::util::escape_username(&data.username),
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

					let mut embed = <$game>::embed_diff(ctx, &player, snapshot, &mut data);

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
		#[poise::command(on_error = "crate::util::error_handler", slash_command, required_bot_permissions = "ATTACH_FILES")]
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
			let mut guild = match $crate::commands::get_guild(ctx, name, uuid, username).await {
				::std::result::Result::Ok(guild) => guild,
				::std::result::Result::Err(::translate::Error::NotLinked) => {
					ctx.send(|m| $crate::util::error_embed(m, ::translate::tr!(ctx, "not-linked"), ::translate::tr!(ctx, "not-linked")))
						.await?;

					return Ok(());
				}
				::std::result::Result::Err(e) => return ::std::result::Result::Err(e),
			};

			let mut duration = ::chrono::Duration::zero();

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
					.map(::api::player::Player::get_display_string_owned),
			)
			.buffered(14)
			.filter_map(|r| async { r.ok() })
			.collect::<::std::vec::Vec<_>>();

			let leader = guild
				.get_leader()
				.map(|m| m.get_player_unchecked().get_display_string_owned());

			let (members, leader) = if let Some(leader) = leader {
				let (members, leader) = ::tokio::join!(members, leader);

				(members, Some(leader.map_err(::std::sync::Arc::new)?))
			} else {
				(members.await, None)
			};

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

				if let Some(leader) = leader {
					::api::canvas::guild::leader(&mut surface, &::minecraft::text::parse::minecraft_string(&leader).collect::<::std::vec::Vec<_>>());
				}

				::api::canvas::guild::members(ctx, &mut surface, &guild, members.as_slice());
				::api::canvas::guild::header(&mut surface, &guild);
				::api::canvas::guild::games(ctx, &mut surface, &mut guild);
				::api::canvas::guild::stats_history(ctx, &mut surface, &guild, daily_xp, weekly_xp, xp_since);
				::api::canvas::guild::level(ctx, &mut surface, &guild);
				::api::canvas::guild::preferred_games(&mut surface, &guild);

				Some(::api::canvas::to_png(&mut surface).into())
			} else {
				None
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
				if let Some(png) = png {
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
		"guild",
	)
)]
#[allow(clippy::unused_async)]
pub async fn from(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
