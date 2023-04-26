use api::player::data::PlayerData;
use minecraft::{minecraft_text, paint::MinecraftPaint, text::MinecraftText};
use num_format::ToFormattedString;
use skia_safe::Surface;

use crate::apply_extras;

use super::{apply_item, apply_item_float};

const OVERALL: [MinecraftText; 2] = minecraft_text!("§c§lSkyWars §f(Overall)");
const SOLO_NORMAL: [MinecraftText; 2] = minecraft_text!("§c§lSkyWars §f(Solo Normal)");
const SOLO_INSANE: [MinecraftText; 2] = minecraft_text!("§c§lSkyWars §f(Solo Insane)");
const TEAM_NORMAL: [MinecraftText; 2] = minecraft_text!("§c§lSkyWars §f(Team Normal)");
const TEAM_INSANE: [MinecraftText; 2] = minecraft_text!("§c§lSkyWars §f(Team Insane)");

pub fn apply(surface: &mut Surface, data: &PlayerData) {
	let stats = &data.stats.sky_wars.overall;

	apply_item(surface, stats.kills, "\u{f889}", MinecraftPaint::Green, 0);
	apply_item(surface, stats.deaths, "\u{f89a}", MinecraftPaint::Red, 1);
	apply_item_float(
		surface,
		stats.kills as f32
			/ if stats.deaths == 0 {
				1.
			} else {
				stats.deaths as f32
			},
		"\u{eaf6}",
		MinecraftPaint::Yellow,
		2,
	);
	apply_item(surface, stats.wins, "\u{e7af}", MinecraftPaint::Green, 3);
	apply_item(surface, stats.losses, "\u{eac2}", MinecraftPaint::Red, 4);
	apply_item_float(
		surface,
		stats.wins as f32
			/ if stats.losses == 0 {
				1.
			} else {
				stats.losses as f32
			},
		"\u{eaf6}",
		MinecraftPaint::Yellow,
		5,
	);

	apply_extras(
		surface,
		[
			[
				MinecraftText {
					text: "• Coins:",
					paint: MinecraftPaint::White,
					font: minecraft::font::MinecraftFont::Normal,
				},
				MinecraftText {
					text: &data
						.stats
						.sky_wars
						.coins
						.to_formatted_string(&num_format::Locale::en),
					paint: MinecraftPaint::Gold,
					font: minecraft::font::MinecraftFont::Normal,
				},
			]
			.iter(),
			[MinecraftText {
				text: "A",
				paint: MinecraftPaint::White,
				font: minecraft::font::MinecraftFont::Normal,
			}]
			.iter(),
			[MinecraftText {
				text: "A",
				paint: MinecraftPaint::White,
				font: minecraft::font::MinecraftFont::Normal,
			}]
			.iter(),
			[MinecraftText {
				text: "A",
				paint: MinecraftPaint::White,
				font: minecraft::font::MinecraftFont::Normal,
			}]
			.iter(),
			[MinecraftText {
				text: "A",
				paint: MinecraftPaint::White,
				font: minecraft::font::MinecraftFont::Normal,
			}]
			.iter(),
			[MinecraftText {
				text: "A",
				paint: MinecraftPaint::White,
				font: minecraft::font::MinecraftFont::Normal,
			}]
			.iter(),
			[MinecraftText {
				text: "A",
				paint: MinecraftPaint::White,
				font: minecraft::font::MinecraftFont::Normal,
			}]
			.iter(),
		],
	)
}
