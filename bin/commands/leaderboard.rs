use api::{canvas, player::Player};
use futures::StreamExt;
use poise::serenity_prelude::{AttachmentType, CreateEmbed};
use translate::{Context, Error};

use crate::{format::Display, util::escape_username};

async fn autocomplete_board(
	_ctx: Context<'_>,
	partial: &str,
) -> Box<dyn Iterator<Item = String> + Send> {
	let Ok(leaderboards) = api::leaderboard::get().await else {
		return Box::new(std::iter::empty());
	};

	let lower = partial.to_ascii_lowercase();

	Box::new(
		leaderboards
			.into_iter()
			.filter_map(|(name, board)| {
				if !name.starts_with(&lower) {
					return None;
				}

				Some(board.display_name)
			})
			.take(10)
			.collect::<Vec<_>>()
			.into_iter(),
	)
}

#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn leaderboard(
	ctx: Context<'_>,
	#[autocomplete = "autocomplete_board"] board: String,
) -> Result<(), Error> {
	let format = crate::util::get_format_from_input(ctx).await;
	let leaderboard = {
		let mut leaderboards = api::leaderboard::get().await?;
		let Some(leaderboard) = leaderboards.remove(&board.to_ascii_lowercase()) else {
			return Err(Error::Custom(format!("No leaderboard found with the name `{}`.", board)));
		};

		leaderboard
	};

	let api::leaderboard::Leaderboard {
		name,
		leaders,
		game,
		path,
		display_name,
	} = leaderboard;

	#[allow(clippy::cast_possible_truncation)]
	let leaders_len = leaders.len() as u8;

	let leaders = futures::stream::iter(
		leaders
			.into_iter()
			.map(Player::from_uuid_unchecked)
			.map(Player::get_data_owned),
	)
	.buffered(10)
	.filter_map(|r| async { r.ok() })
	.collect::<Vec<_>>()
	.await;

	let leaderboard = api::leaderboard::Leaderboard {
		name,
		leaders: vec![],
		game,
		path,
		display_name,
	};

	match format {
		Display::Image | Display::Compact => {
			let png = {
				let mut surface = canvas::leaderboard::create(leaders_len);

				canvas::leaderboard::header(&mut surface, &leaderboard);

				for (idx, player) in leaders.iter().enumerate() {
					let value = player
						.stats
						.get_value(&leaderboard.game, leaderboard.path.as_str())
						.unwrap_or_else(|| Box::new(0));

					#[allow(clippy::cast_possible_truncation)]
					canvas::leaderboard::row(ctx, &mut surface, player, &*value, idx as u8);
				}

				canvas::to_png(&mut surface).into()
			};

			ctx.send(move |m| {
				m.attachment(AttachmentType::Bytes {
					data: png,
					filename: "canvas.png".into(),
				})
			})
			.await?;
		}
		Display::Text => {
			let mut embed = CreateEmbed::default();

			embed.colour(crate::EMBED_COLOUR);
			embed.title(format!("{}", leaderboard.display_name));
			embed.description(format!(
				"{}",
				leaders
					.into_iter()
					.enumerate()
					.map(|(i, player)| {
						let value = player
							.stats
							.get_value(&leaderboard.game, leaderboard.path.as_str())
							.unwrap_or_else(|| Box::new(0));

						let value = value.to_formatted_label(ctx);

						if let Some(prefix) = player.get_rank().as_str() {
							format!(
								"{}. **{} {}** ({})",
								i + 1,
								prefix,
								escape_username(&player.username),
								value
							)
						} else {
							format!(
								"{}. **{}** ({})",
								i + 1,
								escape_username(&player.username),
								value
							)
						}
					})
					.intersperse("\n".to_string())
					.collect::<String>()
			));

			ctx.send(|m| {
				m.embeds.push(embed);
				m
			})
			.await?;
		}
	}

	Ok(())
}
