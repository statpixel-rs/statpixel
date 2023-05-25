use std::borrow::Cow;

use api::canvas::chart;
use chrono::{DateTime, Utc};
use database::schema::snapshot;
use diesel::{ExpressionMethods, QueryDsl};
use poise::serenity_prelude::AttachmentType;
use translate::Context;

/// Shows the help menu.
#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn history(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<::std::string::String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<::std::string::String>,
) -> ::std::result::Result<(), ::translate::Error> {
	ctx.defer().await?;

	let player = crate::util::get_player_from_input(ctx, uuid, username).await?;

	let snapshots = diesel_async::RunQueryDsl::get_results::<(DateTime<Utc>, Vec<u8>)>(
		snapshot::table
			.filter(snapshot::uuid.eq(player.uuid))
			.order(snapshot::created_at.asc())
			.select((snapshot::created_at, snapshot::data)),
		&mut ctx.data().pool.get().await?,
	)
	.await?;

	let (Some(first), Some(last)) = (snapshots.first(), snapshots.last()) else {
		return Err(::translate::Error::Custom("No data found for this player."));
	};

	let first = (first.0, crate::snapshot::user::decode(&first.1)?);
	let last = (last.0, crate::snapshot::user::decode(&last.1)?);

	let lower = first.1.xp * 15 / 16;
	let upper = last.1.xp * 16 / 15;

	let rank = last.1.get_rank();
	let username_paint = rank.get_username_paint();

	let png = {
		let mut buffer = chart::u64::create(
			ctx,
			vec![(
				"Network XP",
				snapshots.into_iter().filter_map(|(created_at, bytes)| {
					let data = crate::snapshot::user::decode(&bytes).ok()?;

					Some((created_at, data.xp))
				}),
			)],
			first.0..last.0,
			lower..upper,
			Some(username_paint),
		)?;

		let mut surface = chart::canvas(&mut buffer)?;

		chart::apply_title(ctx, &mut surface, &last.1);
		chart::round_corners(&mut surface);

		Cow::Owned(
			surface
				.image_snapshot()
				.encode_to_data(skia_safe::EncodedImageFormat::PNG)
				.unwrap()
				.to_vec(),
		)
	};

	ctx.send(move |m| {
		m.attachment(AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".to_string(),
		})
	})
	.await?;

	Ok(())
}
