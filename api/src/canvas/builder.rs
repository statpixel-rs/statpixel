use skia_safe::Surface;

pub const WIDTH: i16 = 750;
pub const BUBBLE_HEIGHT: i16 = 85;
pub const GAP: i16 = 7;
pub const PADDING: i16 = 15;

#[derive(Debug, Clone, Copy, Default)]
pub struct Canvas {
	height: i16,
	width: i16,
}

impl Canvas {
	#[must_use]
	pub fn new() -> Self {
		Self {
			height: 0,
			width: 0,
		}
	}

	pub fn rows<R: Into<i16>>(&mut self, rows: R) -> &mut Self {
		let rows = rows.into();

		self.height += (BUBBLE_HEIGHT + GAP) * rows;
		self
	}

	#[must_use]
	pub fn build(self) -> Option<Surface> {
		Surface::new_raster_n32_premul((i32::from(self.width), i32::from(self.height)))
	}
}
