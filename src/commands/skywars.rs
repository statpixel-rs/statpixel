use std::borrow::Cow;

use canvas::{create_surface, to_png};
use poise::serenity_prelude::AttachmentType;

use crate::{
	util::{error_embed, error_handler, get_player_from_input, success_embed},
	Context, Error,
};

/// Shows the SkyWars stats of a player.
#[poise::command(slash_command, on_error = "error_handler")]
pub async fn skywars(
	ctx: Context<'_>,
	#[description = "Your Minecraft UUID"] uuid: Option<String>,
	#[description = "Your Minecraft username"] username: Option<String>,
) -> Result<(), Error> {
	let player = match get_player_from_input(ctx, ctx.author(), uuid, username).await {
		Ok(player) => player,
		Err(Error::NotLinked) => {
			ctx.send(|m| {
				error_embed(
					m,
					"Missing arguments",
					"Invalid UUID or username provided, and you are not linked.",
				)
			})
			.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	let data = player.get_data().await?;
	let png: Cow<[u8]> = {
		let mut surface = create_surface(2);

		canvas::header::apply(&mut surface, &data);
		canvas::skywars::apply(&mut surface, &data);

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
