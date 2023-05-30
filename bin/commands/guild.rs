use std::{borrow::Cow, sync::Arc};

use api::{
	canvas,
	guild::{member::Member, Guild},
	player::Player,
};
use chrono::{DateTime, Utc};
use database::{extend::modulo, schema::guild_snapshot};
use diesel::{query_dsl::methods::DistinctOnDsl, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use futures::StreamExt;
use minecraft::text::parse::minecraft_string;
use poise::serenity_prelude::AttachmentType;
use translate::{tr, Context};
use uuid::Uuid;

use crate::{
	snapshot,
	util::{error_embed, get_guild_from_input},
	Error,
};

pub async fn get_snapshots_multiple_of_weekday(
	ctx: Context<'_>,
	guild: &Guild,
	after: DateTime<Utc>,
) -> Result<Vec<Guild>, Error> {
	let now = Utc::now();
	#[allow(clippy::cast_possible_truncation)]
	let days = (now.timestamp() / 60 / 60 / 24) as i32;

	let result = guild_snapshot::table
		.filter(guild_snapshot::uuid.eq(Uuid::from_u128(guild.id)))
		.filter(guild_snapshot::created_at.gt(after))
		.filter(modulo(guild_snapshot::days_since_epoch - days, 6).eq(0))
		.select(guild_snapshot::data)
		.order(guild_snapshot::days_since_epoch.desc())
		.distinct_on(guild_snapshot::days_since_epoch)
		.get_results::<Vec<u8>>(&mut ctx.data().pool.get().await?)
		.await?;

	Ok(result
		.into_iter()
		.filter_map(|data| snapshot::guild::decode(data.as_slice()).ok())
		.collect())
}

pub fn get_monthly_xp(guild: &Guild, guilds: &[Guild]) -> u32 {
	let mut xp = 0;

	for snapshot in guilds {
		for member in &snapshot.members {
			// Add up all of the xp from the last 6 days from this snapshot
			xp += member
				.xp_history
				.into_iter()
				.skip(1)
				.map(|(_, xp)| xp)
				.sum::<u32>();
		}
	}

	for member in &guild.members {
		xp += member.xp_history[0].1;
	}

	xp
}

pub fn apply_member_xp(guild: &mut Guild, guilds: &[Guild]) {
	for member in &mut guild.members {
		for (_, xp) in member.xp_history.iter_mut().skip(1) {
			*xp = 0;
		}

		for g in guilds.iter() {
			if let Some(m) = g.members.iter().find(|m| m.uuid == member.uuid) {
				for (idx, (_, xp)) in m.xp_history.into_iter().enumerate().skip(1) {
					member.xp_history[idx].1 += xp;
				}
			}
		}
	}
}

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
	let start = std::time::Instant::now();
	ctx.defer().await?;

	let mut guild = match get_guild_from_input(ctx, ctx.author(), name, uuid, username).await {
		Ok(guild) => guild,
		Err(Error::NotLinked) => {
			ctx.send(|m| error_embed(m, tr!(ctx, "not-linked"), tr!(ctx, "not-linked")))
				.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	let end = std::time::Instant::now();

	println!("Took {}ms to get guild", (end - start).as_millis());

	guild.increase_searches(ctx).await?;

	let end = std::time::Instant::now();

	println!("Took {}ms to get search inc", (end - start).as_millis());

	let guilds =
		get_snapshots_multiple_of_weekday(ctx, &guild, Utc::now() - chrono::Duration::days(30))
			.await?;
	let monthly_xp = get_monthly_xp(&guild, &guilds);

	let end = std::time::Instant::now();

	println!("Took {}ms to compute snapshots", (end - start).as_millis());

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
			.map(Player::get_display_string_owned),
	)
	.buffered(14)
	.filter_map(|r| async { r.ok() })
	.collect::<Vec<_>>()
	.await;
	let end = std::time::Instant::now();

	println!("Took {}ms to get members", (end - start).as_millis());

	let leader = if let Some(name) = guild
		.get_leader()
		.map(|m| m.get_player_unchecked().get_display_string_owned())
	{
		Some(name.await.map_err(Arc::new)?)
	} else {
		None
	};
	let end = std::time::Instant::now();

	println!("Took {}ms to get leader", (end - start).as_millis());

	let png: Cow<_> = {
		let mut surface = canvas::guild::create_surface();

		if let Some(leader) = leader {
			canvas::guild::leader(&mut surface, &minecraft_string(&leader).collect::<Vec<_>>());
		}

		canvas::guild::members(ctx, &mut surface, &guild, members.as_slice());
		canvas::guild::header(&mut surface, &guild);
		canvas::guild::games(ctx, &mut surface, &mut guild);
		canvas::guild::stats(ctx, &mut surface, &guild, monthly_xp);
		canvas::guild::level(ctx, &mut surface, &guild);
		canvas::guild::preferred_games(&mut surface, &guild);
		let end = std::time::Instant::now();

		println!("Took {}ms to get canvas", (end - start).as_millis());

		canvas::to_png(&mut surface).into()
	};

	let end = std::time::Instant::now();

	println!("Took {}ms to get data", (end - start).as_millis());

	ctx.send(move |m| {
		m.attachment(AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".to_string(),
		})
	})
	.await?;

	Ok(())
}
