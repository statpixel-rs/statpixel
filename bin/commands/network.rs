use std::borrow::Cow;

use crate::{format::Display, Context, Error};
use api::canvas::{self, label::ToFormatted};
use minecraft::paint::Paint;
use poise::serenity_prelude::{AttachmentType, CreateEmbed};
use translate::tr;

#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn network(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let (format, player, data, session) = crate::get_data!(ctx, uuid, username);

	player.increase_searches(ctx).await?;

	match format {
		Display::Image | Display::Compact => {
			let png: Cow<[u8]> = {
				let mut surface = canvas::create_surface(1);

				canvas::header::apply_status(ctx, &mut surface, &session);
				canvas::header::apply_name(&mut surface, &data);

				canvas::sidebar::item(
					ctx,
					&mut surface,
					&(tr!(ctx, "experience"), data.xp, Paint::Yellow),
					0,
				);

				canvas::sidebar::item(
					ctx,
					&mut surface,
					&(tr!(ctx, "karma"), data.karma, Paint::LightPurple),
					1,
				);

				canvas::sidebar::item(
					ctx,
					&mut surface,
					&(tr!(ctx, "rewards"), data.rewards, Paint::Gold),
					2,
				);

				canvas::sidebar::item(
					ctx,
					&mut surface,
					&(
						tr!(ctx, "friend-requests"),
						data.friend_requests,
						Paint::Green,
					),
					3,
				);

				canvas::sidebar::item(
					ctx,
					&mut surface,
					&(tr!(ctx, "time-played"), data.playtime, Paint::Gold),
					4,
				);

				canvas::sidebar::item(
					ctx,
					&mut surface,
					&(tr!(ctx, "first-login"), data.first_login, Paint::Aqua),
					5,
				);

				canvas::sidebar::item(
					ctx,
					&mut surface,
					&(tr!(ctx, "last-login"), data.last_login, Paint::Blue),
					6,
				);

				canvas::to_png(&mut surface).into()
			};

			ctx.send(move |m| {
				m.attachment(AttachmentType::Bytes {
					data: png,
					filename: "canvas.png".into(),
				})
			})
			.await?;
		}
		Display::Text => {
			let mut embed = CreateEmbed::default();

			embed.thumbnail(player.get_body_url());

			if let Some(prefix) = data.get_rank().as_str() {
				embed.author(|a| {
					a.name(format!("{} {} :: Network", prefix, player.username))
						.icon_url(player.get_head_url())
				});
			} else {
				embed.author(|a| {
					a.name(format!("{} :: Network", player.username))
						.icon_url(player.get_head_url())
				});
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
				tr!(ctx, "playtime"),
				data.playtime.to_formatted_label(ctx),
				tr!(ctx, "first-login"),
				data.first_login.to_formatted_label(ctx),
				tr!(ctx, "last-login"),
				data.last_login.to_formatted_label(ctx),
			));

			ctx.send(|m| {
				m.embeds.push(embed);
				m
			})
			.await?;
		}
	}

	Ok(())
}
