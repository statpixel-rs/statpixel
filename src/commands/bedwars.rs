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
pub enum BedWarsMode {
	Overall,
	Solo,
	Double,
	Three,
	Four,
}

fn get_game_mode(mode: Option<BedWarsMode>, session: &PlayerSession) -> BedWarsMode {
	if let Some(mode) = mode {
		mode
	} else if session.game_type == Some(GameType::BedWars) && let Some(game_mode) = session.game_mode.as_ref() {
		BedWarsMode::from(game_mode.as_str())
	} else {
		BedWarsMode::Overall
	}
}

/// Shows the SkyWars stats of a player.
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn bedwars(
	ctx: Context<'_>,
	#[max_length = 16] username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
	mode: Option<BedWarsMode>,
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
		let mut surface = create_surface(4);

		canvas::header::apply_name(&mut surface, &data);
		canvas::header::apply_status(ctx, &mut surface, &session);
		canvas::bedwars::apply(ctx, &mut surface, &data, mode.into());

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

impl From<&str> for BedWarsMode {
	fn from(s: &str) -> Self {
		match s {
			"bedwars_eight_one" => Self::Solo,
			"bedwars_eight_two" => Self::Double,
			"bedwars_four_three" => Self::Three,
			"bedwars_four_four" => Self::Four,
			_ => Self::Overall,
		}
	}
}

impl From<BedWarsMode> for canvas::bedwars::BedWarsMode {
	fn from(val: BedWarsMode) -> Self {
		match val {
			BedWarsMode::Overall => canvas::bedwars::BedWarsMode::Overall,
			BedWarsMode::Solo => canvas::bedwars::BedWarsMode::Solo,
			BedWarsMode::Double => canvas::bedwars::BedWarsMode::Double,
			BedWarsMode::Three => canvas::bedwars::BedWarsMode::Three,
			BedWarsMode::Four => canvas::bedwars::BedWarsMode::Four,
		}
	}
}
