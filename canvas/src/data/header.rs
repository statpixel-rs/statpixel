use api::player::data::PlayerData;
use minecraft::text::{self, parse::parse_minecraft_string, MinecraftText};
use skia_safe::Surface;

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

	text::draw_minecraft_text(surface, text.into_iter(), 100., 100., 20.);
}
