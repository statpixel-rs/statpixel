use api::guild::Guild;
use api::player::data::Data;

macro_rules! generate_history_command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr) => {
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
		) -> ::std::result::Result<(), ::translate::Error> {
			let (format, background) = $crate::util::get_format_colour_from_input(ctx).await;

			match format {
				$crate::format::Display::Image | $crate::format::Display::Compact => {
					let (player, data, session, skin, suffix) = $crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;
					let ctx_id = ctx.id();

					player.increase_searches(ctx).await?;

					let status =
						$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - $duration).await?;

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
						let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut Data::clone(&data), &session, skin.as_ref(), mode, suffix.as_deref(), background);

						::api::canvas::to_png(&mut surface).into()
					};

					ctx.send(move |m| {
						m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
							data: png,
							filename: crate::IMAGE_NAME.into(),
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

						let (data, session, skin, suffix) = $crate::commands::get_from_player_data_session_skin_suffix(ctx, &player).await?;

						let content = ::translate::tr_fmt!(
							ctx, "showing-statistics",
							from: ::std::format!("<t:{}:f>", created_at.timestamp()),
							to: ::std::format!("<t:{}:f>", ::chrono::Utc::now().timestamp()),
						);

						let png: ::std::borrow::Cow<[u8]> = {
							let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut Data::clone(&data), &session, skin.as_ref(), Some(mode), suffix.as_deref(), background);

							::api::canvas::to_png(&mut surface).into()
						};

						press
							.create_interaction_response(ctx, |b| {
								b.kind(::poise::serenity_prelude::InteractionResponseType::UpdateMessage).interaction_response_data(|m| {
									m.files([::poise::serenity_prelude::AttachmentType::Bytes {
										data: png,
										filename: crate::IMAGE_NAME.into(),
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
					let (player, data) = $crate::commands::get_player_data(ctx, uuid, username).await?;

					player.increase_searches(ctx).await?;

					let status =
						$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - $duration).await?;

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

					let mut embed = <$game>::embed_diff(ctx, &player, snapshot, &mut Data::clone(&data));

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

macro_rules! generate_large_history_command {
	($game: ty, $mode: ty, $fn: ident, $duration: expr) => {
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
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let (format, background) = $crate::util::get_format_colour_from_input(ctx).await;

			match format {
				$crate::format::Display::Image | $crate::format::Display::Compact => {
					let (player, data, session, skin, suffix) = $crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;
					let ctx_id = ctx.id();

					player.increase_searches(ctx).await?;

					let status =
						$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - $duration).await?;

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
						let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut Data::clone(&data), &session, skin.as_ref(), mode, suffix.as_deref(), background);

						::api::canvas::to_png(&mut surface).into()
					};

					ctx.send(move |m| {
						m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
							data: png,
							filename: crate::IMAGE_NAME.into(),
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

						let (data, session, skin, suffix) = $crate::commands::get_from_player_data_session_skin_suffix(ctx, &player).await?;

						let content = ::translate::tr_fmt!(
							ctx, "showing-statistics",
							from: ::std::format!("<t:{}:f>", created_at.timestamp()),
							to: ::std::format!("<t:{}:f>", ::chrono::Utc::now().timestamp()),
						);

						let png: ::std::borrow::Cow<[u8]> = {
							let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut Data::clone(&data), &session, skin.as_ref(), Some(mode), suffix.as_deref(), background);

							::api::canvas::to_png(&mut surface).into()
						};

						press
							.create_interaction_response(ctx, |b| {
								b.kind(::poise::serenity_prelude::InteractionResponseType::UpdateMessage).interaction_response_data(|m| {
									m.files([::poise::serenity_prelude::AttachmentType::Bytes {
										data: png,
										filename: crate::IMAGE_NAME.into(),
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
					let (player, data) = $crate::commands::get_player_data(ctx, uuid, username).await?;

					player.increase_searches(ctx).await?;

					let status =
						$crate::snapshot::user::get_or_insert(ctx, &player, &data, ::chrono::Utc::now() - $duration).await?;

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

					let mut embed = <$game>::embed_diff(ctx, &player, snapshot, &mut Data::clone(&data));

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

macro_rules! generate_guild_history_command {
	($fn: ident, $duration: expr) => {
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
		) -> ::std::result::Result<(), ::translate::Error> {
			let (_, background) = $crate::util::get_format_colour_from_input(ctx).await;
			let guild = match $crate::commands::get_guild(ctx, name, uuid, username).await {
				::std::result::Result::Ok(guild) => guild,
				::std::result::Result::Err(::translate::Error::NotLinked) => {
					ctx.send(|m| $crate::util::error_embed(m, ::translate::tr!(ctx, "not-linked"), ::translate::tr!(ctx, "not-linked")))
						.await?;

					return Ok(());
				}
				::std::result::Result::Err(e) => return ::std::result::Result::Err(e),
			};

			let after = ::chrono::Utc::now() - $duration;
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

			$crate::commands::guild::apply_member_xp(&mut Guild::clone(&guild), &guilds);

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

				(members, Some(leader?))
			} else {
				(members.await, None)
			};

			let png: ::std::option::Option<::std::borrow::Cow<_>> = if let $crate::snapshot::guild::Status::Found((ref snapshot, _)) = status {
				let diff = ::api::canvas::diff::Diff::diff(&guild, snapshot);
				let mut guild = Guild::clone(&guild);

				guild.coins = diff.coins;
				guild.xp = diff.xp;
				guild
					.xp_by_game
					.iter_mut()
					.for_each(|a| {
						let b = snapshot.xp_by_game.iter().find(|x| x.0 == a.0).unwrap();

						if a.1 > b.1 {
							(*a).1 -= b.1;
						} else {
							(*a).1 = api::xp::Xp(0);
						}
					});

					guild.xp_by_game.sort_unstable_by_key(|g| g.1);

				let level = ::minecraft::calc::guild::get_level(guild.xp);
				let progress = ::api::canvas::shape::WideBubbleProgress(
					::minecraft::calc::guild::get_level_progress(guild.xp),
					[::minecraft::colour::Colour::Gold.into(), ::minecraft::colour::Colour::Gold.into()],
				);

				let mut canvas = ::api::canvas::Canvas::new(720.)
					.gap(7.)
					.push_down(&::api::canvas::shape::Title, ::api::canvas::shape::Title::from_guild(&guild))
					.push_down(
						&::api::canvas::shape::Subtitle,
						if let Some(leader) = leader {
							::api::canvas::body::Body::new(20., ::skia_safe::textlayout::TextAlign::Center)
								.extend_owned(::minecraft::text::parse::minecraft_string(&leader))
								.build()
						} else {
							::api::canvas::body::Body::new(20., ::skia_safe::textlayout::TextAlign::Center)
								.append(::minecraft::text::Text {
									text: ::translate::tr!(ctx, "none").as_ref(),
									paint: ::minecraft::paint::Paint::Gray,
									font: ::minecraft::style::MinecraftFont::Bold,
									..Default::default()
								})
								.build()
						},
					)
					.push_down_post_draw(
						&progress,
						::api::canvas::shape::WideBubbleProgress::from_level_progress(
							ctx,
							&format!("{}6{}", ::minecraft::text::ESCAPE, level.0),
							&::minecraft::calc::guild::get_curr_level_xp(guild.xp),
							&::minecraft::calc::guild::get_level_xp(guild.xp),
						),
					)
					.push_right_start(&::api::canvas::shape::Sidebar, ::api::canvas::shape::Sidebar::from_guild(ctx, &guild))
					.push_right_post_draw(
						&::api::canvas::shape::PreferredGames(&guild.preferred_games),
						::api::canvas::body::Body::empty(),
					)
					.push_down_start(
						&::api::canvas::shape::Bubble,
						::api::canvas::body::Body::from_bubble(ctx, &guild.coins, ::translate::tr!(ctx, "coins").as_ref(), ::minecraft::paint::Paint::Gold),
					)
					.push_right(
						&::api::canvas::shape::Bubble,
						::api::canvas::body::Body::new(30., ::skia_safe::textlayout::TextAlign::Center)
							.extend(&[
								::minecraft::text::Text {
									text: ::translate::tr!(ctx, "created-at").as_ref(),
									paint: ::minecraft::paint::Paint::Aqua,
									font: ::minecraft::style::MinecraftFont::Normal,
									size: Some(20.),
								},
								::minecraft::text::Text {
									text: "\n",
									size: Some(20.),
									..Default::default()
								},
								::minecraft::text::Text {
									text: &::api::canvas::label::ToFormatted::to_formatted_label(&guild.created_at, ctx),
									paint: ::minecraft::paint::Paint::Aqua,
									font: ::minecraft::style::MinecraftFont::Normal,
									size: None,
								},
							])
							.build(),
					)
					.push_right(
						&::api::canvas::shape::Bubble,
						::api::canvas::body::Body::from_bubble(
							ctx,
							&format!("{}/125", guild.members.len()),
							::translate::tr!(ctx, "members").as_ref(),
							::minecraft::paint::Paint::LightPurple,
						),
					)
					.push_down_start(
						&::api::canvas::shape::Bubble,
						::api::canvas::body::Body::from_bubble(
							ctx,
							&daily_xp,
							::translate::tr!(ctx, "daily-xp").as_ref(),
							::minecraft::paint::Paint::DarkGreen,
						),
					)
					.push_right(
						&::api::canvas::shape::Bubble,
						::api::canvas::body::Body::from_bubble(
							ctx,
							&weekly_xp,
							::translate::tr!(ctx, "weekly-xp").as_ref(),
							::minecraft::paint::Paint::DarkGreen,
						),
					)
					.push_right(
						&::api::canvas::shape::Bubble,
						::api::canvas::body::Body::from_bubble(
							ctx,
							&xp_since,
							::translate::tr!(ctx, "xp-since").as_ref(),
							::minecraft::paint::Paint::DarkGreen,
						),
					)
					.push_down_start(
						&::api::canvas::shape::WideTallBubble,
						::api::canvas::shape::WideTallBubble::from_guild(ctx, &guild, members.as_slice(), 0),
					)
					.push_right(
						&::api::canvas::shape::WideTallBubble,
						::api::canvas::shape::WideTallBubble::from_guild(ctx, &guild, members.as_slice(), 1),
					)
					.build(None, background)
					.unwrap();

				Some(::api::canvas::to_png(&mut canvas).into())
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
					name: guild.name.as_str(),
				),
			};

			ctx.send(move |m| {
				if let Some(png) = png {
					m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
						data: png,
						filename: crate::IMAGE_NAME.into(),
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
			use super::*;
			use futures::StreamExt;

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
				turbokartracers,
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
			generate_guild_history_command!(guild, $duration);

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
					"guild"
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
