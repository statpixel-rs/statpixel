use std::borrow::Cow;

use crate::{util, Error};
use api::canvas::{self, body::Body, label::ToFormatted, shape, text, Canvas};
use minecraft::{
	calc::network,
	paint::Paint,
	text::{parse::minecraft_text, Text},
};
use poise::serenity_prelude::CreateAttachment;
use translate::{context, Context};

const LABEL: [Text; 1] = minecraft_text("§e§lRecent Games");

#[allow(clippy::too_many_lines)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn recent(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(uuid)?;
	let ctx = &context::Context::from_poise(&ctx);
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;

	let (player, data, games, session, skin, suffix) =
		crate::commands::get_player_data_games_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png: Cow<_> = {
		let status = shape::Status(&session, skin.image());
		let level = network::get_level(data.xp);
		let progress = shape::WideBubbleProgress(
			network::get_level_progress(data.xp),
			network::get_colours(level),
		);

		let ctx = &ctx;
		let mut canvas = Canvas::new(720.)
			.gap(7.)
			.push_down(
				&shape::Title,
				shape::Title::from_text(&text::from_data(&data, &data.username, suffix.as_deref())),
			)
			.push_down(&shape::Subtitle, shape::Subtitle::from_text(&LABEL))
			.push_down_post_draw(
				&progress,
				shape::WideBubbleProgress::from_level_progress(
					ctx,
					&network::get_level_format(level),
					&network::get_curr_level_xp(data.xp),
					&network::get_level_xp(data.xp),
				),
			)
			.push_right_start(
				&canvas::shape::Sidebar,
				canvas::body::Body::new(17., None)
					.append_item(
						&::translate::tr!(ctx, "experience"),
						&data.xp.to_formatted_label(ctx),
						&Paint::Yellow,
					)
					.append_item(
						&::translate::tr!(ctx, "karma"),
						&data.karma.to_formatted_label(ctx),
						&Paint::LightPurple,
					)
					.append_item(
						&::translate::tr!(ctx, "rewards"),
						&data.rewards.to_formatted_label(ctx),
						&Paint::Gold,
					)
					.append_item(
						&::translate::tr!(ctx, "friend-requests"),
						&data.friend_requests.to_formatted_label(ctx),
						&Paint::Green,
					)
					.append_item(
						&::translate::tr!(ctx, "time-played"),
						&data.playtime.to_formatted_label(ctx),
						&Paint::Gold,
					)
					.append_item(
						&::translate::tr!(ctx, "first-login"),
						&data.first_login.to_formatted_label(ctx),
						&Paint::Aqua,
					)
					.append_item(
						&::translate::tr!(ctx, "last-login"),
						&data.last_login.to_formatted_label(ctx),
						&Paint::Blue,
					)
					.build(),
			)
			.push_right_post_draw(&status, Body::from_status(ctx, &session));

		let games = games
			.iter()
			.map(|game| (shape::RecentGame(&game.kind), game))
			.collect::<Vec<_>>();

		for (shape, game) in &games {
			canvas = canvas.push_checked(shape, shape::RecentGame::from_game(ctx, game));
		}

		let mut surface = canvas.build(None, background).unwrap();

		canvas::to_png(&mut surface).into()
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}
