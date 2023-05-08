use std::borrow::Cow;

use api::canvas::to_png;
use api::player::stats::skywars::{SkyWars, SkyWarsMode};
use poise::serenity_prelude::AttachmentType;
use translate::Error;

use crate::get_data;
use crate::Context;

/// Shows the SkyWars stats of a player.
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn skywars(
	ctx: Context<'_>,
	#[max_length = 16] username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
	mode: Option<SkyWarsMode>,
) -> Result<(), Error> {
	let (_player, data, session) = get_data!(ctx, uuid, username);

	let png: Cow<[u8]> = {
		let mut surface = SkyWars::canvas(ctx, &data, &session, mode);

		to_png(&mut surface).into()
	};

	ctx.send(move |m| {
		m.attachment(AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".into(),
		})
	})
	.await?;

	Ok(())
}
