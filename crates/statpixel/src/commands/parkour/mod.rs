pub mod image;
pub mod run;

use crate::util;
use translate::{context, Context, Error};

#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn parkour(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);

	run::parkour(ctx, player, uuid).await
}
