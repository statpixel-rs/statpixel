pub mod image;
pub mod run;

use crate::{util, Error};
use translate::{context, Context};

#[allow(clippy::too_many_lines)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
async fn general(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);

	run::network(ctx, player, uuid).await
}

#[allow(clippy::unused_async)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES",
	subcommands("general", "super::history::network")
)]
pub async fn network(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
