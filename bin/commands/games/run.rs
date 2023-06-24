use std::borrow::Cow;

use api::{canvas, prelude::Mode};
use poise::serenity_prelude::AttachmentType;
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

			ctx.send(move |m| {
				m.attachments = vec![AttachmentType::Bytes {
					data: png,
					filename: crate::IMAGE_NAME.into(),
				}];
				m.components = Some(G::Mode::as_root(ctx, player.uuid, mode));
				m.content(crate::tip::random(ctx));
				m
			})
			.await?;
		}
		format::Display::Text => {
			let (player, data) = commands::get_player_data(ctx, uuid, username).await?;
			let mut embed = G::embed(ctx, &player, &data);

			player.increase_searches(ctx).await?;
			embed.colour(crate::EMBED_COLOUR);

			ctx.send(|m| {
				m.embeds.push(embed);
				m.content(crate::tip::random(ctx));
				m
			})
			.await?;
		}
	}

	Ok(())
}
