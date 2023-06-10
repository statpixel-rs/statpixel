use std::fmt;

use skia_safe::{
	textlayout::Paragraph, Color, ISize, Paint, Path, Point, RRect, Rect, Size, Surface,
};

use super::shape::Shape;

pub const CORNER_RADIUS: f32 = 20.;
pub const INSET: f32 = 3.5;
pub const MARGIN: i32 = 11;

pub struct Canvas<'c> {
	tl: Point,
	size: Point,
	last_size: Option<Size>,
	max_width: Option<f32>,
	path: Path,
	text: Vec<Item<'c>>,
	inset: f32,
}

pub type Item<'c> = (Rect, Paragraph, bool, Point, Option<&'c dyn Shape>);

impl<'c> fmt::Debug for Canvas<'c> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Canvas")
			.field("tl", &self.tl)
			.field("size", &self.size)
			.field("last_size", &self.last_size)
			.field("path", &self.path)
			.field("inset", &self.inset)
			.finish()
	}
}

pub struct Margin(pub i32);

impl From<i32> for Margin {
	fn from(value: i32) -> Self {
		Self(value)
	}
}

impl From<Margin> for i32 {
	fn from(margin: Margin) -> Self {
		margin.0
	}
}

impl<'c> Canvas<'c> {
	pub const BACKGROUND: Color = Color::from_rgb(31, 48, 64);
	pub const BACKGROUND_U32: u32 = 255 << 24 | 31 << 16 | 48 << 8 | 64;
	pub const ITEM_BACKGROUND: Color = Color::from_argb(192, 20, 28, 36);

	#[must_use]
	pub fn new(max_width: impl Into<Option<f32>>) -> Self {
		Self {
			tl: Point::default(),
			size: Point::default(),
			last_size: None,
			max_width: max_width.into(),
			path: Path::new(),
			text: vec![],
			inset: INSET,
		}
	}

	#[inline]
	#[must_use]
	pub fn gap(mut self, gap: f32) -> Self {
		self.inset = gap / 2.;
		self
	}

	#[must_use]
	pub fn push(mut self, shape: &impl Shape, text: impl Into<Paragraph>) -> Self {
		let mut bounds = Rect::from_size(shape.size());

		bounds.offset(self.tl);
		bounds.offset((self.inset, self.inset));

		shape.draw(&mut self.path, &bounds);

		self.text
			.push((bounds, text.into(), shape.v_align(), shape.insets(), None));
		self.last_size = shape.size().into();
		self
	}

	#[must_use]
	pub fn push_post_draw(mut self, shape: &'c impl Shape, text: impl Into<Paragraph>) -> Self {
		let mut bounds = Rect::from_size(shape.size());

		bounds.offset(self.tl);
		bounds.offset((self.inset, self.inset));

		shape.draw(&mut self.path, &bounds);

		self.last_size = shape.size().into();
		self.text.push((
			bounds,
			text.into(),
			shape.v_align(),
			shape.insets(),
			Some(shape),
		));
		self
	}

	#[must_use]
	pub fn push_checked_post_draw(self, shape: &'c impl Shape, text: impl Into<Paragraph>) -> Self {
		let last_width = match self.last_size {
			Some(size) => size.width,
			None => 0.,
		};

		let size = shape.size();

		if let Some(max_width) = self.max_width {
			if self.tl.x + last_width + size.width > max_width {
				self.push_down_start_post_draw(shape, text)
			} else {
				self.push_right_post_draw(shape, text)
			}
		} else {
			self.push_right_post_draw(shape, text)
		}
	}

	#[must_use]
	pub fn push_checked(self, shape: &impl Shape, text: impl Into<Paragraph>) -> Self {
		let last_width = match self.last_size {
			Some(size) => size.width,
			None => 0.,
		};

		let size = shape.size();

		if let Some(max_width) = self.max_width {
			if self.tl.x + last_width + size.width + self.inset * 2. > max_width {
				self.push_down_start(shape, text)
			} else {
				self.push_right(shape, text)
			}
		} else {
			self.push_right(shape, text)
		}
	}

	#[must_use]
	pub fn push_right_post_draw(
		mut self,
		shape: &'c impl Shape,
		text: impl Into<Paragraph>,
	) -> Self {
		if let Some(size) = self.last_size {
			self.tl.x += size.width + self.inset * 2.;
			self.update_size();
		}

		self.push_post_draw(shape, text)
	}

	#[must_use]
	pub fn push_right(mut self, shape: &impl Shape, text: impl Into<Paragraph>) -> Self {
		if let Some(size) = self.last_size {
			self.tl.x += size.width + self.inset * 2.;
			self.update_size();
		}

		self.push(shape, text)
	}

	#[must_use]
	pub fn push_down_post_draw(
		mut self,
		shape: &'c impl Shape,
		text: impl Into<Paragraph>,
	) -> Self {
		if let Some(size) = self.last_size {
			self.tl.y += size.height + self.inset * 2.;
			self.update_size();
		}

		self.push_post_draw(shape, text)
	}

	#[must_use]
	pub fn push_down(mut self, shape: &impl Shape, text: impl Into<Paragraph>) -> Self {
		if let Some(size) = self.last_size {
			self.tl.y += size.height + self.inset * 2.;
			self.update_size();
		}

		self.push(shape, text)
	}

	#[must_use]
	pub fn push_down_start(mut self, shape: &impl Shape, text: impl Into<Paragraph>) -> Self {
		if let Some(size) = self.last_size {
			self.tl.x += size.width + self.inset * 2.;
			self.tl.y += size.height + self.inset * 2.;
			self.update_size();
		}

		self.tl.x = 0.;
		self.push(shape, text)
	}

	#[must_use]
	pub fn push_down_start_post_draw(
		mut self,
		shape: &'c impl Shape,
		text: impl Into<Paragraph>,
	) -> Self {
		if let Some(size) = self.last_size {
			self.tl.x += size.width + self.inset * 2.;
			self.tl.y += size.height + self.inset * 2.;
			self.update_size();
		}

		self.tl.x = 0.;
		self.push_post_draw(shape, text)
	}

	#[must_use]
	pub fn push_right_start_post_draw(
		mut self,
		shape: &'c impl Shape,
		text: impl Into<Paragraph>,
	) -> Self {
		if let Some(size) = self.last_size {
			self.tl.x += size.width + self.inset * 2.;
			self.tl.y += size.height + self.inset * 2.;
			self.update_size();
		}

		self.tl.y = 0.;
		self.push_post_draw(shape, text)
	}

	#[must_use]
	pub fn push_right_start(mut self, shape: &impl Shape, text: impl Into<Paragraph>) -> Self {
		if let Some(size) = self.last_size {
			self.tl.x += size.width + self.inset * 2.;
			self.tl.y += size.height + self.inset * 2.;
			self.update_size();
		}

		self.tl.y = 0.;
		self.push(shape, text)
	}

	#[inline]
	fn update_size(&mut self) {
		self.size.x = self.size.x.max(self.tl.x);
		self.size.y = self.size.y.max(self.tl.y);
	}

	#[must_use]
	#[allow(clippy::cast_possible_truncation)]
	pub fn build(
		mut self,
		margin: impl Into<Option<Margin>>,
		background: impl Into<Option<Color>>,
	) -> Option<Surface> {
		if let Some(size) = self.last_size {
			self.tl.x += size.width + self.inset * 2.;
			self.tl.y += size.height + self.inset * 2.;

			self.update_size();
		}

		let margin: Margin = margin.into().unwrap_or(Margin(MARGIN));
		let size: ISize = (
			self.size.x as i32 + margin.0 * 2,
			self.size.y as i32 + margin.0 * 2,
		)
			.into();

		let mut surface = Surface::new_raster_n32_premul(size)?;
		let canvas = surface.canvas();
		let offset: Point = (margin.0, margin.0).into();

		canvas.draw_rrect(
			RRect::new_rect_radii(
				#[allow(clippy::cast_precision_loss)]
				Rect::new(0., 0., size.width as f32, size.height as f32),
				&[
					Point::new(30., 30.),
					Point::new(30., 30.),
					Point::new(30., 30.),
					Point::new(30., 30.),
				],
			),
			Paint::default().set_color(background.into().unwrap_or(Self::BACKGROUND)),
		);

		self.path.offset(offset);
		canvas.draw_path(
			&self.path,
			Paint::default().set_color(Self::ITEM_BACKGROUND),
		);

		for (mut bounds, mut text, v_align, insets, shape) in self.text {
			bounds.offset(offset);

			text.layout(bounds.width());
			text.paint(
				canvas,
				(
					bounds.left + insets.x,
					if v_align {
						bounds.center_y() - text.height() / 2.
					} else {
						bounds.top + insets.y
					},
				),
			);

			if let Some(shape) = shape {
				shape.post_draw(canvas, &bounds, &insets);
			}
		}

		Some(surface)
	}
}
