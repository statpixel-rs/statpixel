use api::command::SkyBlockMode;
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt, Error};
use uuid::Uuid;

pub async fn auctions(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	profile_id: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png = super::image::auctions(
		ctx,
		family,
		&player,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
		background,
	)
	.await?;

	let (row, id) =
		SkyBlockMode::as_root(ctx, player.uuid, profile_id, Some(SkyBlockMode::Auctions));

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

#[allow(clippy::too_many_lines)]
pub async fn profile(
	ctx: &context::Context<'_>,
	username: Option<String>,
	profile: Option<String>,
	uuid: Option<Uuid>,
	profile_id: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let (png, profile) = super::image::profile(
		ctx,
		family,
		&player,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
		background,
		profile_id,
		profile,
	)
	.await?;

	let (row, id) = SkyBlockMode::as_root(
		ctx,
		player.uuid,
		Some(profile.id),
		Some(SkyBlockMode::Profile),
	);

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

pub async fn bank(
	ctx: &context::Context<'_>,
	username: Option<String>,
	profile: Option<String>,
	uuid: Option<Uuid>,
	profile_id: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let (player, data) = crate::commands::get_player_data(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let (png, profile) =
		super::image::bank(ctx, family, &data, background, profile_id, profile).await?;

	let (row, id) =
		SkyBlockMode::as_root(ctx, player.uuid, Some(profile.id), Some(SkyBlockMode::Bank));

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

#[allow(clippy::too_many_lines)]
pub async fn networth(
	ctx: &context::Context<'_>,
	username: Option<String>,
	profile: Option<String>,
	uuid: Option<Uuid>,
	profile_id: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let (png, profile) = super::image::networth(
		ctx,
		family,
		&player,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
		background,
		profile_id,
		profile,
	)
	.await?;

	let (row, id) = SkyBlockMode::as_root(
		ctx,
		player.uuid,
		Some(profile.id),
		Some(SkyBlockMode::Networth),
	);

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

#[allow(clippy::too_many_lines)]
pub async fn pets(
	ctx: &context::Context<'_>,
	username: Option<String>,
	profile: Option<String>,
	uuid: Option<Uuid>,
	profile_id: Option<Uuid>,
) -> Result<(), Error> {
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let (png, profile) = super::image::pets(
		ctx,
		family,
		&player,
		&data,
		&session,
		skin.image(),
		suffix.as_deref(),
		background,
		profile_id,
		profile.as_deref(),
	)
	.await?;

	let (row, id) =
		SkyBlockMode::as_root(ctx, player.uuid, Some(profile.id), Some(SkyBlockMode::Pets));

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

#[allow(clippy::too_many_lines)]
pub async fn bazaar(ctx: &context::Context<'_>, product: String) -> Result<(), Error> {
	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;

	let png = super::image::bazaar(ctx, family, product.as_str(), background).await?;

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

macro_rules! inventory_command {
	($fn: ident, $key: ident, $mode: ident) => {
		#[allow(clippy::too_many_lines)]
		pub async fn $fn(
			ctx: &context::Context<'_>,
			username: Option<String>,
			profile: Option<String>,
			uuid: Option<Uuid>,
			profile_id: Option<Uuid>,
		) -> Result<(), Error> {
			let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
			let (player, data, session, skin, suffix) =
				crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let (png, profile) = super::image::$fn(
				ctx,
				family,
				&player,
				&data,
				&session,
				skin.image(),
				suffix.as_deref(),
				background,
				profile_id,
				profile.as_deref(),
			)
			.await?;

			let (row, id) = SkyBlockMode::as_root(
				ctx,
				player.uuid,
				Some(profile.id),
				Some(SkyBlockMode::$mode),
			);

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
	};
}

inventory_command!(inventory, inventory, Inventory);
inventory_command!(enderchest, ender_chest, EnderChest);
inventory_command!(talisman, talisman_bag, Talisman);
inventory_command!(quiver, quiver, Quiver);
inventory_command!(fishing, fishing_bag, Fishing);
inventory_command!(potions, potion_bag, Potions);
inventory_command!(equipment, equipment, Equipment);
inventory_command!(wardrobe, wardrobe, Wardrobe);
inventory_command!(candy, candy, Candy);
inventory_command!(vault, vault, Vault);
