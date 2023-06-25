use std::borrow::Cow;

use api::prelude::Mode;
use chrono::{DateTime, Utc};
use database::schema;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt, Error};
use uuid::Uuid;

use crate::{commands, snapshot, util};

pub async fn command<G: api::prelude::Game>(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	mode: Option<G::Mode>,
	kind: Option<<G::Mode as Mode>::Kind>,
	value: Option<f64>,
) -> Result<(), Error> {
	let (player, session) = commands::get_player_session(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let snapshots = schema::snapshot::table
		.filter(schema::snapshot::uuid.eq(player.uuid))
		.order(schema::snapshot::created_at.asc())
		.select((schema::snapshot::created_at, schema::snapshot::data))
		.get_results::<(DateTime<Utc>, Vec<u8>)>(&mut ctx.data().pool.get().await?)
		.await?;

	if snapshots.is_empty() {
		let data = player.get_data().await?;

		snapshot::user::insert(ctx, &player, &data).await?;

		let content = tr_fmt!(
			ctx, "no-previous-statistics",
			name: util::escape_username(&data.username),
		);

		ctx.send(poise::CreateReply::new().content(content)).await?;

		return Ok(());
	}

	let snapshots = {
		let mut snapshots_ = Vec::with_capacity(snapshots.len());

		for (time, data) in snapshots {
			let data = snapshot::user::decode(&data)?;

			snapshots_.push((time, data));
		}

		snapshots_
	};

	let png = {
		let buffer = G::project(ctx, snapshots, &session, mode, kind, value)?;

		Cow::Owned(buffer)
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
