use std::borrow::Cow;

use crate::{
	game::{mode::Mode, r#type::Type},
	player::{data::Data, status::Session},
};
use minecraft::text::{self, parse::minecraft_string, rank::Rank, Text};
use skia_safe::{textlayout::TextAlign, Rect, Surface};
use translate::{tr, Context};

use super::{
	GAP, HEADER_HEIGHT, HEADER_LEFT_END_X, HEADER_MIDDLE_END_X, HEADER_NAME_HEIGHT, PADDING,
	WIDTH_F,
};

pub fn apply_name(surface: &mut Surface, data: &Data) {
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
		25.,
		Rect::from_xywh(
			PADDING,
			PADDING,
			HEADER_LEFT_END_X - PADDING,
			HEADER_NAME_HEIGHT,
		),
		TextAlign::Center,
		true,
	);
}

pub fn apply_name_str(surface: &mut Surface, display: &str) {
	text::draw(
		surface,
		minecraft_string(display).collect::<Vec<_>>().as_slice(),
		25.,
		Rect::from_xywh(
			PADDING,
			PADDING,
			HEADER_LEFT_END_X - PADDING,
			HEADER_NAME_HEIGHT,
		),
		TextAlign::Center,
		true,
	);
}

pub fn apply_status(ctx: Context<'_>, surface: &mut Surface, data: &Session) {
	let rect = Rect::new(
		HEADER_MIDDLE_END_X + GAP,
		PADDING,
		WIDTH_F - PADDING,
		HEADER_HEIGHT + PADDING,
	)
	.with_inset((0., 15.));

	if data.online {
		text::draw(
			surface,
			&[
				Text {
					text: &tr!(ctx, "online"),
					paint: minecraft::paint::Paint::Green,
					..Default::default()
				},
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: data.game_type.unwrap_or(Type::Lobby).as_clean_name(),
					paint: minecraft::paint::Paint::Gray,
					..Default::default()
				},
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: data
						.game_mode
						.as_deref()
						.and_then(|m| Mode::try_from(m).ok())
						.map_or("Unknown", |m| m.as_clean_name()),
					paint: minecraft::paint::Paint::Aqua,
					..Default::default()
				},
			],
			18.,
			rect,
			TextAlign::Center,
			true,
		);
	} else {
		text::draw(
			surface,
			&[Text {
				text: &tr!(ctx, "offline"),
				paint: minecraft::paint::Paint::DarkGray,
				..Default::default()
			}],
			25.,
			rect,
			TextAlign::Center,
			true,
		);
	}
}
