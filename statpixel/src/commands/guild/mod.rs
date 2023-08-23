pub mod image;
pub mod run;

use std::collections::HashMap;

use api::guild::Guild;
use chrono::{DateTime, Utc};
use database::{extend::modulo, schema::guild_snapshot};
use diesel::{query_dsl::methods::DistinctOnDsl, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use translate::{context, Context};
use uuid::Uuid;

use crate::{snapshot, util, Error};

pub async fn get_snapshots_multiple_of_weekday(
	ctx: &context::Context<'_>,
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

pub fn get_member_monthly_xp(guild: &Guild, guilds: &[Guild]) -> HashMap<Uuid, u32> {
	let mut members = guild
		.members
		.iter()
		.map(|m| (m.uuid, 0))
		.collect::<HashMap<_, _>>();

	for snapshot in guilds {
		for member in &snapshot.members {
			members.entry(member.uuid).and_modify(|xp| {
				*xp += member
					.xp_history
					.into_iter()
					.skip(1)
					.map(|(_, xp)| xp)
					.sum::<u32>();
			});
		}
	}

	for member in &guild.members {
		members.entry(member.uuid).and_modify(|xp| {
			*xp += member.xp_history[0].1;
		});
	}

	members
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

		for g in guilds {
			if let Some(m) = g.members.iter().find(|m| m.uuid == member.uuid) {
				for (idx, (_, xp)) in m.xp_history.into_iter().enumerate().skip(1) {
					member.xp_history[idx].1 += xp;
				}
			}
		}
	}
}

/// Shows the stats of a guild.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
#[allow(clippy::too_many_lines)]
async fn general(
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
	let uuid = util::parse_uuid(uuid.as_deref())?;
	let ctx = &context::Context::from_poise(&ctx);

	run::general(ctx, name, username, uuid, None, None, None).await
}

/// Shows the members of a guild.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
#[allow(clippy::too_many_lines)]
async fn members(
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
	let uuid = util::parse_uuid(uuid.as_deref())?;
	let ctx = &context::Context::from_poise(&ctx);

	run::members(ctx, name, username, uuid, None, None, None).await
}

/// Shows the member of a guild.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
#[allow(clippy::too_many_lines)]
async fn member(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(uuid.as_deref())?;
	let ctx = &context::Context::from_poise(&ctx);

	run::member(ctx, username, uuid, None, None).await
}

/// Shows the members of a guild.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
#[allow(clippy::too_many_lines)]
async fn top(
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
	#[min = 1i64] days: Option<i64>,
	#[min = 1usize]
	#[max = 128usize]
	limit: Option<usize>,
) -> Result<(), Error> {
	let limit = limit.map_or(30, |l| if l % 2 == 0 { l } else { l + 1 });

	let uuid = util::parse_uuid(uuid.as_deref())?;
	let ctx = &context::Context::from_poise(&ctx);

	run::top(
		ctx,
		name,
		username,
		uuid,
		days.map_or(chrono::Duration::days(30), chrono::Duration::days),
		limit,
		None,
	)
	.await
}

#[allow(clippy::unused_async)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES",
	subcommands(
		"general",
		"members",
		"member",
		"top",
		"super::from::guild",
		"super::snapshot::daily::guild",
		"super::snapshot::weekly::guild",
		"super::snapshot::monthly::guild",
		"super::at::guild"
	)
)]
pub async fn guild(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
