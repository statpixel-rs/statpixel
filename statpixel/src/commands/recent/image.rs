use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, label::ToFormatted, shape, text, Canvas},
	player::games::Game,
	Data, Session,
};
use minecraft::{
	calc::network,
	paint::Paint,
	text::{parse::minecraft_text, Text},
};
use translate::context;

const LABEL: [Text; 1] = minecraft_text("§e§lRecent Games");

#[allow(clippy::too_many_lines)]
pub fn recent(
	ctx: &context::Context<'_>,
	data: &Data,
	games: &[Game],
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
	background: Option<skia_safe::Color>,
) -> Cow<'static, [u8]> {
	let status = shape::Status(session, skin);
	let level = network::get_level(data.xp);
	let progress = shape::WideBubbleProgress(
		network::get_level_progress(data.xp),
		network::get_colours(level),
		false,
	);

	let ctx = &ctx;
	let mut canvas = Canvas::new(720.)
		.gap(7.)
		.push_down(
			&shape::Title,
			shape::Title::from_text(&text::from_data(data, &data.username, suffix)),
		)
		.push_down(&shape::Subtitle, shape::Subtitle::from_text(&LABEL))
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
		.push_right_post_draw(&status, Body::from_status(ctx, session));

	let games = games
		.iter()
		.map(|game| (shape::RecentGame(&game.kind), game))
		.collect::<Vec<_>>();

	for (shape, game) in &games {
		canvas = canvas.push_checked(shape, shape::RecentGame::from_game(ctx, game));
	}

	let mut surface = canvas.build(None, background).unwrap();

	canvas::to_png(&mut surface).into()
}
