use std::borrow::Cow;

use api::{Player, Session};
use chrono::{DateTime, Utc};
use database::schema;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use minecraft::style::Family;
use skia_safe::Color;
use translate::{context, tr_fmt, Error};

use crate::snapshot;

#[inline]
pub async fn command<G: api::canvas::prelude::Game>(
	ctx: &context::Context<'_>,
	family: Family,
	mode: Option<G::Mode>,
	player: &Player,
	session: &Session,
	background: Option<Color>,
) -> Result<Option<(Cow<'static, [u8]>, G::Mode)>, Error> {
	let snapshots = schema::snapshot::table
		.filter(schema::snapshot::uuid.eq(player.uuid))
		.order(schema::snapshot::created_at.asc())
		.select((schema::snapshot::created_at, schema::snapshot::data))
		.get_results::<(DateTime<Utc>, Vec<u8>)>(&mut ctx.connection().await?)
		.await?;

	if snapshots.is_empty() {
		let data = player.get_data(ctx).await?;

		snapshot::user::insert(ctx, player, &data).await?;

		let content = tr_fmt!(
			ctx, "no-previous-statistics",
			name: data.username.as_str(),
		);

		ctx.send(poise::CreateReply::new().content(content)).await?;

		return Ok(None);
	}

	let snapshots = {
		let mut snapshots_ = Vec::with_capacity(snapshots.len());

		for (time, data) in snapshots {
			let data = api::snapshot::user::decode(&data)?;

			snapshots_.push((time, data));
		}

		snapshots_
	};

	let (buffer, mode) = G::chart(ctx, family, snapshots, session, background, mode)?;

	Ok(Some((buffer.into(), mode)))
}
