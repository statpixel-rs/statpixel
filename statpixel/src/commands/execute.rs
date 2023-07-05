use translate::{context, tr};

use crate::{
	util::{error_embed, invalid_identifier},
	Context, Error,
};

/// Executes a command by its ID.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "EMBED_LINKS"
)]
pub async fn execute(ctx: Context<'_>, id: String) -> Result<(), Error> {
	let Some(id) = api::id::decode(&id) else {
		ctx.send(invalid_identifier(&ctx)).await?;

		return Ok(());
	};

	let api::id::Id::Command(id) = id else {
		ctx.send(error_embed(
			tr!(&ctx, "invalid-identifier"),
			tr!(&ctx, "invalid-identifier-command-description"),
		))
		.await?;

		return Ok(());
	};

	let ctx = context::Context::from_poise(&ctx);

	crate::id::map(&ctx, id).await
}
