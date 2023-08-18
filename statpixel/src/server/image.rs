use std::sync::Arc;

use axum::{
	extract::{Path, Query, State},
	http::{header, StatusCode},
	response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use translate::context;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize)]
pub struct ImageQuery {
	bg: Option<u32>,
	tl: Option<translate::context::Locale>,
}

pub async fn get(
	State(state): State<Arc<super::Data>>,
	Path(id): Path<String>,
	WithRejection(Query(ImageQuery { bg, tl }), _): super::extract::Query<ImageQuery>,
) -> Result<impl IntoResponse, StatusCode> {
	let api::id::Id::Command(id) = api::id::decode(&id).ok_or(StatusCode::NOT_FOUND)? else {
		return Err(StatusCode::NOT_FOUND);
	};

	let Ok(image) = crate::id::image::map(
		&context::Context::external_with_locale(&state, tl),
		id,
		bg.map(std::convert::Into::into),
	)
	.await
	else {
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
