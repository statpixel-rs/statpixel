use axum::{
	http::{HeaderMap, StatusCode},
	response::IntoResponse,
};

pub async fn get(headers: HeaderMap) -> impl IntoResponse {
	println!("{headers:?}");

	StatusCode::OK
}
