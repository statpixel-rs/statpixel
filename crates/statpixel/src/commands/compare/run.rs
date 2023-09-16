use std::borrow::Cow;

use api::canvas::{self, prelude::Mode};
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt, Error};
use uuid::Uuid;

use crate::{
	commands, format,
	util::{self, parse_uuid},
};

#[allow(clippy::too_many_lines)]
pub async fn command<G: api::canvas::prelude::Game>(
	ctx: &context::Context<'_>,
	lhs: Option<String>,
	rhs: Option<String>,
	mode: Option<G::Mode>,
	uuid_lhs: Option<Uuid>,
	uuid_rhs: Option<Uuid>,
) -> Result<(), Error> {
	let (format, family, background) = util::get_image_options_from_input(ctx).await;

	match format {
		format::Display::Image => {
			let (player_rhs, data_rhs, session, skin, suffix) =
				commands::get_player_data_session_skin_suffix(
					ctx,
					uuid_rhs.or_else(|| parse_uuid(rhs.as_deref())),
					rhs,
				)
				.await?;

			let (player_lhs, data_lhs) = commands::get_player_data(
				ctx,
				uuid_lhs.or_else(|| parse_uuid(lhs.as_deref())),
				lhs,
			)
			.await?;

			player_lhs.increase_searches(ctx).await?;
			player_rhs.increase_searches(ctx).await?;

			let content = tr_fmt!(
				ctx, "showing-comparison",
				from: data_rhs.username.as_str(),
				to: data_lhs.username.as_str(),
			);

			let (png, mode): (Cow<_>, _) = {
				let (mut surface, mode) = G::canvas_diff(
					ctx,
					family,
					&data_lhs,
					&data_rhs,
					&session,
					skin.image(),
					mode,
					suffix.as_deref(),
					background,
				);

				(canvas::to_png(&mut surface).into(), mode)
			};

			let (row, id) = G::Mode::as_compare(ctx, player_lhs.uuid, player_rhs.uuid, Some(mode));

			ctx.send(
				poise::CreateReply::new()
					.content(format!(
						"{}\n{content}",
						tr_fmt!(ctx, "identifier", identifier: api::id::encode(&id)),
					))
					.components(vec![row])
					.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
			)
			.await?;
		}
		format::Display::Compact => {
			let (player_rhs, data_rhs, suffix) = commands::get_player_data_suffix(
				ctx,
				uuid_rhs.or_else(|| parse_uuid(rhs.as_deref())),
				rhs,
			)
			.await?;

			let (player_lhs, data_lhs) = commands::get_player_data(
				ctx,
				uuid_lhs.or_else(|| parse_uuid(lhs.as_deref())),
				lhs,
			)
			.await?;

			player_lhs.increase_searches(ctx).await?;
			player_rhs.increase_searches(ctx).await?;

			let content = tr_fmt!(
				ctx, "showing-comparison",
				from: data_rhs.username.as_str(),
				to: data_lhs.username.as_str(),
			);

			let attachments = G::condensed_diff(
				ctx,
				family,
				&data_lhs,
				&data_rhs,
				suffix.as_deref(),
				background,
			)
			.into_iter()
			.map(|mut surface| {
				CreateAttachment::bytes(Cow::Owned(canvas::to_png(&mut surface)), crate::IMAGE_NAME)
			})
			.collect::<Vec<_>>();

			let (_, id) = G::Mode::as_compare(ctx, player_lhs.uuid, player_rhs.uuid, None);

			let mut reply = poise::CreateReply::new().content(format!(
				"{}\n{content}",
				tr_fmt!(ctx, "identifier", identifier: api::id::encode(&id)),
			));

			reply.attachments = attachments;

			ctx.send(reply).await?;
		}
		format::Display::Text => {
			let (player_rhs, data_rhs) = commands::get_player_data(
				ctx,
				uuid_rhs.or_else(|| parse_uuid(rhs.as_deref())),
				rhs,
			)
			.await?;

			let (player_lhs, data_lhs) = commands::get_player_data(
				ctx,
				uuid_lhs.or_else(|| parse_uuid(lhs.as_deref())),
				lhs,
			)
			.await?;

			player_lhs.increase_searches(ctx).await?;
			player_rhs.increase_searches(ctx).await?;

			let content = tr_fmt!(
				ctx, "showing-comparison",
				from: data_rhs.username.as_str(),
				to: data_lhs.username.as_str(),
			);

			let embed =
				G::embed_diff(ctx, &player_rhs, &data_lhs, &data_rhs).colour(crate::EMBED_COLOUR);

			ctx.send(poise::CreateReply::new().content(content).embed(embed))
				.await?;
		}
	}

	Ok(())
}
