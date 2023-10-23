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

use std::sync::Arc;

pub use reqwest::IntoUrl;
pub use skin::SkinKind;

use renderer::SkinRenderer;
use tokio::sync::OnceCell;

pub static SKIN_RENDERER: OnceCell<Arc<SkinRenderer>> = OnceCell::const_new();

pub async fn render_skin(
	kind: SkinKind,
	skin: Option<impl reqwest::IntoUrl>,
) -> error::Result<Vec<u8>> {
	SKIN_RENDERER
		.get_or_init(create_renderer)
		.await
		.render(kind, skin)
		.await
}

pub async fn create_renderer() -> Arc<SkinRenderer> {
	Arc::new(
		SkinRenderer::new(97, 157)
			.await
			.expect("failed to create renderer"),
	)
}
