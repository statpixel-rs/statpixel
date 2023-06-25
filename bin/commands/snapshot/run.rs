use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, diff::Diff, label::ToFormatted, Canvas},
	prelude::Mode,
	shape, Guild, Member, Player,
};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use minecraft::{
	calc,
	paint::Paint,
	style::MinecraftFont,
	text::{parse, Text},
	Colour,
};
use poise::serenity_prelude::CreateAttachment;
use skia_safe::textlayout::TextAlign;
use translate::{context, tr, tr_fmt, Error};
use uuid::Uuid;

use crate::{commands, format, snapshot, util};

pub async fn command<G: api::prelude::Game>(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	mode: Option<G::Mode>,
	after: DateTime<Utc>,
) -> Result<(), Error> {
	let (format, background) = util::get_format_colour_from_input(ctx).await;

	match format {
		format::Display::Image | format::Display::Compact => {
			let (player, data, session, skin, suffix) =
				commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let status = snapshot::user::get_or_insert(ctx, &player, &data, after).await?;

			let snapshot::user::Status::Found((ref snapshot, created_at)) = status else {
				let content = tr_fmt!(
					ctx, "no-previous-statistics",
					name: util::escape_username(&data.username),
				);

				ctx.send(
					poise::CreateReply::new()
						.content(content)
				)
				.await?;

				return Ok(());
			};

			let content = tr_fmt!(
				ctx, "showing-statistics",
				from: format!("<t:{}:f>", created_at.timestamp()),
				to: format!("<t:{}:f>", Utc::now().timestamp()),
			);

			let png: Cow<_> = {
				let mut surface = G::canvas_diff(
					ctx,
					snapshot,
					&mut api::Data::clone(&data),
					&session,
					skin.image(),
					mode,
					suffix.as_deref(),
					background,
				);

				canvas::to_png(&mut surface).into()
			};

			ctx.send(
				poise::CreateReply::new()
					.content(content)
					.components(vec![G::Mode::as_snapshot(ctx, player.uuid, after, mode)])
					.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
			)
			.await?;
		}
		format::Display::Text => {
			let (player, data) = commands::get_player_data(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let status = snapshot::user::get_or_insert(ctx, &player, &data, after).await?;

			let snapshot::user::Status::Found((ref snapshot, created_at)) = status else {
				let content = tr_fmt!(
					ctx, "no-previous-statistics",
					name: util::escape_username(&data.username),
				);

				ctx.send(
					poise::CreateReply::new()
						.content(content)
				)
				.await?;

				return Ok(());
			};

			let content = tr_fmt!(
				ctx, "showing-statistics",
				from: format!("<t:{}:f>", created_at.timestamp()),
				to: format!("<t:{}:f>", Utc::now().timestamp()),
			);

			let embed = G::embed_diff(ctx, &player, snapshot, &mut api::Data::clone(&data))
				.colour(crate::EMBED_COLOUR);

			ctx.send(poise::CreateReply::new().content(content).embed(embed))
				.await?;
		}
	}

	Ok(())
}

#[allow(clippy::too_many_lines)]
pub async fn guild_command(
	ctx: &context::Context<'_>,
	name: Option<String>,
	username: Option<String>,
	uuid: Option<Uuid>,
	after: DateTime<Utc>,
	guild_id: Option<Uuid>,
) -> Result<(), Error> {
	let (_, background) = util::get_format_colour_from_input(ctx).await;
	let guild = match commands::get_guild(ctx, name, uuid, username, guild_id).await {
		Result::Ok(guild) => guild,
		Result::Err(Error::NotLinked) => {
			ctx.send(util::error_embed(
				tr!(ctx, "not-linked"),
				tr!(ctx, "not-linked"),
			))
			.await?;

			return Ok(());
		}
		Result::Err(e) => return Result::Err(e),
	};

	let status = snapshot::guild::get_or_insert(ctx, &guild, after).await?;
	let guilds = commands::guild::get_snapshots_multiple_of_weekday(ctx, &guild, after).await?;
	let xp_since = commands::guild::get_monthly_xp(&guild, &guilds);

	guild.increase_searches(ctx).await?;

	let daily_xp = guild.members.iter().map(|g| g.xp_history[1].1).sum::<u32>();

	let weekly_xp = guild
		.members
		.iter()
		.map(|g| g.xp_history.iter().map(|h| h.1).sum::<u32>())
		.sum::<u32>();

	commands::guild::apply_member_xp(&mut Guild::clone(&guild), &guilds);

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
		let (members, leader) = tokio::join!(members, leader);

		(members, Some(leader?))
	} else {
		(members.await, None)
	};

	let png: Option<Cow<_>> = if let snapshot::guild::Status::Found((ref snapshot, _)) = status {
		let diff = guild.diff(snapshot);
		let mut guild = Guild::clone(&guild);

		guild.coins = diff.coins;
		guild.xp = diff.xp;
		guild.xp_by_game.iter_mut().for_each(|a| {
			let b = snapshot.xp_by_game.iter().find(|x| x.0 == a.0).unwrap();

			if a.1 > b.1 {
				a.1 -= b.1;
			} else {
				a.1 = api::xp::Xp(0);
			}
		});

		guild.xp_by_game.sort_unstable_by_key(|g| g.1);

		let level = calc::guild::get_level(guild.xp);
		let progress = shape::WideBubbleProgress(
			calc::guild::get_level_progress(guild.xp),
			[Colour::Gold.into(), Colour::Gold.into()],
		);

		let mut canvas = Canvas::new(720.)
			.gap(7.)
			.push_down(&shape::Title, shape::Title::from_guild(&guild))
			.push_down(
				&shape::Subtitle,
				if let Some(leader) = leader {
					Body::new(20., TextAlign::Center)
						.extend_owned(parse::minecraft_string(&leader))
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
					&format!("{}6{}", minecraft::ESCAPE, level.0),
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
					tr!(ctx, "members").as_ref(),
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
					&xp_since,
					tr!(ctx, "xp-since").as_ref(),
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

		Some(canvas::to_png(&mut canvas).into())
	} else {
		None
	};

	let content = match status {
		snapshot::guild::Status::Found((_, created_at)) => tr_fmt!(
			ctx, "showing-guild-statistics",
			from: format!("<t:{}:f>", created_at.timestamp()),
			to: format!("<t:{}:f>", Utc::now().timestamp()),
		),
		snapshot::guild::Status::Inserted => tr_fmt!(
			ctx, "no-previous-guild-statistics",
			name: guild.name.as_str(),
		),
	};

	ctx.send({
		let reply = poise::CreateReply::new().content(content);

		if let Some(png) = png {
			reply.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME))
		} else {
			reply
		}
	})
	.await?;

	Ok(())
}
