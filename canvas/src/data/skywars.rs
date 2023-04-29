use api::player::data::PlayerData;
use minecraft::{
	minecraft_text,
	paint::{self, MinecraftPaint},
	text::Text,
};
use skia_safe::Surface;
use translate::{tr, Context};

use crate::{BROKEN_HEART_ICON, MEDAL_ICON, RATIO_ICON, SKULL_ICON, SWORD_ICON};

use super::{apply_extras, apply_item, apply_item_float, apply_label};

pub enum SkyWarsMode {
	Overall,
	SoloNormal,
	SoloInsane,
	TeamNormal,
	TeamInsane,
}

const LABEL: [Text; 2] = minecraft_text!("§b§lSky§d§lWars");

pub fn apply(ctx: Context<'_>, surface: &mut Surface, data: &PlayerData, mode: SkyWarsMode) {
	let stats = &data.stats.sky_wars;

	let (kills, deaths, wins, losses, label) = match mode {
		SkyWarsMode::Overall => (
			stats.overall.kills,
			stats.overall.deaths,
			stats.overall.wins,
			stats.overall.losses,
			tr!(ctx, "Overall"),
		),
		SkyWarsMode::SoloNormal => (
			stats.solo_normal.kills,
			stats.solo_normal.deaths,
			stats.solo_normal.wins,
			stats.solo_normal.losses,
			tr!(ctx, "SoloNormal"),
		),
		SkyWarsMode::SoloInsane => (
			stats.solo_insane.kills,
			stats.solo_insane.deaths,
			stats.solo_insane.wins,
			stats.solo_insane.losses,
			tr!(ctx, "SoloInsane"),
		),
		SkyWarsMode::TeamNormal => (
			stats.team_normal.kills,
			stats.team_normal.deaths,
			stats.team_normal.wins,
			stats.team_normal.losses,
			tr!(ctx, "TeamNormal"),
		),
		SkyWarsMode::TeamInsane => (
			stats.team_insane.kills,
			stats.team_insane.deaths,
			stats.team_insane.wins,
			stats.team_insane.losses,
			tr!(ctx, "TeamInsane"),
		),
	};

	apply_label(
		surface,
		&[
			LABEL[0],
			LABEL[1],
			Text {
				text: &format!(" ({label})"),
				paint: paint::MinecraftPaint::White,
				font: minecraft::style::MinecraftFont::Normal,
			},
		],
	);

	apply_item(&ctx, surface, kills, SWORD_ICON, MinecraftPaint::Green, 0);
	apply_item(&ctx, surface, deaths, SKULL_ICON, MinecraftPaint::Red, 1);
	apply_item_float(
		surface,
		kills as f32 / if deaths == 0 { 1. } else { deaths as f32 },
		RATIO_ICON,
		MinecraftPaint::Gold,
		2,
	);
	apply_item(&ctx, surface, wins, MEDAL_ICON, MinecraftPaint::Green, 3);
	apply_item(
		&ctx,
		surface,
		losses,
		BROKEN_HEART_ICON,
		MinecraftPaint::Red,
		4,
	);
	apply_item_float(
		surface,
		wins as f32 / if losses == 0 { 1. } else { losses as f32 },
		RATIO_ICON,
		MinecraftPaint::Gold,
		5,
	);

	apply_extras(
		ctx,
		surface,
		[
			(tr!(ctx, "coins"), stats.coins, paint::MinecraftPaint::Gold),
			(
				tr!(ctx, "loot-chests"),
				stats.loot_chests,
				paint::MinecraftPaint::Yellow,
			),
			(
				tr!(ctx, "opals"),
				stats.opals as u32,
				paint::MinecraftPaint::Blue,
			),
			(
				tr!(ctx, "heads"),
				stats.heads,
				paint::MinecraftPaint::DarkPurple,
			),
			(tr!(ctx, "souls"), stats.souls, paint::MinecraftPaint::Aqua),
			(
				tr!(ctx, "tokens"),
				stats.tokens,
				paint::MinecraftPaint::DarkGreen,
			),
			(
				tr!(ctx, "most-kills"),
				stats.highest_kill_game as u32,
				paint::MinecraftPaint::Red,
			),
		],
	)
}
