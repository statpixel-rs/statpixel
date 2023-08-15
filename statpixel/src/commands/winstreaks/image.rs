use std::borrow::Cow;

use api::{
	canvas::{self, body::Body, label::ToFormatted, shape, text, Canvas},
	game::{self, mode::Mode, r#type::Type},
	Data, Session,
};
use minecraft::{
	calc::network,
	paint::Paint,
	style::MinecraftFont,
	text::{parse::minecraft_text, Text},
};
use skia_safe::textlayout::TextAlign;
use translate::context;

const LABEL: [Text; 1] = minecraft_text("§6§lWin Streaks");

struct WinStreak {
	game: game::r#type::Type,
	mode: game::mode::Mode,
	streak: u32,
}

#[allow(clippy::too_many_lines)]
pub fn winstreaks(
	ctx: &context::Context<'_>,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
	background: Option<skia_safe::Color>,
) -> Cow<'static, [u8]> {
	let status = shape::Status(session, skin);
	let level = network::get_level(data.xp);
	let progress = shape::WideBubbleProgress(
		network::get_level_progress(data.xp),
		network::get_colours(level),
		false,
	);

	let mut win_streaks = vec![
		WinStreak {
			game: Type::Arena,
			mode: Mode::Solo,
			streak: data.stats.arena.solo.win_streak,
		},
		WinStreak {
			game: Type::Arena,
			mode: Mode::Doubles,
			streak: data.stats.arena.double.win_streak,
		},
		WinStreak {
			game: Type::Arena,
			mode: Mode::Four,
			streak: data.stats.arena.four.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::Overall,
			streak: data.stats.bed_wars.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::Solo,
			streak: data.stats.bed_wars.solo.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::Doubles,
			streak: data.stats.bed_wars.double.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::Three,
			streak: data.stats.bed_wars.three.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::Four,
			streak: data.stats.bed_wars.four.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::SoloRush,
			streak: data.stats.bed_wars.solo_rush.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::RushDoubles,
			streak: data.stats.bed_wars.double_rush.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::RushFour,
			streak: data.stats.bed_wars.four_rush.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::UltimateMeme,
			streak: data.stats.bed_wars.solo_ultimate.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::UltimateDoubles,
			streak: data.stats.bed_wars.double_ultimate.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::UltimateFour,
			streak: data.stats.bed_wars.four_ultimate.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::Castle,
			streak: data.stats.bed_wars.castle.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::LuckyBlockDoubles,
			streak: data.stats.bed_wars.double_lucky.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::LuckyBlockFour,
			streak: data.stats.bed_wars.four_lucky.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::VoidlessDoubles,
			streak: data.stats.bed_wars.double_voidless.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::VoidlessFour,
			streak: data.stats.bed_wars.four_voidless.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::ArmedDoubles,
			streak: data.stats.bed_wars.double_armed.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::ArmedFour,
			streak: data.stats.bed_wars.four_armed.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::UnderworldDoubles,
			streak: data.stats.bed_wars.double_underworld.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::UnderworldFour,
			streak: data.stats.bed_wars.four_underworld.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::SwappageDoubles,
			streak: data.stats.bed_wars.double_swap.win_streak,
		},
		WinStreak {
			game: Type::BedWars,
			mode: Mode::SwappageFour,
			streak: data.stats.bed_wars.four_swap.win_streak,
		},
		WinStreak {
			game: Type::SkyWars,
			mode: Mode::Overall,
			streak: data.stats.sky_wars.win_streak,
		},
		WinStreak {
			game: Type::SkyWars,
			mode: Mode::Solo,
			streak: data.stats.sky_wars.solo_normal.win_streak,
		},
		WinStreak {
			game: Type::SkyWars,
			mode: Mode::Team,
			streak: data.stats.sky_wars.team_normal.win_streak,
		},
		WinStreak {
			game: Type::SkyWars,
			mode: Mode::LabSolo,
			streak: data.stats.sky_wars.solo_lab.win_streak,
		},
		WinStreak {
			game: Type::SkyWars,
			mode: Mode::LabTeam,
			streak: data.stats.sky_wars.team_lab.win_streak,
		},
		WinStreak {
			game: Type::SkyWars,
			mode: Mode::Tourney,
			streak: data.stats.sky_wars.tourney.win_streak,
		},
		WinStreak {
			game: Type::Warlords,
			mode: Mode::Overall,
			streak: data.stats.warlords.win_streak,
		},
	];

	win_streaks.sort_unstable_by_key(|g| std::cmp::Reverse(g.streak));
	win_streaks.drain(9..);

	let ctx = &ctx;
	let mut canvas = Canvas::new(720.)
		.gap(7.)
		.push_down(
			&shape::Title,
			shape::Title::from_text(&text::from_data(data, &data.username, suffix)),
		)
		.push_down(&shape::Subtitle, shape::Subtitle::from_text(&LABEL))
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				&network::get_level_format(level),
				&network::get_curr_level_xp(data.xp),
				&network::get_level_xp(data.xp),
			),
		)
		.push_right_start(
			&canvas::shape::Sidebar,
			canvas::body::Body::new(17., None)
				.append_item(
					&::translate::tr!(ctx, "experience"),
					&data.xp.to_formatted(ctx),
					&Paint::Yellow,
				)
				.append_item(
					&::translate::tr!(ctx, "karma"),
					&data.karma.to_formatted(ctx),
					&Paint::LightPurple,
				)
				.append_item(
					&::translate::tr!(ctx, "rewards"),
					&data.rewards.to_formatted(ctx),
					&Paint::Gold,
				)
				.append_item(
					&::translate::tr!(ctx, "friend-requests"),
					&data.friend_requests.to_formatted(ctx),
					&Paint::Green,
				)
				.append_item(
					&::translate::tr!(ctx, "time-played"),
					&data.playtime.to_formatted(ctx),
					&Paint::Gold,
				)
				.append_item(
					&::translate::tr!(ctx, "first-login"),
					&data.first_login.to_formatted(ctx),
					&Paint::Aqua,
				)
				.append_item(
					&::translate::tr!(ctx, "last-login"),
					&data.last_login.to_formatted(ctx),
					&Paint::Blue,
				)
				.build(),
		)
		.push_right_post_draw(&status, Body::from_status(ctx, session));

	for winstreak in &win_streaks {
		canvas = canvas.push_checked(
			&shape::TallBubble,
			Body::new(40., TextAlign::Center)
				.extend_owned(winstreak.game.as_text().iter().map(|t| Text {
					text: t.text,
					paint: t.paint,
					font: t.font,
					size: Some(25.),
				}))
				.extend(&[
					Text {
						text: "\n",
						size: Some(20.),
						..Default::default()
					},
					Text {
						text: winstreak.mode.as_clean_name(),
						size: Some(20.),
						..Default::default()
					},
					Text {
						text: "\n",
						size: Some(20.),
						..Default::default()
					},
					Text {
						text: &winstreak.streak.to_formatted(ctx),
						paint: match winstreak.streak {
							0 => Paint::Gray,
							1..5 => Paint::White,
							5..10 => Paint::Bronze,
							10..25 => Paint::Yellow,
							25..50 => Paint::Gold,
							50..100 => Paint::Red,
							100..250 => Paint::LightPurple,
							250..500 => Paint::DarkRed,
							500..1_000 => Paint::DarkAqua,
							_ => Paint::DarkGreen,
						},
						font: MinecraftFont::Normal,
						size: None,
					},
				])
				.build(),
		);
	}

	let mut surface = canvas.build(None, background).unwrap();

	canvas::to_png(&mut surface).into()
}
