use std::sync::Arc;

use axum::{
	extract::{Path, State},
	http::{header, StatusCode},
	response::IntoResponse,
};
use translate::context;

pub async fn get(
	State(state): State<Arc<super::Data>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
	let api::id::Id::Command(id) = api::id::decode(&id).ok_or(StatusCode::NOT_FOUND)? else {
		return Err(StatusCode::NOT_FOUND);
	};

	let Some(image) = crate::id::image::map(&context::Context::external(&state), id).await else {
		return Err(StatusCode::NOT_FOUND);
	};

	Ok((
		StatusCode::OK,
		[
			(header::CONTENT_TYPE, "image/png"),
			(header::CACHE_CONTROL, "public, max-age=86400"),
		],
		image,
	))
}
