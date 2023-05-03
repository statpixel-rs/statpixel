use api::player::data::PlayerData;
use minecraft::{
	calc, minecraft_text,
	paint::{self, MinecraftPaint},
	text::Text,
};
use skia_safe::Surface;
use translate::{tr, Context};

use super::{apply_data, apply_extras, apply_item, apply_item_float, apply_label};

pub enum ArenaMode {
	Overall,
	Solo,
	Double,
	Four,
}

const LABEL: [Text; 1] = minecraft_text!("§d§lArena");

pub fn apply(ctx: Context<'_>, surface: &mut Surface, data: &PlayerData, mode: ArenaMode) {
	let stats = &data.stats.arena;

	let (kills, deaths, wins, losses, label) = match mode {
		ArenaMode::Overall => (
			stats.solo.kills + stats.double.kills + stats.four.kills,
			stats.solo.deaths + stats.double.deaths + stats.four.deaths,
			stats.solo.wins + stats.double.wins + stats.four.wins,
			stats.solo.losses + stats.double.losses + stats.four.losses,
			tr!(ctx, "Overall"),
		),
		ArenaMode::Solo => (
			stats.solo.kills,
			stats.solo.deaths,
			stats.solo.wins,
			stats.solo.losses,
			tr!(ctx, "Solo"),
		),
		ArenaMode::Double => (
			stats.double.kills,
			stats.double.deaths,
			stats.double.wins,
			stats.double.losses,
			tr!(ctx, "Double"),
		),
		ArenaMode::Four => (
			stats.four.kills,
			stats.four.deaths,
			stats.four.wins,
			stats.four.losses,
			tr!(ctx, "Four"),
		),
	};

	apply_label(
		surface,
		&[
			LABEL[0],
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
		"§71",
		calc::skywars::get_level_progress(0),
		calc::skywars::get_curr_level_xp(0),
		calc::skywars::get_level_xp(0),
		&calc::skywars::get_colours(calc::skywars::get_level(0)),
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
				tr!(ctx, "magical-keys"),
				stats.magical_keys,
				paint::MinecraftPaint::Gold,
				None,
			),
			(
				tr!(ctx, "magical-chests"),
				stats.magical_chests,
				paint::MinecraftPaint::DarkPurple,
				None,
			),
		],
	)
}
