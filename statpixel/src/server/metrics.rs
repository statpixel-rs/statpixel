use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use database::schema::usage;
use diesel::{dsl::sum, NullableExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::Serialize;
use statpixel::GUILDS;

#[derive(Serialize)]
pub struct Metrics {
	guilds: usize,
	commands: i64,
}

pub async fn get(State(state): State<Arc<super::Data>>) -> Result<impl IntoResponse, StatusCode> {
	let guilds = GUILDS.read().await.len();
	let commands = usage::table
		.select(sum(usage::count).assume_not_null())
		.get_result::<i64>(
			&mut state
				.pool
				.get()
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
		)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(Json(Metrics { guilds, commands }))
}
