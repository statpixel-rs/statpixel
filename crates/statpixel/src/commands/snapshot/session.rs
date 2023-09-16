use std::borrow::Cow;

use api::canvas;
use chrono::Utc;
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt, Error};
use uuid::Uuid;

use crate::{commands, format, snapshot, util};

#[allow(clippy::too_many_lines)]
pub async fn command<G: api::canvas::prelude::Game>(
	ctx: &context::Context<'_>,
	uuid_lhs: Uuid,
	uuid_rhs: Uuid,
	mode: Option<G::Mode>,
) -> Result<(), Error> {
	let (format, family, background) = util::get_image_options_from_input(ctx).await;

	match format {
		format::Display::Image => {
			let (player, data_rhs, session, skin, suffix) =
				commands::get_player_data_session_skin_suffix(ctx, Some(uuid_rhs), None).await?;

			player.increase_searches(ctx).await?;

			let status = snapshot::user::get_snapshot_by_session_id(ctx, uuid_lhs).await?;

			let Some((ref data_lhs, created_at)) = status else {
				return Err(Error::SessionNotFound);
			};

			let content = tr_fmt!(
				ctx, "showing-statistics",
				from: format!("<t:{}:f>", created_at.timestamp()),
				to: format!("<t:{}:f>", Utc::now().timestamp()),
			);

			let (png, _): (Cow<_>, _) = {
				let (mut surface, mode) = G::canvas_diff(
					ctx,
					family,
					data_lhs,
					&data_rhs,
					&session,
					skin.image(),
					mode,
					suffix.as_deref(),
					background,
				);

				(canvas::to_png(&mut surface).into(), mode)
			};

			ctx.send(
				poise::CreateReply::new()
					.content(content)
					.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
			)
			.await?;
		}
		format::Display::Compact => {
			let (player, data_rhs, suffix) =
				commands::get_player_data_suffix(ctx, Some(uuid_rhs), None).await?;

			player.increase_searches(ctx).await?;

			let status = snapshot::user::get_snapshot_by_session_id(ctx, uuid_lhs).await?;

			let Some((ref data_lhs, created_at)) = status else {
				return Err(Error::SessionNotFound);
			};

			let content = tr_fmt!(
				ctx, "showing-statistics",
				from: format!("<t:{}:f>", created_at.timestamp()),
				to: format!("<t:{}:f>", Utc::now().timestamp()),
			);

			let attachments = G::condensed_diff(
				ctx,
				family,
				data_lhs,
				&data_rhs,
				suffix.as_deref(),
				background,
			)
			.into_iter()
			.map(|mut surface| {
				CreateAttachment::bytes(Cow::Owned(canvas::to_png(&mut surface)), crate::IMAGE_NAME)
			})
			.collect::<Vec<_>>();

			let mut reply = poise::CreateReply::new().content(content);

			reply.attachments = attachments;

			ctx.send(reply).await?;
		}
		format::Display::Text => {
			let (player, data_rhs) = commands::get_player_data(ctx, Some(uuid_rhs), None).await?;

			player.increase_searches(ctx).await?;

			let status = snapshot::user::get_snapshot_by_session_id(ctx, uuid_lhs).await?;

			let Some((data_lhs, created_at)) = status else {
				return Err(Error::SessionNotFound);
			};

			let content = tr_fmt!(
				ctx, "showing-statistics",
				from: format!("<t:{}:f>", created_at.timestamp()),
				to: format!("<t:{}:f>", Utc::now().timestamp()),
			);

			let embed =
				G::embed_diff(ctx, &player, &data_lhs, &data_rhs).colour(crate::EMBED_COLOUR);

			ctx.send(poise::CreateReply::new().content(content).embed(embed))
				.await?;
		}
	}

	Ok(())
}
