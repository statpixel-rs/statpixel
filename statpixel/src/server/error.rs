use axum::{
	extract::rejection::{JsonRejection, QueryRejection},
	response::IntoResponse,
	Json,
};
use serde::Serialize;
use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
pub enum ServerError {
	#[error(transparent)]
	JsonExtractorRejection(#[from] JsonRejection),
	#[error(transparent)]
	QueryExtractorRejection(#[from] QueryRejection),
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize)]
pub struct ErrorMessage {
	pub success: bool,
	pub message: String,
	pub origin: &'static str,
}

impl IntoResponse for ServerError {
	fn into_response(self) -> axum::response::Response {
		let (status, message) = match self {
			ServerError::JsonExtractorRejection(rejection) => {
				(rejection.status(), rejection.body_text())
			}
			ServerError::QueryExtractorRejection(rejection) => {
				(rejection.status(), rejection.body_text())
			}
		};

		(
			status,
			Json(ErrorMessage {
				success: false,
				origin: "with_rejection",
				message,
			}),
		)
			.into_response()
	}
}
