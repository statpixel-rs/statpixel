use api::player::data::PlayerData;
use minecraft::{
	minecraft_text,
	paint::MinecraftPaint,
	text::{measure_minecraft_text_ref, MinecraftText},
};
use num_format::ToFormattedString;
use skia_safe::Surface;

use crate::{BROKEN_HEART_ICON, MEDAL_ICON, RATIO_ICON, SKULL_ICON, SWORD_ICON};

use super::{apply_extras, apply_item, apply_item_float, apply_label};

pub enum SkyWarsMode {
	Overall,
	SoloNormal,
	SoloInsane,
	TeamNormal,
	TeamInsane,
}

const OVERALL: [MinecraftText; 2] = minecraft_text!("§e§lSkyWars §f(Overall)");
const SOLO_NORMAL: [MinecraftText; 4] = minecraft_text!("§e§lSkyWars §f(Solo §aNormal§f)");
const SOLO_INSANE: [MinecraftText; 4] = minecraft_text!("§e§lSkyWars §f(Solo §cInsane§f)");
const TEAM_NORMAL: [MinecraftText; 4] = minecraft_text!("§e§lSkyWars §f(Team §aNormal§f)");
const TEAM_INSANE: [MinecraftText; 4] = minecraft_text!("§e§lSkyWars §f(Team §cInsane§f)");

pub fn apply(surface: &mut Surface, data: &PlayerData, mode: SkyWarsMode) {
	let stats = &data.stats.sky_wars;

	let (kills, deaths, wins, losses, label, width) = match mode {
		SkyWarsMode::Overall => (
			stats.overall.kills,
			stats.overall.deaths,
			stats.overall.wins,
			stats.overall.losses,
			OVERALL.iter(),
			measure_minecraft_text_ref(OVERALL.iter(), 20.),
		),
		SkyWarsMode::SoloNormal => (
			stats.solo_normal.kills,
			stats.solo_normal.deaths,
			stats.solo_normal.wins,
			stats.solo_normal.losses,
			SOLO_NORMAL.iter(),
			measure_minecraft_text_ref(SOLO_NORMAL.iter(), 20.),
		),
		SkyWarsMode::SoloInsane => (
			stats.solo_insane.kills,
			stats.solo_insane.deaths,
			stats.solo_insane.wins,
			stats.solo_insane.losses,
			SOLO_INSANE.iter(),
			measure_minecraft_text_ref(SOLO_INSANE.iter(), 20.),
		),
		SkyWarsMode::TeamNormal => (
			stats.team_normal.kills,
			stats.team_normal.deaths,
			stats.team_normal.wins,
			stats.team_normal.losses,
			TEAM_NORMAL.iter(),
			measure_minecraft_text_ref(TEAM_NORMAL.iter(), 20.),
		),
		SkyWarsMode::TeamInsane => (
			stats.team_insane.kills,
			stats.team_insane.deaths,
			stats.team_insane.wins,
			stats.team_insane.losses,
			TEAM_INSANE.iter(),
			measure_minecraft_text_ref(TEAM_INSANE.iter(), 20.),
		),
	};

	apply_label(surface, label, width);

	apply_item(surface, kills, SWORD_ICON, MinecraftPaint::Green, 0);
	apply_item(surface, deaths, SKULL_ICON, MinecraftPaint::Red, 1);
	apply_item_float(
		surface,
		kills as f32 / if deaths == 0 { 1. } else { deaths as f32 },
		RATIO_ICON,
		MinecraftPaint::Yellow,
		2,
	);
	apply_item(surface, wins, MEDAL_ICON, MinecraftPaint::Green, 3);
	apply_item(surface, losses, BROKEN_HEART_ICON, MinecraftPaint::Red, 4);
	apply_item_float(
		surface,
		wins as f32 / if losses == 0 { 1. } else { losses as f32 },
		RATIO_ICON,
		MinecraftPaint::Yellow,
		5,
	);

	apply_extras(
		surface,
		[
			[
				MinecraftText {
					text: "• Coins: ",
					paint: MinecraftPaint::White,
					font: minecraft::font::MinecraftFont::Normal,
				},
				MinecraftText {
					text: &stats.coins.to_formatted_string(&num_format::Locale::en),
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
