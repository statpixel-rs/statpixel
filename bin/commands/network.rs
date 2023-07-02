use std::borrow::Cow;

use crate::{format::Display, util, Error};
use api::canvas::{self, body::Body, label::ToFormatted, shape, text, Canvas};
use minecraft::{
	calc::network,
	paint::Paint,
	text::{parse::minecraft_text, Text},
};
use poise::serenity_prelude::{self as serenity, CreateAttachment, CreateEmbed};
use translate::{context, tr, Context};

const LABEL: [Text; 1] = minecraft_text("§f§lNetwork");

#[allow(clippy::too_many_lines)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn network(
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
	let (format, background) = crate::util::get_format_colour_from_input(ctx).await;

	match format {
		Display::Image | Display::Compact => {
			let (player, data, guild, session, skin, suffix) =
				crate::commands::get_player_data_guild_session_skin_suffix(ctx, uuid, username)
					.await?;

			player.increase_searches(ctx).await?;

			let png: Cow<_> = {
				let status = shape::Status(&session, skin.image());
				let level = network::get_level(data.xp);
				let progress = shape::WideBubbleProgress(
					network::get_level_progress(data.xp),
					network::get_colours(level),
				);

				let member = guild
					.as_ref()
					.and_then(|g| g.members.iter().find(|m| m.uuid == player.uuid));

				let ctx = &ctx;
				let mut surface = Canvas::new(720.)
					.gap(7.)
					.push_down(
						&shape::Title,
						shape::Title::from_text(&text::from_data(
							&data,
							&data.username,
							suffix.as_deref(),
						)),
					)
					.push_down(
						&shape::Subtitle,
						if let Some(ref guild) = guild {
							shape::Subtitle::from_guild(guild)
						} else {
							shape::Subtitle::from_label(ctx, &LABEL, "member-profile")
						},
					)
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
					.push_right_post_draw(&status, Body::from_status(ctx, &session))
					.push_down_start(
						&shape::Bubble,
						Body::from_bubble(
							ctx,
							&data.quests,
							tr!(ctx, "quests").as_ref(),
							Paint::Gold,
						),
					)
					.push_right(
						&shape::Bubble,
						Body::from_bubble(
							ctx,
							&data.challenges,
							tr!(ctx, "challenges").as_ref(),
							Paint::Gold,
						),
					)
					.push_right(
						&shape::Bubble,
						Body::from_bubble(
							ctx,
							&data.achivement_points,
							tr!(ctx, "achievement-points").as_ref(),
							Paint::Gold,
						),
					)
					.push_down_start(
						&shape::Bubble,
						Body::from_bubble_small(
							ctx,
							&data.language,
							tr!(ctx, "language").as_ref(),
							Paint::Aqua,
						),
					)
					.push_right(
						&shape::Bubble,
						Body::from_bubble(
							ctx,
							&data.gifting.gifts_given,
							tr!(ctx, "gifts-given").as_ref(),
							Paint::LightPurple,
						),
					)
					.push_right(
						&shape::Bubble,
						Body::from_bubble(
							ctx,
							&data.gifting.ranks_given,
							tr!(ctx, "ranks-given").as_ref(),
							Paint::LightPurple,
						),
					)
					.push_down_start(
						&shape::Bubble,
						Body::from_bubble(
							ctx,
							&member.map_or(0, |m| m.xp_history[0].1),
							tr!(ctx, "daily-xp").as_ref(),
							Paint::DarkGreen,
						),
					)
					.push_right(
						&shape::Bubble,
						Body::from_bubble(
							ctx,
							&member.map_or(0, |m| m.xp_history.iter().map(|(_, x)| x).sum::<u32>()),
							tr!(ctx, "weekly-xp").as_ref(),
							Paint::DarkGreen,
						),
					)
					.push_right(
						&shape::Bubble,
						Body::from_bubble(
							ctx,
							&member.map_or(0, |m| m.quests),
							tr!(ctx, "guild-quests").as_ref(),
							Paint::DarkGreen,
						),
					)
					.build(None, background)
					.unwrap();

				canvas::to_png(&mut surface).into()
			};

			ctx.send(
				poise::CreateReply::new()
					.content(crate::tip::random(ctx))
					.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
			)
			.await?;
		}
		Display::Text => {
			let (player, data) = crate::commands::get_player_data(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let embed = {
				let ctx = &ctx;
				let mut embed = CreateEmbed::default().thumbnail(player.get_body_url());

				if let Some(prefix) = data.get_rank().as_str() {
					embed = embed.author(
						serenity::CreateEmbedAuthor::new(format!(
							"{} {} :: Network",
							prefix, data.username
						))
						.icon_url(player.get_head_url()),
					);
				} else {
					embed = embed.author(
						serenity::CreateEmbedAuthor::new(format!("{} :: Network", data.username))
							.icon_url(player.get_head_url()),
					);
				}

				embed.description(format!(
					"{}: **{}**\n{}: **{}**\n{}: **{}**\n{}: **{}**\n{}: **{}**\n{}: **{}**\n{}: **{}**",
					tr!(ctx, "experience"),
					data.xp.to_formatted_label(ctx),
					tr!(ctx, "karma"),
					data.karma.to_formatted_label(ctx),
					tr!(ctx, "rewards"),
					data.rewards.to_formatted_label(ctx),
					tr!(ctx, "friend-requests"),
					data.friend_requests.to_formatted_label(ctx),
					tr!(ctx, "time-played"),
					data.playtime.to_formatted_label(ctx),
					tr!(ctx, "first-login"),
					data.first_login.to_formatted_label(ctx),
					tr!(ctx, "last-login"),
					data.last_login.to_formatted_label(ctx),
				))
			};

			ctx.send(
				poise::CreateReply::new()
					.content(crate::tip::random(ctx))
					.embed(embed),
			)
			.await?;
		}
	}

	Ok(())
}
