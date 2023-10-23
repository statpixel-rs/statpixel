use image::{DynamicImage, GenericImage, GenericImageView, ImageResult, Rgba};
use reqwest::Client;

use crate::error::{SkinRendererError, SkinRendererResult};

pub struct SkinLoader {
	client: Client,
}

#[derive(Debug)]
enum SkinFormat {
	/// Legacy skin format (64x32)
	Legacy = 32,
	/// Modern skin format (64x64)
	Modern = 64,
}

trait SkinSize {
	fn width(&self) -> u32;
	fn height(&self) -> u32;
}

impl SkinSize for SkinFormat {
	fn height(&self) -> u32 {
		match self {
			SkinFormat::Legacy => 32,
			SkinFormat::Modern => 64,
		}
	}

	fn width(&self) -> u32 {
		match self {
			SkinFormat::Legacy => 64,
			SkinFormat::Modern => 64,
		}
	}
}

impl SkinLoader {
	pub fn new() -> Self {
		Self {
			client: Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/118.0").build().unwrap(),
		}
	}

	pub async fn get_skin(&self, url: &str) -> SkinRendererResult<DynamicImage> {
		if url == "input" {
			let skin = image::open("input.png")?;
			return Ok(skin);
		}

		let response = self.client.get(url).send().await?;

		if response.status().is_success() {
			let bytes = response.bytes().await?;
			let image = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)?;
			let skin = Self::process_skin(image)?;

			Ok(skin)
		} else {
			Err(SkinRendererError::MissingSkinTexture)
		}
	}

	fn process_skin(skin: DynamicImage) -> ImageResult<DynamicImage> {
		let format = Self::skin_format(&skin);

		let mut skin = match format {
			SkinFormat::Legacy => Self::convert_legacy_skin(skin)?,
			SkinFormat::Modern => skin,
		};

		Self::fix_opaque_skin(&mut skin, format);
		Self::fix_transparent_skin(&mut skin);

		Ok(skin)
	}

	fn skin_format(skin: &DynamicImage) -> SkinFormat {
		if skin.height() == SkinFormat::Legacy.height() {
			SkinFormat::Legacy
		} else {
			SkinFormat::Modern
		}
	}

	/// Legacy skins are 64x32 and are missing separate textures for the left leg and arm.
	/// Legacy skins also do not have any 2nd layer textures.
	fn convert_legacy_skin(legacy_skin: DynamicImage) -> ImageResult<DynamicImage> {
		let modern = SkinFormat::Modern;
		let mut skin = DynamicImage::new_rgba8(modern.width(), modern.height());

		skin.copy_from(&legacy_skin, 0, 0)?;

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

		Ok(skin)
	}

	// See https://github.com/bs-community/skinview3d/issues/93
	/// Fixes skins with opaque backgrounds by making the background transparent.
	fn fix_opaque_skin(skin: &mut DynamicImage, original_format: SkinFormat) {
		let transparent = Rgba([0, 0, 0, 0]);

		// force all out-of-bounds locations to be transparent
		fill_rect(skin, 0, 0, 8, 8, transparent);
		fill_rect(skin, 24, 0, 16, 8, transparent);
		fill_rect(skin, 56, 0, 8, 8, transparent);
		fill_rect(skin, 0, 16, 4, 4, transparent);
		fill_rect(skin, 12, 16, 8, 4, transparent);
		fill_rect(skin, 36, 16, 8, 4, transparent);
		fill_rect(skin, 52, 16, 12, 4, transparent);
		fill_rect(skin, 56, 20, 8, 28, transparent);

		if matches!(original_format, SkinFormat::Modern) {
			fill_rect(skin, 0, 32, 4, 4, transparent);
			fill_rect(skin, 12, 32, 8, 4, transparent);
			fill_rect(skin, 36, 32, 8, 4, transparent);
			fill_rect(skin, 52, 32, 4, 4, transparent);
			fill_rect(skin, 0, 48, 4, 4, transparent);
			fill_rect(skin, 12, 48, 8, 4, transparent);
			fill_rect(skin, 28, 48, 8, 4, transparent);
			fill_rect(skin, 44, 48, 8, 4, transparent);
			fill_rect(skin, 60, 48, 4, 4, transparent);
		}
	}

	fn fix_transparent_skin(skin: &mut DynamicImage) {
		remove_transparency(skin, 0, 8, 32, 8);
		remove_transparency(skin, 8, 0, 16, 8);
		remove_transparency(skin, 0, 20, 56, 12);
		remove_transparency(skin, 4, 16, 8, 4);
		remove_transparency(skin, 20, 16, 16, 4);
		remove_transparency(skin, 44, 16, 8, 4);
		remove_transparency(skin, 16, 52, 32, 12);
		remove_transparency(skin, 20, 48, 8, 4);
		remove_transparency(skin, 36, 48, 8, 4);
	}
}

fn fill_rect(image: &mut DynamicImage, x: u32, y: u32, width: u32, height: u32, pixel: Rgba<u8>) {
	for x in x..x + width {
		for y in y..y + height {
			image.put_pixel(x, y, pixel);
		}
	}
}

fn remove_transparency(image: &mut DynamicImage, x: u32, y: u32, width: u32, height: u32) {
	for x in x..x + width {
		for y in y..y + height {
			let mut pixel = image.get_pixel(x, y);
			let opacity = pixel[3];

			if opacity == u8::MAX {
				continue;
			}

			let opacity = opacity as f32 / 255.0;

			pixel[0] = (opacity * pixel[0] as f32) as u8;
			pixel[1] = (opacity * pixel[1] as f32) as u8;
			pixel[2] = (opacity * pixel[2] as f32) as u8;
			pixel[3] = 255;

			image.put_pixel(x, y, pixel);
		}
	}
}
