use api::prelude::Mode;
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt};
use uuid::Uuid;

use crate::{commands, format, util, Error};

pub async fn command<G: api::prelude::Game>(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	mode: Option<G::Mode>,
) -> Result<(), Error> {
	let (format, family, background) = util::get_image_options_from_input(ctx).await;

	match format {
		format::Display::Image | format::Display::Compact => {
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
