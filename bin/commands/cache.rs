use api::cache::{PLAYER_CACHE, PLAYER_DATA_CACHE, PLAYER_SESSION_CACHE};

use crate::{util::success_embed, Context, Error};

/// Views the current cache data
#[poise::command(slash_command)]
pub async fn cache(ctx: Context<'_>) -> Result<(), Error> {
	let player_data_count = PLAYER_DATA_CACHE.entry_count();
	let player_session_count = PLAYER_SESSION_CACHE.entry_count();
	let player_count = PLAYER_CACHE.entry_count();

	ctx.send(|m| {
		success_embed(
			m,
			"Cache statistics",
			&format!(
				"Cached profiles: {player_data_count}\nCached sessions: {player_session_count}\nCached players: {player_count}"
			),
		)
	})
	.await?;

	Ok(())
}
