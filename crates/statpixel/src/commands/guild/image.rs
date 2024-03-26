use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, label::ToFormatted, shape, Canvas},
	guild::member::Member,
	player::Player,
	Guild,
};
use chrono::{DateTime, NaiveTime, Utc};
use futures::StreamExt;
use minecraft::{
	calc,
	colour::Colour,
	paint::Paint,
	style::{Family, MinecraftFont},
	text::{parse::minecraft_string, Text, ESCAPE},
};
use skia_safe::{textlayout::TextAlign, Color};
use tokio::join;
use tracing::error;
use translate::{context, tr};

use crate::Error;

use super::{get_member_monthly_xp, get_monthly_xp, get_snapshots_multiple_of_weekday};

#[allow(clippy::too_many_lines)]
pub async fn top(
	ctx: &context::Context<'_>,
	family: Family,
	guild: &Guild,
	limit: usize,
	after: DateTime<Utc>,
	background: Option<Color>,
) -> Result<Cow<'static, [u8]>, Error> {
	let guilds =
		crate::commands::guild::get_snapshots_multiple_of_weekday(ctx, guild, after).await?;

	let member_xp = get_member_monthly_xp(guild, &guilds);
	let mut members = member_xp.into_iter().collect::<Vec<_>>();

	members.sort_by_key(|m| std::cmp::Reverse(m.1));

	let members = futures::stream::iter(
		members
			.into_iter()
			.take(limit)
			.map(|m| (Player::from_uuid_unchecked(m.0), m.1))
			.map(|(p, v)| async move {
				let uuid = p.uuid;

				match p.get_display_string_owned(ctx).await {
					Ok(s) => Ok((uuid, s, v)),
					Err(e) => Err(e),
				}
			}),
	)
	.buffered(20)
	.filter_map(|r| async {
		match r {
			Err(e) => {
				error!("Failed to get player display string: {:?}", e);

				None
			}
			d => d.ok(),
		}
	})
	.collect::<Vec<_>>()
	.await;

	let mut canvas = Canvas::new(1_176.666_6, family).gap(7.).push_down(
		&shape::GuildXpTitle,
		shape::Title::from_guild(family, guild),
	);

	for (idx, (_, name, xp)) in members.iter().enumerate().take(limit / 2) {
		canvas = canvas
			.push_down_start(
				&shape::GuildLeaderboardPlace,
				shape::LeaderboardPlace::from_usize(family, idx + 1),
			)
			.push_right(
				&shape::GuildXpName,
				shape::LeaderboardName::from_text(family, name),
			)
			.push_right(
				&shape::GuildXpValue,
				shape::LeaderboardValue::from_value(ctx, family, xp),
			);

		if let Some((_, name, xp)) = members.get(idx + limit / 2) {
			canvas = canvas
				.push_right(
					&shape::GuildLeaderboardPlace,
					shape::LeaderboardPlace::from_usize(family, idx + limit / 2 + 1),
				)
				.push_right(
					&shape::GuildXpName,
					shape::LeaderboardName::from_text(family, name),
				)
				.push_right(
					&shape::GuildXpValue,
					shape::LeaderboardValue::from_value(ctx, family, xp),
				);
		} else {
			canvas = canvas
				.push_right(
					&shape::GuildLeaderboardPlace,
					shape::LeaderboardPlace::from_usize(family, idx + limit / 2 + 1),
				)
				.push_right(
					&shape::GuildXpName,
					shape::LeaderboardName::from_text(family, "§7§oNone"),
				)
				.push_right(
					&shape::GuildXpValue,
					shape::LeaderboardValue::from_value(ctx, family, &0),
				);
		};
	}

	if members.len() < limit / 2 {
		for idx in members.len()..(limit / 2) {
			canvas = canvas
				.push_down_start(
					&shape::GuildLeaderboardPlace,
					shape::LeaderboardPlace::from_usize(family, idx + 1),
				)
				.push_right(
					&shape::GuildXpName,
					shape::LeaderboardName::from_text(family, "§7§oNone"),
				)
				.push_right(
					&shape::GuildXpValue,
					shape::LeaderboardValue::from_value(ctx, family, &0),
				)
				.push_right(
					&shape::GuildLeaderboardPlace,
					shape::LeaderboardPlace::from_usize(family, idx + limit / 2 + 1),
				)
				.push_right(
					&shape::GuildXpName,
					shape::LeaderboardName::from_text(family, "§7§oNone"),
				)
				.push_right(
					&shape::GuildXpValue,
					shape::LeaderboardValue::from_value(ctx, family, &0),
				);
		}
	}

	let mut canvas = canvas.build(None, background).unwrap();

	Ok(canvas::to_png(&mut canvas).into())
}

#[allow(clippy::too_many_lines)]
pub async fn members(
	ctx: &context::Context<'_>,
	family: Family,
	guild: &Guild,
	background: Option<Color>,
) -> Result<Cow<'static, [u8]>, Error> {
	let mut members =
		futures::stream::iter(guild.members.iter().map(Member::get_player_unchecked).map(
			|p| async {
				match p.get_display_string_owned(ctx).await {
					Ok(s) => {
						let paragraph = shape::Custom::from_text(
							family,
							&minecraft_string(&s).collect::<Vec<_>>(),
						);

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

	let mut canvas = Canvas::new(1_176.666_6, family).gap(7.).push_down(
		&shape::FullWidthBigTitle,
		shape::FullWidthBigTitle::from_guild(ctx, family, guild),
	);

	let mut iter = member_rank_indices.into_iter();
	let mut ranks_iter = guild.ranks.iter();

	if let Some((i, r)) = iter.next() {
		let mut last_rank = r;

		if let Some((shape, paragraph)) = members.get_mut(i).and_then(std::option::Option::take) {
			let text = shape::Title::from_text(
				family,
				&[Text {
					text: "Guild Master",
					..Default::default()
				}],
			);

			canvas = canvas
				.push_down_start(&shape::FullWidthTitle, text)
				.push_down_start(&shape, paragraph);
		}

		for (i, r) in iter {
			let Some((shape, paragraph)) = members.get_mut(i).and_then(std::option::Option::take)
			else {
				continue;
			};

			if r == last_rank {
				canvas = canvas.push_checked(&shape, paragraph);
			} else {
				last_rank = r;

				let text = shape::Title::from_text(
					family,
					&[Text {
						text: &ranks_iter.next().unwrap().name,
						..Default::default()
					}],
				);

				canvas = canvas
					.push_down_start(&shape::FullWidthTitle, text)
					.push_down_start(&shape, paragraph);
			}
		}
	}

	let mut canvas = canvas.build(None, background).unwrap();

	Ok(canvas::to_png(&mut canvas).into())
}

#[allow(clippy::too_many_lines)]
pub async fn general(
	ctx: &context::Context<'_>,
	family: Family,
	guild: &Guild,
	background: Option<Color>,
) -> Result<Cow<'static, [u8]>, Error> {
	let guilds = get_snapshots_multiple_of_weekday(
		ctx,
		guild,
		Utc::now() - chrono::Duration::try_days(30).unwrap(),
	)
	.await?;
	let monthly_xp = get_monthly_xp(guild, &guilds);

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
		let (members, leader) = join!(members, leader);

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

	let daily_xp = guild.members.iter().map(|m| m.xp_history[0].1).sum::<u32>();
	let weekly_xp = guild
		.members
		.iter()
		.map(|m| m.xp_history.iter().map(|h| h.1).sum::<u32>())
		.sum::<u32>();

	let mut canvas = Canvas::new(720., family)
		.gap(7.)
		.push_down(&shape::Title, shape::Title::from_guild(family, guild))
		.push_down(
			&shape::Subtitle,
			if let Some(leader) = leader {
				Body::new(20., TextAlign::Center, family)
					.extend_owned(minecraft_string(&leader))
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
				&format!("{ESCAPE}6{}", level.0),
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
				tr(ctx, "members_label").as_ref(),
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

	Ok(canvas::to_png(&mut canvas).into())
}

#[allow(clippy::too_many_lines)]
pub async fn member(
	ctx: &context::Context<'_>,
	family: Family,
	guild: &Guild,
	player: &Player,
	background: Option<Color>,
) -> Result<Cow<'static, [u8]>, Error> {
	let guilds = get_snapshots_multiple_of_weekday(
		ctx,
		guild,
		Utc::now() - chrono::Duration::try_days(30).unwrap(),
	)
	.await?;

	let member_xp = get_member_monthly_xp(guild, &guilds);
	let member = player.get_display_string(ctx).await?;

	let preferred = shape::PreferredGames(&guild.preferred_games);
	let level = calc::guild::get_level(guild.xp);
	let progress = shape::WideBubbleProgress(
		calc::guild::get_level_progress(guild.xp),
		[Colour::Gold.into(), Colour::Gold.into()],
		false,
	);

	let member_data = guild
		.members
		.iter()
		.find(|m| m.uuid == player.uuid)
		.unwrap();

	let daily_xp = member_data.xp_history[0].1;
	let weekly_xp = member_data.xp_history.iter().map(|h| h.1).sum::<u32>();

	let mut canvas = Canvas::new(720., family)
		.gap(7.)
		.push_down(
			&shape::Title,
			Body::new(25., TextAlign::Center, family)
				.extend_owned(minecraft_string(&member))
				.build(),
		)
		.push_down(&shape::Subtitle, shape::Subtitle::from_guild(family, guild))
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&format!("{ESCAPE}6{}", level.0),
				&calc::guild::get_curr_level_xp(guild.xp),
				&calc::guild::get_level_xp(guild.xp),
			),
		)
		.push_right_start(
			&shape::Sidebar,
			shape::Sidebar::from_guild(ctx, family, guild),
		)
		.push_right_post_draw(&preferred, Body::empty())
		.push_down_start(
			&shape::Bubble,
			Body::new(30., TextAlign::Center, family)
				.extend(&[
					Text {
						text: tr(ctx, "joined-at").as_ref(),
						paint: Paint::Gold,
						font: MinecraftFont::Normal,
						size: Some(20.),
					},
					Text {
						text: "\n",
						size: Some(20.),
						..Default::default()
					},
					Text {
						text: &member_data.joined_at.to_formatted(ctx),
						paint: Paint::Gold,
						font: MinecraftFont::Normal,
						size: None,
					},
				])
				.build(),
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
				tr(ctx, "members_label").as_ref(),
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
				&member_xp.get(&player.uuid).copied().unwrap_or(0),
				tr(ctx, "monthly-xp").as_ref(),
				Paint::DarkGreen,
			),
		);

	canvas = canvas
		.push_down_start(
			&shape::BubbleSubtitle,
			shape::Subtitle::from_text(
				family,
				&[Text {
					text: tr(ctx, "date").as_ref(),
					..Default::default()
				}],
			),
		)
		.push_right(
			&shape::BubbleSubtitle,
			shape::Subtitle::from_text(
				family,
				&[Text {
					text: tr(ctx, "weekly-gexp").as_ref(),
					..Default::default()
				}],
			),
		)
		.push_right(
			&shape::BubbleSubtitle,
			shape::Subtitle::from_text(
				family,
				&[Text {
					text: tr(ctx, "position").as_ref(),
					..Default::default()
				}],
			),
		);

	for (idx, (date, xp)) in member_data.xp_history.iter().enumerate() {
		let position = guild
			.members
			.iter()
			.filter(|m| m.xp_history[idx].1 > *xp)
			.count() + 1;

		canvas = canvas
			.push_down_start(
				&shape::BubbleSubtitle,
				shape::Subtitle::from_formatted(
					ctx,
					family,
					&date.and_time(NaiveTime::MIN).and_utc(),
					Paint::White,
				),
			)
			.push_right(
				&shape::BubbleSubtitle,
				shape::Subtitle::from_formatted(ctx, family, xp, Paint::DarkGreen),
			)
			.push_right(
				&shape::BubbleSubtitle,
				shape::Subtitle::from_text(
					family,
					&[
						Text {
							text: "#",
							paint: match position {
								1 => Paint::Gold,
								2 => Paint::Gray,
								3 => Paint::Bronze,
								_ => Paint::White,
							},
							font: MinecraftFont::Bold,
							..Default::default()
						},
						Text {
							text: position.to_formatted(ctx).as_ref(),
							paint: match position {
								1 => Paint::Gold,
								2 => Paint::Gray,
								3 => Paint::Bronze,
								_ => Paint::White,
							},
							font: MinecraftFont::Bold,
							..Default::default()
						},
						Text {
							text: "/",
							paint: Paint::Gray,
							..Default::default()
						},
						Text {
							text: guild.members.len().to_formatted(ctx).as_ref(),
							paint: Paint::Gray,
							..Default::default()
						},
					],
				),
			);
	}

	let mut canvas = canvas.build(None, background).unwrap();

	Ok(canvas::to_png(&mut canvas).into())
}
