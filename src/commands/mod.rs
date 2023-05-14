use database::{extend::lower, schema};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use translate::Context;

pub mod cache;
pub mod daily;
pub mod display;
pub mod games;
pub mod link;
pub mod monthly;
pub mod ser;
pub mod unlink;
pub mod weekly;

#[allow(clippy::unused_async)]
pub async fn autocomplete_username(
	ctx: Context<'_>,
	partial: &str,
) -> Box<dyn Iterator<Item = String> + Send> {
	tracing::info!("Autocompleting username `{partial}`");

	if let Ok(mut connection) = ctx.data().pool.get() {
		if partial.is_empty() || partial.contains('%') {
			let result = schema::autocomplete::table
				.filter(schema::autocomplete::name.is_not_null())
				.limit(10)
				.select(schema::autocomplete::name)
				.get_results::<String>(&mut connection);

			if let Ok(result) = result {
				return Box::new(result.into_iter());
			}
		} else {
			let result = schema::autocomplete::table
				.filter(
					lower(schema::autocomplete::name)
						.like(format!("{}%", partial.to_ascii_lowercase())),
				)
				.limit(9)
				.select(schema::autocomplete::name)
				.get_results::<String>(&mut connection);

			if let Ok(result) = result {
				return Box::new(std::iter::once(partial.to_string()).chain(result.into_iter()));
			}
		}
	}

	Box::new(std::iter::once(partial.to_string()))
}

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
