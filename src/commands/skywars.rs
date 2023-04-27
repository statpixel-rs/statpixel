use std::borrow::Cow;

use api::{game::r#type::GameType, player::Player};
use canvas::{create_surface, to_png};
use poise::{futures_util::future::join, serenity_prelude::AttachmentType, ChoiceParameter};

use crate::{
	util::{error_embed, get_player_from_input},
	Context, Error,
};

#[derive(ChoiceParameter, Debug)]
pub enum SkyWarsMode {
	#[name = "Overall"]
	Overall,
	#[name = "Solo Normal"]
	SoloNormal,
	#[name = "Solo Insane"]
	SoloInsane,
	#[name = "Team Normal"]
	TeamNormal,
	#[name = "Team Insane"]
	TeamInsane,
}

async fn get_game_mode(player: &Player, mode: Option<SkyWarsMode>) -> Result<SkyWarsMode, Error> {
	if let Some(mode) = mode {
		Ok(mode)
	} else {
		let session = player.get_session().await?;

		if session.game_type == Some(GameType::SkyWars) && let Some(game_mode) = session.game_mode {
			Ok(SkyWarsMode::from(game_mode.as_str()))
		} else {
			Ok(SkyWarsMode::Overall)
		}
	}
}

/// Shows the SkyWars stats of a player.
#[poise::command(slash_command)]
pub async fn skywars(
	ctx: Context<'_>,
	#[description = "The Minecraft username to view"]
	#[max = 16]
	username: Option<String>,
	#[description = "The Minecraft UUID to view"]
	#[min = 32]
	#[max = 36]
	uuid: Option<String>,
	#[description = "The mode to view"] mode: Option<SkyWarsMode>,
) -> Result<(), Error> {
	let player = match get_player_from_input(ctx, ctx.author(), uuid, username).await {
		Ok(player) => player,
		Err(Error::NotLinked) => {
			ctx.send(|m| {
				error_embed(
					m,
					"Missing arguments",
					"Invalid UUID or username provided, and you are not linked.",
				)
			})
			.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	let (data, mode) = join(player.get_data(), get_game_mode(&player, mode)).await;

	let data = data?;
	let mode = mode?;

	let png: Cow<[u8]> = {
		let mut surface = create_surface(2);

		canvas::header::apply(&mut surface, &data);
		canvas::skywars::apply(&mut surface, &data, mode.into());

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

impl From<&str> for SkyWarsMode {
	fn from(s: &str) -> Self {
		match s {
			"solo_normal" => Self::SoloNormal,
			"solo_insane" => Self::SoloInsane,
			"teams_normal" => Self::TeamNormal,
			"teams_insane" => Self::TeamInsane,
			_ => Self::Overall,
		}
	}
}

impl From<SkyWarsMode> for canvas::skywars::SkyWarsMode {
	fn from(val: SkyWarsMode) -> Self {
		match val {
			SkyWarsMode::Overall => canvas::skywars::SkyWarsMode::Overall,
			SkyWarsMode::SoloNormal => canvas::skywars::SkyWarsMode::SoloNormal,
			SkyWarsMode::SoloInsane => canvas::skywars::SkyWarsMode::SoloInsane,
			SkyWarsMode::TeamNormal => canvas::skywars::SkyWarsMode::TeamNormal,
			SkyWarsMode::TeamInsane => canvas::skywars::SkyWarsMode::TeamInsane,
		}
	}
}
