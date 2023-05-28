use std::borrow::Cow;

use minecraft::{
	paint,
	style::MinecraftFont,
	text::{draw, parse::minecraft_string, rank::Rank, Text},
};
use skia_safe::{textlayout::TextAlign, Path, Point, RRect, Rect, Surface};
use translate::Context;

use super::{label::ToFormatted, GAP, PADDING, WIDTH, WIDTH_F};
use crate::{leaderboard::Leaderboard, player::data::Data};

const ITEM_HEIGHT: f32 = 35.;
const NUM_WIDTH: f32 = 50.;
const VALUE_WIDTH: f32 = 200.;
const NAME_WIDTH: f32 = WIDTH_F - NUM_WIDTH - GAP * 2. - PADDING * 2. - VALUE_WIDTH;

const HEADER_HEIGHT: f32 = 50.;

#[must_use]
/// # Panics
/// This will not panic, since the dimensions are already >0.
pub fn create(rows: u8) -> Surface {
	let height = PADDING * 2. + GAP + HEADER_HEIGHT + (GAP + ITEM_HEIGHT) * f32::from(rows);

	#[allow(clippy::cast_possible_truncation)]
	let mut surface = Surface::new_raster_n32_premul((WIDTH, height as i32)).unwrap();

	let mut path = Path::new();
	let mut rect = RRect::new();

	// Background
	rect.set_rect_radii(
		Rect::new(0., 0., WIDTH_F, height),
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

	rect.set_rect_radii(
		Rect::from_xywh(PADDING, PADDING, WIDTH_F - PADDING * 2., HEADER_HEIGHT),
		&[
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
			Point::new(20., 20.),
		],
	);

	path.add_rrect(rect, None);

	let mut y = PADDING + GAP + HEADER_HEIGHT;
	let x = PADDING;

	for _ in 0..rows {
		rect.set_rect_radii(
			Rect::from_xywh(x, y, NUM_WIDTH, ITEM_HEIGHT),
			&[
				Point::new(20., 20.),
				Point::new(20., 20.),
				Point::new(20., 20.),
				Point::new(20., 20.),
			],
		);

		path.add_rrect(rect, None);

		rect.set_rect_radii(
			Rect::from_xywh(x + NUM_WIDTH + GAP, y, NAME_WIDTH, ITEM_HEIGHT),
			&[
				Point::new(20., 20.),
				Point::new(20., 20.),
				Point::new(20., 20.),
				Point::new(20., 20.),
			],
		);

		path.add_rrect(rect, None);

		rect.set_rect_radii(
			Rect::from_xywh(
				x + NUM_WIDTH + NAME_WIDTH + GAP * 2.,
				y,
				VALUE_WIDTH,
				ITEM_HEIGHT,
			),
			&[
				Point::new(20., 20.),
				Point::new(20., 20.),
				Point::new(20., 20.),
				Point::new(20., 20.),
			],
		);

		path.add_rrect(rect, None);

		y += ITEM_HEIGHT + GAP;
	}

	surface.canvas().draw_path(&path, &paint::BACKGROUND);
	surface
}

pub fn header(surface: &mut Surface, leaderboard: &Leaderboard) {
	let name = format!(" ({})", leaderboard.name);
	let mut text = Vec::with_capacity(2);

	text.extend(leaderboard.game.as_text());
	text.push(Text {
		text: &name,
		paint: paint::Paint::White,
		..Default::default()
	});

	draw(
		surface,
		text.as_slice(),
		24.,
		Rect::from_xywh(0., PADDING, WIDTH_F, HEADER_HEIGHT),
		TextAlign::Center,
		true,
	);
}

pub fn row(
	ctx: Context<'_>,
	surface: &mut Surface,
	player: &Data,
	value: &dyn ToFormatted,
	idx: u8,
) {
	let y = PADDING + (ITEM_HEIGHT + GAP) * f32::from(idx) + HEADER_HEIGHT + GAP;
	let mut x = PADDING;

	draw(
		surface,
		&[Text {
			text: &format!("{}", idx + 1),
			paint: paint::Paint::White,
			font: MinecraftFont::Bold,
			..Default::default()
		}],
		20.,
		Rect::from_xywh(x, y, NUM_WIDTH, ITEM_HEIGHT),
		TextAlign::Center,
		true,
	);

	x += NUM_WIDTH + GAP;

	let mut name = Vec::with_capacity(2);
	let rank = player.get_rank();

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

	draw(
		surface,
		name.as_slice(),
		20.,
		Rect::from_xywh(x + 10., y, NAME_WIDTH, ITEM_HEIGHT),
		TextAlign::Left,
		true,
	);

	x += NAME_WIDTH + GAP;

	draw(
		surface,
		&[Text {
			text: &value.to_formatted_label(ctx),
			paint: paint::Paint::White,
			..Default::default()
		}],
		20.,
		Rect::from_xywh(x, y, VALUE_WIDTH, ITEM_HEIGHT),
		TextAlign::Center,
		true,
	);
}
