use api::player::data::PlayerData;
use minecraft::{
	minecraft_text,
	paint::{self, MinecraftPaint},
	text::Text,
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

const OVERALL: [Text; 3] = minecraft_text!("§b§lSky§d§lWars §f(Overall)");
const SOLO_NORMAL: [Text; 5] = minecraft_text!("§b§lSky§d§lWars §f(Solo §aNormal§f)");
const SOLO_INSANE: [Text; 5] = minecraft_text!("§b§lSky§d§lWars §f(Solo §cInsane§f)");
const TEAM_NORMAL: [Text; 5] = minecraft_text!("§b§lSky§d§lWars §f(Team §aNormal§f)");
const TEAM_INSANE: [Text; 5] = minecraft_text!("§b§lSky§d§lWars §f(Team §cInsane§f)");

type SkyWarsData<'a> = (
	u32,            // kills
	u32,            // deaths
	u32,            // wins
	u32,            // losses
	&'a [Text<'a>], // label
);

pub fn apply(surface: &mut Surface, data: &PlayerData, mode: SkyWarsMode) {
	let stats = &data.stats.sky_wars;

	let (kills, deaths, wins, losses, label): SkyWarsData = match mode {
		SkyWarsMode::Overall => (
			stats.overall.kills,
			stats.overall.deaths,
			stats.overall.wins,
			stats.overall.losses,
			&OVERALL,
		),
		SkyWarsMode::SoloNormal => (
			stats.solo_normal.kills,
			stats.solo_normal.deaths,
			stats.solo_normal.wins,
			stats.solo_normal.losses,
			&SOLO_NORMAL,
		),
		SkyWarsMode::SoloInsane => (
			stats.solo_insane.kills,
			stats.solo_insane.deaths,
			stats.solo_insane.wins,
			stats.solo_insane.losses,
			&SOLO_INSANE,
		),
		SkyWarsMode::TeamNormal => (
			stats.team_normal.kills,
			stats.team_normal.deaths,
			stats.team_normal.wins,
			stats.team_normal.losses,
			&TEAM_NORMAL,
		),
		SkyWarsMode::TeamInsane => (
			stats.team_insane.kills,
			stats.team_insane.deaths,
			stats.team_insane.wins,
			stats.team_insane.losses,
			&TEAM_INSANE,
		),
	};

	apply_label(surface, label);

	apply_item(surface, kills, SWORD_ICON, MinecraftPaint::Green, 0);
	apply_item(surface, deaths, SKULL_ICON, MinecraftPaint::Red, 1);
	apply_item_float(
		surface,
		kills as f32 / if deaths == 0 { 1. } else { deaths as f32 },
		RATIO_ICON,
		MinecraftPaint::Gold,
		2,
	);
	apply_item(surface, wins, MEDAL_ICON, MinecraftPaint::Green, 3);
	apply_item(surface, losses, BROKEN_HEART_ICON, MinecraftPaint::Red, 4);
	apply_item_float(
		surface,
		wins as f32 / if losses == 0 { 1. } else { losses as f32 },
		RATIO_ICON,
		MinecraftPaint::Gold,
		5,
	);

	apply_extras(
		surface,
		[
			&[
				Text {
					text: "• Coins: ",
					paint: paint::MinecraftPaint::White,
					font: minecraft::style::MinecraftFont::Normal,
				},
				Text {
					text: &stats.coins.to_formatted_string(&num_format::Locale::en),
					paint: paint::MinecraftPaint::Gold,
					font: minecraft::style::MinecraftFont::Normal,
				},
			],
			&[Text {
				text: "A",
				paint: paint::MinecraftPaint::White,
				font: minecraft::style::MinecraftFont::Normal,
			}],
			&[Text {
				text: "A",
				paint: paint::MinecraftPaint::White,
				font: minecraft::style::MinecraftFont::Normal,
			}],
			&[Text {
				text: "A",
				paint: paint::MinecraftPaint::White,
				font: minecraft::style::MinecraftFont::Normal,
			}],
			&[Text {
				text: "A",
				paint: paint::MinecraftPaint::White,
				font: minecraft::style::MinecraftFont::Normal,
			}],
			&[Text {
				text: "A",
				paint: paint::MinecraftPaint::White,
				font: minecraft::style::MinecraftFont::Normal,
			}],
			&[Text {
				text: "A",
				paint: paint::MinecraftPaint::White,
				font: minecraft::style::MinecraftFont::Normal,
			}],
		],
	)
}
