pub mod loader;

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

const LEGACY_SKIN_HEIGHT: u32 = 32;
const MODERN_SKIN_HEIGHT: u32 = 64;
const SKIN_WIDTH: u32 = 64;

macro_rules! rect_slice {
	($([$x:expr, $y:expr, $width:expr, $height:expr]),*) => {
		&[
			$(
				image::math::Rect {
					x: $x,
					y: $y,
					width: $width,
					height: $height,
				},
			)*
		]
	};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkinFormat {
	Legacy,
	Modern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkinKind {
	Classic,
	Slim,
}

pub struct Skin {
	pub format: SkinFormat,
	pub kind: SkinKind,
	pub image: image::DynamicImage,
}

impl Skin {
	pub fn new(skin: DynamicImage, kind: SkinKind) -> Option<Self> {
		Some(Self {
			kind,
			format: SkinFormat::from_height(skin.height())?,
			image: skin,
		})
	}

	pub fn is_modern(&self) -> bool {
		self.format == SkinFormat::Modern
	}

	pub fn into_modern(mut self) -> Self {
		if self.is_modern() {
			return self;
		}

		let mut skin = DynamicImage::new_rgba8(SKIN_WIDTH, MODERN_SKIN_HEIGHT);

		// This should never panic, as legacy skins are always 64x32
		skin.copy_from(&self.image, 0, 0)
			.expect("legacy skin is too large");
		skin.copy_within(
			image::math::Rect {
				x: 0,
				y: 16,
				width: 16,
				height: 16,
			},
			16,
			48,
		);

		skin.copy_within(
			image::math::Rect {
				x: 40,
				y: 16,
				width: 16,
				height: 16,
			},
			32,
			48,
		);

		self.image = skin;
		self.format = SkinFormat::Modern;
		self
	}

	pub fn preprocess(&mut self) {
		const FILL: Rgba<u8> = Rgba([0, 0, 0, 0]);
		const OOB_REGIONS: &[image::math::Rect] = rect_slice![
			[0, 0, 8, 8],
			[24, 0, 16, 8],
			[56, 0, 8, 8],
			[0, 16, 4, 4],
			[12, 16, 8, 4],
			[36, 16, 8, 4],
			[52, 16, 12, 4],
			[56, 20, 8, 28]
		];
		const OOB_REGIONS_MODERN: &[image::math::Rect] = rect_slice![
			[0, 32, 4, 4],
			[12, 32, 8, 4],
			[36, 32, 8, 4],
			[52, 32, 4, 4],
			[0, 48, 4, 4],
			[12, 48, 8, 4],
			[28, 48, 8, 4],
			[44, 48, 8, 4],
			[60, 48, 4, 4]
		];
		const REQUIRED_REGIONS: &[image::math::Rect] = rect_slice![
			[0, 8, 32, 8],
			[8, 0, 16, 8],
			[0, 20, 56, 12],
			[4, 16, 8, 4],
			[20, 16, 16, 4],
			[44, 16, 8, 4],
			[16, 52, 32, 12],
			[20, 48, 8, 4],
			[36, 48, 8, 4]
		];

		// SAFETY: The following unsafe blocks are used for performance reasons,
		// allowing pixel manipulation without bounds checking.
		// The coordinates used here are guaranteed to be within the bounds of the image.
		unsafe {
			for region in OOB_REGIONS {
				self.image.unsafe_fill_rect(region, FILL);
			}

			if self.is_modern() {
				for region in OOB_REGIONS_MODERN {
					self.image.unsafe_fill_rect(region, FILL);
				}
			}

			for region in REQUIRED_REGIONS {
				self.image.unsafe_strip_transparency_rect(region);
			}
		}
	}
}

impl SkinFormat {
	pub fn from_height(height: u32) -> Option<Self> {
		match height {
			LEGACY_SKIN_HEIGHT => Some(Self::Legacy),
			MODERN_SKIN_HEIGHT => Some(Self::Modern),
			_ => None,
		}
	}
}

trait RectOps {
	unsafe fn unsafe_fill_rect(&mut self, rect: &image::math::Rect, pixel: image::Rgba<u8>);
	unsafe fn unsafe_strip_transparency_rect(&mut self, rect: &image::math::Rect);
}

impl RectOps for DynamicImage {
	unsafe fn unsafe_fill_rect(&mut self, rect: &image::math::Rect, pixel: Rgba<u8>) {
		for x in rect.x..rect.x + rect.width {
			for y in rect.y..rect.y + rect.height {
				self.unsafe_put_pixel(x, y, pixel);
			}
		}
	}

	unsafe fn unsafe_strip_transparency_rect(&mut self, rect: &image::math::Rect) {
		for x in rect.x..rect.x + rect.width {
			for y in rect.y..rect.y + rect.height {
				let mut pixel = self.unsafe_get_pixel(x, y);
				let opacity = pixel[3];

				if opacity == u8::MAX {
					continue;
				}

				let opacity = opacity as f32 / 255.0;

				pixel[0] = (opacity * f32::from(pixel[0])) as u8;
				pixel[1] = (opacity * f32::from(pixel[1])) as u8;
				pixel[2] = (opacity * f32::from(pixel[2])) as u8;
				pixel[3] = u8::MAX;

				// SAFETY: x and y are guaranteed to be within the image bounds
				self.unsafe_put_pixel(x, y, pixel);
			}
		}
	}
}
