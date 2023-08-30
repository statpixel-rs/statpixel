use std::borrow::Cow;

use api::canvas::{self, prelude::Mode};
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt};
use uuid::Uuid;

use crate::{commands, format, util, Error};

pub async fn command<G: api::canvas::prelude::Game>(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	mode: Option<G::Mode>,
) -> Result<(), Error> {
	let (format, family, background) = util::get_image_options_from_input(ctx).await;

	match format {
		format::Display::Image => {
			let (player, data, session, skin, suffix) =
				crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let (png, mode) = super::image::command::<G>(
				ctx,
				family,
				mode,
				background,
				&data,
				&session,
				skin.image(),
				suffix.as_deref(),
			);

			let (row, id) = G::Mode::as_root(ctx, player.uuid, Some(mode));

			ctx.send(
				poise::CreateReply::new()
					.content(format!(
						"{}\n{}",
						tr_fmt!(ctx, "identifier", identifier: api::id::encode(&id)),
						crate::tip::random(ctx),
					))
					.components(vec![row])
					.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
			)
			.await?;
		}
		format::Display::Compact => {
			let (player, data, suffix) =
				crate::commands::get_player_data_suffix(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let attachments = G::condensed(ctx, family, &data, suffix.as_deref(), background)
				.into_iter()
				.map(|mut surface| {
					CreateAttachment::bytes(
						Cow::Owned(canvas::to_png(&mut surface)),
						crate::IMAGE_NAME,
					)
				})
				.collect::<Vec<_>>();

			let (_, id) = G::Mode::as_root(ctx, player.uuid, None);
			let mut reply = poise::CreateReply::new().content(format!(
				"{}\n{}",
				tr_fmt!(ctx, "identifier", identifier: api::id::encode(&id)),
				crate::tip::random(ctx),
			));

			reply.attachments = attachments;

			ctx.send(reply).await?;
		}
		format::Display::Text => {
			let (player, data) = commands::get_player_data(ctx, uuid, username).await?;
			let embed = G::embed(ctx, &player, &data).colour(crate::EMBED_COLOUR);

			player.increase_searches(ctx).await?;

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
