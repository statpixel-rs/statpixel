pub mod image;
pub mod run;

use crate::util;
use translate::{context, Context, Error};

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn hotbar(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);

	run::hotbar(ctx, player, uuid).await
}

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn shop(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);

	run::shop(ctx, player, uuid).await
}

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn practice(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);

	run::practice(ctx, player, uuid).await
}
