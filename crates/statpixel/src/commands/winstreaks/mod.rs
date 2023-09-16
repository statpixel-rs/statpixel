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
pub async fn winstreaks(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);

	run::winstreaks(ctx, player, uuid).await
}
