use crate::Error;
use api::{
	command::{Id, Mode},
	id,
};
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt};
use uuid::Uuid;

#[allow(clippy::too_many_lines)]
pub async fn winstreaks(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;

	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png = super::image::winstreaks(
		ctx,
		family,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
		background,
	);

	let id = id::command(Id::Root {
		uuid: player.uuid,
		kind: Mode::Winstreaks,
		background: None,
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

	Ok(())
}
