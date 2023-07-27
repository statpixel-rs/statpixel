use std::borrow::Cow;

use api::{canvas, prelude::Mode};
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt, Error};
use uuid::Uuid;

use crate::{
	commands, format,
	util::{self, parse_uuid},
};

pub async fn command<G: api::prelude::Game>(
	ctx: &context::Context<'_>,
	from: Option<String>,
	to: Option<String>,
	mode: Option<G::Mode>,
	from_uuid: Option<Uuid>,
	to_uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (format, background) = util::get_format_colour_from_input(ctx).await;

	match format {
		format::Display::Image | format::Display::Compact => {
			let (player, data, session, skin, suffix) =
				commands::get_player_data_session_skin_suffix(
					ctx,
					from_uuid.or_else(|| parse_uuid(from.as_deref()).ok().flatten()),
					from,
				)
				.await?;

			let (player_to, data_to) = commands::get_player_data(
				ctx,
				to_uuid.or_else(|| parse_uuid(to.as_deref()).ok().flatten()),
				to,
			)
			.await?;

			player.increase_searches(ctx).await?;
			player_to.increase_searches(ctx).await?;

			let content = tr_fmt!(
				ctx, "showing-comparison",
				from: data.username.as_str(),
				to: data_to.username.as_str(),
			);

			let (png, mode): (Cow<_>, _) = {
				let (mut surface, mode) = G::canvas_diff(
					ctx,
					&data_to,
					&mut api::Data::clone(&data),
					&session,
					skin.image(),
					mode,
					suffix.as_deref(),
					background,
				);

				(canvas::to_png(&mut surface).into(), mode)
			};

			let (row, id) = G::Mode::as_compare(ctx, player.uuid, player_to.uuid, Some(mode));

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
		format::Display::Text => {
			let (player, data) = commands::get_player_data(
				ctx,
				from_uuid.or_else(|| parse_uuid(from.as_deref()).ok().flatten()),
				from,
			)
			.await?;

			let (player_to, data_to) = commands::get_player_data(
				ctx,
				to_uuid.or_else(|| parse_uuid(to.as_deref()).ok().flatten()),
				to,
			)
			.await?;

			player.increase_searches(ctx).await?;
			player_to.increase_searches(ctx).await?;

			let content = tr_fmt!(
				ctx, "showing-comparison",
				from: data.username.as_str(),
				to: data_to.username.as_str(),
			);

			let embed = G::embed_diff(ctx, &player, &data_to, &mut api::Data::clone(&data))
				.colour(crate::EMBED_COLOUR);

			ctx.send(poise::CreateReply::new().content(content).embed(embed))
				.await?;
		}
	}

	Ok(())
}
