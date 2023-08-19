use crate::Error;
use api::{
	command::{Id, Mode},
	id,
};
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt};
use uuid::Uuid;

#[allow(clippy::too_many_lines)]
pub async fn recent(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;

	let (player, data, games, session, skin, suffix) =
		crate::commands::get_player_data_games_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png = super::image::recent(
		ctx,
		family,
		&data,
		&games,
		&session,
		skin.image(),
		suffix.as_deref(),
		background,
	);

	let id = id::command(Id::Root {
		uuid: player.uuid,
		kind: Mode::RecentGames,
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
