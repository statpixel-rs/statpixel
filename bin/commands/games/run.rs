use std::borrow::Cow;

use api::{canvas, prelude::Mode};
use poise::serenity_prelude::CreateAttachment;
use translate::context;
use uuid::Uuid;

use crate::{commands, format, util, Error};

pub async fn command<G: api::prelude::Game>(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	mode: Option<G::Mode>,
) -> Result<(), Error> {
	let (format, background) = util::get_format_colour_from_input(ctx).await;

	match format {
		format::Display::Image | format::Display::Compact => {
			let (player, data, session, skin, suffix) =
				crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let png: Cow<[u8]> = {
				let mut surface = G::canvas(
					ctx,
					&data,
					&session,
					skin.image(),
					mode,
					suffix.as_deref(),
					background,
				);

				canvas::to_png(&mut surface).into()
			};

			ctx.send(
				poise::CreateReply::new()
					.content(crate::tip::random(ctx))
					.components(vec![G::Mode::as_root(ctx, player.uuid, mode)])
					.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
			)
			.await?;
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
