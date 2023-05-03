use api::player::data::PlayerData;
use minecraft::{
	calc, minecraft_text,
	paint::{self, MinecraftPaint},
	text::Text,
};
use skia_safe::Surface;
use translate::{tr, Context};

use super::{apply_data, apply_extras, apply_item, apply_item_float, apply_label};

pub enum BedWarsMode {
	Overall,
	Solo,
	Double,
	Three,
	Four,
}

const LABEL: [Text; 2] = minecraft_text!("§c§lBed§d§lWars");

pub fn apply(ctx: Context<'_>, surface: &mut Surface, data: &PlayerData, mode: BedWarsMode) {
	let stats = &data.stats.bed_wars;

	let (
		wins,
		losses,
		kills,
		deaths,
		f_kills,
		f_deaths,
		beds_broken,
		beds_lost,
		iron,
		gold,
		diamond,
		emerald,
		items,
		label,
	) = match mode {
		BedWarsMode::Overall => (
			stats.overall.wins,
			stats.overall.losses,
			stats.overall.kills,
			stats.overall.deaths,
			stats.overall.final_kills,
			stats.overall.final_deaths,
			stats.overall.beds_broken,
			stats.overall.beds_lost,
			stats.overall.iron_collected,
			stats.overall.gold_collected,
			stats.overall.diamond_collected,
			stats.overall.emerald_collected,
			stats.overall.items_purchased,
			tr!(ctx, "Overall"),
		),
		BedWarsMode::Solo => (
			stats.solo.wins,
			stats.solo.losses,
			stats.solo.kills,
			stats.solo.deaths,
			stats.solo.final_kills,
			stats.solo.final_deaths,
			stats.solo.beds_broken,
			stats.solo.beds_lost,
			stats.solo.iron_collected,
			stats.solo.gold_collected,
			stats.solo.diamond_collected,
			stats.solo.emerald_collected,
			stats.solo.items_purchased,
			tr!(ctx, "Solo"),
		),
		BedWarsMode::Double => (
			stats.double.wins,
			stats.double.losses,
			stats.double.kills,
			stats.double.deaths,
			stats.double.final_kills,
			stats.double.final_deaths,
			stats.double.beds_broken,
			stats.double.beds_lost,
			stats.double.iron_collected,
			stats.double.gold_collected,
			stats.double.diamond_collected,
			stats.double.emerald_collected,
			stats.double.items_purchased,
			tr!(ctx, "Double"),
		),
		BedWarsMode::Three => (
			stats.three.wins,
			stats.three.losses,
			stats.three.kills,
			stats.three.deaths,
			stats.three.final_kills,
			stats.three.final_deaths,
			stats.three.beds_broken,
			stats.three.beds_lost,
			stats.three.iron_collected,
			stats.three.gold_collected,
			stats.three.diamond_collected,
			stats.three.emerald_collected,
			stats.three.items_purchased,
			tr!(ctx, "Three"),
		),
		BedWarsMode::Four => (
			stats.four.wins,
			stats.four.losses,
			stats.four.kills,
			stats.four.deaths,
			stats.four.final_kills,
			stats.four.final_deaths,
			stats.four.beds_broken,
			stats.four.beds_lost,
			stats.four.iron_collected,
			stats.four.gold_collected,
			stats.four.diamond_collected,
			stats.four.emerald_collected,
			stats.four.items_purchased,
			tr!(ctx, "Four"),
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
		&calc::bedwars::get_level_format(calc::bedwars::get_level(stats.xp)),
		calc::bedwars::get_level_progress(stats.xp),
		calc::bedwars::get_curr_level_xp(stats.xp),
		calc::bedwars::get_level_xp(stats.xp),
		&calc::bedwars::get_colours(calc::bedwars::get_level(stats.xp)),
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

	apply_item(
		ctx,
		surface,
		f_kills,
		&tr!(ctx, "final-kills"),
		MinecraftPaint::Green,
		6,
	);
	apply_item(
		ctx,
		surface,
		f_deaths,
		&tr!(ctx, "final-deaths"),
		MinecraftPaint::Red,
		7,
	);
	apply_item_float(
		ctx,
		surface,
		f_kills as f32 / if f_deaths == 0 { 1. } else { f_deaths as f32 },
		&tr!(ctx, "fkdr"),
		MinecraftPaint::Gold,
		8,
	);

	apply_item(
		ctx,
		surface,
		beds_broken,
		&tr!(ctx, "beds-broken"),
		MinecraftPaint::Green,
		9,
	);
	apply_item(
		ctx,
		surface,
		beds_lost,
		&tr!(ctx, "beds-lost"),
		MinecraftPaint::Red,
		10,
	);
	apply_item_float(
		ctx,
		surface,
		beds_broken as f32 / if beds_lost == 0 { 1. } else { beds_lost as f32 },
		&tr!(ctx, "bblr"),
		MinecraftPaint::Gold,
		11,
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
				tr!(ctx, "iron-collected"),
				iron,
				paint::MinecraftPaint::Gray,
				None,
			),
			(
				tr!(ctx, "gold-collected"),
				gold,
				paint::MinecraftPaint::Gold,
				None,
			),
			(
				tr!(ctx, "diamond-collected"),
				diamond,
				paint::MinecraftPaint::Aqua,
				None,
			),
			(
				tr!(ctx, "emerald-collected"),
				emerald,
				paint::MinecraftPaint::DarkGreen,
				None,
			),
			(
				tr!(ctx, "items-purchased"),
				items,
				paint::MinecraftPaint::Red,
				None,
			),
		],
	)
}
