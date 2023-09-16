#![allow(dead_code)]

use poise::serenity_prelude as serenity;

pub const COOKIE: &str = "<:cookie:1110754829392953364>";
pub const SPYGLASS: &str = "<:spyglass:1110754818433228991>";
pub const RECOVERY_COMPASS: &str = "<a:recovery_compass:1110755004463198218>";
pub const CLOCK: &str = "<a:clock:1110754973794451557>";
pub const BOOK_AND_QUILL: &str = "<:book_and_quill:1110754805724479569>";
pub const GOLD_INGOT: &str = "<:gold_ingot:1112803224307630212>";

pub const ARROW_LEFT: serenity::ReactionType = serenity::ReactionType::Custom {
	id: serenity::EmojiId::new(1_152_732_951_990_251_630),
	animated: false,
	name: None,
};

pub const ARROW_RIGHT: serenity::ReactionType = serenity::ReactionType::Custom {
	id: serenity::EmojiId::new(1_152_732_657_592_057_898),
	animated: false,
	name: None,
};
