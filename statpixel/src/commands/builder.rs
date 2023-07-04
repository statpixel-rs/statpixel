use std::borrow::Cow;

use crate::Error;
use api::{
	builder::{
		self, Action, Id, LevelKind, Location, PartialShape, Shape, ShapeData, ShapeKind, State,
		Statistic,
	},
	canvas::{self, body::Body, text, Canvas},
	game::r#type::Type,
	image::Image,
	player::DEFAULT_SKIN,
	shape, Data, Player, Session,
};
use minecraft::{calc, paint::Paint, text::Text};
use translate::{context, tr, tr_fmt, Context};

use poise::serenity_prelude::{
	self as serenity, ActionRowComponent, ComponentInteractionDataKind, CreateActionRow,
	CreateAttachment, CreateButton, CreateInputText, CreateInteractionResponseMessage, CreateModal,
	CreateSelectMenu, CreateSelectMenuOption,
};
use uuid::Uuid;

pub const MAX_SHAPES: usize = 16;

/// Initializes a new stat image builder.
#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn builder(ctx: Context<'_>) -> Result<(), Error> {
	let ctx = &context::Context::from_poise(&ctx);

	ctx.send(
		poise::CreateReply::new()
			.components(create_components(ctx, State::default()))
			.content(tr!(&ctx, "builder-welcome")),
	)
	.await?;

	Ok(())
}

#[allow(clippy::too_many_lines)]
pub fn build(
	ctx: &context::Context<'_>,
	shapes: &Vec<Shape>,
	data: &Data,
	session: &Session,
	skin: &Image,
) -> Cow<'static, [u8]> {
	let mut canvas = Canvas::new(750.);
	let skin = shape::Status(session, skin.image());

	for shape in shapes {
		match shape.data {
			ShapeData::Title => {
				let kind = shape::Title;
				let body = shape::Title::from_text(&text::from_data(data, &data.username, None));

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down(&kind, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start(&kind, body);
					}
					Location::Right => {
						canvas = canvas.push_right(&kind, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start(&kind, body);
					}
				}
			}
			ShapeData::Skin => {
				let body = Body::empty();

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down_post_draw(&skin, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start_post_draw(&skin, body);
					}
					Location::Right => {
						canvas = canvas.push_right_post_draw(&skin, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start_post_draw(&skin, body);
					}
				}
			}
			ShapeData::Level(ref kind) => {
				let (level, current, needed, progress, colours) = match kind {
					LevelKind::BedWars => {
						let xp = calc::bed_wars::convert(&data.stats.bed_wars.xp);
						let level = calc::bed_wars::get_level(xp);
						let current = calc::bed_wars::get_curr_level_xp(xp);
						let needed = calc::bed_wars::get_level_xp(xp);
						let format = calc::bed_wars::get_level_format(level);
						let progress = calc::bed_wars::get_level_progress(xp);
						let colours = calc::bed_wars::get_colours(level);

						(format, current, needed, progress, colours)
					}
					LevelKind::BuildBattle => {
						let xp = calc::build_battle::convert(&data.stats.build_battle.score);
						let level = calc::build_battle::get_level(xp);
						let current = calc::build_battle::get_curr_level_xp(xp);
						let needed = calc::build_battle::get_level_xp(xp);
						let format = calc::build_battle::get_level_format(level);
						let progress = calc::build_battle::get_level_progress(xp);
						let colours = calc::build_battle::get_colours(level);

						(
							format,
							u64::from(current),
							u64::from(needed),
							progress,
							colours,
						)
					}
					LevelKind::Duels => {
						let xp = calc::duels::overall::convert(&0);
						let level = calc::duels::overall::get_level(xp);
						let current = calc::duels::overall::get_curr_level_xp(xp);
						let needed = calc::duels::overall::get_level_xp(xp);
						let format = calc::duels::overall::get_level_format(level);
						let progress = calc::duels::overall::get_level_progress(xp);
						let colours = calc::duels::overall::get_colours(level);

						(
							format,
							u64::from(current),
							u64::from(needed),
							progress,
							colours,
						)
					}
					LevelKind::Network => {
						let xp = calc::network::convert(&data.xp);
						let level = calc::network::get_level(xp);
						let current = calc::network::get_curr_level_xp(xp);
						let needed = calc::network::get_level_xp(xp);
						let format = calc::network::get_level_format(level);
						let progress = calc::network::get_level_progress(xp);
						let colours = calc::network::get_colours(level);

						(format, current, needed, progress, colours)
					}
					LevelKind::Pit => {
						let xp = calc::pit::convert(&data.stats.pit.profile.level);
						let level = calc::pit::get_level(xp);
						let current = calc::pit::get_curr_level_xp(xp);
						let needed = calc::pit::get_level_xp(xp);
						let format = calc::pit::get_level_format(level);
						let progress = calc::pit::get_level_progress(xp);
						let colours = calc::pit::get_colours(level);

						(format, current, needed, progress, colours)
					}
					LevelKind::SkyWars => {
						let xp = calc::sky_wars::convert(&data.stats.sky_wars.xp);
						let level = calc::sky_wars::get_level(xp);
						let current = calc::sky_wars::get_curr_level_xp(xp);
						let needed = calc::sky_wars::get_level_xp(xp);
						let format = data.stats.sky_wars.level_fmt.clone();
						let progress = calc::sky_wars::get_level_progress(xp);
						let colours = calc::sky_wars::get_colours(level);

						(format, current, needed, progress, colours)
					}
					LevelKind::WoolWars => {
						let xp = calc::wool_wars::convert(&data.stats.wool_wars.progression.xp);
						let level = calc::wool_wars::get_level(xp);
						let current = calc::wool_wars::get_curr_level_xp(xp);
						let needed = calc::wool_wars::get_level_xp(xp);
						let format = calc::wool_wars::get_level_format(level);
						let progress = calc::wool_wars::get_level_progress(xp);
						let colours = calc::wool_wars::get_colours(level);

						(format, current, needed, progress, colours)
					}
				};

				let kind = shape::WideBubbleProgress(progress, colours);
				let body =
					shape::WideBubbleProgress::from_level_progress(ctx, &level, &current, &needed);

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down(&kind, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start(&kind, body);
					}
					Location::Right => {
						canvas = canvas.push_right(&kind, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start(&kind, body);
					}
				}
			}
			ShapeData::Subtitle(ref text) => {
				let kind = shape::Subtitle;
				let body = shape::Subtitle::from_text(&[Text {
					text,
					paint: shape.colour,
					..Default::default()
				}]);

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down(&kind, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start(&kind, body);
					}
					Location::Right => {
						canvas = canvas.push_right(&kind, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start(&kind, body);
					}
				}
			}
			ShapeData::Bubble(ref statistic) => {
				use api::player::stats::*;

				let (value, label) = match statistic {
					Statistic::Arcade(kind) => {
						(arcade::Arcade::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::Arena(kind) => {
						(arena::Arena::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::BedWars(kind) => {
						(bed_wars::BedWars::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::BlitzSg(kind) => {
						(blitz_sg::BlitzSg::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::BuildBattle(kind) => (
						build_battle::BuildBattle::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::CopsAndCrims(kind) => (
						cops_and_crims::CopsAndCrims::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Duels(kind) => {
						(duels::Duels::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::MegaWalls(kind) => (
						mega_walls::MegaWalls::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::MurderMystery(kind) => (
						murder_mystery::MurderMystery::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Paintball(kind) => (
						paintball::Paintball::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Pit(kind) => (pit::Pit::from_kind(ctx, data, kind), kind.get_tr()),
					Statistic::Quake(kind) => {
						(quake::Quake::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::SkyWars(kind) => {
						(sky_wars::SkyWars::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::SmashHeroes(kind) => (
						smash_heroes::SmashHeroes::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::SpeedUhc(kind) => (
						speed_uhc::SpeedUhc::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::TntGames(kind) => (
						tnt_games::TntGames::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::TurboKartRacers(kind) => (
						turbo_kart_racers::TurboKartRacers::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Uhc(kind) => (uhc::Uhc::from_kind(ctx, data, kind), kind.get_tr()),
					Statistic::VampireZ(kind) => (
						vampire_z::VampireZ::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Walls(kind) => {
						(walls::Walls::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::Warlords(kind) => (
						warlords::Warlords::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::WoolWars(kind) => (
						wool_wars::WoolWars::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
				};

				let kind = shape::Bubble;
				let body = Body::from_bubble_cow(value, tr!(ctx, label).as_ref(), shape.colour);

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down(&kind, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start(&kind, body);
					}
					Location::Right => {
						canvas = canvas.push_right(&kind, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start(&kind, body);
					}
				}
			}
		};
	}

	canvas::to_png(&mut canvas.build(None, None).unwrap()).into()
}

#[allow(clippy::too_many_lines)]
pub fn create_components(ctx: &context::Context<'_>, state: State) -> Vec<CreateActionRow> {
	let empty = state.shapes.is_empty();
	let has_subtitle = state.shapes.iter().any(|shape| shape.data.is_subtitle());

	vec![
		CreateActionRow::SelectMenu(
			CreateSelectMenu::new(
				builder::set_next_position(state.clone()),
				serenity::CreateSelectMenuKind::String {
					options: vec![
						CreateSelectMenuOption::new(tr!(ctx, "down"), "down")
							.description(tr!(ctx, "down-description")),
						CreateSelectMenuOption::new(tr!(ctx, "down-start"), "down_start")
							.description(tr!(ctx, "down-start-description")),
						CreateSelectMenuOption::new(tr!(ctx, "right"), "right")
							.description(tr!(ctx, "right-description")),
						CreateSelectMenuOption::new(tr!(ctx, "right-start"), "right_start")
							.description(tr!(ctx, "right-start-description")),
					],
				},
			)
			.placeholder(match state.next.location {
				None => tr!(ctx, "select-position"),
				Some(Location::Down) => tr!(ctx, "down"),
				Some(Location::DownStart) => tr!(ctx, "down-start"),
				Some(Location::Right) => tr!(ctx, "right"),
				Some(Location::RightStart) => tr!(ctx, "right-start"),
			}),
		),
		CreateActionRow::SelectMenu(
			CreateSelectMenu::new(
				builder::set_next_shape(state.clone()),
				serenity::CreateSelectMenuKind::String {
					options: {
						let mut options = vec![
							CreateSelectMenuOption::new(tr!(ctx, "title"), "title")
								.description(tr!(ctx, "title-description")),
							CreateSelectMenuOption::new(tr!(ctx, "level"), "level")
								.description(tr!(ctx, "level-description")),
							CreateSelectMenuOption::new(tr!(ctx, "skin"), "skin")
								.description(tr!(ctx, "skin-description")),
							CreateSelectMenuOption::new(tr!(ctx, "bubble"), "bubble")
								.description(tr!(ctx, "bubble-description")),
						];

						// Only allow one subtitle
						if !has_subtitle {
							options.push(
								CreateSelectMenuOption::new(tr!(ctx, "subtitle"), "subtitle")
									.description(tr!(ctx, "subtitle-description")),
							);
						}

						options
					},
				},
			)
			.placeholder(match state.next.kind {
				None => tr!(ctx, "select-shape"),
				Some(ShapeKind::Title) => tr!(ctx, "title"),
				Some(ShapeKind::Subtitle) => tr!(ctx, "subtitle"),
				Some(ShapeKind::Level) => tr!(ctx, "level"),
				Some(ShapeKind::Skin) => tr!(ctx, "skin"),
				Some(ShapeKind::Bubble) => tr!(ctx, "bubble"),
			}),
		),
		CreateActionRow::SelectMenu(
			CreateSelectMenu::new(
				builder::set_next_colour(state.clone()),
				serenity::CreateSelectMenuKind::String {
					options: vec![
						CreateSelectMenuOption::new(tr!(ctx, "aqua"), "AQUA"),
						CreateSelectMenuOption::new(tr!(ctx, "black"), "BLACK"),
						CreateSelectMenuOption::new(tr!(ctx, "blue"), "BLUE"),
						CreateSelectMenuOption::new(tr!(ctx, "dark-aqua"), "DARK_AQUA"),
						CreateSelectMenuOption::new(tr!(ctx, "dark-blue"), "DARK_BLUE"),
						CreateSelectMenuOption::new(tr!(ctx, "dark-gray"), "DARK_GRAY"),
						CreateSelectMenuOption::new(tr!(ctx, "dark-green"), "DARK_GREEN"),
						CreateSelectMenuOption::new(tr!(ctx, "dark-red"), "DARK_RED"),
						CreateSelectMenuOption::new(tr!(ctx, "gold"), "GOLD"),
						CreateSelectMenuOption::new(tr!(ctx, "gray"), "GRAY"),
						CreateSelectMenuOption::new(tr!(ctx, "green"), "GREEN"),
						CreateSelectMenuOption::new(tr!(ctx, "light-purple"), "LIGHT_PURPLE"),
						CreateSelectMenuOption::new(tr!(ctx, "dark-purple"), "DARK_PURPLE"),
						CreateSelectMenuOption::new(tr!(ctx, "red"), "RED"),
						CreateSelectMenuOption::new(tr!(ctx, "white"), "WHITE"),
						CreateSelectMenuOption::new(tr!(ctx, "yellow"), "YELLOW"),
					],
				},
			)
			.placeholder(match state.next.colour {
				None => tr!(ctx, "select-colour"),
				Some(Paint::Aqua) => tr!(ctx, "aqua"),
				Some(Paint::Black) => tr!(ctx, "black"),
				Some(Paint::Blue) => tr!(ctx, "blue"),
				Some(Paint::DarkAqua) => tr!(ctx, "dark-aqua"),
				Some(Paint::DarkBlue) => tr!(ctx, "dark-blue"),
				Some(Paint::DarkGray) => tr!(ctx, "dark-gray"),
				Some(Paint::DarkGreen) => tr!(ctx, "dark-green"),
				Some(Paint::DarkRed) => tr!(ctx, "dark-red"),
				Some(Paint::Gold) => tr!(ctx, "gold"),
				Some(Paint::Gray) => tr!(ctx, "gray"),
				Some(Paint::Green) => tr!(ctx, "green"),
				Some(Paint::LightPurple) => tr!(ctx, "light-purple"),
				Some(Paint::DarkPurple) => tr!(ctx, "dark-purple"),
				Some(Paint::Red) => tr!(ctx, "red"),
				Some(Paint::White) => tr!(ctx, "white"),
				Some(Paint::Yellow) => tr!(ctx, "yellow"),
				// This colour is not used in the builder
				Some(Paint::Bronze) => tr!(ctx, "bronze"),
			}),
		),
		CreateActionRow::Buttons(vec![
			CreateButton::new(builder::add_shape(state.clone()))
				.label(tr!(ctx, "add-shape"))
				.style(serenity::ButtonStyle::Primary)
				.disabled(state.shapes.len() >= MAX_SHAPES || !state.next.is_complete()),
			CreateButton::new(builder::undo(state.clone()))
				.label(tr!(ctx, "undo"))
				.style(serenity::ButtonStyle::Danger)
				.disabled(empty),
			CreateButton::new(builder::create(state))
				.label(tr!(ctx, "create"))
				.style(serenity::ButtonStyle::Success)
				.disabled(empty),
		]),
	]
}

#[allow(clippy::too_many_lines)]
pub async fn handler(
	ctx: &context::Context<'_>,
	interaction: &serenity::ComponentInteraction,
	mut id: Id,
) -> Result<(), Error> {
	let mut update = false;
	match &interaction.data.kind {
		ComponentInteractionDataKind::StringSelect { values } => match id.action {
			Action::SetNextPosition => {
				id.state.next.location = Some(match values[0].as_str() {
					"down" => builder::Location::Down,
					"down_start" => builder::Location::DownStart,
					"right" => builder::Location::Right,
					"right_start" => builder::Location::RightStart,
					_ => return Ok(()),
				});
			}
			Action::SetNextShape => {
				id.state.next.kind = Some(match values[0].as_str() {
					"title" => builder::ShapeKind::Title,
					"subtitle" => builder::ShapeKind::Subtitle,
					"level" => builder::ShapeKind::Level,
					"skin" => builder::ShapeKind::Skin,
					"bubble" => builder::ShapeKind::Bubble,
					_ => return Ok(()),
				});
			}
			Action::SetNextColour => {
				id.state.next.colour =
					Some(Paint::try_from(values[0].as_str()).unwrap_or_default());
			}
			_ => return Ok(()),
		},
		ComponentInteractionDataKind::Button => {
			match (id.action, &id.state.next) {
				(
					Action::AddShape,
					PartialShape {
						location: Some(location),
						kind: Some(kind),
						colour: Some(colour),
						..
					},
				) => {
					// create the modal for it if needed, otherwise add it in
					match kind {
						builder::ShapeKind::Title => {
							id.state.shapes.push(builder::Shape {
								location: *location,
								colour: *colour,
								data: ShapeData::Title,
							});

							update = true;
						}
						builder::ShapeKind::Subtitle => {
							return Ok(interaction
								.create_response(
									ctx.discord(),
									serenity::CreateInteractionResponse::Modal(
										create_subtitle_modal(ctx, id.state),
									),
								)
								.await?)
						}
						builder::ShapeKind::Level => {
							return Ok(interaction
								.create_response(
									ctx.discord(),
									serenity::CreateInteractionResponse::Modal(create_level_modal(
										ctx, id.state,
									)),
								)
								.await?)
						}
						builder::ShapeKind::Skin => {
							id.state.shapes.push(builder::Shape {
								location: *location,
								colour: *colour,
								data: ShapeData::Skin,
							});

							update = true;
						}
						builder::ShapeKind::Bubble => {
							return Ok(interaction
								.create_response(
									ctx.discord(),
									serenity::CreateInteractionResponse::Modal(
										create_bubble_modal(ctx, id.state),
									),
								)
								.await?)
						}
					}
				}
				(Action::Undo, ..) => {
					// remove the last shape
					id.state.shapes.pop();

					update = true;
				}
				(Action::Create, ..) => {
					return Ok(interaction
						.create_response(
							ctx.discord(),
							serenity::CreateInteractionResponse::Modal(create_create_modal(
								ctx, id.state,
							)),
						)
						.await?)
				}
				_ => return Ok(()),
			}
		}
		_ => return Ok(()),
	}

	if update {
		let data = Data::placeholder();
		let session = Session {
			online: false,
			game_type: None,
			game_mode: None,
		};
		let bytes = build(ctx, &id.state.shapes, &data, &session, &DEFAULT_SKIN);

		ctx.send(
			poise::CreateReply::new()
				.components(create_components(ctx, id.state))
				.attachment(CreateAttachment::bytes(bytes, crate::IMAGE_NAME)),
		)
		.await?;
	} else {
		ctx.send(poise::CreateReply::new().components(create_components(ctx, id.state)))
			.await?;
	}

	Ok(())
}

pub async fn finish(ctx: &context::Context<'_>, state: State, uuid: Uuid) -> Result<(), Error> {
	let (_, data, session, skin, _) =
		super::get_player_data_session_skin_suffix(ctx, Some(uuid), None).await?;
	let bytes = build(ctx, &state.shapes, &data, &session, &skin);

	let id = api::id::command(api::command::Id::Builder {
		shapes: state.shapes,
		uuid,
	});
	let id = id.as_str();

	Ok(ctx
		.send(
			poise::CreateReply::new()
				.components(vec![])
				.content(
					tr_fmt!(ctx, "image-created", id: id, link: format!("<https://images.statpixel.xyz/{id}>")),
				)
				.attachment(serenity::CreateAttachment::bytes(bytes, crate::IMAGE_NAME)),
		)
		.await?)
}

pub fn create_subtitle_modal(ctx: &context::Context<'_>, state: State) -> CreateModal {
	CreateModal::new(
		builder::set_subtitle_data(state),
		tr!(ctx, "subtitle-modal-title"),
	)
	.components(vec![CreateActionRow::InputText(
		CreateInputText::new(
			serenity::InputTextStyle::Short,
			tr!(ctx, "subtitle-text"),
			"subtitle_text",
		)
		.max_length(16)
		.placeholder(tr!(ctx, "subtitle-placeholder")),
	)])
}

pub fn create_level_modal(ctx: &context::Context<'_>, state: State) -> CreateModal {
	CreateModal::new(
		builder::set_level_data(state),
		tr!(ctx, "level-modal-title"),
	)
	.components(vec![CreateActionRow::InputText(
		CreateInputText::new(
			serenity::InputTextStyle::Short,
			tr!(ctx, "level-type"),
			"level_type",
		)
		.max_length(16)
		.placeholder(tr!(ctx, "level-type-placeholder")),
	)])
}

pub fn create_bubble_modal(ctx: &context::Context<'_>, state: State) -> CreateModal {
	CreateModal::new(
		builder::set_bubble_data(state),
		tr!(ctx, "bubble-modal-title"),
	)
	.components(vec![
		CreateActionRow::InputText(
			CreateInputText::new(
				serenity::InputTextStyle::Short,
				tr!(ctx, "game-type"),
				"game_type",
			)
			.placeholder(tr!(ctx, "game-type-placeholder")),
		),
		CreateActionRow::InputText(
			CreateInputText::new(
				serenity::InputTextStyle::Short,
				tr!(ctx, "statistic"),
				"statistic",
			)
			.placeholder(tr!(ctx, "statistic-placeholder")),
		),
	])
}

pub fn create_create_modal(ctx: &context::Context<'_>, state: State) -> CreateModal {
	CreateModal::new(builder::create(state), tr!(ctx, "create-modal-title")).components(vec![
		CreateActionRow::InputText(
			CreateInputText::new(
				serenity::InputTextStyle::Short,
				tr!(ctx, "username"),
				"username",
			)
			.placeholder(tr!(ctx, "username-placeholder")),
		),
	])
}

macro_rules! impl_type_branch {
	($kind: ty, $variant: ident, $id: tt, $statistic: tt, $interaction: tt, $ctx: tt, $location: tt, $colour: tt, $game_type: tt) => {
		{
			let Some(statistic) =
				<$kind>::try_from_str_lower(&$statistic.to_ascii_lowercase())
			else {
				return Ok($interaction.create_response(
					$ctx.discord(),
					serenity::CreateInteractionResponse::Message(
						CreateInteractionResponseMessage::new()
							.content(tr_fmt!($ctx, "invalid-statistic", statistic: format!("`{}`", $statistic), game: format!("`{}`", $game_type.as_clean_name())))
							.ephemeral(true),
					),
				).await?);
			};

			$id.state.shapes.push(builder::Shape {
				location: *$location,
				colour: *$colour,
				data: ShapeData::Bubble(Statistic::$variant (statistic)),
			});
		}
	};
}

#[allow(clippy::too_many_lines)]
pub async fn modal_handler(
	ctx: &serenity::Context,
	interaction: &serenity::ModalInteraction,
	data: &translate::Data,
	mut id: Id,
) -> Result<(), Error> {
	let local_ctx = &context::Context::from_modal(ctx, data, interaction);

	match (id.action, &id.state.next) {
		(Action::Create, ..) => {
			let ActionRowComponent::InputText(username) =
				&interaction.data.components[0].components[0]
			else {
				return Ok(());
			};
			let username = username.value.as_ref().unwrap().as_str();
			let player = Player::from_username(username).await?;

			return finish(local_ctx, id.state, player.uuid).await;
		}
		(
			Action::SetSubtitleData,
			PartialShape {
				location: Some(location),
				colour: Some(colour),
				..
			},
		) => {
			let ActionRowComponent::InputText(subtitle) =
				&interaction.data.components[0].components[0]
			else {
				return Ok(());
			};

			let subtitle = subtitle.value.clone().unwrap();

			id.state.shapes.push(builder::Shape {
				location: *location,
				colour: *colour,
				data: ShapeData::Subtitle(subtitle),
			});
		}
		(
			Action::SetLevelData,
			PartialShape {
				location: Some(location),
				colour: Some(colour),
				..
			},
		) => {
			let ActionRowComponent::InputText(level) =
				&interaction.data.components[0].components[0]
			else {
				return Ok(());
			};

			let level = level.value.as_ref().unwrap().as_str();
			let level = match level {
				"bedwars" => LevelKind::BedWars,
				"buildbattle" => LevelKind::BuildBattle,
				"duels" => LevelKind::Duels,
				"network" => LevelKind::Network,
				"pit" => LevelKind::Pit,
				"skywars" => LevelKind::SkyWars,
				"woolwars" => LevelKind::WoolWars,
				_ => {
					return Ok(interaction
						.create_response(
							ctx,
							serenity::CreateInteractionResponse::Message(
								CreateInteractionResponseMessage::new()
									.content(
										tr_fmt!(local_ctx, "invalid-level-type", kind: format!("`{level}`")),
									)
									.ephemeral(true),
							),
						)
						.await?)
				}
			};

			id.state.shapes.push(builder::Shape {
				location: *location,
				colour: *colour,
				data: ShapeData::Level(level),
			});
		}
		(
			Action::SetBubbleData,
			PartialShape {
				location: Some(location),
				colour: Some(colour),
				..
			},
		) => {
			use api::player::stats::*;

			let (
				ActionRowComponent::InputText(game_type),
				ActionRowComponent::InputText(statistic),
			) = (
				&interaction.data.components[0].components[0],
				&interaction.data.components[1].components[0],
			)
			else {
				return Ok(());
			};

			// These are always Some, as per the documentation
			let game_type = game_type.value.as_ref().unwrap();
			let statistic = statistic.value.as_ref().unwrap();

			let Some(game_type) = Type::try_from_clean_name_lower(&game_type.to_ascii_lowercase())
			else {
				return Ok(interaction
					.create_response(
						ctx,
						serenity::CreateInteractionResponse::Message(
							CreateInteractionResponseMessage::new()
								.content(
									tr_fmt!(local_ctx, "invalid-game-type", game: format!("`{game_type}`")),
								)
								.ephemeral(true),
						),
					)
					.await?);
			};

			match game_type {
				Type::Arcade => impl_type_branch!(
					arcade::ArcadeKind,
					Arcade,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::Arena => impl_type_branch!(
					arena::ArenaKind,
					Arena,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::BedWars => impl_type_branch!(
					bed_wars::BedWarsKind,
					BedWars,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::BlitzSg => impl_type_branch!(
					blitz_sg::BlitzSgKind,
					BlitzSg,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::BuildBattle => impl_type_branch!(
					build_battle::BuildBattleKind,
					BuildBattle,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::Duels => impl_type_branch!(
					duels::DuelsKind,
					Duels,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::MegaWalls => impl_type_branch!(
					mega_walls::MegaWallsKind,
					MegaWalls,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::MurderMystery => impl_type_branch!(
					murder_mystery::MurderMysteryKind,
					MurderMystery,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::Paintball => impl_type_branch!(
					paintball::PaintballKind,
					Paintball,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::Pit => impl_type_branch!(
					pit::PitKind,
					Pit,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::Quake => impl_type_branch!(
					quake::QuakeKind,
					Quake,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::SkyWars => impl_type_branch!(
					sky_wars::SkyWarsKind,
					SkyWars,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::SmashHeroes => impl_type_branch!(
					smash_heroes::SmashHeroesKind,
					SmashHeroes,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::SpeedUhc => impl_type_branch!(
					speed_uhc::SpeedUhcKind,
					SpeedUhc,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::TntGames => impl_type_branch!(
					tnt_games::TntGamesKind,
					TntGames,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::TurboKartRacers => impl_type_branch!(
					turbo_kart_racers::TurboKartRacersKind,
					TurboKartRacers,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::Uhc => impl_type_branch!(
					uhc::UhcKind,
					Uhc,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::VampireZ => impl_type_branch!(
					vampire_z::VampireZKind,
					VampireZ,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::Walls => impl_type_branch!(
					walls::WallsKind,
					Walls,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::Warlords => impl_type_branch!(
					warlords::WarlordsKind,
					Warlords,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				Type::WoolWars => impl_type_branch!(
					wool_wars::WoolWarsKind,
					WoolWars,
					id,
					statistic,
					interaction,
					local_ctx,
					location,
					colour,
					game_type
				),
				_ => return Ok(()),
			};
		}
		_ => return Ok(()),
	}

	let bytes = {
		let data = Data::placeholder();
		let session = Session {
			online: false,
			game_type: None,
			game_mode: None,
		};

		build(local_ctx, &id.state.shapes, &data, &session, &DEFAULT_SKIN)
	};

	interaction
		.create_response(
			ctx,
			serenity::CreateInteractionResponse::Message(
				poise::CreateReply::new()
					.components(create_components(local_ctx, id.state))
					.attachment(CreateAttachment::bytes(bytes, crate::IMAGE_NAME))
					.to_slash_initial_response(),
			),
		)
		.await?;

	Ok(())
}
