use crate::Error;
use poise::serenity_prelude::CreateAttachment;
use translate::context;
use uuid::Uuid;

#[allow(clippy::too_many_lines)]
pub async fn winstreaks(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;

	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png = super::image::winstreaks(
		ctx,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
		background,
	);

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}
