use std::{borrow::Cow, cmp::max, ops::Range, sync::Arc};

use api::{
	canvas::{
		self,
		body::Body,
		chart::{self, BACKGROUND, CANVAS_BACKGROUND, WIDTH_F},
		label::ToFormatted,
		project::{BUBBLE_HEIGHT_I, GAP_I},
		Canvas,
	},
	nbt::inventory::Item,
	player::Player,
	shape::{BUBBLE_HEIGHT, GAP},
	skyblock::{
		self,
		materials::MATERIALS,
		networth::{self, calc::Category},
		profile::{Profile, TransactionAction},
	},
	Data, Session,
};
use canvas::{shape, text};
use chrono::{DateTime, Utc};
use database::schema::{bazaar, bazaar_item};
use diesel::{ExpressionMethods, QueryDsl};
use minecraft::{
	calc::{network, sky_block},
	paint::Paint,
	style::{Family, MinecraftFont},
	text::{
		parse::{minecraft_string, minecraft_text},
		Text,
	},
};
use plotters::{
	prelude::{
		BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, Rectangle,
		SeriesLabelPosition,
	},
	series::LineSeries,
	style::{self, Color, IntoTextStyle, Palette, Palette99, RGBAColor, RGBColor, ShapeStyle},
};
use skia_safe::{textlayout::TextAlign, ClipOp, Color4f, Point, RRect, Rect, Surface};
use translate::{context, prelude::GetChronoLocale, tr, Error};
use uuid::Uuid;

const LABEL: [Text; 2] = minecraft_text("§b§lSky§a§lBlock");
const EMPTY_ROW: &[Option<Item>] = &[None, None, None, None, None, None, None, None, None];

fn round_corners(surface: &mut Surface) {
	let mut rect = RRect::new();

	rect.set_rect_radii(
		Rect::new(0., 0., WIDTH_F, 400. + BUBBLE_HEIGHT * 2. + GAP * 3.),
		&[
			Point::new(30., 30.),
			Point::new(30., 30.),
			Point::new(30., 30.),
			Point::new(30., 30.),
		],
	);

	surface.canvas().clip_rrect(rect, ClipOp::Difference, false);
	surface.canvas().save();
	surface.canvas().clear(Color4f::new(0., 0., 0., 0.));
	surface.canvas().restore();
}

#[allow(clippy::type_complexity)]
fn create<const BUBBLE: bool>(
	ctx: &context::Context<'_>,
	family: Family,
	series: Vec<(Cow<str>, Vec<(DateTime<Utc>, f64)>)>,
	range_x: Range<DateTime<Utc>>,
	range_y: Range<f64>,
	colour: Option<Paint>,
	background: Option<skia_safe::Color>,
) -> Result<Vec<u8>, Error> {
	const PIXELS: usize = 750 * (400 + BUBBLE_HEIGHT_I as usize * 2 + GAP_I as usize * 3);
	const BUF_LEN_RGBA: usize = 4 * PIXELS;

	let foreground = background.map_or(style::colors::WHITE, |c| {
		RGBColor(255 - c.r(), 255 - c.g(), 255 - c.b())
	});
	let background = background.map_or(CANVAS_BACKGROUND, |c| {
		RGBAColor(c.r(), c.g(), c.b(), f64::from(c.a()) / 255.)
	});

	// Allocate a buffer large enough to hold an RGBA representation of the image
	let mut buffer = vec![u8::MAX; BUF_LEN_RGBA];

	// The BitMapBackend uses RGB, so we will need to convert it later
	let backend =
		BitMapBackend::with_buffer(&mut buffer, (750, 400 + BUBBLE_HEIGHT_I * 2 + GAP_I * 3))
			.into_drawing_area();

	backend.fill(&background).map_err(|_| Error::Plotters)?;

	// set start time to `created_at`, and end to last time
	let mut chart = ChartBuilder::on(&backend)
		.margin_top(71 + BUBBLE_HEIGHT_I * 2 + GAP_I * 3)
		.margin_bottom(20)
		.margin_right(30)
		.set_label_area_size(LabelAreaPosition::Left, 90)
		.set_label_area_size(LabelAreaPosition::Bottom, 30)
		.build_cartesian_2d(range_x, range_y)
		.map_err(|_| Error::Plotters)?;

	let locale = ctx.get_chrono_locale();

	chart
		.configure_mesh()
		.y_label_formatter(&|y| y.to_formatted(ctx).into_owned())
		.x_label_formatter(&|x| x.format_localized("%d/%m %H:%M", locale).to_string())
		.x_labels(5)
		.light_line_style(foreground.mix(0.05))
		.bold_line_style(foreground.mix(0.1))
		.axis_style(foreground.mix(0.5))
		.label_style(
			(family.as_str(), 20)
				.into_text_style(&backend)
				.with_color(foreground),
		)
		.draw()
		.map_err(|_| Error::Plotters)?;

	if let Some(colour) = colour {
		let colour: ShapeStyle = colour.as_plotters().into();

		for (name, series) in series {
			chart
				.draw_series(if BUBBLE {
					LineSeries::new(series, colour.filled().stroke_width(2)).point_size(2)
				} else {
					LineSeries::new(series, colour.filled().stroke_width(2))
				})
				.map_err(|_| Error::Plotters)?
				.label(name.into_owned())
				.legend(move |(x, y)| {
					Rectangle::new([(x, y - 5), (x + 10, y + 5)], colour.filled())
				});
		}
	} else {
		for (idx, (name, series)) in series.into_iter().enumerate() {
			let colour = Palette99::pick(idx).mix(0.9);

			chart
				.draw_series(if BUBBLE {
					LineSeries::new(series, colour.filled().stroke_width(2)).point_size(2)
				} else {
					LineSeries::new(series, colour.filled().stroke_width(2))
				})
				.map_err(|_| Error::Plotters)?
				.label(name.into_owned())
				.legend(move |(x, y)| {
					Rectangle::new([(x, y - 5), (x + 10, y + 5)], colour.filled())
				});
		}
	}

	chart
		.configure_series_labels()
		.position(SeriesLabelPosition::UpperLeft)
		.border_style(style::colors::TRANSPARENT)
		.background_style(BACKGROUND.mix(0.6))
		.label_font(
			(family.as_str(), 17)
				.into_text_style(&backend)
				.with_color(style::colors::WHITE),
		)
		.draw()
		.map_err(|_| Error::Plotters)?;

	backend.present().map_err(|_| Error::Plotters)?;

	drop(chart);
	drop(backend);

	// Convert the RGB buffer to RGBA, in place
	for i in (0..PIXELS).rev() {
		buffer.copy_within(i * 3..i * 3 + 3, i * 4);

		// Set fourth channel to max
		buffer[i * 4 + 3] = u8::MAX;
	}

	Ok(buffer)
}

/// (buy, sell)
pub fn apply_bazaar_data(
	ctx: &context::Context<'_>,
	family: Family,
	surface: &mut Surface,
	text: &[Text],
	products: &[(f64, i32, f64, i32, DateTime<Utc>)],
	background: Option<skia_safe::Color>,
) {
	#[allow(clippy::cast_precision_loss)]
	let length = products.len().min(60) as f64;

	let buy_avg_hour = products.iter().take(60).map(|p| p.0).sum::<f64>() / length;
	let sell_avg_hour = products.iter().take(60).map(|p| p.2).sum::<f64>() / length;

	#[allow(clippy::cast_precision_loss)]
	let length = products.len().min(60 * 24) as f64;

	let buy_avg_day = products.iter().take(60 * 24).map(|p| p.0).sum::<f64>() / length;
	let sell_avg_day = products.iter().take(60 * 24).map(|p| p.2).sum::<f64>() / length;

	#[allow(clippy::cast_precision_loss)]
	let length = products.len().min(60 * 24 * 7) as f64;

	let buy_avg_week = products.iter().take(60 * 24 * 7).map(|p| p.0).sum::<f64>() / length;
	let sell_avg_week = products.iter().take(60 * 24 * 7).map(|p| p.2).sum::<f64>() / length;

	Canvas::new(720., family)
		.push_right(&shape::LongTitle, shape::Title::from_text(family, text))
		.push_down_start(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sell_avg_hour,
				tr(ctx, "last-hour").as_ref(),
				Paint::Red,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sell_avg_day,
				tr(ctx, "last-day").as_ref(),
				Paint::Red,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sell_avg_week,
				tr(ctx, "last-week").as_ref(),
				Paint::Red,
			),
		)
		.push_down_start(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&buy_avg_hour,
				tr(ctx, "last-hour").as_ref(),
				Paint::Green,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&buy_avg_day,
				tr(ctx, "last-day").as_ref(),
				Paint::Green,
			),
		)
		.push_right(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&buy_avg_week,
				tr(ctx, "last-week").as_ref(),
				Paint::Green,
			),
		)
		.build_with(surface, None, background);
}

#[allow(clippy::too_many_arguments)]
pub async fn auctions(
	ctx: &context::Context<'_>,
	family: Family,
	player: &Player,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
	background: Option<skia_safe::Color>,
) -> Result<Cow<'static, [u8]>, Error> {
	let auctions = player.get_auctions().await?;
	let status = shape::Status(session, skin);
	let level = network::get_level(data.xp);
	let progress = shape::WideBubbleProgress(
		network::get_level_progress(data.xp),
		network::get_colours(level),
		false,
	);

	let mut canvas = Canvas::new(720., family)
		.gap(7.)
		.push_down(
			&shape::Title,
			shape::Title::from_text(family, &text::from_data(data, &data.username, suffix)),
		)
		.push_down(
			&shape::Subtitle,
			shape::Subtitle::from_label(ctx, family, &LABEL, "player-auctions"),
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&network::get_level_format(level),
				&network::get_curr_level_xp(data.xp),
				&network::get_level_xp(data.xp),
			),
		)
		.push_right_start_post_draw(&status, Body::from_status(ctx, family, session));

	for auction in auctions.iter() {
		let mut text = minecraft_string(&auction.item.name).collect::<Vec<_>>();
		let bid = max(auction.starting_bid, auction.highest_bid);
		let bid = bid.to_formatted(ctx);

		text.extend([
			Text {
				text: "\n",
				size: None,
				..Default::default()
			},
			Text {
				text: bid.as_ref(),
				paint: Paint::Gold,
				font: MinecraftFont::Normal,
				size: Some(30.),
			},
		]);

		canvas = canvas.push_checked(
			&shape::TallBubble,
			Body::build_slice(family, text.as_slice(), 23., TextAlign::Center),
		);
	}

	Ok(canvas::to_png(&mut canvas.build(None, background).unwrap()).into())
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
pub async fn profile(
	ctx: &context::Context<'_>,
	family: Family,
	player: &Player,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
	background: Option<skia_safe::Color>,
	profile_id: Option<Uuid>,
	profile: Option<String>,
) -> Result<(Cow<'static, [u8]>, Arc<Profile>), Error> {
	let Some(profile) = (match (profile_id, profile) {
		(Some(id), ..) => data.stats.sky_block.profiles.iter().find(|p| p.id == id),
		(.., Some(profile)) => data
			.stats
			.sky_block
			.profiles
			.iter()
			.find(|p| p.name == profile),
		(None, None) => data.stats.sky_block.profiles.first(),
	}) else {
		return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
	};

	let name = profile.name.as_str();
	let profile = Player::get_skyblock_profile(profile, &data.username).await?;

	let Some(member) = profile.members.get(&player.uuid) else {
		return Err(Error::MemberPlayerNotFound(data.username.clone()));
	};

	let status = shape::Status(session, skin);
	let level = sky_block::get_level(member.leveling.xp);
	let progress = shape::WideBubbleProgress(
		sky_block::get_level_progress(member.leveling.xp),
		sky_block::get_colours(level),
		false,
	);

	let ctx = &ctx;
	let mut surface = Canvas::new(720., family)
		.gap(7.)
		.push_down(
			&shape::Title,
			shape::Title::from_text(family, &text::from_data(data, &data.username, suffix)),
		)
		.push_down(
			&shape::Subtitle,
			shape::Subtitle::from_label_str(family, &LABEL, name),
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&sky_block::get_level_format(level),
				&sky_block::get_curr_level_xp(member.leveling.xp),
				&sky_block::get_level_xp(member.leveling.xp),
			),
		)
		.push_right_start(
			&canvas::shape::Sidebar,
			canvas::body::Body::new(17., None, family)
				.append_item(
					&::translate::tr(ctx, "coins"),
					&canvas::label::ToFormatted::to_formatted(&member.coin_purse, ctx),
					&Paint::Gold,
				)
				.append_item(
					&::translate::tr(ctx, "fairy-souls"),
					&canvas::label::ToFormatted::to_formatted(&member.fairy_souls_collected, ctx),
					&Paint::Aqua,
				)
				.append_item(
					&::translate::tr(ctx, "fairy-exchanges"),
					&canvas::label::ToFormatted::to_formatted(&member.fairy_exchanges, ctx),
					&Paint::LightPurple,
				)
				.append_item(
					&::translate::tr(ctx, "fishing-treasure"),
					&canvas::label::ToFormatted::to_formatted(&member.fishing_treasure_caught, ctx),
					&Paint::Blue,
				)
				.append_item(
					&::translate::tr(ctx, "zones-visited"),
					&canvas::label::ToFormatted::to_formatted(&member.zones_visited, ctx),
					&Paint::Green,
				)
				.append_item(
					&::translate::tr(ctx, "generators-crafted"),
					&canvas::label::ToFormatted::to_formatted(&member.generators_crafted, ctx),
					&Paint::White,
				)
				.append_item(
					&::translate::tr(ctx, "highest-crit"),
					&canvas::label::ToFormatted::to_formatted(
						&member.stats.highest_critical_damage,
						ctx,
					),
					&Paint::Red,
				)
				.build(),
		)
		.push_right_post_draw(&status, Body::from_status(ctx, family, session))
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.farming),
				tr(ctx, "farming").as_ref(),
				Paint::Gold,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.mining),
				tr(ctx, "mining").as_ref(),
				Paint::Gray,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.combat),
				tr(ctx, "combat").as_ref(),
				Paint::Gray,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.foraging),
				tr(ctx, "foraging").as_ref(),
				Paint::Green,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.fishing),
				tr(ctx, "fishing-skill").as_ref(),
				Paint::White,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.enchanting),
				tr(ctx, "enchanting").as_ref(),
				Paint::DarkPurple,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.alchemy),
				tr(ctx, "alchemy").as_ref(),
				Paint::Yellow,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.taming),
				tr(ctx, "taming").as_ref(),
				Paint::Gray,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_dungeoneering(
					member.dungeons.types.catacombs.experience,
				),
				tr(ctx, "dungeoneering").as_ref(),
				Paint::Gray,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.carpentry),
				tr(ctx, "carpentry").as_ref(),
				Paint::Red,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.runecrafting),
				tr(ctx, "runecrafting").as_ref(),
				Paint::LightPurple,
			),
		)
		.push_checked(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&sky_block::skills::get_level_general(member.skills.social),
				tr(ctx, "social").as_ref(),
				Paint::Green,
			),
		)
		.build(None, background)
		.unwrap();

	Ok((canvas::to_png(&mut surface).into(), profile))
}

pub async fn bank(
	ctx: &context::Context<'_>,
	family: Family,
	data: &Data,
	background: Option<skia_safe::Color>,
	profile_id: Option<Uuid>,
	profile: Option<String>,
) -> Result<(Cow<'static, [u8]>, Arc<Profile>), Error> {
	let Some(profile) = (match (profile_id, profile) {
		(Some(id), ..) => data.stats.sky_block.profiles.iter().find(|p| p.id == id),
		(.., Some(profile)) => data
			.stats
			.sky_block
			.profiles
			.iter()
			.find(|p| p.name == profile),
		(None, None) => data.stats.sky_block.profiles.first(),
	}) else {
		return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
	};

	let profile = Player::get_skyblock_profile(profile, &data.username).await?;

	let mut bank = skyblock::profile::Banking::clone(&profile.banking);

	let (lower, upper) = if bank.transactions.is_empty() {
		(0, 100)
	} else {
		let mut lower = u64::MAX;
		// The upper bound should be at least 100
		let mut upper = 100u64;

		// overwrite the bank transactions and replace the "change" by the total at that time
		for transaction in bank.transactions.iter_mut().rev() {
			match transaction.action {
				TransactionAction::Withdraw => bank.balance += transaction.amount,
				TransactionAction::Deposit => bank.balance -= transaction.amount,
			}

			transaction.amount = bank.balance;

			if transaction.amount < lower {
				lower = transaction.amount;
			}

			if transaction.amount > upper {
				upper = transaction.amount;
			}
		}

		(lower, upper)
	};

	let first = bank
		.transactions
		.first()
		.map_or_else(Utc::now, |t| t.timestamp);

	let last = bank
		.transactions
		.last()
		.map_or_else(Utc::now, |t| t.timestamp);

	let mut buffer = chart::u64::create::<true>(
		ctx,
		family,
		vec![(
			tr(ctx, "bank-balance"),
			bank.transactions
				.iter()
				.map(|t| (t.timestamp, t.amount))
				.collect(),
		)],
		first..last,
		(lower * 7 / 8)..(upper * 8 / 7),
		Some(Paint::Gold),
		background,
	)?;

	let mut surface = chart::canvas(&mut buffer)?;

	chart::apply_title(
		ctx,
		family,
		&mut surface,
		data,
		&[Text {
			text: tr(&ctx, "island-bank-balance").as_ref(),
			paint: Paint::Gold,
			..Default::default()
		}],
		background,
	);
	chart::round_corners(&mut surface);

	Ok((canvas::to_png(&mut surface).into(), profile))
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
pub async fn networth(
	ctx: &context::Context<'_>,
	family: Family,
	player: &Player,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
	background: Option<skia_safe::Color>,
	profile_id: Option<Uuid>,
	profile: Option<String>,
) -> Result<(Cow<'static, [u8]>, Arc<Profile>), Error> {
	let Some(profile) = (match (profile_id, profile) {
		(Some(id), ..) => data.stats.sky_block.profiles.iter().find(|p| p.id == id),
		(.., Some(profile)) => data
			.stats
			.sky_block
			.profiles
			.iter()
			.find(|p| p.name == profile),
		(None, None) => data.stats.sky_block.profiles.first(),
	}) else {
		return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
	};

	let name = profile.name.as_str();
	let profile = Player::get_skyblock_profile(profile, &data.username).await?;

	let Some(member) = profile.members.get(&player.uuid) else {
		return Err(Error::MemberPlayerNotFound(data.username.clone()));
	};

	let prices = networth::prices().await?;
	let networth = member.networth(&prices);

	#[allow(clippy::cast_precision_loss)]
	let purse = member.coin_purse as f64;
	#[allow(clippy::cast_precision_loss)]
	let bank = profile.banking.balance as f64;

	let status = shape::Status(session, skin);
	let level = sky_block::get_level(member.leveling.xp);
	let progress = shape::WideBubbleProgress(
		sky_block::get_level_progress(member.leveling.xp),
		sky_block::get_colours(level),
		false,
	);

	let mut canvas = Canvas::new(720., family)
		.gap(7.)
		.push_down(
			&shape::Title,
			shape::Title::from_text(family, &text::from_data(data, &data.username, suffix)),
		)
		.push_down(
			&shape::Subtitle,
			shape::Subtitle::from_label_str(family, &LABEL, name),
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&sky_block::get_level_format(level),
				&sky_block::get_curr_level_xp(member.leveling.xp),
				&sky_block::get_level_xp(member.leveling.xp),
			),
		)
		.push_right_start(
			&shape::Bubble,
			Body::from_bubble(
				ctx,
				family,
				&(networth.value + bank + purse),
				"Networth",
				Paint::Gold,
			),
		)
		.push_down(
			&shape::Bubble,
			Body::from_bubble(ctx, family, &bank, "Bank", Paint::Gold),
		)
		.push_right_start_post_draw(&status, Body::from_status(ctx, family, session));

	let categories = networth
		.categories
		.iter()
		.map(|c| {
			(
				c,
				c.items
					.iter()
					.map(|s| {
						(
							s,
							shape::NetworthSlot(
								if let Some(v) = if s.id.starts_with("ENCHANTED_") {
									MATERIALS.get(&s.id[10..])
								} else if let Some(idx) = s.id.find(':') {
									MATERIALS
										.get(&format!("{}:{}", &s.id[..idx], s.damage))
										.or_else(|| MATERIALS.get(&s.id))
								} else if s.damage != 0 {
									MATERIALS
										.get(&format!("{}:{}", &s.id, s.damage))
										.or_else(|| MATERIALS.get(&s.id))
								} else {
									MATERIALS.get(&s.id)
								} {
									Some(v.image())
								} else {
									tracing::warn!(id = s.id, "unknown item");

									None
								},
								s.count,
							),
						)
					})
					.collect::<Vec<_>>(),
			)
		})
		.collect::<Vec<_>>();

	let default = (&Category::default(), vec![]);

	for i in categories.chunks(2) {
		let (left, left_items) = i.first().unwrap();
		let (right, right_items) = i.get(1).unwrap_or(&default);

		let rows = left_items.len().max(right_items.len());

		canvas = canvas
			.push_down_start(
				&shape::Title,
				shape::Title::from_category(ctx, family, left),
			)
			.push_right(
				&shape::Title,
				shape::Title::from_category(ctx, family, right),
			);

		for i in 0..rows {
			let left = left_items.get(i);
			let right = right_items.get(i);

			if let Some((item, slot)) = left {
				canvas = canvas
					.push_down_start_post_draw(slot, Body::empty())
					.push_right(
						&shape::NetworthName,
						Body::build_slice(
							family,
							&minecraft_string(&item.name).collect::<Vec<_>>(),
							17.,
							None,
						),
					);
			} else {
				canvas = canvas
					.push_down_start_post_draw(&shape::EmptyNetworthSlot, Body::empty())
					.push_right(&shape::EmptyNetworthName, Body::empty());
			}

			if let Some((item, slot)) = right {
				canvas = canvas.push_right_post_draw(slot, Body::empty()).push_right(
					&shape::NetworthName,
					Body::build_slice(
						family,
						&minecraft_string(&item.name).collect::<Vec<_>>(),
						17.,
						None,
					),
				);
			} else {
				canvas = canvas
					.push_right(&shape::EmptyNetworthSlot, Body::empty())
					.push_right(&shape::EmptyNetworthName, Body::empty());
			}
		}
	}

	let mut surface = canvas.build(None, background).unwrap();

	Ok((canvas::to_png(&mut surface).into(), profile))
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
pub async fn pets(
	ctx: &context::Context<'_>,
	family: Family,
	player: &Player,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
	background: Option<skia_safe::Color>,
	profile_id: Option<Uuid>,
	profile: Option<&str>,
) -> Result<(Cow<'static, [u8]>, Arc<Profile>), Error> {
	let Some(profile) = (match (profile_id, profile) {
		(Some(id), ..) => data.stats.sky_block.profiles.iter().find(|p| p.id == id),
		(.., Some(profile)) => data
			.stats
			.sky_block
			.profiles
			.iter()
			.find(|p| p.name == profile),
		(None, None) => data.stats.sky_block.profiles.first(),
	}) else {
		return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
	};

	let name = profile.name.as_str();
	let profile = Player::get_skyblock_profile(profile, &data.username).await?;

	let Some(member) = profile.members.get(&player.uuid) else {
		return Err(Error::MemberPlayerNotFound(data.username.clone()));
	};

	let default = vec![];
	let items = member.pets.as_ref().unwrap_or(&default);

	let status = shape::Status(session, skin);
	let level = sky_block::get_level(member.leveling.xp);
	let progress = shape::WideBubbleProgress(
		sky_block::get_level_progress(member.leveling.xp),
		sky_block::get_colours(level),
		false,
	);

	let mut canvas = Canvas::new(720., family)
		.gap(7.)
		.push_down(
			&shape::Title,
			shape::Title::from_text(family, &text::from_data(data, &data.username, suffix)),
		)
		.push_down(
			&shape::Subtitle,
			shape::Subtitle::from_label_str(family, &LABEL, name),
		)
		.push_down_post_draw(
			&progress,
			shape::WideBubbleProgress::from_level_progress(
				ctx,
				family,
				&sky_block::get_level_format(level),
				&sky_block::get_curr_level_xp(member.leveling.xp),
				&sky_block::get_level_xp(member.leveling.xp),
			),
		)
		.push_right_start(
			&canvas::shape::Sidebar,
			canvas::body::Body::new(17., None, family)
				.append_item(
					&::translate::tr(ctx, "coins"),
					&canvas::label::ToFormatted::to_formatted(&member.coin_purse, ctx),
					&Paint::Gold,
				)
				.append_item(
					&::translate::tr(ctx, "fairy-souls"),
					&canvas::label::ToFormatted::to_formatted(&member.fairy_souls_collected, ctx),
					&Paint::Aqua,
				)
				.append_item(
					&::translate::tr(ctx, "fairy-exchanges"),
					&canvas::label::ToFormatted::to_formatted(&member.fairy_exchanges, ctx),
					&Paint::LightPurple,
				)
				.append_item(
					&::translate::tr(ctx, "fishing-treasure"),
					&canvas::label::ToFormatted::to_formatted(&member.fishing_treasure_caught, ctx),
					&Paint::Blue,
				)
				.append_item(
					&::translate::tr(ctx, "zones-visited"),
					&canvas::label::ToFormatted::to_formatted(&member.zones_visited, ctx),
					&Paint::Green,
				)
				.append_item(
					&::translate::tr(ctx, "generators-crafted"),
					&canvas::label::ToFormatted::to_formatted(&member.generators_crafted, ctx),
					&Paint::White,
				)
				.append_item(
					&::translate::tr(ctx, "highest-crit"),
					&canvas::label::ToFormatted::to_formatted(
						&member.stats.highest_critical_damage,
						ctx,
					),
					&Paint::Red,
				)
				.build(),
		)
		.push_right_post_draw(&status, Body::from_status(ctx, family, session));

	let slots = items
		.iter()
		.map(|s| {
			shape::Slot(
				if let Some(v) = MATERIALS.get(&s.id) {
					Some(v.image())
				} else {
					tracing::warn!(id = s.id, "unknown item");

					None
				},
				1,
			)
		})
		.collect::<Vec<_>>();

	for slot in &slots {
		canvas = canvas.push_checked_post_draw(slot, Body::empty());
	}

	let mut surface = canvas.build(None, background).unwrap();

	Ok((canvas::to_png(&mut surface).into(), profile))
}

#[allow(clippy::too_many_lines)]
pub async fn bazaar(
	ctx: &context::Context<'_>,
	family: Family,
	product: &str,
	background: Option<skia_safe::Color>,
) -> Result<Cow<'static, [u8]>, Error> {
	let id = diesel_async::RunQueryDsl::first::<i16>(
		bazaar_item::table
			.filter(bazaar_item::name.eq(product))
			.select(bazaar_item::id),
		&mut ctx.connection().await?,
	)
	.await?;

	let history = diesel_async::RunQueryDsl::get_results::<(f64, i32, f64, i32, DateTime<Utc>)>(
		bazaar::table
			.filter(bazaar::item_id.eq(id))
			.select((
				bazaar::buy_price,
				bazaar::buy_volume,
				bazaar::sell_price,
				bazaar::sell_volume,
				bazaar::created_at,
			))
			.order(bazaar::created_at.asc())
			.limit(60 * 24 * 7),
		&mut ctx.connection().await?,
	)
	.await?;

	let range_x = history.first().unwrap().4..history.last().unwrap().4;

	let mut range_y_start: f64 = f64::MAX;
	let mut range_y_end: f64 = f64::MIN;

	for product in &history {
		range_y_start = range_y_start.min(product.0).min(product.2);
		range_y_end = range_y_end.max(product.0).max(product.2);
	}

	let mut buffer = create::<false>(
		ctx,
		family,
		vec![
			(
				::translate::tr(ctx, "sell-price"),
				history
					.iter()
					.map(|(_, _, price, _, created_at)| (*created_at, *price))
					.collect(),
			),
			(
				::translate::tr(ctx, "buy-price"),
				history
					.iter()
					.map(|(price, _, _, _, created_at)| (*created_at, *price))
					.collect(),
			),
		],
		range_x,
		range_y_start..range_y_end,
		None,
		background,
	)?;

	let mut surface = chart::canvas_with_size(
		&mut buffer,
		(750, 400 + BUBBLE_HEIGHT_I as usize * 2 + GAP_I as usize * 3),
	)?;

	apply_bazaar_data(
		ctx,
		family,
		&mut surface,
		&[
			Text {
				text: "Bazaar",
				paint: Paint::Gold,
				font: MinecraftFont::Bold,
				..Default::default()
			},
			Text {
				text: tr(ctx, "statistics-history").as_ref(),
				..Default::default()
			},
			Text {
				text: product,
				paint: Paint::Aqua,
				..Default::default()
			},
		],
		&history,
		background,
	);
	round_corners(&mut surface);

	Ok(canvas::to_png(&mut surface).into())
}

macro_rules! inventory_command {
	($fn: ident, $key: ident, $mode: ident) => {
		#[allow(clippy::too_many_lines)]
		#[allow(clippy::too_many_arguments)]
		pub async fn $fn(
			ctx: &context::Context<'_>,
			family: Family,
			player: &Player,
			data: &Data,
			session: &Session,
			skin: &skia_safe::Image,
			suffix: Option<&str>,
			background: Option<skia_safe::Color>,
			profile_id: Option<Uuid>,
			profile: Option<&str>,
		) -> Result<(Cow<'static, [u8]>, Arc<Profile>), Error> {
			#[rustfmt::skip]
			let Some(profile) = (match (profile_id, profile) {
				(Some(id), ..) => data.stats.sky_block.profiles.iter().find(|p| p.id == id),
				(.., Some(profile)) => data
					.stats
					.sky_block
					.profiles
					.iter()
					.find(|p| p.name == profile),
				(None, None) => data.stats.sky_block.profiles.first(),
			}) else {
				return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
			};

			let name = profile.name.as_str();
			let profile = Player::get_skyblock_profile(profile, &data.username).await?;

			#[rustfmt::skip]
			let Some(member) = profile.members.get(&player.uuid) else {
				return Err(Error::MemberPlayerNotFound(data.username.clone()));
			};

			#[rustfmt::skip]
			let items = member.$key.as_ref().map_or(EMPTY_ROW, |i| i.items.as_slice());

			let status = shape::Status(&session, skin);
			let level = sky_block::get_level(member.leveling.xp);
			let progress = shape::WideBubbleProgress(
				sky_block::get_level_progress(member.leveling.xp),
				sky_block::get_colours(level),
				false,
			);

			let ctx = &ctx;
			let mut canvas = Canvas::new(720., family)
				.gap(7.)
				.push_down(
					&shape::Title,
					shape::Title::from_text(
						family,
						&text::from_data(&data, &data.username, suffix.as_deref()),
					),
				)
				.push_down(
					&shape::Subtitle,
					shape::Subtitle::from_label_str(family, &LABEL, name),
				)
				.push_down_post_draw(
					&progress,
					shape::WideBubbleProgress::from_level_progress(
						ctx,
						family,
						&sky_block::get_level_format(level),
						&sky_block::get_curr_level_xp(member.leveling.xp),
						&sky_block::get_level_xp(member.leveling.xp),
					),
				)
				.push_right_start(
					&canvas::shape::Sidebar,
					canvas::body::Body::new(17., None, family)
						.append_item(
							&::translate::tr(ctx, "coins"),
							&canvas::label::ToFormatted::to_formatted(&member.coin_purse, ctx),
							&Paint::Gold,
						)
						.append_item(
							&::translate::tr(ctx, "fairy-souls"),
							&canvas::label::ToFormatted::to_formatted(
								&member.fairy_souls_collected,
								ctx,
							),
							&Paint::Aqua,
						)
						.append_item(
							&::translate::tr(ctx, "fairy-exchanges"),
							&canvas::label::ToFormatted::to_formatted(&member.fairy_exchanges, ctx),
							&Paint::LightPurple,
						)
						.append_item(
							&::translate::tr(ctx, "fishing-treasure"),
							&canvas::label::ToFormatted::to_formatted(
								&member.fishing_treasure_caught,
								ctx,
							),
							&Paint::Blue,
						)
						.append_item(
							&::translate::tr(ctx, "zones-visited"),
							&canvas::label::ToFormatted::to_formatted(&member.zones_visited, ctx),
							&Paint::Green,
						)
						.append_item(
							&::translate::tr(ctx, "generators-crafted"),
							&canvas::label::ToFormatted::to_formatted(
								&member.generators_crafted,
								ctx,
							),
							&Paint::White,
						)
						.append_item(
							&::translate::tr(ctx, "highest-crit"),
							&canvas::label::ToFormatted::to_formatted(
								&member.stats.highest_critical_damage,
								ctx,
							),
							&Paint::Red,
						)
						.build(),
				)
				.push_right_post_draw(&status, Body::from_status(ctx, family, &session));

			let slots = items
				.iter()
				.map(|s| {
					s.as_ref().map_or(shape::Slot(None, 0), |s| {
						shape::Slot(
							if let Some(v) = if s.id.starts_with("ENCHANTED_") {
								MATERIALS.get(&s.id[10..])
							} else if let Some(idx) = s.id.find(':') {
								MATERIALS
									.get(&format!("{}:{}", &s.id[..idx], s.damage))
									.or_else(|| MATERIALS.get(&s.id))
							} else if s.damage != 0 {
								MATERIALS
									.get(&format!("{}:{}", &s.id, s.damage))
									.or_else(|| MATERIALS.get(&s.id))
							} else {
								MATERIALS.get(&s.id)
							} {
								Some(v.image())
							} else {
								tracing::warn!(id = s.id, "unknown item");

								None
							},
							s.count,
						)
					})
				})
				.collect::<Vec<_>>();

			for slot in &slots {
				canvas = canvas.push_checked_post_draw(slot, Body::empty());
			}

			let mut surface = canvas.build(None, background).unwrap();

			Ok((canvas::to_png(&mut surface).into(), profile))
		}
	};
}

inventory_command!(inventory, inventory, Inventory);
inventory_command!(enderchest, ender_chest, EnderChest);
inventory_command!(talisman, talisman_bag, Talisman);
inventory_command!(quiver, quiver, Quiver);
inventory_command!(fishing, fishing_bag, Fishing);
inventory_command!(potions, potion_bag, Potions);
inventory_command!(equipment, equipment, Equipment);
inventory_command!(wardrobe, wardrobe, Wardrobe);
inventory_command!(candy, candy, Candy);
inventory_command!(vault, vault, Vault);
