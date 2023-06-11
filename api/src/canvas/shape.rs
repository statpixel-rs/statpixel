use crate::{
	canvas::{label::ToFormatted, util},
	game::r#type::Type,
	guild::Guild,
	player::status::Session,
};

use super::{body::Body, CORNER_RADIUS};

use minecraft::{
	paint::Paint,
	style::MinecraftFont,
	text::{parse::minecraft_string, Text, ESCAPE},
};
use skia_safe::{
	gradient_shader,
	textlayout::{Paragraph, TextAlign},
	Canvas, Color, Image, Path, Point, RRect, Rect, Size,
};
use translate::{tr, Context};

pub const BUBBLE_WIDTH: f32 = 706. / 3.;
pub const BUBBLE_HEIGHT: f32 = 85.;
pub const GAP: f32 = 7.;

pub trait Shape {
	fn draw(&self, path: &mut Path, bounds: &Rect);
	#[allow(unused_variables)]
	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, insets: &Point) {}
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
pub struct FullWidthTitle;
pub struct Subtitle;

pub struct Bubble;
pub struct WideBubble;
pub struct TallBubble;
pub struct WideTallBubble;

pub struct Sidebar;
pub struct Gutter;
pub struct Status<'s>(pub &'s Session, pub &'s [u8]);
pub struct PreferredGames<'g>(pub &'g [Type]);

pub struct LeaderboardTitle;
pub struct LeaderboardPlace;
pub struct LeaderboardName;
pub struct LeaderboardValue;

pub struct GuildXpName;
pub struct GuildXpValue;
pub struct GuildXpTitle;

pub struct WideBubbleProgress(pub f32, pub [Color; 2]);

impl Custom {
	#[must_use]
	pub fn from_text_large(text: &[Text]) -> Paragraph {
		let mut paragraph = Body::new(25., TextAlign::Center).extend(text).build();

		paragraph.layout(f32::MAX);
		paragraph
	}

	#[must_use]
	pub fn from_text(text: &[Text]) -> Paragraph {
		let mut paragraph = Body::new(20., TextAlign::Center).extend(text).build();

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
	pub fn from_guild(ctx: Context<'_>, guild: &Guild) -> Paragraph {
		let mut body = Body::new(17., None);
		let mut iter = guild.xp_by_game.iter().rev();

		if let Some((game, xp)) = iter.next() {
			body = body.append_item(
				game.as_short_clean_name(),
				xp.to_formatted_label(ctx).as_ref(),
				&Paint::Gold,
			);
		}

		if let Some((game, xp)) = iter.next() {
			body = body.append_item(
				game.as_short_clean_name(),
				xp.to_formatted_label(ctx).as_ref(),
				&Paint::Gray,
			);
		}

		if let Some((game, xp)) = iter.next() {
			body = body.append_item(
				game.as_short_clean_name(),
				xp.to_formatted_label(ctx).as_ref(),
				&Paint::Red,
			);
		}

		for (game, xp) in iter.take(4) {
			body = body.append_item(
				game.as_short_clean_name(),
				xp.to_formatted_label(ctx).as_ref(),
				&Paint::DarkGray,
			);
		}

		body.build()
	}
}

impl WideTallBubble {
	#[must_use]
	pub fn from_guild(
		ctx: Context<'_>,
		guild: &Guild,
		players: &[String],
		idx: usize,
	) -> Paragraph {
		let count = guild.members.len();
		let mut body = Body::new(17., None);

		for (idx, player) in players.iter().enumerate().skip(idx * 7).take(7) {
			let text = guild.members[count - idx - 1]
				.xp_history
				.iter()
				.map(|h| h.1)
				.sum::<u32>();
			let text = text.to_formatted_label(ctx);

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
	pub fn from_text(text: &[Text]) -> Paragraph {
		Body::new(25., TextAlign::Center).extend(text).build()
	}

	#[must_use]
	pub fn from_guild(guild: &Guild) -> Paragraph {
		let colour: char = guild.tag_colour.into();
		let name = guild.name.as_str();
		let tag = guild.tag.as_ref();

		let text = if let Some(tag) = tag {
			format!("{ESCAPE}{colour}{name} [{tag}]")
		} else {
			format!("{ESCAPE}{colour}{name}")
		};

		Body::new(25., TextAlign::Center)
			.extend_owned(minecraft_string(&text))
			.build()
	}
}

impl Subtitle {
	#[must_use]
	pub fn from_text(text: &[Text]) -> Paragraph {
		Body::new(20., TextAlign::Center).extend(text).build()
	}

	#[must_use]
	pub fn from_label(ctx: Context<'_>, label: &[Text], tr: &str) -> Paragraph {
		let text = tr!(ctx, tr);
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

		Self::from_text(text.as_slice())
	}
}

impl WideBubbleProgress {
	#[must_use]
	pub fn from_text(text: &[Text]) -> Paragraph {
		Body::new(20., TextAlign::Center).extend(text).build()
	}

	#[must_use]
	pub fn from_level_progress(
		ctx: Context<'_>,
		level: &str,
		current: &impl ToFormatted,
		needed: &impl ToFormatted,
	) -> Paragraph {
		let label = tr!(ctx, "level");
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

		let label = tr!(ctx, "progress");
		let current = current.to_formatted_label(ctx);
		let needed = needed.to_formatted_label(ctx);

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

		Self::from_text(text.as_slice())
	}
}

impl LeaderboardPlace {
	#[must_use]
	pub fn from_usize(value: usize) -> Paragraph {
		let text = value.to_string();

		Body::new(20., TextAlign::Center)
			.extend(&[Text {
				text: &text,
				font: MinecraftFont::Bold,
				paint: Paint::White,
				..Default::default()
			}])
			.build()
	}
}

impl LeaderboardName {
	#[must_use]
	pub fn from_text(text: &str) -> Paragraph {
		Body::new(20., TextAlign::Left)
			.extend(&minecraft_string(text).collect::<Vec<_>>())
			.build()
	}
}

impl LeaderboardValue {
	#[must_use]
	pub fn from_value(ctx: Context<'_>, value: &impl ToFormatted) -> Paragraph {
		Body::new(20., TextAlign::Center)
			.extend(&[Text {
				text: &value.to_formatted_label(ctx),
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
impl_rect_shape!(FullWidthTitle, BUBBLE_WIDTH * 5. + GAP * 4., 45., true);
impl_rect_shape!(Subtitle, BUBBLE_WIDTH * 1.5 + GAP / 2., 33., true);

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

impl_rect_shape!(LeaderboardTitle, BUBBLE_WIDTH * 3. + GAP * 2., 50., true);
impl_rect_shape!(LeaderboardPlace, 50., 35., true);
impl_rect_shape!(LeaderboardValue, 200., 35., true);

impl_rect_shape!(GuildXpTitle, (50. + 300. + 125.) * 2. + GAP * 5., 45., true);
impl_rect_shape!(GuildXpValue, 125., 35., true);

impl Shape for Custom {
	fn size(&self) -> Size {
		Size {
			width: self.width,
			height: self.height,
		}
	}

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

	fn v_align(&self) -> bool {
		true
	}
}

impl Shape for GuildXpName {
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

impl Shape for LeaderboardName {
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
			width: 456.,
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

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, insets: &Point) {
		let mut iter = self
			.0
			.iter()
			.filter_map(crate::game::r#type::Type::as_image_bytes)
			.enumerate()
			.take(6);

		while let Some((idx, bytes)) = iter.next() {
			let x = bounds.x() + insets.x;
			#[allow(clippy::cast_precision_loss)]
			let y = bounds.y() + insets.y + (40. + 7.) * idx as f32 / 2.;

			// `bytes` lives for 'static, so it will always be valid.
			let image = Image::from_encoded(unsafe { skia_safe::Data::new_bytes(bytes) }).unwrap();

			canvas.draw_image(image, (x, y), None);

			if let Some((_, bytes)) = iter.next() {
				let x = x + 40. + 7.;
				let image =
					Image::from_encoded(unsafe { skia_safe::Data::new_bytes(bytes) }).unwrap();

				canvas.draw_image(image, (x, y), None);
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

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, _insets: &Point) {
		if !self.0.online {
			let image = Image::from_encoded(unsafe { skia_safe::Data::new_bytes(self.1) }).unwrap();

			canvas.draw_image(image, (bounds.x() + 10., bounds.y() + 10.), None);
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
			width: BUBBLE_WIDTH,
			height: BUBBLE_HEIGHT * 2. + GAP,
		}
	}

	fn v_align(&self) -> bool {
		false
	}

	fn insets(&self) -> Point {
		(13., 19.).into()
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

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, _insets: &Point) {
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
			height: BUBBLE_HEIGHT,
		}
	}

	fn v_align(&self) -> bool {
		true
	}
}
