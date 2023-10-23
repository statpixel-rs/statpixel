use reqwest::Client;

use crate::{error, skin::Skin, SkinKind};

pub struct SkinLoader {
	client: Client,
}

impl SkinLoader {
	pub fn new() -> Self {
		Self {
			client: Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/118.0").build().unwrap(),
		}
	}

	pub async fn load_from_url(&self, url: impl reqwest::IntoUrl) -> error::Result<Skin> {
		let response = self.client.get(url).send().await?;

		if response.status().is_success() {
			let bytes = response.bytes().await?;
			let image = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)?;

			// TODO: Implement automatic detection of skin type (slim or classic) based on the transparency of the first layer.
			let mut skin = Skin::new(image, SkinKind::Classic)
				.ok_or(error::Error::InvalidTextureDimensions)?
				.into_modern();

			skin.preprocess();

			Ok(skin)
		} else {
			Err(error::Error::MissingSkinTexture)
		}
	}
}
