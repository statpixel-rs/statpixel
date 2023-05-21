use std::borrow::Cow;

use api::{canvas, guild::member::Member, player::Player};
use futures::StreamExt;
use poise::serenity_prelude::AttachmentType;
use translate::{tr, Context};

use crate::{
	util::{error_embed, get_guild_from_input},
	Error,
};

/// Shows the stats of a guild.
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn guild(
	ctx: Context<'_>,
	#[min_length = 3]
	#[max_length = 32]
	#[autocomplete = "crate::commands::autocomplete_guild_name"]
	name: Option<String>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let mut guild = match get_guild_from_input(ctx, ctx.author(), name, uuid, username).await {
		Ok(guild) => guild,
		Err(Error::NotLinked) => {
			ctx.send(|m| error_embed(m, tr!(ctx, "not-linked"), tr!(ctx, "not-linked")))
				.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	guild.increase_searches(ctx)?;

	let data = if let Some(leader) = guild.get_leader() {
		let player = leader.get_player_unchecked();

		Some(player.get_data().await?)
	} else {
		None
	};

	guild
		.members
		.sort_by_cached_key(|m| m.xp_history.iter().map(|h| h.1).sum::<u32>());

	let members = futures::stream::iter(
		guild
			.members
			.iter()
			.rev()
			.take(14)
			.map(Member::get_player_unchecked)
			.map(Player::get_data_owned),
	)
	.buffered(14)
	.filter_map(|r| async { r.ok() })
	.collect::<Vec<_>>()
	.await;

	let png: Cow<_> = {
		let mut surface = canvas::guild::create_surface();

		if let Some(ref data) = data {
			canvas::guild::leader(&mut surface, data);
		}

		canvas::guild::members(ctx, &mut surface, &guild, members.as_slice());
		canvas::guild::header(&mut surface, &guild);
		canvas::guild::games(ctx, &mut surface, &mut guild);
		canvas::guild::stats(ctx, &mut surface, &guild);
		canvas::guild::level(ctx, &mut surface, &guild);
		canvas::guild::preferred_games(&mut surface, &guild);

		canvas::to_png(&mut surface).into()
	};

	ctx.send(move |m| {
		m.attachment(AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".to_string(),
		})
	})
	.await?;

	Ok(())
}
