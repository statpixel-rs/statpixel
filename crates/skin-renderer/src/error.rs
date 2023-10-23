#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	ObjLoad(#[from] tobj::LoadError),
	#[error("A material is missing a diffuse texture")]
	MissingDiffuseTexture,
	#[error("The provided url is not a valid skin texture")]
	MissingSkinTexture,
	#[error("The provided skin is invalid")]
	InvalidTextureDimensions,
	#[error("Reqwest error: status: {:?}", .0.status())]
	Reqwest(#[from] reqwest::Error),
	#[error(transparent)]
	Image(#[from] image::ImageError),
	#[error("Failed to render skin")]
	RenderFailure,
	#[error("The renderer has not been initialized")]
	Uninitialized,
}

pub type Result<T> = std::result::Result<T, Error>;
