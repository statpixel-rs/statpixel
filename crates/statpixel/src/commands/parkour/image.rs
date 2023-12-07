use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, label::ToFormatted, shape, text, Canvas},
	Data, Session,
};
use minecraft::{calc::network, paint::Paint, style::Family, text::Text};
use skia_safe::{textlayout::TextAlign, Color};
use translate::context;

pub fn parkour(
	ctx: &context::Context<'_>,
	family: Family,
	background: Option<Color>,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
) -> Cow<'static, [u8]> {
	let canvas = Canvas::new(720., family).gap(7.).push_down(
		&shape::Title,
		shape::Title::from_text(family, &text::from_data(data, &data.username, suffix)),
	);

	let status = shape::Status(session, skin);
	let level = network::get_level(data.xp);
	let progress = shape::WideBubbleProgress(
		network::get_level_progress(data.xp),
		network::get_colours(level),
		false,
	);

	let mut canvas = canvas
		.push_down(
			&shape::Subtitle,
			shape::Subtitle::from_label(
				ctx,
				family,
				&crate::commands::network::image::LABEL,
				"parkour-label",
			),
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&network::get_level_format(level),
				&network::get_curr_level_xp(data.xp),
				&network::get_level_xp(data.xp),
			),
		)
		.push_right_start(
			&canvas::shape::Sidebar,
			canvas::body::Body::new(17., None, family)
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
		.push_right_post_draw(&status, Body::from_status(ctx, family, session));

	for (game, time) in &data.parkour {
		canvas = canvas.push_checked(
			&shape::Bubble,
			Body::new(20., TextAlign::Center, family)
				.extend(game.as_text())
				.extend(&[
					Text::NEW_LINE,
					Text {
						text: &time.first().map(|t| t.time).to_formatted(ctx),
						size: Some(40.),
						..Default::default()
					},
				])
				.build(),
		);
	}

	let mut surface = canvas.build(None, background).unwrap();

	canvas::to_png(&mut surface).into()
}
