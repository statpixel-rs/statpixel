use crate::{
	game::r#type::GameType,
	player::{data::PlayerData, status::PlayerSession},
};
use minecraft::text::{self, parse::parse_minecraft_string, Text};
use skia_safe::{textlayout::TextAlign, Rect, Surface};
use translate::{tr, Context};

use super::{
	GAP, HEADER_HEIGHT, HEADER_LEFT_END_X, HEADER_MIDDLE_END_X, HEADER_NAME_HEIGHT, PADDING,
	WIDTH_F,
};

pub fn apply_name(surface: &mut Surface, data: &PlayerData) {
	let rank = data.get_rank();

	let mut text = if let Some(text) = rank.get_text() {
		text.to_vec()
	} else if let Some(prefix) = data.prefix.as_ref() {
		parse_minecraft_string(prefix).by_ref().collect()
	} else {
		unreachable!();
	};

	let username = format!(" {}", data.username);

	text.push(Text {
		text: &username,
		paint: rank.get_username_paint(),
		..Default::default()
	});

	text::draw(
		surface,
		text.as_slice(),
		25.,
		Rect::from_xywh(PADDING, PADDING, HEADER_LEFT_END_X, HEADER_NAME_HEIGHT),
		TextAlign::Center,
		true,
	);
}

pub fn apply_status(ctx: Context<'_>, surface: &mut Surface, data: &PlayerSession) {
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
					paint: minecraft::paint::MinecraftPaint::Green,
					..Default::default()
				},
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: data.game_type.unwrap_or(GameType::Lobby).as_clean_name(),
					paint: minecraft::paint::MinecraftPaint::White,
					..Default::default()
				},
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: data.game_mode.as_deref().unwrap_or(""),
					paint: minecraft::paint::MinecraftPaint::White,
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
				paint: minecraft::paint::MinecraftPaint::DarkGray,
				..Default::default()
			}],
			25.,
			rect,
			TextAlign::Center,
			true,
		);
	}
}
