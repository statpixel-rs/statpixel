use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, label::ToFormatted, shape, Canvas},
	guild::{member::Member, Guild},
	player::Player,
};
use chrono::{DateTime, Utc};
use database::{extend::modulo, schema::guild_snapshot};
use diesel::{query_dsl::methods::DistinctOnDsl, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use futures::StreamExt;
use minecraft::{
	calc,
	colour::Colour,
	paint::Paint,
	style::MinecraftFont,
	text::{parse::minecraft_string, Text, ESCAPE},
};
use poise::serenity_prelude::AttachmentType;
use skia_safe::textlayout::TextAlign;
use tokio::join;
use tracing::error;
use translate::{tr, Context};
use uuid::Uuid;

use crate::{snapshot, util::error_embed, Error};

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
	let guild = match crate::commands::get_guild(ctx, name, uuid, username).await {
		Ok(guild) => guild,
		Err(Error::NotLinked) => {
			ctx.send(|m| error_embed(m, tr!(ctx, "not-linked"), tr!(ctx, "not-linked")))
				.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	guild.increase_searches(ctx).await?;

	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let guilds =
		get_snapshots_multiple_of_weekday(ctx, &guild, Utc::now() - chrono::Duration::days(30))
			.await?;
	let monthly_xp = get_monthly_xp(&guild, &guilds);

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
	.collect::<Vec<_>>();

	let leader = guild
		.get_leader()
		.map(|m| m.get_player_unchecked().get_display_string_owned());

	let (members, leader) = if let Some(leader) = leader {
		let (members, leader) = join!(members, leader);

		(members, Some(leader?))
	} else {
		(members.await, None)
	};

	let png: Cow<_> = {
		let level = calc::guild::get_level(guild.xp);
		let progress = shape::WideBubbleProgress(
			calc::guild::get_level_progress(guild.xp),
			[Colour::Gold.into(), Colour::Gold.into()],
		);

		let daily_xp = guild.members.iter().map(|m| m.xp_history[0].1).sum::<u32>();
		let weekly_xp = guild
			.members
			.iter()
			.map(|m| m.xp_history.iter().map(|h| h.1).sum::<u32>())
			.sum::<u32>();

		let mut canvas = Canvas::new(720.)
			.gap(7.)
			.push_down(&shape::Title, shape::Title::from_guild(&guild))
			.push_down(
				&shape::Subtitle,
				if let Some(leader) = leader {
					Body::new(20., TextAlign::Center)
						.extend_owned(minecraft_string(&leader))
						.build()
				} else {
					Body::new(20., TextAlign::Center)
						.append(Text {
							text: tr!(ctx, "none").as_ref(),
							paint: Paint::Gray,
							font: MinecraftFont::Bold,
							..Default::default()
						})
						.build()
				},
			)
			.push_down_post_draw(
				&progress,
				shape::WideBubbleProgress::from_level_progress(
					ctx,
					&format!("{ESCAPE}6{level}"),
					&calc::guild::get_curr_level_xp(guild.xp),
					&calc::guild::get_level_xp(guild.xp),
				),
			)
			.push_right_start(&shape::Sidebar, shape::Sidebar::from_guild(ctx, &guild))
			.push_right_post_draw(
				&shape::PreferredGames(&guild.preferred_games),
				Body::empty(),
			)
			.push_down_start(
				&shape::Bubble,
				Body::from_bubble(ctx, &guild.coins, tr!(ctx, "coins").as_ref(), Paint::Gold),
			)
			.push_right(
				&shape::Bubble,
				Body::new(30., TextAlign::Center)
					.extend(&[
						Text {
							text: tr!(ctx, "created-at").as_ref(),
							paint: Paint::Aqua,
							font: MinecraftFont::Normal,
							size: Some(20.),
						},
						Text {
							text: "\n",
							size: Some(20.),
							..Default::default()
						},
						Text {
							text: &guild.created_at.to_formatted_label(ctx),
							paint: Paint::Aqua,
							font: MinecraftFont::Normal,
							size: None,
						},
					])
					.build(),
			)
			.push_right(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&format!("{}/125", guild.members.len()),
					tr!(ctx, "members_label").as_ref(),
					Paint::LightPurple,
				),
			)
			.push_down_start(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&daily_xp,
					tr!(ctx, "daily-xp").as_ref(),
					Paint::DarkGreen,
				),
			)
			.push_right(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&weekly_xp,
					tr!(ctx, "weekly-xp").as_ref(),
					Paint::DarkGreen,
				),
			)
			.push_right(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&monthly_xp,
					tr!(ctx, "monthly-xp").as_ref(),
					Paint::DarkGreen,
				),
			)
			.push_down_start(
				&shape::WideTallBubble,
				shape::WideTallBubble::from_guild(ctx, &guild, members.as_slice(), 0),
			)
			.push_right(
				&shape::WideTallBubble,
				shape::WideTallBubble::from_guild(ctx, &guild, members.as_slice(), 1),
			)
			.build(None, background)
			.unwrap();

		canvas::to_png(&mut canvas).into()
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
	let guild = match crate::commands::get_guild(ctx, name, uuid, username).await {
		Ok(guild) => guild,
		Err(Error::NotLinked) => {
			ctx.send(|m| error_embed(m, tr!(ctx, "not-linked"), tr!(ctx, "not-linked")))
				.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	guild.increase_searches(ctx).await?;

	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let mut members =
		futures::stream::iter(guild.members.iter().map(Member::get_player_unchecked).map(
			|p| async {
				match p.get_display_string_owned().await {
					Ok(s) => {
						let paragraph =
							shape::Custom::from_text(&minecraft_string(&s).collect::<Vec<_>>());

						Ok((shape::Custom::get_from_paragraph(&paragraph), paragraph))
					}
					Err(e) => Err(e),
				}
			},
		))
		.buffered(20)
		.filter_map(|r| async {
			match r {
				Err(e) => {
					error!("Failed to get player display string: {:?}", e);

					None
				}
				d => d.ok().map(Some),
			}
		})
		.collect::<Vec<_>>()
		.await;

	let mut member_rank_indices = guild
		.members
		.iter()
		.enumerate()
		.map(|(i, m)| {
			(
				i,
				guild
					.ranks
					.iter()
					.find(|r| r.name == m.rank)
					.map_or(u8::MAX, |r| r.priority),
			)
		})
		.collect::<Vec<_>>();

	member_rank_indices.sort_by_key(|(_, rank)| std::cmp::Reverse(*rank));

	let png: Cow<_> = {
		let mut canvas = Canvas::new(1_176.666_6)
			.gap(7.)
			.push_down(&shape::FullWidthTitle, shape::Title::from_guild(&guild));

		let mut iter = member_rank_indices.into_iter();
		let mut ranks_iter = guild.ranks.iter();

		if let Some((i, r)) = iter.next() {
			let mut last_rank = r;

			if let Some((shape, paragraph)) = members.get_mut(i).and_then(std::option::Option::take)
			{
				let text = shape::Custom::from_text_large(&[Text {
					text: "Guild Master",
					..Default::default()
				}]);

				canvas = canvas
					.push_down_start(&shape::Custom::get_from_paragraph(&text), text)
					.push_down_start(&shape, paragraph);
			}

			for (i, r) in iter {
				let Some((shape, paragraph)) = members.get_mut(i).and_then(std::option::Option::take) else {
					continue;
				};

				if r == last_rank {
					canvas = canvas.push_checked(&shape, paragraph);
				} else {
					last_rank = r;

					let text = shape::Custom::from_text_large(&[Text {
						text: &ranks_iter.next().unwrap().name,
						..Default::default()
					}]);

					canvas = canvas
						.push_down_start(&shape::Custom::get_from_paragraph(&text), text)
						.push_down_start(&shape, paragraph);
				}
			}
		}

		let mut canvas = canvas.build(None, background).unwrap();

		canvas::to_png(&mut canvas).into()
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

#[allow(clippy::unused_async)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES",
	subcommands("general", "members")
)]
pub async fn guild(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
