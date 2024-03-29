use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt};
use uuid::Uuid;

use crate::{util, Error};

pub async fn hotbar(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = util::get_image_options_from_input(ctx).await;

	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png = super::image::hotbar(
		ctx,
		family,
		background,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
	);

	let id = api::id::command(api::command::Id::Root {
		kind: api::command::Mode::BedWarsHotbar,
		uuid: player.uuid,
	});

	ctx.send(
		poise::CreateReply::new()
			.content(format!(
				"{}\n{}",
				tr_fmt!(ctx, "identifier", identifier: id),
				crate::tip::random(ctx)
			))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

pub async fn shop(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = util::get_image_options_from_input(ctx).await;

	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png = super::image::shop(
		ctx,
		family,
		background,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
	);

	let id = api::id::command(api::command::Id::Root {
		kind: api::command::Mode::BedWarsShop,
		uuid: player.uuid,
	});

	ctx.send(
		poise::CreateReply::new()
			.content(format!(
				"{}\n{}",
				tr_fmt!(ctx, "identifier", identifier: id),
				crate::tip::random(ctx)
			))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

pub async fn practice(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = util::get_image_options_from_input(ctx).await;

	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png = super::image::practice(
		ctx,
		family,
		background,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
	);

	let id = api::id::command(api::command::Id::Root {
		kind: api::command::Mode::BedWarsPractice,
		uuid: player.uuid,
	});

	ctx.send(
		poise::CreateReply::new()
			.content(format!(
				"{}\n{}",
				tr_fmt!(ctx, "identifier", identifier: id),
				crate::tip::random(ctx)
			))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}
