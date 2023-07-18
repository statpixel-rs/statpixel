use std::borrow::Cow;

use api::{
	builder::{LevelKind, Location, Shape, ShapeData, Statistic},
	canvas::{self, body::Body, text, Canvas},
	image::Image,
	shape, Data, Session,
};
use minecraft::{calc, text::Text};
use translate::{context, tr};

#[allow(clippy::too_many_lines)]
pub fn build(
	ctx: &context::Context<'_>,
	shapes: &Vec<Shape>,
	data: &Data,
	session: &Session,
	skin: &Image,
) -> Cow<'static, [u8]> {
	let mut canvas = Canvas::new(750.);
	let skin = shape::Status(session, skin.image());

	for shape in shapes {
		match shape.data {
			ShapeData::Title => {
				let kind = shape::Title;
				let body = shape::Title::from_text(&text::from_data(data, &data.username, None));

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down(&kind, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start(&kind, body);
					}
					Location::Right => {
						canvas = canvas.push_right(&kind, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start(&kind, body);
					}
				}
			}
			ShapeData::Skin => {
				let body = Body::from_status(ctx, session);

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down_post_draw(&skin, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start_post_draw(&skin, body);
					}
					Location::Right => {
						canvas = canvas.push_right_post_draw(&skin, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start_post_draw(&skin, body);
					}
				}
			}
			ShapeData::Level { ref kind } => {
				let (level, current, needed, progress, colours) = match kind {
					LevelKind::BedWars => {
						let xp = calc::bed_wars::convert(&data.stats.bed_wars.xp);
						let level = calc::bed_wars::get_level(xp);
						let current = calc::bed_wars::get_curr_level_xp(xp);
						let needed = calc::bed_wars::get_level_xp(xp);
						let format = calc::bed_wars::get_level_format(level);
						let progress = calc::bed_wars::get_level_progress(xp);
						let colours = calc::bed_wars::get_colours(level);

						(format, current, needed, progress, colours)
					}
					LevelKind::BuildBattle => {
						let xp = calc::build_battle::convert(&data.stats.build_battle.score);
						let level = calc::build_battle::get_level(xp);
						let current = calc::build_battle::get_curr_level_xp(xp);
						let needed = calc::build_battle::get_level_xp(xp);
						let format = calc::build_battle::get_level_format(level);
						let progress = calc::build_battle::get_level_progress(xp);
						let colours = calc::build_battle::get_colours(level);

						(
							format,
							u64::from(current),
							u64::from(needed),
							progress,
							colours,
						)
					}
					LevelKind::Duels => {
						let xp = calc::duels::overall::convert(
							&(data.stats.duels.arena_solo.wins
								+ data.stats.duels.blitz_solo.wins
								+ data.stats.duels.bow_solo.wins + data
								.stats
								.duels
								.bow_spleef_solo
								.wins + data.stats.duels.boxing_solo.wins
								+ data.stats.duels.bridge_double.wins
								+ data.stats.duels.bridge_double_duel.wins
								+ data.stats.duels.bridge_four.wins
								+ data.stats.duels.bridge_solo.wins
								+ data.stats.duels.bridge_three.wins
								+ data.stats.duels.bridge_three_duel.wins
								+ data.stats.duels.capture_three.wins
								+ data.stats.duels.classic_solo.wins
								+ data.stats.duels.combo_solo.wins
								+ data.stats.duels.mega_walls_solo.wins
								+ data.stats.duels.op_double.wins
								+ data.stats.duels.op_solo.wins + data.stats.duels.parkour.wins
								+ data.stats.duels.potion_solo.wins
								+ data.stats.duels.sky_wars_double.wins
								+ data.stats.duels.sky_wars_solo.wins
								+ data.stats.duels.sky_wars_tournament.wins
								+ data.stats.duels.sumo_solo.wins
								+ data.stats.duels.sumo_tournament.wins
								+ data.stats.duels.uhc_double.wins
								+ data.stats.duels.uhc_four.wins + data.stats.duels.uhc_meetup.wins
								+ data.stats.duels.uhc_solo.wins),
						);
						let level = calc::duels::overall::get_level(xp);
						let current = calc::duels::overall::get_curr_level_xp(xp);
						let needed = calc::duels::overall::get_level_xp(xp);
						let format = calc::duels::overall::get_level_format(level);
						let progress = calc::duels::overall::get_level_progress(xp);
						let colours = calc::duels::overall::get_colours(level);

						(
							format,
							u64::from(current),
							u64::from(needed),
							progress,
							colours,
						)
					}
					LevelKind::Network => {
						let xp = calc::network::convert(&data.xp);
						let level = calc::network::get_level(xp);
						let current = calc::network::get_curr_level_xp(xp);
						let needed = calc::network::get_level_xp(xp);
						let format = calc::network::get_level_format(level);
						let progress = calc::network::get_level_progress(xp);
						let colours = calc::network::get_colours(level);

						(format, current, needed, progress, colours)
					}
					LevelKind::Pit => {
						let xp = calc::pit::convert(&data.stats.pit.profile.level);
						let level = calc::pit::get_level(xp);
						let current = calc::pit::get_curr_level_xp(xp);
						let needed = calc::pit::get_level_xp(xp);
						let format = calc::pit::get_level_format(level);
						let progress = calc::pit::get_level_progress(xp);
						let colours = calc::pit::get_colours(level);

						(format, current, needed, progress, colours)
					}
					LevelKind::SkyWars => {
						let xp = calc::sky_wars::convert(&data.stats.sky_wars.xp);
						let level = calc::sky_wars::get_level(xp);
						let current = calc::sky_wars::get_curr_level_xp(xp);
						let needed = calc::sky_wars::get_level_xp(xp);
						let format = data.stats.sky_wars.level_fmt.clone();
						let progress = calc::sky_wars::get_level_progress(xp);
						let colours = calc::sky_wars::get_colours(level);

						(format, current, needed, progress, colours)
					}
					LevelKind::WoolWars => {
						let xp = calc::wool_wars::convert(&data.stats.wool_wars.progression.xp);
						let level = calc::wool_wars::get_level(xp);
						let current = calc::wool_wars::get_curr_level_xp(xp);
						let needed = calc::wool_wars::get_level_xp(xp);
						let format = calc::wool_wars::get_level_format(level);
						let progress = calc::wool_wars::get_level_progress(xp);
						let colours = calc::wool_wars::get_colours(level);

						(format, current, needed, progress, colours)
					}
				};

				let kind = shape::WideBubbleProgress(progress, colours);
				let body =
					shape::WideBubbleProgress::from_level_progress(ctx, &level, &current, &needed);

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down(&kind, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start(&kind, body);
					}
					Location::Right => {
						canvas = canvas.push_right(&kind, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start(&kind, body);
					}
				}
			}
			ShapeData::Subtitle { ref text } => {
				let kind = shape::Subtitle;
				let body = shape::Subtitle::from_text(&[Text {
					text,
					paint: shape.colour,
					..Default::default()
				}]);

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down(&kind, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start(&kind, body);
					}
					Location::Right => {
						canvas = canvas.push_right(&kind, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start(&kind, body);
					}
				}
			}
			ShapeData::Bubble { ref statistic } => {
				use api::player::stats::*;

				let (value, label) = match statistic {
					Statistic::Arcade { kind } => {
						(arcade::Arcade::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::Arena { kind } => {
						(arena::Arena::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::BedWars { kind } => {
						(bed_wars::BedWars::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::BlitzSg { kind } => {
						(blitz_sg::BlitzSg::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::BuildBattle { kind } => (
						build_battle::BuildBattle::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::CopsAndCrims { kind } => (
						cops_and_crims::CopsAndCrims::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Duels { kind } => {
						(duels::Duels::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::MegaWalls { kind } => (
						mega_walls::MegaWalls::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::MurderMystery { kind } => (
						murder_mystery::MurderMystery::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Paintball { kind } => (
						paintball::Paintball::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Pit { kind } => {
						(pit::Pit::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::Quake { kind } => {
						(quake::Quake::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::SkyWars { kind } => {
						(sky_wars::SkyWars::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::SmashHeroes { kind } => (
						smash_heroes::SmashHeroes::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::SpeedUhc { kind } => (
						speed_uhc::SpeedUhc::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::TntGames { kind } => (
						tnt_games::TntGames::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::TurboKartRacers { kind } => (
						turbo_kart_racers::TurboKartRacers::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Uhc { kind } => {
						(uhc::Uhc::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::VampireZ { kind } => (
						vampire_z::VampireZ::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::Walls { kind } => {
						(walls::Walls::from_kind(ctx, data, kind), kind.get_tr())
					}
					Statistic::Warlords { kind } => (
						warlords::Warlords::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
					Statistic::WoolWars { kind } => (
						wool_wars::WoolWars::from_kind(ctx, data, kind),
						kind.get_tr(),
					),
				};

				let kind = shape::Bubble;
				let body = Body::from_bubble_cow(value, tr!(ctx, label).as_ref(), shape.colour);

				match shape.location {
					Location::Down => {
						canvas = canvas.push_down(&kind, body);
					}
					Location::DownStart => {
						canvas = canvas.push_down_start(&kind, body);
					}
					Location::Right => {
						canvas = canvas.push_right(&kind, body);
					}
					Location::RightStart => {
						canvas = canvas.push_right_start(&kind, body);
					}
				}
			}
		};
	}

	canvas::to_png(&mut canvas.build(None, None).unwrap()).into()
}
