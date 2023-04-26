use api::player::PlayerData;
use skia_safe::Surface;

use crate::text::{self, minecraft_text};

const HEADING: [text::MinecraftText; 3] = minecraft_text!("§c§lHello, world! §1This is §ocool!");

pub fn apply(surface: &mut Surface, data: &PlayerData) {
	text::draw_minecraft_text(surface, HEADING.iter())
}
