use axum::{
	extract::rejection::{JsonRejection, QueryRejection},
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::Serialize;
use thiserror::Error;
use tower::BoxError;
use tower_governor::GovernorError;

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
	fn into_response(self) -> Response {
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

#[allow(clippy::module_name_repetitions)]
pub fn display_error(e: BoxError) -> Response {
	if e.is::<GovernorError>() {
		// It shouldn't be possible for this to panic, since we already know it's a GovernorError
		let error = *e.downcast::<GovernorError>().unwrap();

		match error {
			GovernorError::TooManyRequests { headers, .. } => (
				StatusCode::TOO_MANY_REQUESTS,
				headers,
				Json(ErrorMessage {
					success: false,
					origin: "governor",
					message: "too many requests".to_string(),
				}),
			)
				.into_response(),
			GovernorError::UnableToExtractKey => (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(ErrorMessage {
					success: false,
					origin: "governor",
					message: "unable to extract key".to_string(),
				}),
			)
				.into_response(),
			GovernorError::Other { msg, code, headers } => (
				code,
				headers,
				Json(ErrorMessage {
					success: false,
					origin: "governor",
					message: if let Some(msg) = msg {
						msg
					} else {
						"unknown governor error".to_string()
					},
				}),
			)
				.into_response(),
		}
	} else {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			Json(ErrorMessage {
				success: false,
				origin: "governor",
				message: "internal server error".to_string(),
			}),
		)
			.into_response()
	}
}
