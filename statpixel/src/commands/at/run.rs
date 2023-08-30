use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, label::ToFormatted, Canvas, prelude::Mode},
	shape, Member,
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

use crate::{
	commands::{
		self,
		guild::{get_monthly_xp, get_snapshots_multiple_of_weekday},
	},
	format, snapshot, util,
};

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
				ctx, "showing-statistics-at",
				at: format!("<t:{}:f>", created_at.timestamp()),
			);

			let (png, mode): (Cow<_>, _) = {
				let (mut surface, mode) = G::canvas(
					ctx,
					family,
					data_lhs,
					&session,
					skin.image(),
					mode,
					suffix.as_deref(),
					background,
				);

				(canvas::to_png(&mut surface).into(), mode)
			};

			let (row, id) = G::Mode::as_at(
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
				ctx, "showing-statistics-at",
				at: format!("<t:{}:f>", created_at.timestamp()),
			);

			let attachments = G::condensed(ctx, family, data_lhs, suffix.as_deref(), background)
				.into_iter()
				.map(|mut surface| {
					CreateAttachment::bytes(
						Cow::Owned(canvas::to_png(&mut surface)),
						crate::IMAGE_NAME,
					)
				})
				.collect::<Vec<_>>();

			let (_, id) = G::Mode::as_at(
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
				ctx, "showing-statistics-at",
				at: format!("<t:{}:f>", created_at.timestamp()),
			);

			let embed = G::embed(ctx, &player, &data_lhs).colour(crate::EMBED_COLOUR);

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

	guild.increase_searches(ctx).await?;

	let png: Option<Cow<_>> = if let snapshot::guild::Status::Found((ref guild, _)) = status {
		let guilds =
			get_snapshots_multiple_of_weekday(ctx, guild, Utc::now() - chrono::Duration::days(30))
				.await?;
		let monthly_xp = get_monthly_xp(guild, &guilds);

		let daily_xp = guild.members.iter().map(|g| g.xp_history[1].1).sum::<u32>();
		let weekly_xp = guild
			.members
			.iter()
			.map(|g| g.xp_history.iter().map(|h| h.1).sum::<u32>())
			.sum::<u32>();

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

		let level = calc::guild::get_level(guild.xp);
		let progress = shape::WideBubbleProgress(
			calc::guild::get_level_progress(guild.xp),
			[Colour::Gold.into(), Colour::Gold.into()],
			false,
		);

		let mut canvas = Canvas::new(720., family)
			.gap(7.)
			.push_down(&shape::Title, shape::Title::from_guild(family, guild))
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
				shape::Sidebar::from_guild(ctx, family, guild),
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
					&monthly_xp,
					tr(ctx, "monthly-xp").as_ref(),
					Paint::DarkGreen,
				),
			)
			.push_down_start(
				&shape::WideTallBubble,
				shape::WideTallBubble::from_guild(ctx, family, guild, members.as_slice(), 0),
			)
			.push_right(
				&shape::WideTallBubble,
				shape::WideTallBubble::from_guild(ctx, family, guild, members.as_slice(), 1),
			)
			.build(None, background)
			.unwrap();

		Some(canvas::to_png(&mut canvas).into())
	} else {
		None
	};

	let content = match status {
		snapshot::guild::Status::Found((_, created_at)) => tr_fmt!(
			ctx, "showing-guild-statistics-at",
			at: format!("<t:{}:f>", created_at.timestamp()),
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
