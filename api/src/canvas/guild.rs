use std::borrow::Cow;

use minecraft::{
	calc,
	colour::Colour,
	paint::{self, Paint},
	style::MinecraftFont,
	text::{
		self, draw,
		parse::{self, minecraft_string, ESCAPE},
		rank::Rank,
		Text,
	},
};
use skia_safe::{textlayout::TextAlign, Path, Point, RRect, Rect, Surface};
use translate::{tr, Context};

use super::{
	game, gutter, label::ToFormatted, sidebar, GAP, HEADER_DATA_RAD, HEADER_HEIGHT,
	HEADER_LABEL_HEIGHT, HEADER_LEFT_END_X, HEADER_MIDDLE_END_X, HEADER_NAME_HEIGHT, ITEM_HEIGHT,
	ITEM_WIDTH, PADDING, WIDTH, WIDTH_F,
};
use crate::{guild::Guild, player::data::Data};

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

pub fn leader(surface: &mut Surface, data: &Data) {
	let rank = data.get_rank();

	let mut text = if let Some(text) = rank.get_text() {
		text.to_vec()
	} else if let Some(prefix) = data.prefix.as_ref() {
		minecraft_string(prefix).by_ref().collect()
	} else {
		unreachable!();
	};

	let username = if rank == Rank::Default {
		Cow::Borrowed(data.username.as_str())
	} else {
		Cow::Owned(format!(" {}", data.username))
	};

	text.push(Text {
		text: &username,
		paint: rank.get_username_paint(),
		..Default::default()
	});

	text::draw(
		surface,
		text.as_slice(),
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
			&(game.as_short_clean_cow(), Box::new(*xp), Paint::Gold, false),
			0,
		);
	}

	if let Some((game, xp)) = iter.next() {
		sidebar::item(
			ctx,
			surface,
			&(game.as_short_clean_cow(), Box::new(*xp), Paint::Gray, false),
			1,
		);
	}

	if let Some((game, xp)) = iter.next() {
		sidebar::item(
			ctx,
			surface,
			&(game.as_short_clean_cow(), Box::new(*xp), Paint::Red, false),
			2,
		);
	}

	for (idx, (game, xp)) in iter.take(4).enumerate() {
		sidebar::item(
			ctx,
			surface,
			&(
				game.as_short_clean_cow(),
				Box::new(*xp),
				Paint::DarkGray,
				false,
			),
			3 + idx as u8,
		);
	}
}

/// There should be at most 14 `players` should be sorted by weekly XP
pub fn members(ctx: Context<'_>, surface: &mut Surface, guild: &Guild, players: &[Data]) {
	let mut y = PADDING + HEADER_HEIGHT + GAP * 2. + ITEM_HEIGHT + 13.;
	let count = players.len();

	for (idx, player) in players.iter().enumerate().rev().take(7) {
		let rank = player.get_rank();
		let mut name = Vec::with_capacity(2);
		let idx_string = format!("{}. ", count - idx);

		name.push(Text {
			text: &idx_string,
			paint: Paint::White,
			..Default::default()
		});

		if let Some(text) = rank.get_text() {
			name.extend(text);
		} else if let Some(prefix) = player.prefix.as_ref() {
			name.extend(minecraft_string(prefix).by_ref());
		}

		let username = if rank == Rank::Default {
			Cow::Borrowed(player.username.as_str())
		} else {
			Cow::Owned(format!(" {}", player.username))
		};

		name.push(Text {
			text: &username,
			paint: rank.get_username_paint(),
			..Default::default()
		});

		let text = format!(
			" ({})",
			(guild.members[idx]
				.xp_history
				.iter()
				.map(|h| h.1)
				.sum::<u32>())
			.to_formatted_label(ctx, false)
		);

		name.push(Text {
			text: &text,
			paint: Paint::Yellow,
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

	let mut y = PADDING + HEADER_HEIGHT + GAP * 2. + ITEM_HEIGHT + 13.;

	for (idx, player) in players.iter().enumerate().rev().skip(7).take(7) {
		let rank = player.get_rank();
		let mut name = Vec::with_capacity(2);
		let idx_string = format!("{}. ", count - idx);

		name.push(Text {
			text: &idx_string,
			paint: Paint::White,
			..Default::default()
		});

		if let Some(text) = rank.get_text() {
			name.extend(text);
		} else if let Some(prefix) = player.prefix.as_ref() {
			name.extend(minecraft_string(prefix).by_ref());
		}

		let username = if rank == Rank::Default {
			Cow::Borrowed(player.username.as_str())
		} else {
			Cow::Owned(format!(" {}", player.username))
		};

		name.push(Text {
			text: &username,
			paint: rank.get_username_paint(),
			..Default::default()
		});

		let text = format!(
			" ({})",
			(guild.members[idx]
				.xp_history
				.iter()
				.map(|h| h.1)
				.sum::<u32>())
			.to_formatted_label(ctx, false)
		);

		name.push(Text {
			text: &text,
			paint: Paint::Yellow,
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

pub fn stats(ctx: Context<'_>, surface: &mut Surface, guild: &Guild) {
	game::bubble(
		ctx,
		surface,
		guild.coins,
		tr!(ctx, "coins").as_ref(),
		Paint::Gold,
		None,
		0,
	);

	game::bubble(
		ctx,
		surface,
		guild.xp,
		tr!(ctx, "experience").as_ref(),
		Paint::Yellow,
		None,
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
			text: &guild.created_at.to_formatted_label(ctx, false),
			paint: Paint::Aqua,
			font: MinecraftFont::Normal,
			size: None,
		},
	];

	let rect = super::get_item_rect(1);

	draw(surface, &text, 30., rect, TextAlign::Center, true);
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
pub fn ranks(surface: &mut Surface, guild: &mut Guild) {
	guild.ranks.sort_by_key(|r| r.priority);

	for (idx, rank) in guild
		.ranks
		.iter()
		.enumerate()
		.filter_map(|(idx, r)| {
			if idx > 0 && r.priority == guild.ranks[idx - 1].priority {
				None
			} else {
				Some(r)
			}
		})
		.rev()
		.enumerate()
	{
		gutter::item(
			surface,
			&(Cow::Borrowed(&rank.name), paint::Paint::Blue),
			idx as u8,
		);
	}
}

/// # Panics
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn create_surface() -> Surface {
	const HEIGHT: f32 = PADDING * 2. + HEADER_HEIGHT + (GAP + ITEM_HEIGHT) * 3.;

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

	let mut x = PADDING;

	for _ in 0..3 {
		rect.set_rect_radii(
			Rect::new(
				x,
				PADDING + HEADER_HEIGHT + GAP,
				x + ITEM_WIDTH,
				PADDING + HEADER_HEIGHT + GAP + ITEM_HEIGHT,
			),
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

	// Bottom left panel
	rect.set_rect_radii(
		Rect::new(
			PADDING,
			PADDING + HEADER_HEIGHT + GAP * 2. + ITEM_HEIGHT,
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
			PADDING + HEADER_HEIGHT + GAP * 2. + ITEM_HEIGHT,
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
