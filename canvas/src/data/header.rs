use api::player::data::PlayerData;
use minecraft::text::{self, parse::parse_minecraft_string, MinecraftText};
use skia_safe::Surface;

use crate::{HEADER_LEFT_END_X, HEADER_NAME_HEIGHT, PADDING};

pub fn apply(surface: &mut Surface, data: &PlayerData) {
	let rank = data.get_rank();

	let mut text = if let Some(text) = rank.get_text() {
		text.copied().collect::<Vec<_>>()
	} else if let Some(prefix) = data.prefix.as_ref() {
		parse_minecraft_string(prefix).collect()
	} else {
		unreachable!();
	};

	let username = format!(" {}", data.username);

	text.push(MinecraftText {
		text: &username,
		paint: rank.get_username_paint(),
		font: minecraft::font::MinecraftFont::Normal,
	});

	let username_width = text::measure_minecraft_text_ref(text.iter(), 25.);

	text::draw_minecraft_text(
		surface,
		text.into_iter(),
		PADDING + (HEADER_LEFT_END_X - PADDING) / 2. - username_width / 2.,
		PADDING + HEADER_NAME_HEIGHT / 2. + 18. / 2.,
		25.,
	);
}
