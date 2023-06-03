#[allow(clippy::wildcard_imports)]
use api::player::stats::*;

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
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			ctx.defer().await?;

			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let format = $crate::util::get_format_from_input(ctx).await;

			match format {
				// TODO: Add compact format support
				$crate::format::Display::Image | $crate::format::Display::Compact => {
					let (player, data, session, skin) = $crate::get_all!(ctx, uuid, username);

					player.increase_searches(ctx).await?;

					let png: ::std::borrow::Cow<[u8]> = {
						let mut surface =
							<$game>::canvas(ctx, &data, &session, skin.as_ref(), mode);

						::api::canvas::to_png(&mut surface).into()
					};

					ctx.send(move |m| {
						m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
							data: png,
							filename: "canvas.png".into(),
						});
						m.components = Some(<$mode>::as_components(ctx));
						m
					})
					.await?;

					let ctx_id = ctx.id();

					while let Some(press) =
						::poise::serenity_prelude::CollectComponentInteraction::new(ctx)
							.filter(move |press| press.data.custom_id.eq(&ctx_id.to_string()))
							.timeout(std::time::Duration::from_secs(60 * 5))
							.await
					{
						let mode = &press.data.values.first().unwrap();
						let mode = <$mode>::from_u8_str(mode.as_str());

						let (data, session, skin) = $crate::get_from_player!(ctx, player);

						let png: ::std::borrow::Cow<[u8]> = {
							let mut surface =
								<$game>::canvas(ctx, &data, &session, skin.as_ref(), Some(mode));

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
									m
								})
							})
							.await?;
					}
				}
				$crate::format::Display::Text => {
					let (player, data) = $crate::get_data!(ctx, uuid, username);

					player.increase_searches(ctx).await?;

					let mut embed = <$game>::embed(ctx, &player, &data);

					embed.colour($crate::EMBED_COLOUR);

					ctx.send(|m| {
						m.embeds.push(embed);
						m
					})
					.await?;

					return Ok(());
				}
			}

			Ok(())
		}
	};
}

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
		) -> ::std::result::Result<(), ::translate::Error> {
			ctx.defer().await?;

			let format = $crate::util::get_format_from_input(ctx).await;

			match format {
				// TODO: Add compact format support
				$crate::format::Display::Image | $crate::format::Display::Compact => {
					let (player, data, session, skin) = $crate::get_all!(ctx, uuid, username);
					let ctx_id = ctx.id();

					let png: ::std::borrow::Cow<[u8]> = {
						let mut surface =
							<$game>::canvas(ctx, &data, &session, skin.as_ref(), mode);

						::api::canvas::to_png(&mut surface).into()
					};

					ctx.send(move |m| {
						m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
							data: png,
							filename: "canvas.png".into(),
						});
						m.components = Some(<$mode>::as_components(ctx));
						m
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

						let (data, session, skin) = $crate::get_from_player!(ctx, player);

						let png: ::std::borrow::Cow<[u8]> = {
							let mut surface =
								<$game>::canvas(ctx, &data, &session, skin.as_ref(), Some(mode));

							::api::canvas::to_png(&mut surface).into()
						};

						press
							.create_interaction_response(ctx, |b| {
								b.kind(::poise::serenity_prelude::InteractionResponseType::UpdateMessage)
																					.interaction_response_data(|m| {
																						m.files([::poise::serenity_prelude::AttachmentType::Bytes {
																							data: png,
																							filename: "canvas.png".into(),
																						}]);
																						m.set_components(<$mode>::as_components(ctx));
																						m
																					})
							})
							.await?;
					}
				}
				$crate::format::Display::Text => {
					let (player, data) = $crate::get_data!(ctx, uuid, username);
					let mut embed = <$game>::embed(ctx, &player, &data);

					player.increase_searches(ctx).await?;
					embed.colour($crate::EMBED_COLOUR);

					ctx.send(|m| {
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
