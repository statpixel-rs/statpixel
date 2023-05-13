use database::{extend::lower, schema};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
use translate::Context;

pub mod arcade;
pub mod arena;
pub mod bed_wars;
pub mod blitz;
pub mod build_battle;
pub mod cache;
pub mod cops_and_crims;
pub mod display;
pub mod duels;
pub mod link;
pub mod mega_walls;
pub mod murder_mystery;
pub mod paintball;
pub mod pit;
pub mod quake;
pub mod ser;
pub mod sky_wars;
pub mod smash_heroes;
pub mod speed_uhc;
pub mod tnt_games;
pub mod turbo_kart_racers;
pub mod uhc;
pub mod unlink;
pub mod vampire_z;
pub mod walls;
pub mod warlords;
pub mod wool_wars;

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
			let start = std::time::Instant::now();
			let result = schema::autocomplete::table
				.filter(
					lower(schema::autocomplete::name)
						.like(format!("{}%", partial.to_ascii_lowercase())),
				)
				// No null fields will be matched by the LIKE filter, and
				// there will be no null fields once it has all been populated.
				// .filter(schema::autocomplete::name.is_not_null())
				.limit(9)
				.select(schema::autocomplete::name)
				.get_results::<String>(&mut connection);

			if let Ok(result) = result {
				tracing::info!(
					partial = partial,
					"Found {} autocompletions in {}s",
					result.len(),
					start.elapsed().as_secs_f32()
				);

				return Box::new(std::iter::once(partial.to_string()).chain(result.into_iter()));
			}
		}
	}

	Box::new(std::iter::once(partial.to_string()))
}

#[macro_export]
macro_rules! generate_large_command {
	($game: ty, $mode: ty, $fn: ident) => {
		async fn autocomplete_mode<'a>(
			ctx: $crate::Context<'a>,
			partial: &'a str,
		) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
			let partial = partial.to_ascii_lowercase();

			<$game>::autocomplete(ctx, partial).await
		}

		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
		pub async fn $fn<'a>(
			ctx: $crate::Context<'a>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let (_player, data, session) = $crate::get_data!(ctx, uuid, username);

			let png: ::std::borrow::Cow<[u8]> = {
				let mut surface = <$game>::canvas(ctx, &data, &session, mode);

				::api::canvas::to_png(&mut surface).into()
			};

			ctx.send(move |m| {
				m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
					data: png,
					filename: "canvas.png".into(),
				})
			})
			.await?;

			Ok(())
		}
	};
}

#[macro_export]
macro_rules! generate_command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			mode: Option<$mode>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let (_player, data, session) = $crate::get_data!(ctx, uuid, username);

			let png: ::std::borrow::Cow<[u8]> = {
				let mut surface = <$game>::canvas(ctx, &data, &session, mode);

				::api::canvas::to_png(&mut surface).into()
			};

			ctx.send(move |m| {
				m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
					data: png,
					filename: "canvas.png".into(),
				})
			})
			.await?;

			Ok(())
		}
	};
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
