use api::command::GuildMode;
use chrono::Utc;
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr, tr_fmt};
use uuid::Uuid;

use crate::{util::error_embed, Error};

#[allow(clippy::too_many_lines)]
pub async fn top(
	ctx: &context::Context<'_>,
	name: Option<String>,
	username: Option<String>,
	uuid: Option<Uuid>,
	past: chrono::Duration,
	limit: usize,
	guild_id: Option<Uuid>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (guild, player) =
		match crate::commands::get_guild_with_member_opt(ctx, name, uuid, username, guild_id).await
		{
			Ok(data) => data,
			Err(Error::NotLinked) => {
				ctx.send(error_embed(tr(ctx, "not-linked"), tr(ctx, "not-linked")))
					.await?;

				return Ok(());
			}
			Err(e) => return Err(e),
		};

	guild.increase_searches(ctx).await?;

	let after = Utc::now() - past;
	let status = crate::snapshot::guild::get_or_insert(ctx, &guild, after).await?;

	let png = super::image::top(ctx, &guild, limit, after, background).await?;
	let content = match status {
		crate::snapshot::guild::Status::Found((_, created_at)) => ::translate::tr_fmt!(
			ctx, "showing-guild-xp-statistics",
			from: ::std::format!("<t:{}:f>", created_at.timestamp()),
			to: ::std::format!("<t:{}:f>", chrono::Utc::now().timestamp()),
		),
		crate::snapshot::guild::Status::Inserted => ::translate::tr_fmt!(
			ctx, "no-previous-guild-statistics",
			name: guild.name.as_str(),
		),
	};

	let (row, id) = GuildMode::as_root(
		ctx,
		Uuid::from_u128(guild.id),
		Some(limit),
		past.num_nanoseconds(),
		uuid.or_else(|| player.map(|p| p.uuid)),
		Some(GuildMode::Top),
	);

	ctx.send(
		poise::CreateReply::new()
			.content(format!(
				"{}\n{}",
				tr_fmt!(ctx, "identifier", identifier: api::id::encode(&id)),
				content,
			))
			.components(vec![row])
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

#[allow(clippy::too_many_lines)]
pub async fn members(
	ctx: &context::Context<'_>,
	name: Option<String>,
	username: Option<String>,
	uuid: Option<Uuid>,
	guild_id: Option<Uuid>,
	limit: Option<usize>,
	past_nanos: Option<i64>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (guild, player) =
		match crate::commands::get_guild_with_member_opt(ctx, name, uuid, username, guild_id).await
		{
			Ok(data) => data,
			Err(Error::NotLinked) => {
				ctx.send(error_embed(tr(ctx, "not-linked"), tr(ctx, "not-linked")))
					.await?;

				return Ok(());
			}
			Err(e) => return Err(e),
		};

	guild.increase_searches(ctx).await?;

	let png = super::image::members(ctx, &guild, background).await?;

	let (row, id) = GuildMode::as_root(
		ctx,
		Uuid::from_u128(guild.id),
		limit,
		past_nanos,
		uuid.or_else(|| player.map(|p| p.uuid)),
		Some(GuildMode::Members),
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
pub async fn general(
	ctx: &context::Context<'_>,
	name: Option<String>,
	username: Option<String>,
	uuid: Option<Uuid>,
	guild_id: Option<Uuid>,
	limit: Option<usize>,
	past_nanos: Option<i64>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (guild, player) =
		match crate::commands::get_guild_with_member_opt(ctx, name, uuid, username, guild_id).await
		{
			Ok(data) => data,
			Err(Error::NotLinked) => {
				ctx.send(error_embed(tr(ctx, "not-linked"), tr(ctx, "not-linked")))
					.await?;

				return Ok(());
			}
			Err(e) => return Err(e),
		};

	guild.increase_searches(ctx).await?;

	let png = super::image::general(ctx, &guild, background).await?;

	let (row, id) = GuildMode::as_root(
		ctx,
		Uuid::from_u128(guild.id),
		limit,
		past_nanos,
		uuid.or_else(|| player.map(|p| p.uuid)),
		Some(GuildMode::General),
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
pub async fn member(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	limit: Option<usize>,
	past_nanos: Option<i64>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (guild, player) = match crate::commands::get_guild_with_member(ctx, uuid, username).await {
		Ok(guild) => guild,
		Err(Error::NotLinked) => {
			ctx.send(error_embed(tr(ctx, "not-linked"), tr(ctx, "not-linked")))
				.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	guild.increase_searches(ctx).await?;

	let png = super::image::member(ctx, &guild, &player, background).await?;

	let (row, id) = GuildMode::as_root(
		ctx,
		Uuid::from_u128(guild.id),
		limit,
		past_nanos,
		Some(player.uuid),
		Some(GuildMode::Member),
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
