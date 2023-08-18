use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, label::ToFormatted, shape, text, Canvas},
	Data, Guild, Player, Session,
};
use minecraft::{
	calc::network,
	paint::Paint,
	text::{parse::minecraft_text, Text},
};
use skia_safe::Color;
use translate::{context, tr};

const LABEL: [Text; 1] = minecraft_text("§f§lNetwork");

#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
pub fn network(
	ctx: &context::Context<'_>,
	player: &Player,
	guild: Option<&Guild>,
	data: &Data,
	session: &Session,
	suffix: Option<&str>,
	skin: &skia_safe::Image,
	background: Option<Color>,
) -> Cow<'static, [u8]> {
	let status = shape::Status(session, skin);
	let level = network::get_level(data.xp);
	let progress = shape::WideBubbleProgress(
		network::get_level_progress(data.xp),
		network::get_colours(level),
		false,
	);

	let member = guild.and_then(|g| g.members.iter().find(|m| m.uuid == player.uuid));

	let ctx = &ctx;
	let mut surface = Canvas::new(720.)
		.gap(7.)
		.push_down(
			&shape::Title,
			shape::Title::from_text(&text::from_data(data, &data.username, suffix)),
		)
		.push_down(
			&shape::Subtitle,
			if let Some(guild) = guild {
				shape::Subtitle::from_guild(guild)
			} else {
				shape::Subtitle::from_label(ctx, &LABEL, "member-profile")
			},
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				&network::get_level_format(level),
				&network::get_curr_level_xp(data.xp),
				&network::get_level_xp(data.xp),
			),
		)
		.push_right_start(
			&canvas::shape::Sidebar,
			canvas::body::Body::new(17., None)
				.append_item(
					&::translate::tr(ctx, "experience"),
					&data.xp.to_formatted(ctx),
					&Paint::Yellow,
				)
				.append_item(
					&::translate::tr(ctx, "karma"),
					&data.karma.to_formatted(ctx),
					&Paint::LightPurple,
				)
				.append_item(
					&::translate::tr(ctx, "rewards"),
					&data.rewards.to_formatted(ctx),
					&Paint::Gold,
				)
				.append_item(
					&::translate::tr(ctx, "friend-requests"),
					&data.friend_requests.to_formatted(ctx),
					&Paint::Green,
				)
				.append_item(
					&::translate::tr(ctx, "time-played"),
					&data.playtime.to_formatted(ctx),
					&Paint::Gold,
				)
				.append_item(
					&::translate::tr(ctx, "first-login"),
					&data.first_login.to_formatted(ctx),
					&Paint::Aqua,
				)
				.append_item(
					&::translate::tr(ctx, "last-login"),
					&data.last_login.to_formatted(ctx),
					&Paint::Blue,
				)
				.build(),
		)
		.push_right_post_draw(&status, Body::from_status(ctx, session))
		.push_down_start(
			&shape::Bubble,
			Body::from_bubble(ctx, &data.quests, tr(ctx, "quests").as_ref(), Paint::Gold),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				&data.challenges,
				tr(ctx, "challenges").as_ref(),
				Paint::Gold,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				&data.achivement_points,
				tr(ctx, "achievement-points").as_ref(),
				Paint::Gold,
			),
		)
		.push_down_start(
			&shape::Bubble,
			Body::from_bubble_small(
				ctx,
				&data.language,
				tr(ctx, "language").as_ref(),
				Paint::Aqua,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				&data.gifting.gifts_given,
				tr(ctx, "gifts-given").as_ref(),
				Paint::LightPurple,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				&data.gifting.ranks_given,
				tr(ctx, "ranks-given").as_ref(),
				Paint::LightPurple,
			),
		)
		.push_down_start(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				&member.map_or(0, |m| m.xp_history[0].1),
				tr(ctx, "daily-xp").as_ref(),
				Paint::DarkGreen,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				&member.map_or(0, |m| m.xp_history.iter().map(|(_, x)| x).sum::<u32>()),
				tr(ctx, "weekly-xp").as_ref(),
				Paint::DarkGreen,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				&member.map_or(0, |m| m.quests),
				tr(ctx, "guild-quests").as_ref(),
				Paint::DarkGreen,
			),
		)
		.build(None, background)
		.unwrap();

	canvas::to_png(&mut surface).into()
}
