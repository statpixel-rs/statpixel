use std::borrow::Cow;

use crate::canvas::util;
use label::ToFormatted;

use crate::{
	guild::Guild,
	milliseconds::Milliseconds,
	player::{self, status::Session},
	skyblock::networth::calc::Category,
};

use super::{body::Body, CORNER_RADIUS};

use hypixel::game::{self, r#type::Type};
use minecraft::{
	paint::Paint,
	style::{Family, MinecraftFont},
	text::{parse::minecraft_string, Text, ESCAPE},
};
use skia_safe::{
	gradient_shader,
	textlayout::{Paragraph, TextAlign},
	Canvas, Color, Image, Path, Point, RRect, Rect, Size,
};
use translate::{context::Context, prelude::GetChronoLocale, tr};

pub const BUBBLE_WIDTH: f32 = 706. / 3.;
pub const BUBBLE_HEIGHT: f32 = 85.;
pub const GAP: f32 = 7.;

pub trait Shape {
	fn draw(&self, path: &mut Path, bounds: &Rect) {
		path.add_rrect(
			RRect::new_rect_radii(
				bounds,
				&[
					(CORNER_RADIUS, CORNER_RADIUS).into(),
					(CORNER_RADIUS, CORNER_RADIUS).into(),
					(CORNER_RADIUS, CORNER_RADIUS).into(),
					(CORNER_RADIUS, CORNER_RADIUS).into(),
				],
			),
			None,
		);
	}

	#[allow(unused_variables)]
	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, insets: &Point, family: Family) {}
	fn size(&self) -> Size;
	fn v_align(&self) -> bool;
	fn insets(&self) -> Point {
		Point::default()
	}
}

pub struct Custom {
	pub width: f32,
	pub height: f32,
}
pub struct Title;
pub struct LongTitle;
pub struct FullWidthTitle;
pub struct FullWidthBigTitle;
pub struct Subtitle;
pub struct LongSubtitle;
pub struct BubbleSubtitle;
pub struct ShortSubtitle;

/// Expects a 48x48 PNG
pub struct Slot<'a>(pub Option<&'a Image>, pub u8);

/// Expects a 48x48 PNG
pub struct NetworthSlot<'a>(pub Option<&'a Image>, pub u8);
pub struct EmptyNetworthSlot;
pub struct NetworthName;
pub struct EmptyNetworthName;

pub struct Bubble;
pub struct WideBubble;
pub struct TallBubble;
pub struct WideTallBubble;
pub struct RecentGame<'g>(pub &'g game::r#type::Type);

/// Displays the stats of an entire game mode.
///
/// Ensure the `lines` are equal to all horizontally-adjacent [`CondensedBubble`]s
/// to avoid weird spacing. Put the game mode in white bold at the top, then
/// all of the labels (only static labels in Overall, only mode-specific in each mode),
/// also aligned to take up the same number of lines. Then, all of the statistics
/// go at the bottom.
pub struct CondensedBubble {
	pub lines: u8,
}

pub struct Sidebar;
pub struct Gutter;
pub struct Status<'s>(pub &'s Session, pub &'s Image);
pub struct PreferredGames<'g>(pub &'g [Type]);

pub struct LeaderboardTitle {
	pub extras: u8,
}
pub struct LeaderboardPlace;
pub struct LeaderboardName;
pub struct LeaderboardNameLabel;
pub struct LeaderboardValue;

pub struct GuildXpName;
pub struct GuildXpValue;
pub struct GuildXpTitle;

pub struct WideBubbleProgress(pub f32, pub [Color; 2], pub bool);

impl Custom {
	#[must_use]
	pub fn from_text_large(family: Family, text: &[Text]) -> Paragraph {
		let mut paragraph = Body::new(25., TextAlign::Center, family)
			.extend(text)
			.build();

		paragraph.layout(f32::MAX);
		paragraph
	}

	#[must_use]
	pub fn from_text(family: Family, text: &[Text]) -> Paragraph {
		let mut paragraph = Body::new(20., TextAlign::Center, family)
			.extend(text)
			.build();

		paragraph.layout(f32::MAX);
		paragraph
	}

	#[must_use]
	/// Layout the paragraph first
	pub fn get_from_paragraph(paragraph: &Paragraph) -> Self {
		Self {
			width: paragraph.max_intrinsic_width() + 20.,
			height: paragraph.height() + 10.,
		}
	}
}

impl Sidebar {
	#[must_use]
	pub fn from_guild(ctx: &Context<'_>, family: Family, guild: &Guild) -> Paragraph {
		let mut body = Body::new(17., None, family);
		let mut iter = guild.xp_by_game.iter().rev();

		if let Some((game, xp)) = iter.next() {
			body = body.append_item(
				game.as_short_clean_name(),
				xp.to_formatted(ctx).as_ref(),
				&Paint::Gold,
			);
		}

		if let Some((game, xp)) = iter.next() {
			body = body.append_item(
				game.as_short_clean_name(),
				xp.to_formatted(ctx).as_ref(),
				&Paint::Gray,
			);
		}

		if let Some((game, xp)) = iter.next() {
			body = body.append_item(
				game.as_short_clean_name(),
				xp.to_formatted(ctx).as_ref(),
				&Paint::Red,
			);
		}

		for (game, xp) in iter.take(4) {
			body = body.append_item(
				game.as_short_clean_name(),
				xp.to_formatted(ctx).as_ref(),
				&Paint::DarkGray,
			);
		}

		body.build()
	}
}

impl RecentGame<'_> {
	#[must_use]
	pub fn from_game(ctx: &Context<'_>, family: Family, game: &player::games::Game) -> Paragraph {
		let fmt = "%d/%m %r";
		let locale = ctx.get_chrono_locale();
		let duration = game
			.ended
			.map(|e| Milliseconds((e - game.started).num_milliseconds()));

		Body::new(20., None, family)
			.extend_owned(game.kind.as_text().iter().map(|t| Text {
				text: t.text,
				paint: t.paint,
				size: Some(25.),
				font: t.font,
			}))
			.extend(&[
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: tr(ctx, "mode").as_ref(),
					..Default::default()
				},
				Text {
					text: ": ",
					..Default::default()
				},
				Text {
					text: game.mode.as_clean_name(),
					paint: Paint::Blue,
					..Default::default()
				},
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: tr(ctx, "map").as_ref(),
					..Default::default()
				},
				Text {
					text: ": ",
					..Default::default()
				},
				Text {
					text: game
						.map
						.as_ref()
						.map_or_else(|| tr(ctx, "none"), |s| Cow::Borrowed(s))
						.as_ref(),
					paint: Paint::LightPurple,
					..Default::default()
				},
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: tr(ctx, "started").as_ref(),
					..Default::default()
				},
				Text {
					text: ": ",
					..Default::default()
				},
				Text {
					text: game
						.started
						.format_localized(fmt, locale)
						.to_string()
						.as_str(),
					paint: Paint::Gray,
					..Default::default()
				},
				Text {
					text: "\n",
					..Default::default()
				},
				Text {
					text: tr(ctx, "duration").as_ref(),
					..Default::default()
				},
				Text {
					text: ": ",
					..Default::default()
				},
				Text {
					text: duration
						.as_ref()
						.map_or_else(|| tr(ctx, "playing"), |e| e.to_formatted(ctx))
						.as_ref(),
					paint: if duration.is_none() {
						Paint::Green
					} else {
						Paint::Gray
					},
					..Default::default()
				},
			])
			.build()
	}
}

impl WideTallBubble {
	#[must_use]
	pub fn from_guild(
		ctx: &Context<'_>,
		family: Family,
		guild: &Guild,
		players: &[String],
		idx: usize,
	) -> Paragraph {
		let count = guild.members.len();
		let mut body = Body::new(17., None, family);

		for (idx, player) in players.iter().enumerate().skip(idx * 7).take(7) {
			let text = guild.members[count - idx - 1]
				.xp_history
				.iter()
				.map(|h| h.1)
				.sum::<u32>();
			let text = text.to_formatted(ctx);

			body = body.extend_owned(minecraft_string(player)).extend(&[
				Text {
					text: " • ",
					paint: Paint::DarkGray,
					..Default::default()
				},
				Text {
					text: text.as_ref(),
					paint: Paint::Gray,
					..Default::default()
				},
			]);

			if idx % 7 != 6 {
				body = body.append(Text {
					text: "\n",
					..Default::default()
				});
			}
		}

		body.build()
	}
}

impl Title {
	#[must_use]
	pub fn from_text(family: Family, text: &[Text]) -> Paragraph {
		Body::new(25., TextAlign::Center, family)
			.extend(text)
			.build()
	}

	#[must_use]
	pub fn from_category(ctx: &Context<'_>, family: Family, category: &Category) -> Paragraph {
		let Some(ref kind) = category.kind else {
			return Body::empty();
		};

		Body::new(25., TextAlign::Center, family)
			.extend(kind.as_text())
			.extend(&[
				Text {
					text: " [",
					paint: Paint::DarkGray,
					..Default::default()
				},
				Text {
					text: category.value.to_formatted(ctx).as_ref(),
					paint: Paint::Gold,
					..Default::default()
				},
				Text {
					text: "]",
					paint: Paint::DarkGray,
					..Default::default()
				},
			])
			.build()
	}

	#[must_use]
	pub fn from_guild(family: Family, guild: &Guild) -> Paragraph {
		let colour: char = guild.tag_colour.into();
		let name = guild.name.as_str();
		let tag = guild.tag.as_ref();

		let text = if let Some(tag) = tag {
			format!("{ESCAPE}{colour}{name} [{tag}]")
		} else {
			format!("{ESCAPE}{colour}{name}")
		};

		Body::new(25., TextAlign::Center, family)
			.extend_owned(minecraft_string(&text))
			.build()
	}
}

impl FullWidthBigTitle {
	#[must_use]
	pub fn from_text(family: Family, text: &[Text]) -> Paragraph {
		Body::new(40., TextAlign::Center, family)
			.extend(text)
			.build()
	}

	#[must_use]
	pub fn from_guild(ctx: &Context<'_>, family: Family, guild: &Guild) -> Paragraph {
		let colour: char = guild.tag_colour.into();
		let name = guild.name.as_str();
		let tag = guild.tag.as_ref();

		let text = if let Some(tag) = tag {
			format!("{ESCAPE}{colour}{name} [{tag}]")
		} else {
			format!("{ESCAPE}{colour}{name}")
		};

		let mut text = minecraft_string(&text).collect::<Vec<_>>();
		let members = guild.members.len();
		let members = members.to_formatted(ctx);

		text.extend([
			Text {
				text: " (",
				..Default::default()
			},
			Text {
				text: members.as_ref(),
				..Default::default()
			},
			Text {
				text: "/125)",
				..Default::default()
			},
		]);

		Body::new(40., TextAlign::Center, family)
			.extend(&text)
			.build()
	}
}

impl Subtitle {
	#[must_use]
	pub fn from_text(family: Family, text: &[Text]) -> Paragraph {
		Body::new(20., TextAlign::Center, family)
			.extend(text)
			.build()
	}

	#[must_use]
	pub fn from_formatted(
		ctx: &Context<'_>,
		family: Family,
		text: &impl ToFormatted,
		paint: Paint,
	) -> Paragraph {
		Body::new(20., TextAlign::Center, family)
			.append(Text {
				text: text.to_formatted(ctx).as_ref(),
				paint,
				..Default::default()
			})
			.build()
	}

	#[must_use]
	pub fn from_guild(family: Family, guild: &Guild) -> Paragraph {
		let colour: char = guild.tag_colour.into();
		let name = guild.name.as_str();
		let tag = guild.tag.as_ref();

		let text = if let Some(tag) = tag {
			format!("{ESCAPE}{colour}{name} [{tag}]")
		} else {
			format!("{ESCAPE}{colour}{name}")
		};

		Body::new(20., TextAlign::Center, family)
			.extend_owned(minecraft_string(&text))
			.build()
	}

	#[must_use]
	pub fn from_label(ctx: &Context<'_>, family: Family, label: &[Text], id: &str) -> Paragraph {
		let text = tr(ctx, id);
		let text = [
			label,
			&[
				Text {
					text: " (",
					paint: Paint::White,
					..Default::default()
				},
				Text {
					text: text.as_ref(),
					paint: Paint::White,
					..Default::default()
				},
				Text {
					text: ")",
					paint: Paint::White,
					..Default::default()
				},
			],
		]
		.concat();

		Self::from_text(family, text.as_slice())
	}

	#[must_use]
	pub fn from_label_str(family: Family, label: &[Text], sub: &str) -> Paragraph {
		let text = [
			label,
			&[
				Text {
					text: " (",
					paint: Paint::White,
					..Default::default()
				},
				Text {
					text: sub,
					paint: Paint::White,
					..Default::default()
				},
				Text {
					text: ")",
					paint: Paint::White,
					..Default::default()
				},
			],
		]
		.concat();

		Self::from_text(family, text.as_slice())
	}
}

impl WideBubbleProgress {
	#[must_use]
	pub fn from_text(family: Family, text: &[Text]) -> Paragraph {
		Body::new(20., TextAlign::Center, family)
			.extend(text)
			.build()
	}

	#[must_use]
	pub fn from_level_diff(
		ctx: &Context<'_>,
		family: Family,
		level: &str,
		total: &impl ToFormatted,
		positive: bool,
		short: bool,
	) -> Paragraph {
		let label = tr(ctx, "levels-gained");
		let mut text = vec![
			Text {
				text: &label,
				paint: Paint::White,
				..Default::default()
			},
			Text {
				text: ": ",
				paint: Paint::White,
				..Default::default()
			},
			Text {
				text: if positive { "+" } else { "-" },
				paint: Paint::White,
				..Default::default()
			},
		];

		text.extend(minecraft_string(level));

		if short {
			Self::from_text(family, text.as_slice())
		} else {
			text.reserve_exact(7);

			let label = tr(ctx, "total");
			let total = total.to_formatted(ctx);

			text.push(Text {
				text: "\n",
				paint: Paint::White,
				..Default::default()
			});

			text.push(Text {
				text: &label,
				paint: Paint::White,
				..Default::default()
			});

			text.push(Text {
				text: ": ",
				paint: Paint::White,
				..Default::default()
			});

			text.push(Text {
				text: &total,
				paint: Paint::Green,
				..Default::default()
			});

			Self::from_text(family, text.as_slice())
		}
	}

	#[must_use]
	pub fn from_level_progress(
		ctx: &Context<'_>,
		family: Family,
		level: &str,
		current: &impl ToFormatted,
		needed: &impl ToFormatted,
	) -> Paragraph {
		let label = tr(ctx, "level");
		let mut text = vec![
			Text {
				text: &label,
				paint: Paint::White,
				..Default::default()
			},
			Text {
				text: ": ",
				paint: Paint::White,
				..Default::default()
			},
		];

		text.extend(minecraft_string(level));
		text.reserve_exact(7);

		let label = tr(ctx, "progress");
		let current = current.to_formatted(ctx);
		let needed = needed.to_formatted(ctx);

		text.push(Text {
			text: "\n",
			paint: Paint::White,
			..Default::default()
		});

		text.push(Text {
			text: &label,
			paint: Paint::White,
			..Default::default()
		});

		text.push(Text {
			text: ": ",
			paint: Paint::White,
			..Default::default()
		});

		text.push(Text {
			text: &current,
			paint: Paint::Aqua,
			..Default::default()
		});

		text.push(Text {
			text: "/",
			paint: Paint::White,
			..Default::default()
		});

		text.push(Text {
			text: &needed,
			paint: Paint::Green,
			..Default::default()
		});

		Self::from_text(family, text.as_slice())
	}
}

impl LeaderboardPlace {
	pub fn label(ctx: &Context<'_>, family: Family) -> Paragraph {
		Body::build_slice(
			family,
			&[Text {
				text: tr(ctx, "position").as_ref(),
				font: MinecraftFont::Bold,
				..Default::default()
			}],
			17.,
			TextAlign::Center,
		)
	}

	#[must_use]
	pub fn from_usize(family: Family, value: usize) -> Paragraph {
		let text = format!("#{value}");

		Body::new(
			if value > 100_000 { 17. } else { 20. },
			TextAlign::Center,
			family,
		)
		.extend(&[Text {
			text: &text,
			font: MinecraftFont::Bold,
			paint: match value {
				1 => Paint::Gold,
				2 => Paint::Gray,
				3 => Paint::Bronze,
				_ => Paint::White,
			},
			..Default::default()
		}])
		.build()
	}
}

impl LeaderboardName {
	pub fn label(ctx: &Context<'_>, family: Family) -> Paragraph {
		Body::build_slice(
			family,
			&[Text {
				text: tr(ctx, "player").as_ref(),
				font: MinecraftFont::Bold,
				..Default::default()
			}],
			17.,
			TextAlign::Center,
		)
	}

	#[must_use]
	pub fn from_text(family: Family, text: &str) -> Paragraph {
		Body::new(20., TextAlign::Left, family)
			.extend(&minecraft_string(text).collect::<Vec<_>>())
			.build()
	}
}

impl LeaderboardValue {
	#[must_use]
	pub fn label(family: Family, label: Cow<str>) -> Paragraph {
		Body::new(17., TextAlign::Center, family)
			.extend(&[Text {
				text: label.as_ref(),
				paint: Paint::White,
				font: MinecraftFont::Bold,
				..Default::default()
			}])
			.build()
	}

	#[must_use]
	pub fn from_value(ctx: &Context<'_>, family: Family, value: &impl ToFormatted) -> Paragraph {
		Body::new(20., TextAlign::Center, family)
			.extend(&[Text {
				text: &value.to_formatted(ctx),
				paint: Paint::White,
				..Default::default()
			}])
			.build()
	}
}

macro_rules! impl_rect_shape {
	($ty: ty, $width: expr, $height: expr, $v_align: expr) => {
		impl Shape for $ty {
			fn draw(&self, path: &mut Path, bounds: &Rect) {
				path.add_rrect(
					RRect::new_rect_radii(
						bounds,
						&[
							(CORNER_RADIUS, CORNER_RADIUS).into(),
							(CORNER_RADIUS, CORNER_RADIUS).into(),
							(CORNER_RADIUS, CORNER_RADIUS).into(),
							(CORNER_RADIUS, CORNER_RADIUS).into(),
						],
					),
					None,
				);
			}

			fn size(&self) -> Size {
				Size {
					width: $width,
					height: $height,
				}
			}

			fn v_align(&self) -> bool {
				$v_align
			}
		}
	};
}

impl_rect_shape!(Title, BUBBLE_WIDTH * 1.5 + GAP / 2., 45., true);
impl_rect_shape!(LongTitle, BUBBLE_WIDTH * 3. + GAP * 2., 45., true);
impl_rect_shape!(FullWidthTitle, BUBBLE_WIDTH * 5. + GAP * 4., 45., true);
impl_rect_shape!(FullWidthBigTitle, BUBBLE_WIDTH * 5. + GAP * 4., 75., true);
impl_rect_shape!(Subtitle, BUBBLE_WIDTH * 1.5 + GAP / 2., 33., true);
impl_rect_shape!(LongSubtitle, BUBBLE_WIDTH * 3. + GAP * 2., 33., true);
impl_rect_shape!(BubbleSubtitle, BUBBLE_WIDTH, 33., true);
impl_rect_shape!(ShortSubtitle, BUBBLE_WIDTH / 2. - GAP / 2., 33., true);

impl_rect_shape!(Bubble, BUBBLE_WIDTH, BUBBLE_HEIGHT, true);
impl_rect_shape!(
	WideBubble,
	BUBBLE_WIDTH * 1.5 + GAP / 2.,
	BUBBLE_HEIGHT,
	true
);
impl_rect_shape!(TallBubble, BUBBLE_WIDTH, BUBBLE_HEIGHT * 2. + GAP, true);

impl_rect_shape!(
	Gutter,
	(BUBBLE_WIDTH - GAP) / 2.,
	BUBBLE_HEIGHT * 2. + GAP,
	true
);

impl_rect_shape!(LeaderboardPlace, 150., 35., true);
impl_rect_shape!(LeaderboardValue, 150., 35., true);
impl_rect_shape!(LeaderboardNameLabel, 406., 35., true);

impl_rect_shape!(GuildLeaderboardPlace, 50., 35., true);

impl_rect_shape!(GuildXpTitle, (50. + 300. + 125.) * 2. + GAP * 5., 45., true);
impl_rect_shape!(GuildXpValue, 125., 35., true);

impl Shape for CondensedBubble {
	fn size(&self) -> Size {
		Size {
			width: BUBBLE_WIDTH,
			height: 20. * f32::from(self.lines) + 20.,
		}
	}

	fn v_align(&self) -> bool {
		true
	}

	fn insets(&self) -> Point {
		(13., 0.).into()
	}
}

impl Shape for RecentGame<'_> {
	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, _insets: &Point, _family: Family) {
		if let Some(image) = self.0.as_image_bytes() {
			canvas.draw_image(
				image.image(),
				(bounds.right() - 40. - 10., bounds.top() + 10.),
				None,
			);
		}
	}

	fn size(&self) -> Size {
		Size {
			width: BUBBLE_WIDTH * 1.5 + GAP / 2.,
			height: BUBBLE_HEIGHT * 2. + GAP,
		}
	}

	fn v_align(&self) -> bool {
		true
	}

	fn insets(&self) -> Point {
		(30., 0.).into()
	}
}

impl Shape for EmptyNetworthSlot {
	fn size(&self) -> Size {
		Size {
			width: 48.,
			height: 48.,
		}
	}

	fn draw(&self, _path: &mut Path, _bounds: &Rect) {}

	fn v_align(&self) -> bool {
		true
	}
}

impl Shape for EmptyNetworthName {
	fn draw(&self, _path: &mut Path, _bounds: &Rect) {}

	fn size(&self) -> Size {
		Size {
			width: (BUBBLE_WIDTH * 1.5 + GAP / 2.) - 48. - GAP,
			height: 48.,
		}
	}

	fn v_align(&self) -> bool {
		true
	}

	fn insets(&self) -> Point {
		(10., 0.).into()
	}
}

impl Shape for NetworthName {
	fn size(&self) -> Size {
		Size {
			width: (BUBBLE_WIDTH * 1.5 + GAP / 2.) - 48. - GAP,
			height: 48.,
		}
	}

	fn v_align(&self) -> bool {
		true
	}

	fn insets(&self) -> Point {
		(10., 0.).into()
	}
}

impl Shape for NetworthSlot<'_> {
	fn size(&self) -> Size {
		Size {
			width: 48.,
			height: 48.,
		}
	}

	fn draw(&self, _path: &mut Path, _bounds: &Rect) {}

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, insets: &Point, family: Family) {
		if let Some(image) = self.0 {
			canvas.draw_image(image, (bounds.x() + insets.x, bounds.y() + insets.y), None);

			if self.1 > 1 {
				let mut paragraph = Body::build_slice(
					family,
					&[Text {
						text: &self.1.to_string(),
						..Default::default()
					}],
					27.,
					TextAlign::Center,
				);

				paragraph.layout(40.);
				// bottom right corner
				paragraph.paint(canvas, (bounds.right() - 20., bounds.bottom() - 18.));
			}
		}
	}

	fn v_align(&self) -> bool {
		true
	}
}

impl Shape for Slot<'_> {
	fn size(&self) -> Size {
		Size {
			width: 73.777_78,
			height: 73.777_78,
		}
	}

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, insets: &Point, family: Family) {
		if let Some(image) = self.0 {
			canvas.draw_image(image, (bounds.x() + insets.x, bounds.y() + insets.y), None);

			if self.1 > 1 {
				let mut paragraph = Body::build_slice(
					family,
					&[Text {
						text: &self.1.to_string(),
						..Default::default()
					}],
					27.,
					TextAlign::Center,
				);

				paragraph.layout(40.);
				// bottom right corner
				paragraph.paint(canvas, (bounds.right() - 32., bounds.bottom() - 30.));
			}
		}
	}

	fn insets(&self) -> Point {
		(12.888_89, 12.888_89).into()
	}

	fn v_align(&self) -> bool {
		true
	}
}

impl Shape for Custom {
	fn size(&self) -> Size {
		Size {
			width: self.width,
			height: self.height,
		}
	}

	fn v_align(&self) -> bool {
		true
	}
}

impl Shape for GuildXpName {
	fn size(&self) -> Size {
		Size {
			width: 300.,
			height: 35.,
		}
	}

	fn v_align(&self) -> bool {
		true
	}

	fn insets(&self) -> Point {
		(10., 0.).into()
	}
}

impl Shape for WideTallBubble {
	fn size(&self) -> Size {
		Size {
			width: BUBBLE_WIDTH * 1.5 + GAP / 2.,
			height: BUBBLE_HEIGHT * 2. + GAP,
		}
	}

	fn v_align(&self) -> bool {
		false
	}

	fn insets(&self) -> Point {
		(17., 20.).into()
	}
}

impl Shape for LeaderboardTitle {
	fn size(&self) -> Size {
		Size {
			width: BUBBLE_WIDTH * 3. + GAP * 2. + self.extras as f32 * (150. + GAP),
			height: 50.,
		}
	}

	fn v_align(&self) -> bool {
		true
	}
}

impl Shape for LeaderboardName {
	fn size(&self) -> Size {
		Size {
			width: 406.,
			height: 35.,
		}
	}

	fn v_align(&self) -> bool {
		true
	}

	fn insets(&self) -> Point {
		(10., 0.).into()
	}
}

impl<'g> Shape for PreferredGames<'g> {
	fn draw(&self, path: &mut Path, bounds: &Rect) {
		let rrect = RRect::new_rect_radii(
			bounds,
			&[
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
			],
		);

		path.add_rrect(rrect, None);
	}

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, insets: &Point, _family: Family) {
		let mut iter = self
			.0
			.iter()
			.filter_map(hypixel::game::r#type::Type::as_image_bytes)
			.enumerate()
			.take(6);

		while let Some((idx, image)) = iter.next() {
			let x = bounds.x() + insets.x;
			#[allow(clippy::cast_precision_loss)]
			let y = bounds.y() + insets.y + (40. + 7.) * idx as f32 / 2.;

			canvas.draw_image(image.image(), (x, y), None);

			if let Some((_, image)) = iter.next() {
				let x = x + 40. + 7.;

				canvas.draw_image(image.image(), (x, y), None);
			}
		}
	}

	fn size(&self) -> Size {
		Size {
			width: (BUBBLE_WIDTH - GAP) / 2.,
			height: BUBBLE_HEIGHT * 2. + GAP,
		}
	}

	fn v_align(&self) -> bool {
		true
	}

	fn insets(&self) -> Point {
		(14.5, 22.).into()
	}
}

impl<'s> Shape for Status<'s> {
	fn draw(&self, path: &mut Path, bounds: &Rect) {
		let rrect = RRect::new_rect_radii(
			bounds,
			&[
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
			],
		);

		path.add_rrect(rrect, None);
	}

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, _insets: &Point, _family: Family) {
		if !self.0.online {
			canvas.draw_image(self.1, (bounds.x() + 10., bounds.y() + 10.), None);
		}
	}

	fn size(&self) -> Size {
		Size {
			width: (BUBBLE_WIDTH - GAP) / 2.,
			height: BUBBLE_HEIGHT * 2. + GAP,
		}
	}

	fn v_align(&self) -> bool {
		true
	}
}

impl Shape for Sidebar {
	fn size(&self) -> Size {
		Size {
			width: BUBBLE_WIDTH,
			height: BUBBLE_HEIGHT * 2. + GAP,
		}
	}

	fn v_align(&self) -> bool {
		true
	}

	fn insets(&self) -> Point {
		(13., 0.).into()
	}
}

impl Shape for WideBubbleProgress {
	fn draw(&self, path: &mut Path, bounds: &Rect) {
		let rrect = RRect::new_rect_radii(
			bounds,
			&[
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
				(CORNER_RADIUS, CORNER_RADIUS).into(),
			],
		);

		path.add_rrect(rrect, None);
	}

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, _insets: &Point, _family: Family) {
		canvas.draw_path(
			util::progress::rrect(
				RRect::new_rect_radii(
					bounds.with_inset((1.5, 1.5)),
					&[
						(CORNER_RADIUS, CORNER_RADIUS).into(),
						(CORNER_RADIUS, CORNER_RADIUS).into(),
						(CORNER_RADIUS, CORNER_RADIUS).into(),
						(CORNER_RADIUS, CORNER_RADIUS).into(),
					],
				),
				self.0,
			)
			.offset((bounds.left + 1.5, bounds.top + 1.5)),
			skia_safe::Paint::default()
				.set_stroke_width(3.)
				.set_style(skia_safe::paint::Style::Stroke)
				.set_stroke_cap(skia_safe::paint::Cap::Round)
				.set_alpha(192)
				.set_shader(gradient_shader::linear(
					((bounds.left, bounds.top), (bounds.right, bounds.bottom)),
					self.1.as_ref(),
					None,
					skia_safe::TileMode::Clamp,
					None,
					None,
				)),
		);
	}

	fn size(&self) -> Size {
		Size {
			width: BUBBLE_WIDTH * 1.5 + GAP / 2.,
			height: if self.2 {
				BUBBLE_HEIGHT - 33. - GAP
			} else {
				BUBBLE_HEIGHT
			},
		}
	}

	fn v_align(&self) -> bool {
		true
	}
}
