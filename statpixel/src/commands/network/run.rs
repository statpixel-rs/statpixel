use crate::{format::Display, Error};
use api::canvas::label::ToFormatted;
use poise::serenity_prelude::{self as serenity, CreateAttachment, CreateEmbed};
use translate::{context, tr, tr_fmt};
use uuid::Uuid;

#[allow(clippy::too_many_lines)]
pub async fn network(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (format, family, background) = crate::util::get_image_options_from_input(ctx).await;

	match format {
		Display::Image | Display::Compact => {
			let (player, data, guild, session, skin, suffix) =
				crate::commands::get_player_data_guild_session_skin_suffix(ctx, uuid, username)
					.await?;

			player.increase_searches(ctx).await?;

			let png = super::image::network(
				ctx,
				family,
				&player,
				guild.as_deref(),
				&data,
				&session,
				suffix.as_deref(),
				skin.image(),
				background,
			);

			let id = api::id::command(api::command::Id::Root {
				kind: api::command::Mode::Network,
				uuid: player.uuid,
			});

			ctx.send(
				poise::CreateReply::new()
					.content(format!(
						"{}\n{}",
						tr_fmt!(ctx, "identifier", identifier: id),
						crate::tip::random(ctx),
					))
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
					tr(ctx, "experience"),
					data.xp.to_formatted(ctx),
					tr(ctx, "karma"),
					data.karma.to_formatted(ctx),
					tr(ctx, "rewards"),
					data.rewards.to_formatted(ctx),
					tr(ctx, "friend-requests"),
					data.friend_requests.to_formatted(ctx),
					tr(ctx, "time-played"),
					data.playtime.to_formatted(ctx),
					tr(ctx, "first-login"),
					data.first_login.to_formatted(ctx),
					tr(ctx, "last-login"),
					data.last_login.to_formatted(ctx),
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
