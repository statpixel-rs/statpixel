use api::prelude::Mode;
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt, Error};
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
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let (player, session) = commands::get_player_session(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let Some((png, mode)) =
		super::image::command::<G>(ctx, family, &player, &session, background, mode, kind, value).await?
	else {
		return Ok(());
	};

	let (row, id) = G::Mode::as_project(ctx, player.uuid, kind.unwrap_or_default(), Some(mode));

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

	Ok(())
}
