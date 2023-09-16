use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, label::ToFormatted, shape, text, Canvas},
	player::stats::bed_wars::{self, HotbarItem, ShopItem},
	Data, Session,
};
use minecraft::{calc, paint::Paint, style::Family};
use skia_safe::Color;
use translate::{context, tr};

pub fn hotbar(
	ctx: &context::Context<'_>,
	family: Family,
	background: Option<Color>,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
) -> Cow<'static, [u8]> {
	let game = &data.stats.bed_wars;

	let canvas = Canvas::new(720., family).gap(7.).push_down(
		&shape::Title,
		shape::Title::from_text(family, &text::from_data(data, &data.username, suffix)),
	);

	let (xp, level) = {
		let xp = calc::bed_wars::convert(&game.xp);
		let level = calc::bed_wars::get_level(xp);

		(xp, level)
	};

	let status = shape::Status(session, skin);
	let progress = shape::WideBubbleProgress(
		calc::bed_wars::get_level_progress(xp),
		calc::bed_wars::get_colours(level),
		false,
	);

	let canvas = canvas
		.push_down(
			&shape::Subtitle,
			shape::Subtitle::from_label(ctx, family, &bed_wars::LABEL, "hotbar"),
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&calc::bed_wars::get_level_format(level),
				&calc::bed_wars::get_curr_level_xp(xp),
				&calc::bed_wars::get_level_xp(xp),
			),
		);

	let mut canvas = bed_wars::Overall
		.add_header(ctx, family, canvas, data)
		.push_right_post_draw(&status, Body::from_status(ctx, family, session));

	let slots = game
		.hotbar
		.iter()
		.map(HotbarItem::into_slot)
		.collect::<Vec<_>>();

	for slot in &slots {
		canvas = canvas.push_checked_post_draw(slot, Body::empty());
	}

	let mut surface = canvas.build(None, background).unwrap();

	canvas::to_png(&mut surface).into()
}

pub fn shop(
	ctx: &context::Context<'_>,
	family: Family,
	background: Option<Color>,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
) -> Cow<'static, [u8]> {
	let game = &data.stats.bed_wars;

	let canvas = Canvas::new(720., family).gap(7.).push_down(
		&shape::Title,
		shape::Title::from_text(family, &text::from_data(data, &data.username, suffix)),
	);

	let (xp, level) = {
		let xp = calc::bed_wars::convert(&game.xp);
		let level = calc::bed_wars::get_level(xp);

		(xp, level)
	};

	let status = shape::Status(session, skin);
	let progress = shape::WideBubbleProgress(
		calc::bed_wars::get_level_progress(xp),
		calc::bed_wars::get_colours(level),
		false,
	);

	let canvas = canvas
		.push_down(
			&shape::Subtitle,
			shape::Subtitle::from_label(ctx, family, &bed_wars::LABEL, "shop"),
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&calc::bed_wars::get_level_format(level),
				&calc::bed_wars::get_curr_level_xp(xp),
				&calc::bed_wars::get_level_xp(xp),
			),
		);

	let mut canvas = bed_wars::Overall
		.add_header(ctx, family, canvas, data)
		.push_right_post_draw(&status, Body::from_status(ctx, family, session));

	let slots = game
		.shop
		.iter()
		.map(ShopItem::into_slot)
		.collect::<Vec<_>>();

	let empty_slot = shape::Slot(None, 1);

	// First row is empty
	for _ in 0..9 {
		canvas = canvas.push_checked(&empty_slot, Body::empty());
	}

	// 3 rows of 7 items, so pad the first and last with empty slots
	for slots in slots.iter().array_chunks::<7>() {
		canvas = canvas.push_checked(&empty_slot, Body::empty());

		for slot in slots {
			canvas = canvas.push_checked_post_draw(slot, Body::empty());
		}

		canvas = canvas.push_checked(&empty_slot, Body::empty());
	}

	for _ in 0..9 {
		canvas = canvas.push_checked(&empty_slot, Body::empty());
	}

	let mut surface = canvas.build(None, background).unwrap();

	canvas::to_png(&mut surface).into()
}

#[allow(clippy::too_many_lines)]
pub fn practice(
	ctx: &context::Context<'_>,
	family: Family,
	background: Option<Color>,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
) -> Cow<'static, [u8]> {
	let game = &data.stats.bed_wars;

	let canvas = Canvas::new(720., family).gap(7.).push_down(
		&shape::Title,
		shape::Title::from_text(family, &text::from_data(data, &data.username, suffix)),
	);

	let (xp, level) = {
		let xp = calc::bed_wars::convert(&game.xp);
		let level = calc::bed_wars::get_level(xp);

		(xp, level)
	};

	let status = shape::Status(session, skin);
	let progress = shape::WideBubbleProgress(
		calc::bed_wars::get_level_progress(xp),
		calc::bed_wars::get_colours(level),
		false,
	);

	let mut records = vec![
		("straight-none-30", game.practice.records.straight_none_30),
		("straight-none-50", game.practice.records.straight_none_50),
		("straight-none-100", game.practice.records.straight_none_100),
		(
			"straight-slight-30",
			game.practice.records.straight_slight_30,
		),
		(
			"straight-slight-50",
			game.practice.records.straight_slight_50,
		),
		(
			"straight-slight-100",
			game.practice.records.straight_slight_100,
		),
		(
			"straight-staircase-30",
			game.practice.records.straight_staircase_30,
		),
		(
			"straight-staircase-50",
			game.practice.records.straight_staircase_50,
		),
		(
			"straight-staircase-100",
			game.practice.records.straight_staircase_100,
		),
		("diagonal-none-30", game.practice.records.diagonal_none_30),
		("diagonal-none-50", game.practice.records.diagonal_none_50),
		("diagonal-none-100", game.practice.records.diagonal_none_100),
		(
			"diagonal-slight-30",
			game.practice.records.diagonal_slight_30,
		),
		(
			"diagonal-slight-50",
			game.practice.records.diagonal_slight_50,
		),
		(
			"diagonal-slight-100",
			game.practice.records.diagonal_slight_100,
		),
		(
			"diagonal-staircase-30",
			game.practice.records.diagonal_staircase_30,
		),
		(
			"diagonal-staircase-50",
			game.practice.records.diagonal_staircase_50,
		),
		(
			"diagonal-staircase-100",
			game.practice.records.diagonal_staircase_100,
		),
	];

	records.sort_by(|a, b| b.1.cmp(&a.1));
	records.drain(5..);

	let mut sidebar_body = Body::new(17., None, family)
		.append_item(
			tr(ctx, "attempts").as_ref(),
			(game.practice.bridging.successes
				+ game.practice.bridging.failures
				+ game.practice.fireball.successes
				+ game.practice.fireball.failures
				+ game.practice.mlg.successes
				+ game.practice.mlg.failures
				+ game.practice.pearl.successes
				+ game.practice.pearl.failures)
				.to_formatted(ctx)
				.as_ref(),
			&Paint::Green,
		)
		.append_item(
			tr(ctx, "blocks-placed").as_ref(),
			(game.practice.bridging.blocks_placed
				+ game.practice.fireball.blocks_placed
				+ game.practice.mlg.blocks_placed
				+ game.practice.pearl.blocks_placed)
				.to_formatted(ctx)
				.as_ref(),
			&Paint::Blue,
		);

	for (name, value) in records {
		sidebar_body = sidebar_body.append_item(
			tr(ctx, name).as_ref(),
			value.to_formatted(ctx).as_ref(),
			&Paint::Yellow,
		);
	}

	let mut surface = canvas
		.push_down(
			&shape::Subtitle,
			shape::Subtitle::from_label(ctx, family, &bed_wars::LABEL, "practice"),
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&calc::bed_wars::get_level_format(level),
				&calc::bed_wars::get_curr_level_xp(xp),
				&calc::bed_wars::get_level_xp(xp),
			),
		)
		.push_right_start(&shape::Sidebar, sidebar_body.build())
		.push_right_post_draw(&status, Body::from_status(ctx, family, session))
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&game.practice.bridging.successes,
				tr(ctx, "bridging-wins").as_ref(),
				Paint::Green,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&game.practice.bridging.failures,
				tr(ctx, "bridging-losses").as_ref(),
				Paint::Red,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&(f64::from(game.practice.bridging.successes)
					/ if game.practice.bridging.failures == 0 {
						1.
					} else {
						f64::from(game.practice.bridging.failures)
					}),
				tr(ctx, "wlr").as_ref(),
				Paint::Gold,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&game.practice.fireball.successes,
				tr(ctx, "fireball-wins").as_ref(),
				Paint::Green,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&game.practice.fireball.failures,
				tr(ctx, "fireball-losses").as_ref(),
				Paint::Red,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&(f64::from(game.practice.fireball.successes)
					/ if game.practice.fireball.failures == 0 {
						1.
					} else {
						f64::from(game.practice.fireball.failures)
					}),
				tr(ctx, "wlr").as_ref(),
				Paint::Gold,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&game.practice.mlg.successes,
				tr(ctx, "mlg-wins").as_ref(),
				Paint::Green,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&game.practice.mlg.failures,
				tr(ctx, "mlg-losses").as_ref(),
				Paint::Red,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&(f64::from(game.practice.mlg.successes)
					/ if game.practice.mlg.failures == 0 {
						1.
					} else {
						f64::from(game.practice.mlg.failures)
					}),
				tr(ctx, "wlr").as_ref(),
				Paint::Gold,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&game.practice.pearl.successes,
				tr(ctx, "pearl-wins").as_ref(),
				Paint::Green,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&game.practice.pearl.failures,
				tr(ctx, "pearl-losses").as_ref(),
				Paint::Red,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&(f64::from(game.practice.pearl.successes)
					/ if game.practice.pearl.failures == 0 {
						1.
					} else {
						f64::from(game.practice.pearl.failures)
					}),
				tr(ctx, "wlr").as_ref(),
				Paint::Gold,
			),
		)
		.build(None, background)
		.unwrap();

	canvas::to_png(&mut surface).into()
}
