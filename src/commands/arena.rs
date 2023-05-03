use std::borrow::Cow;

use api::{game::r#type::GameType, player::status::PlayerSession};
use canvas::{create_surface, to_png};
use poise::{futures_util::future::join, serenity_prelude::AttachmentType, ChoiceParameter};
use translate::tr;

use crate::{
	util::{error_embed, get_player_from_input},
	Context, Error,
};

#[derive(ChoiceParameter, Debug)]
pub enum ArenaMode {
	Overall,
	Solo,
	Double,
	Four,
}

fn get_game_mode(mode: Option<ArenaMode>, session: &PlayerSession) -> ArenaMode {
	if let Some(mode) = mode {
		mode
	} else if session.game_type == Some(GameType::Arena) && let Some(game_mode) = session.game_mode.as_ref() {
		ArenaMode::from(game_mode.as_str())
	} else {
		ArenaMode::Overall
	}
}

/// Shows the Arena stats of a player.
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn arena(
	ctx: Context<'_>,
	#[max_length = 16] username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
	mode: Option<ArenaMode>,
) -> Result<(), Error> {
	let player = match get_player_from_input(ctx, ctx.author(), uuid, username).await {
		Ok(player) => player,
		Err(Error::NotLinked) => {
			ctx.send(|m| error_embed(m, tr!(ctx, "not-linked"), tr!(ctx, "not-linked")))
				.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	let (data, session) = join(player.get_data(), player.get_session()).await;

	let data = data?;
	let session = session?;

	let mode = get_game_mode(mode, &session);

	let png: Cow<[u8]> = {
		let mut surface = create_surface(2);

		canvas::header::apply_name(&mut surface, &data);
		canvas::header::apply_status(ctx, &mut surface, &session);
		canvas::arena::apply(ctx, &mut surface, &data, mode.into());

		to_png(&mut surface).into()
	};

	ctx.send(move |m| {
		m.attachment(AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".into(),
		})
	})
	.await?;

	Ok(())
}

impl From<&str> for ArenaMode {
	fn from(s: &str) -> Self {
		match s {
			"arena_1v1" => Self::Solo,
			"arena_2v2" => Self::Double,
			"arena_4v4" => Self::Four,
			_ => Self::Overall,
		}
	}
}

impl From<ArenaMode> for canvas::arena::ArenaMode {
	fn from(val: ArenaMode) -> Self {
		match val {
			ArenaMode::Overall => canvas::arena::ArenaMode::Overall,
			ArenaMode::Solo => canvas::arena::ArenaMode::Solo,
			ArenaMode::Double => canvas::arena::ArenaMode::Double,
			ArenaMode::Four => canvas::arena::ArenaMode::Four,
		}
	}
}
