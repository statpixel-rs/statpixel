#![allow(dead_code)]

use poise::serenity_prelude as serenity;

pub const COOKIE: &str = "<:cookie:1115091335565811775>";
pub const SPYGLASS: &str = "<:spyglass:1115091333657411625>";
pub const RECOVERY_COMPASS: &str = "<a:recovery_compass:1115091332680126504>";
pub const CLOCK: &str = "<a:clock:1115091329958019253>";
pub const BOOK_AND_QUILL: &str = "<:book_and_quill:1115091331379900428>";
pub const GOLD_INGOT: &str = "<:gold_ingot:741662819031973970>";

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

pub const ARROW_START: serenity::ReactionType = serenity::ReactionType::Custom {
	id: serenity::EmojiId::new(1_153_009_086_934_286_447),
	animated: false,
	name: None,
};

pub const ARROW_END: serenity::ReactionType = serenity::ReactionType::Custom {
	id: serenity::EmojiId::new(1_153_009_104_588_120_164),
	animated: false,
	name: None,
};

pub const SEARCH: serenity::ReactionType = serenity::ReactionType::Custom {
	id: serenity::EmojiId::new(1_153_009_113_488_437_318),
	animated: false,
	name: None,
};
