use std::borrow::Cow;

use api::canvas;
use translate::{tr, Context, Error};

use crate::util::{error_embed, get_guild_from_input};

/// Shows the stats of a guild.
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn guild(
	ctx: Context<'_>,
	#[min_length = 3]
	#[max_length = 32]
	name: Option<String>,
	#[max_length = 16] username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let guild = match get_guild_from_input(ctx, ctx.author(), name, uuid, username).await {
		Ok(guild) => guild,
		Err(Error::NotLinked) => {
			ctx.send(|m| error_embed(m, tr!(ctx, "not-linked"), tr!(ctx, "not-linked")))
				.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	let png: Cow<_> = {
		let mut surface = canvas::create_surface(2);

		canvas::header::apply_guild(&mut surface, &guild);
		canvas::to_png(&mut surface).into()
	};

	ctx.send(move |m| {
		m.attachment(poise::serenity_prelude::AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".to_string(),
		})
	})
	.await?;

	Ok(())
}
