use crate::{
	canvas::{label::ToFormatted, util},
	player::status::Session,
};

use super::{body::Body, CORNER_RADIUS};

use minecraft::{
	paint::Paint,
	style::MinecraftFont,
	text::{parse::minecraft_string, Text},
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

pub struct Title;
pub struct Subtitle;

pub struct Bubble;
pub struct WideBubble;
pub struct TallBubble;

pub struct Sidebar;
pub struct Gutter;
pub struct Status<'s>(pub &'s Session, pub &'s [u8]);

pub struct LeaderboardTitle;
pub struct LeaderboardPlace;
pub struct LeaderboardName;
pub struct LeaderboardValue;

pub struct WideBubbleProgress(pub f32, pub [Color; 2]);

impl Title {
	#[must_use]
	pub fn from_text(text: &[Text]) -> Paragraph {
		Body::new(25., TextAlign::Center).extend(text).build()
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

	fn post_draw(&self, canvas: &mut Canvas, bounds: &Rect, insets: &Point) {
		if !self.0.online {
			let image = Image::from_encoded(unsafe { skia_safe::Data::new_bytes(self.1) }).unwrap();

			canvas.draw_image(image, (bounds.x() + insets.x, bounds.y() + insets.y), None);
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
				.set_alpha(64)
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
