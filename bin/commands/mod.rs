use database::{extend::lower, schema};
use diesel::{ExpressionMethods, QueryDsl, TextExpressionMethods};
use diesel_async::RunQueryDsl;
use translate::Context;

pub mod cache;
pub mod display;
pub mod games;
pub mod guild;
pub mod help;
pub mod history;
pub mod link;
pub mod ser;
pub mod snapshot;
pub mod unlink;

#[allow(clippy::unused_async)]
pub async fn autocomplete_username(
	ctx: Context<'_>,
	partial: &str,
) -> Box<dyn Iterator<Item = String> + Send> {
	tracing::debug!("Autocompleting username `{partial}`");

	if let Ok(mut connection) = ctx.data().pool.get().await {
		if partial.is_empty() || partial.contains('%') {
			let result = schema::autocomplete::table
				.filter(schema::autocomplete::name.is_not_null())
				.order(schema::autocomplete::searches.desc())
				.limit(10)
				.select(schema::autocomplete::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return Box::new(result.into_iter());
			}
		} else {
			let result = schema::autocomplete::table
				.filter(
					lower(schema::autocomplete::name)
						.like(format!("{}%", partial.to_ascii_lowercase())),
				)
				.order(schema::autocomplete::searches.desc())
				.limit(9)
				.select(schema::autocomplete::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return Box::new(std::iter::once(partial.to_string()).chain(result.into_iter()));
			}
		}
	}

	Box::new(std::iter::once(partial.to_string()))
}

#[allow(clippy::unused_async)]
pub async fn autocomplete_guild_name(
	ctx: Context<'_>,
	partial: &str,
) -> Box<dyn Iterator<Item = String> + Send> {
	tracing::debug!("Autocompleting guild name `{partial}`");

	if let Ok(mut connection) = ctx.data().pool.get().await {
		if partial.is_empty() || partial.contains('%') {
			let result = schema::guild_autocomplete::table
				.filter(schema::guild_autocomplete::name.is_not_null())
				.order(schema::guild_autocomplete::xp.desc())
				.limit(10)
				.select(schema::guild_autocomplete::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return Box::new(result.into_iter());
			}
		} else {
			let result = schema::guild_autocomplete::table
				.filter(
					lower(schema::guild_autocomplete::name)
						.like(format!("{}%", partial.to_ascii_lowercase())),
				)
				.order(schema::guild_autocomplete::xp.desc())
				.limit(9)
				.select(schema::guild_autocomplete::name)
				.get_results::<String>(&mut connection)
				.await;

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
		let player = match $crate::util::get_player_from_input($ctx, $uuid, $username).await {
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

		let format = $crate::util::get_format_from_input($ctx, $ctx.author()).await;
		let (data, session) =
			poise::futures_util::future::join(player.get_data(), player.get_session()).await;

		let data = data?;
		let session = session?;

		(format, player, data, session)
	}};
}
