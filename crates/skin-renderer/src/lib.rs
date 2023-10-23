#![feature(iter_array_chunks)]

mod buffer_dimensions;
mod camera;
mod error;
mod instance;
mod light;
mod model;
mod renderer;
mod resources;
mod skin_loader;
mod texture;

use async_once::AsyncOnce;
use error::SkinRendererResult;
use lazy_static::lazy_static;
use renderer::{SkinModelType, SkinRenderer};

lazy_static! {
	static ref SKIN_RENDERER: AsyncOnce<SkinRenderer> = AsyncOnce::new(async {
		SkinRenderer::new(97 * 10, 157 * 10)
			.await
			.expect("Failed to initialize skin renderer")
	});
}

pub async fn render_skin(
	skin: impl Into<Option<&str>>,
	is_slim: bool,
) -> SkinRendererResult<Vec<u8>> {
	let renderer = SKIN_RENDERER.get().await;

	let model_type = if is_slim {
		SkinModelType::Slim
	} else {
		SkinModelType::Classic
	};

	let skin_render = if let Some(skin) = skin.into() {
		renderer.render(model_type, skin).await?
	} else {
		renderer.render_default_texture(model_type).await?
	};

	Ok(skin_render)
}
