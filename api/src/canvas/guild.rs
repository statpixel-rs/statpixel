use minecraft::{
	calc,
	colour::Colour,
	paint::{self, Paint},
	style::MinecraftFont,
	text::{
		self, draw,
		parse::{self, minecraft_string, ESCAPE},
		Text,
	},
};
use skia_safe::{textlayout::TextAlign, Image, Path, Point, RRect, Rect, Surface};
use translate::{tr, Context};

use super::{
	game, label::ToFormatted, sidebar, GAP, HEADER_DATA_RAD, HEADER_HEIGHT, HEADER_LABEL_HEIGHT,
	HEADER_LEFT_END_X, HEADER_MIDDLE_END_X, HEADER_NAME_HEIGHT, ITEM_HEIGHT, ITEM_WIDTH, PADDING,
	WIDTH, WIDTH_F,
};
use crate::guild::Guild;

pub fn header(surface: &mut Surface, guild: &Guild) {
	let colour: char = guild.tag_colour.into();
	let name = guild.name.as_str();
	let tag = guild.tag.as_ref();

	let text = if let Some(tag) = tag {
		format!("{ESCAPE}{colour}{name} [{tag}]")
	} else {
		format!("{ESCAPE}{colour}{name}")
	};

	let text = parse::minecraft_string(&text).collect::<Vec<_>>();

	text::draw(
		surface,
		text.as_slice(),
		25.,
		Rect::from_xywh(PADDING, PADDING, HEADER_LEFT_END_X, HEADER_NAME_HEIGHT),
		TextAlign::Center,
		true,
	);
}

pub fn leader(surface: &mut Surface, name: &[Text]) {
	text::draw(
		surface,
		name,
		20.,
		Rect::from_xywh(
			PADDING,
			PADDING + HEADER_NAME_HEIGHT + GAP,
			HEADER_LEFT_END_X,
			HEADER_LABEL_HEIGHT,
		),
		TextAlign::Center,
		true,
	);
}

#[allow(clippy::cast_possible_truncation)]
pub fn games(ctx: Context<'_>, surface: &mut Surface, guild: &mut Guild) {
	guild.xp_by_game.sort_unstable_by_key(|g| g.1);

	let mut iter = guild.xp_by_game.iter().rev();

	if let Some((game, xp)) = iter.next() {
		sidebar::item(
			ctx,
			surface,
			&(game.as_short_clean_cow(), *xp, Paint::Gold),
			0,
		);
	}

	if let Some((game, xp)) = iter.next() {
		sidebar::item(
			ctx,
			surface,
			&(game.as_short_clean_cow(), *xp, Paint::Gray),
			1,
		);
	}

	if let Some((game, xp)) = iter.next() {
		sidebar::item(
			ctx,
			surface,
			&(game.as_short_clean_cow(), *xp, Paint::Red),
			2,
		);
	}

	for (idx, (game, xp)) in iter.take(4).enumerate() {
		sidebar::item(
			ctx,
			surface,
			&(game.as_short_clean_cow(), *xp, Paint::DarkGray),
			3 + idx as u8,
		);
	}
}

/// There should be at most 14 `players` should be sorted by weekly XP
#[allow(clippy::too_many_lines)]
pub fn members(ctx: Context<'_>, surface: &mut Surface, guild: &Guild, players: &[String]) {
	let mut y = PADDING + HEADER_HEIGHT + GAP * 3. + ITEM_HEIGHT * 2. + 13.;
	let count = guild.members.len();

	for (idx, player) in players.iter().enumerate().take(7) {
		let mut name = minecraft_string(player).collect::<Vec<_>>();

		name.push(Text {
			text: " • ",
			paint: Paint::DarkGray,
			..Default::default()
		});

		let text = guild.members[count - idx - 1]
			.xp_history
			.iter()
			.map(|h| h.1)
			.sum::<u32>();
		let text = text.to_formatted_label(ctx);

		name.push(Text {
			text: text.as_ref(),
			paint: Paint::Gray,
			..Default::default()
		});

		text::draw(
			surface,
			name.as_slice(),
			17.,
			Rect::from_xywh(PADDING + 17., y, HEADER_LEFT_END_X, 21.8),
			TextAlign::Left,
			true,
		);

		y += 21.8;
	}

	let mut y = PADDING + HEADER_HEIGHT + GAP * 3. + ITEM_HEIGHT * 2. + 13.;

	for (idx, player) in players.iter().enumerate().skip(7).take(7) {
		let mut name = minecraft_string(player).collect::<Vec<_>>();

		name.push(Text {
			text: " • ",
			paint: Paint::DarkGray,
			..Default::default()
		});

		let text = guild.members[count - idx - 1]
			.xp_history
			.iter()
			.map(|h| h.1)
			.sum::<u32>();
		let text = text.to_formatted_label(ctx);

		name.push(Text {
			text: text.as_ref(),
			paint: Paint::Gray,
			..Default::default()
		});

		text::draw(
			surface,
			name.as_slice(),
			17.,
			Rect::from_xywh(HEADER_LEFT_END_X + GAP + 17., y, HEADER_LEFT_END_X, 21.8),
			TextAlign::Left,
			true,
		);

		y += 21.8;
	}
}

pub fn stats(ctx: Context<'_>, surface: &mut Surface, guild: &Guild, monthly_xp: impl ToFormatted) {
	game::bubble(
		ctx,
		surface,
		guild.coins,
		tr!(ctx, "coins").as_ref(),
		Paint::Gold,
		0,
	);

	game::bubble(
		ctx,
		surface,
		format!("{}/125", guild.members.len()),
		tr!(ctx, "members").as_ref(),
		Paint::LightPurple,
		2,
	);

	let text = tr!(ctx, "created-at");
	let text = [
		Text {
			text: text.as_ref(),
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
	];

	let rect = super::get_item_rect(1);

	draw(surface, &text, 30., rect, TextAlign::Center, true);

	let daily_xp = guild.members.iter().map(|m| m.xp_history[0].1).sum::<u32>();
	let weekly_xp = guild
		.members
		.iter()
		.map(|m| m.xp_history.iter().map(|h| h.1).sum::<u32>())
		.sum::<u32>();

	game::bubble(
		ctx,
		surface,
		daily_xp,
		tr!(ctx, "daily-xp").as_ref(),
		Paint::DarkGreen,
		3,
	);

	game::bubble(
		ctx,
		surface,
		weekly_xp,
		tr!(ctx, "weekly-xp").as_ref(),
		Paint::DarkGreen,
		4,
	);

	game::bubble(
		ctx,
		surface,
		monthly_xp,
		tr!(ctx, "monthly-xp").as_ref(),
		Paint::DarkGreen,
		5,
	);
}

pub fn stats_history(
	ctx: Context<'_>,
	surface: &mut Surface,
	guild: &Guild,
	daily_xp: impl ToFormatted,
	weekly_xp: impl ToFormatted,
	xp_since: impl ToFormatted,
) {
	game::bubble(
		ctx,
		surface,
		guild.coins,
		tr!(ctx, "coins").as_ref(),
		Paint::Gold,
		0,
	);

	game::bubble(
		ctx,
		surface,
		format!("{}/125", guild.members.len()),
		tr!(ctx, "members").as_ref(),
		Paint::LightPurple,
		2,
	);

	let text = tr!(ctx, "created-at");
	let text = [
		Text {
			text: text.as_ref(),
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
	];

	let rect = super::get_item_rect(1);

	draw(surface, &text, 30., rect, TextAlign::Center, true);

	game::bubble(
		ctx,
		surface,
		daily_xp,
		tr!(ctx, "daily-xp").as_ref(),
		Paint::DarkGreen,
		3,
	);

	game::bubble(
		ctx,
		surface,
		weekly_xp,
		tr!(ctx, "weekly-xp").as_ref(),
		Paint::DarkGreen,
		4,
	);

	game::bubble(
		ctx,
		surface,
		xp_since,
		tr!(ctx, "xp-since").as_ref(),
		Paint::DarkGreen,
		5,
	);
}

pub fn level(ctx: Context<'_>, surface: &mut Surface, guild: &Guild) {
	let level = calc::guild::get_level(guild.xp);

	game::apply_data(
		ctx,
		surface,
		&format!("{ESCAPE}6{level}"),
		calc::guild::get_level_progress(guild.xp),
		calc::guild::get_curr_level_xp(guild.xp),
		calc::guild::get_level_xp(guild.xp),
		&[Colour::Gold.into(), Colour::Gold.into()],
	);
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
/// # Panics
/// This function will not panic as the image bytes live for 'static.
pub fn preferred_games(surface: &mut Surface, guild: &Guild) {
	let mut iter = guild
		.preferred_games
		.iter()
		.filter_map(crate::game::r#type::Type::as_image_bytes)
		.enumerate()
		.take(6);

	while let Some((idx, bytes)) = iter.next() {
		let x = HEADER_MIDDLE_END_X + GAP + 17.;
		let y = PADDING + 20. + (40. + 7.) * idx as f32 / 2.;

		// `bytes` lives for 'static, so it will always be valid.
		let image = Image::from_encoded(unsafe { skia_safe::Data::new_bytes(bytes) }).unwrap();

		surface.canvas().draw_image(image, (x, y), None);

		if let Some((_, bytes)) = iter.next() {
			let x = x + 40. + 7.;
			let image = Image::from_encoded(unsafe { skia_safe::Data::new_bytes(bytes) }).unwrap();

			surface.canvas().draw_image(image, (x, y), None);
		}
	}
}

/// # Panics
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn create_surface() -> Surface {
	const HEIGHT: f32 = PADDING * 2. + HEADER_HEIGHT + (GAP + ITEM_HEIGHT) * 4.;

	#[allow(clippy::cast_possible_truncation)]
	let mut surface = Surface::new_raster_n32_premul((WIDTH, HEIGHT as i32)).unwrap();

	let mut path = Path::new();
	let mut rect = RRect::new();

	// Background
	rect.set_rect_radii(
		Rect::new(0., 0., WIDTH_F, HEIGHT),
		&[
			Point::new(30., 30.),
			Point::new(30., 30.),
			Point::new(30., 30.),
			Point::new(30., 30.),
		],
	);

	path.add_rrect(rect, None);
	surface.canvas().draw_path(&path, &paint::CANVAS_BACKGROUND);

	path.reset();

	// Header, left block top
	rect.set_rect_radii(
		Rect::new(
			PADDING,
			PADDING,
			HEADER_LEFT_END_X,
			HEADER_NAME_HEIGHT + PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	// Header, left block middle
	rect.set_rect_radii(
		Rect::new(
			PADDING,
			PADDING + HEADER_NAME_HEIGHT + GAP,
			HEADER_LEFT_END_X,
			PADDING + HEADER_NAME_HEIGHT + GAP + HEADER_LABEL_HEIGHT,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	// Header, left block bottom
	rect.set_rect_radii(
		Rect::new(
			PADDING,
			PADDING + HEADER_NAME_HEIGHT + GAP * 2. + HEADER_LABEL_HEIGHT,
			HEADER_LEFT_END_X,
			HEADER_HEIGHT + PADDING,
		),
		&[
			Point::new(HEADER_DATA_RAD, HEADER_DATA_RAD),
			Point::new(HEADER_DATA_RAD, HEADER_DATA_RAD),
			Point::new(HEADER_DATA_RAD, HEADER_DATA_RAD),
			Point::new(HEADER_DATA_RAD, HEADER_DATA_RAD),
		],
	);

	path.add_rrect(rect, None);

	// Header, middle block
	rect.set_rect_radii(
		Rect::new(
			HEADER_LEFT_END_X + GAP,
			PADDING,
			HEADER_MIDDLE_END_X,
			HEADER_HEIGHT + PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	// Header, right block
	rect.set_rect_radii(
		Rect::new(
			HEADER_MIDDLE_END_X + GAP,
			PADDING,
			WIDTH_F - PADDING,
			HEADER_HEIGHT + PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	let mut y = PADDING + HEADER_HEIGHT + GAP;

	for _ in 0..2 {
		let mut x = PADDING;

		for _ in 0..3 {
			rect.set_rect_radii(
				Rect::new(x, y, x + ITEM_WIDTH, y + ITEM_HEIGHT),
				&[
					Point::new(20., 20.),
					Point::new(20., 20.),
					Point::new(20., 20.),
					Point::new(20., 20.),
				],
			);

			path.add_rrect(rect, None);

			x += ITEM_WIDTH + GAP;
		}

		y += ITEM_HEIGHT + GAP;
	}

	// Bottom left panel
	rect.set_rect_radii(
		Rect::new(
			PADDING,
			PADDING + HEADER_HEIGHT + GAP * 3. + ITEM_HEIGHT * 2.,
			HEADER_LEFT_END_X,
			HEIGHT - PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	// Bottom right panel
	rect.set_rect_radii(
		Rect::new(
			HEADER_LEFT_END_X + GAP,
			PADDING + HEADER_HEIGHT + GAP * 3. + ITEM_HEIGHT * 2.,
			WIDTH_F - PADDING,
			HEIGHT - PADDING,
		),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	surface.canvas().draw_path(&path, &paint::BACKGROUND);
	surface
}
