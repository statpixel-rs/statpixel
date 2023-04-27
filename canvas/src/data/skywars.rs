use api::player::data::PlayerData;
use minecraft::{paint::MinecraftPaint, text::MinecraftText};
use num_format::ToFormattedString;
use skia_safe::Surface;

use crate::apply_extras;

use super::{apply_item, apply_item_float};

pub enum SkyWarsMode {
	Overall,
	SoloNormal,
	SoloInsane,
	TeamNormal,
	TeamInsane,
}

pub fn apply(surface: &mut Surface, data: &PlayerData, mode: SkyWarsMode) {
	let stats = &data.stats.sky_wars;

	let (kills, deaths, wins, losses) = match mode {
		SkyWarsMode::Overall => (
			stats.overall.kills,
			stats.overall.deaths,
			stats.overall.wins,
			stats.overall.losses,
		),
		SkyWarsMode::SoloNormal => (
			stats.solo_normal.kills,
			stats.solo_normal.deaths,
			stats.solo_normal.wins,
			stats.solo_normal.losses,
		),
		SkyWarsMode::SoloInsane => (
			stats.solo_insane.kills,
			stats.solo_insane.deaths,
			stats.solo_insane.wins,
			stats.solo_insane.losses,
		),
		SkyWarsMode::TeamNormal => (
			stats.team_normal.kills,
			stats.team_normal.deaths,
			stats.team_normal.wins,
			stats.team_normal.losses,
		),
		SkyWarsMode::TeamInsane => (
			stats.team_insane.kills,
			stats.team_insane.deaths,
			stats.team_insane.wins,
			stats.team_insane.losses,
		),
	};

	apply_item(surface, kills, "\u{f889}", MinecraftPaint::Green, 0);
	apply_item(surface, deaths, "\u{f89a}", MinecraftPaint::Red, 1);
	apply_item_float(
		surface,
		kills as f32 / if deaths == 0 { 1. } else { deaths as f32 },
		"\u{eaf6}",
		MinecraftPaint::Yellow,
		2,
	);
	apply_item(surface, wins, "\u{e7af}", MinecraftPaint::Green, 3);
	apply_item(surface, losses, "\u{eac2}", MinecraftPaint::Red, 4);
	apply_item_float(
		surface,
		wins as f32 / if losses == 0 { 1. } else { losses as f32 },
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
