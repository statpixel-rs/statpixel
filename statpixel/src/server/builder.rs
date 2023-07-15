use api::{
	builder::{Location, Shape, ShapeData, Statistic},
	player::stats::arena,
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use minecraft::paint::Paint;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const DEFAULT_UUID: Uuid = Uuid::from_u128(0xf7c77d99_9f15_4a66_a87d_c4a51ef30d19);

#[derive(Serialize)]
pub struct PreviewResponse {
	pub success: bool,
	pub id: String,
}

#[derive(Deserialize)]
pub struct PreviewQuery {
	pub uuid: Option<Uuid>,
}

#[allow(clippy::unused_async)]
pub async fn preview(
	_: super::auth::Claims,
	WithRejection(Query(PreviewQuery { uuid }), _): super::extract::Query<PreviewQuery>,
	WithRejection(Json(shapes), _): super::extract::Json<Vec<Shape>>,
) -> impl IntoResponse {
	if shapes.len() > 32 {
		return (
			StatusCode::BAD_REQUEST,
			Json(super::error::ErrorMessage {
				success: false,
				origin: "with_rejection",
				message: "too many shapes".to_string(),
			}),
		)
			.into_response();
	}

	let id = api::id::command(api::command::Id::Builder {
		shapes,
		uuid: uuid.unwrap_or(DEFAULT_UUID),
	});

	(StatusCode::OK, Json(PreviewResponse { success: true, id })).into_response()
}

#[allow(clippy::unused_async)]
pub async fn get() -> impl IntoResponse {
	Json(vec![
		Shape {
			location: Location::Down,
			colour: Paint::Red,
			data: ShapeData::Bubble {
				statistic: Statistic::Arena {
					kind: arena::ArenaKind::kills,
				},
			},
		},
		Shape {
			location: Location::Down,
			colour: Paint::Red,
			data: ShapeData::Title,
		},
	])
}
