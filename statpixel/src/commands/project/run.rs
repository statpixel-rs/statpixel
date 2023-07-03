use api::prelude::Mode;
use poise::serenity_prelude::CreateAttachment;
use translate::{context, Error};
use uuid::Uuid;

use crate::commands;

pub async fn command<G: api::prelude::Game>(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	mode: Option<G::Mode>,
	kind: Option<<G::Mode as Mode>::Kind>,
	value: Option<f64>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (player, session) = commands::get_player_session(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let Some(png) =
		super::image::command::<G>(ctx, &player, &session, background, mode, kind, value).await?
	else {
		return Ok(());
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.components(vec![G::Mode::as_project(
				ctx,
				player.uuid,
				kind.unwrap_or_default(),
				mode,
			)])
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}
