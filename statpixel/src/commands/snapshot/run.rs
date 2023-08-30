use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, diff::Diff, label::ToFormatted, prelude::Mode, Canvas},
	shape, Guild, Member,
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

#[allow(clippy::too_many_lines)]
pub async fn command<G: api::canvas::prelude::Game>(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
	mode: Option<G::Mode>,
	past: chrono::Duration,
) -> Result<(), Error> {
	let after = Utc::now() - past;
	let (format, family, background) = util::get_image_options_from_input(ctx).await;

	match format {
		format::Display::Image => {
			let (player, data_rhs, session, skin, suffix) =
				commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let status = snapshot::user::get_or_insert(ctx, &player, &data_rhs, after).await?;

			let snapshot::user::Status::Found((ref data_lhs, created_at)) = status else {
				let content = tr_fmt!(
					ctx, "no-previous-statistics",
					name: util::escape_username(&data_rhs.username),
				);

				ctx.send(poise::CreateReply::new().content(content)).await?;

				return Ok(());
			};

			let content = tr_fmt!(
				ctx, "showing-statistics",
				from: format!("<t:{}:f>", created_at.timestamp()),
				to: format!("<t:{}:f>", Utc::now().timestamp()),
			);

			let (png, mode): (Cow<_>, _) = {
				let (mut surface, mode) = G::canvas_diff(
					ctx,
					family,
					data_lhs,
					&data_rhs,
					&session,
					skin.image(),
					mode,
					suffix.as_deref(),
					background,
				);

				(canvas::to_png(&mut surface).into(), mode)
			};

			let (row, id) = G::Mode::as_snapshot(
				ctx,
				player.uuid,
				past.num_nanoseconds().unwrap_or_default(),
				Some(mode),
			);

			ctx.send(
				poise::CreateReply::new()
					.content(format!(
						"{}\n{content}",
						tr_fmt!(ctx, "identifier", identifier: api::id::encode(&id)),
					))
					.components(vec![row])
					.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
			)
			.await?;
		}
		format::Display::Compact => {
			let (player, data_rhs, suffix) =
				commands::get_player_data_suffix(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let status = snapshot::user::get_or_insert(ctx, &player, &data_rhs, after).await?;

			let snapshot::user::Status::Found((ref data_lhs, created_at)) = status else {
				let content = tr_fmt!(
					ctx, "no-previous-statistics",
					name: util::escape_username(&data_rhs.username),
				);

				ctx.send(poise::CreateReply::new().content(content)).await?;

				return Ok(());
			};

			let content = tr_fmt!(
				ctx, "showing-statistics",
				from: format!("<t:{}:f>", created_at.timestamp()),
				to: format!("<t:{}:f>", Utc::now().timestamp()),
			);

			let attachments = G::condensed_diff(
				ctx,
				family,
				data_lhs,
				&data_rhs,
				suffix.as_deref(),
				background,
			)
			.into_iter()
			.map(|mut surface| {
				CreateAttachment::bytes(Cow::Owned(canvas::to_png(&mut surface)), crate::IMAGE_NAME)
			})
			.collect::<Vec<_>>();

			let (_, id) = G::Mode::as_snapshot(
				ctx,
				player.uuid,
				past.num_nanoseconds().unwrap_or_default(),
				None,
			);

			let mut reply = poise::CreateReply::new().content(format!(
				"{}\n{content}",
				tr_fmt!(ctx, "identifier", identifier: api::id::encode(&id)),
			));

			reply.attachments = attachments;

			ctx.send(reply).await?;
		}
		format::Display::Text => {
			let (player, data_rhs) = commands::get_player_data(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			let status = snapshot::user::get_or_insert(ctx, &player, &data_rhs, after).await?;

			let snapshot::user::Status::Found((data_lhs, created_at)) = status else {
				let content = tr_fmt!(
					ctx, "no-previous-statistics",
					name: util::escape_username(&data_rhs.username),
				);

				ctx.send(poise::CreateReply::new().content(content)).await?;

				return Ok(());
			};

			let content = tr_fmt!(
				ctx, "showing-statistics",
				from: format!("<t:{}:f>", created_at.timestamp()),
				to: format!("<t:{}:f>", Utc::now().timestamp()),
			);

			let embed =
				G::embed_diff(ctx, &player, &data_lhs, &data_rhs).colour(crate::EMBED_COLOUR);

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
	let (_, family, background) = util::get_image_options_from_input(ctx).await;
	let guild = match commands::get_guild(ctx, name, uuid, username, guild_id).await {
		Result::Ok(guild) => guild,
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
			.map(|p| p.get_display_string_owned(ctx)),
	)
	.buffered(14)
	.filter_map(|r| async { r.ok() })
	.collect::<Vec<_>>();

	let leader = guild
		.get_leader()
		.map(|m| m.get_player_unchecked().get_display_string_owned(ctx));

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
			false,
		);

		let mut canvas = Canvas::new(720., family)
			.gap(7.)
			.push_down(&shape::Title, shape::Title::from_guild(family, &guild))
			.push_down(
				&shape::Subtitle,
				if let Some(leader) = leader {
					Body::new(20., TextAlign::Center, family)
						.extend_owned(parse::minecraft_string(&leader))
						.build()
				} else {
					Body::new(20., TextAlign::Center, family)
						.append(Text {
							text: tr(ctx, "none").as_ref(),
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
					family,
					&format!("{}6{}", minecraft::ESCAPE, level.0),
					&calc::guild::get_curr_level_xp(guild.xp),
					&calc::guild::get_level_xp(guild.xp),
				),
			)
			.push_right_start(
				&shape::Sidebar,
				shape::Sidebar::from_guild(ctx, family, &guild),
			)
			.push_right_post_draw(
				&shape::PreferredGames(&guild.preferred_games),
				Body::empty(),
			)
			.push_down_start(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					family,
					&guild.coins,
					tr(ctx, "coins").as_ref(),
					Paint::Gold,
				),
			)
			.push_right(
				&shape::Bubble,
				Body::new(30., TextAlign::Center, family)
					.extend(&[
						Text {
							text: tr(ctx, "created-at").as_ref(),
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
							text: &guild.created_at.to_formatted(ctx),
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
					family,
					&format!("{}/125", guild.members.len()),
					tr(ctx, "members").as_ref(),
					Paint::LightPurple,
				),
			)
			.push_down_start(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					family,
					&daily_xp,
					tr(ctx, "daily-xp").as_ref(),
					Paint::DarkGreen,
				),
			)
			.push_right(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					family,
					&weekly_xp,
					tr(ctx, "weekly-xp").as_ref(),
					Paint::DarkGreen,
				),
			)
			.push_right(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					family,
					&xp_since,
					tr(ctx, "xp-since").as_ref(),
					Paint::DarkGreen,
				),
			)
			.push_down_start(
				&shape::WideTallBubble,
				shape::WideTallBubble::from_guild(ctx, family, &guild, members.as_slice(), 0),
			)
			.push_right(
				&shape::WideTallBubble,
				shape::WideTallBubble::from_guild(ctx, family, &guild, members.as_slice(), 1),
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
