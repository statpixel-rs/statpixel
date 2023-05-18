use crate::{
	game::r#type::Type,
	guild::Guild,
	player::{data::Data, status::Session},
};
use minecraft::text::{
	self,
	parse::{self, minecraft_string, ESCAPE},
	Text,
};
use skia_safe::{textlayout::TextAlign, Rect, Surface};
use translate::{tr, Context};

use super::{
	GAP, HEADER_HEIGHT, HEADER_LEFT_END_X, HEADER_MIDDLE_END_X, HEADER_NAME_HEIGHT, PADDING,
	WIDTH_F,
};

pub fn apply_guild(surface: &mut Surface, guild: &Guild) {
	let colour: char = guild.tag_colour.into();
	let name = guild.name.as_ref();
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

pub fn apply_name(surface: &mut Surface, data: &Data) {
	let rank = data.get_rank();

	let mut text = if let Some(text) = rank.get_text() {
		text.to_vec()
	} else if let Some(prefix) = data.prefix.as_ref() {
		minecraft_string(prefix).by_ref().collect()
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
					paint: minecraft::paint::Paint::White,
					..Default::default()
				},
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: data.game_mode.as_deref().unwrap_or(""),
					paint: minecraft::paint::Paint::White,
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
