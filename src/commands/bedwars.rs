use std::borrow::Cow;

use api::canvas::to_png;
use api::player::stats::bedwars::{BedWars, BedWarsMode};
use poise::{futures_util::future::join, serenity_prelude::AttachmentType};
use translate::tr;

use crate::get_data;
use crate::{
	util::{error_embed, get_player_from_input},
	Context, Error,
};

/// Shows the BedWars stats of a player.
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn bedwars(
	ctx: Context<'_>,
	#[max_length = 16] username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
	mode: Option<BedWarsMode>,
) -> Result<(), Error> {
	let (_player, data, session) = get_data!(ctx, uuid, username);

	let png: Cow<[u8]> = {
		let mut surface = BedWars::canvas(ctx, &data, &session, mode);

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
