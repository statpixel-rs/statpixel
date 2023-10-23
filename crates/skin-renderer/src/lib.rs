#![feature(iter_array_chunks)]

mod camera;
mod dimensions;
mod error;
mod instance;
mod light;
mod model;
mod renderer;
mod resources;
mod skin;
mod texture;

pub use reqwest::IntoUrl;
pub use skin::SkinKind;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use renderer::SkinRenderer;

lazy_static! {
	static ref SKIN_RENDERER: AsyncOnce<SkinRenderer> = AsyncOnce::new(async {
		SkinRenderer::new(97, 157)
			.await
			.expect("Failed to initialize skin renderer")
	});
}

pub async fn render_skin(
	kind: SkinKind,
	skin: Option<impl reqwest::IntoUrl>,
) -> error::Result<Vec<u8>> {
	SKIN_RENDERER.get().await.render(kind, skin).await
}
