use api::player::data::PlayerData;
use minecraft::{
	calc, minecraft_text,
	paint::{self, MinecraftPaint},
	text::Text,
};
use skia_safe::Surface;
use translate::{tr, Context};

use super::{apply_data, apply_extras, apply_item, apply_item_float, apply_label};

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
				size: None,
			},
		],
	);

	apply_data(
		ctx,
		surface,
		stats.level_fmt.as_deref().unwrap_or("§71"),
		calc::skywars::get_level_progress(stats.xp),
		calc::skywars::get_curr_level_xp(stats.xp),
		calc::skywars::get_level_xp(stats.xp),
		&calc::skywars::get_colours(calc::skywars::get_level(stats.xp)),
	);

	apply_item(
		ctx,
		surface,
		wins,
		&tr!(ctx, "wins"),
		MinecraftPaint::Green,
		0,
	);
	apply_item(
		ctx,
		surface,
		losses,
		&tr!(ctx, "losses"),
		MinecraftPaint::Red,
		1,
	);
	apply_item_float(
		ctx,
		surface,
		wins as f32 / if losses == 0 { 1. } else { losses as f32 },
		&tr!(ctx, "wlr"),
		MinecraftPaint::Gold,
		2,
	);

	apply_item(
		ctx,
		surface,
		kills,
		&tr!(ctx, "kills"),
		MinecraftPaint::Green,
		3,
	);
	apply_item(
		ctx,
		surface,
		deaths,
		&tr!(ctx, "deaths"),
		MinecraftPaint::Red,
		4,
	);
	apply_item_float(
		ctx,
		surface,
		kills as f32 / if deaths == 0 { 1. } else { deaths as f32 },
		&tr!(ctx, "kdr"),
		MinecraftPaint::Gold,
		5,
	);

	apply_extras(
		ctx,
		surface,
		&[
			(
				tr!(ctx, "coins"),
				stats.coins,
				paint::MinecraftPaint::Gold,
				None,
			),
			(
				tr!(ctx, "loot-chests"),
				stats.loot_chests,
				paint::MinecraftPaint::Yellow,
				None,
			),
			(
				tr!(ctx, "opals"),
				stats.opals as u32,
				paint::MinecraftPaint::Blue,
				None,
			),
			(
				tr!(ctx, "heads"),
				stats.heads,
				paint::MinecraftPaint::DarkPurple,
				None,
			),
			(
				tr!(ctx, "souls"),
				stats.souls,
				paint::MinecraftPaint::Aqua,
				None,
			),
			(
				tr!(ctx, "tokens"),
				stats.tokens,
				paint::MinecraftPaint::DarkGreen,
				None,
			),
			(
				tr!(ctx, "bow-accuracy"),
				((stats.arrows_hit * 100) as f32 / stats.arrows_shot as f32).round() as u32,
				paint::MinecraftPaint::Red,
				Some('%'),
			),
		],
	)
}
