mod bedwars;
mod cache;
mod display;
mod link;
mod skywars;
mod unlink;

pub use bedwars::*;
pub use cache::*;
pub use display::*;
pub use link::*;
pub use skywars::*;
pub use unlink::*;

/// Generates the code needed to fetch the player, their data, and their session.
#[macro_export]
macro_rules! get_data {
	($ctx: ident, $uuid: ident, $username: ident) => {{
		let player = match $crate::util::get_player_from_input(
			$ctx,
			$ctx.author(),
			$uuid,
			$username,
		)
		.await
		{
			Ok(player) => player,
			Err($crate::Error::NotLinked) => {
				$ctx.send(|m| {
					$crate::util::error_embed(
						m,
						::translate::tr!($ctx, "not-linked"),
						::translate::tr!($ctx, "not-linked-description"),
					)
				})
				.await?;

				return Ok(());
			}
			Err(e) => return Err(e),
		};

		let (data, session) =
			poise::futures_util::future::join(player.get_data(), player.get_session()).await;

		let data = data?;
		let session = session?;

		(player, data, session)
	}};
}
