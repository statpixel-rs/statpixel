use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::WithRejection;
use chrono::{DateTime, Utc};
use database::schema::{track, user};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Track {
	pub guild_id: Option<u64>,
	pub channel_id: u64,
	pub uuid: Uuid,
	pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct TrackInput {
	pub guild_id: Option<u64>,
	pub channel_id: u64,
	pub uuid: Uuid,
}

pub async fn get(
	State(state): State<Arc<super::Data>>,
	claims: super::auth::Claims,
) -> Result<impl IntoResponse, StatusCode> {
	let tracks: Vec<(Option<i64>, i64, Uuid, DateTime<Utc>)> = track::table
		.filter(track::user_id.eq(claims.id as i64))
		.select((
			track::guild_id,
			track::channel_id,
			track::uuid,
			track::created_at,
		))
		.load(
			&mut state
				.pool
				.get()
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
		)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	#[allow(clippy::cast_sign_loss)]
	Ok(Json(
		tracks
			.into_iter()
			.map(|t| Track {
				guild_id: t.0.map(|i| i as u64),
				channel_id: t.1 as u64,
				uuid: t.2,
				created_at: t.3,
			})
			.collect::<Vec<_>>(),
	))
}

pub async fn delete(
	State(state): State<Arc<super::Data>>,
	claims: super::auth::Claims,
	WithRejection(Json(track), _): super::extract::Json<TrackInput>,
) -> Result<impl IntoResponse, StatusCode> {
	let mut connection = state
		.pool
		.get()
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	let status = connection
		.transaction::<_, crate::Error, _>(|connection| {
			async move {
				let status = diesel::delete(
					track::table.filter(
						track::user_id
							.eq(claims.id as i64)
							.and(track::uuid.eq(track.uuid))
							.and(track::channel_id.eq(track.channel_id as i64))
							.and(track::guild_id.eq(track.guild_id.map(|i| i as i64))),
					),
				)
				.execute(&mut state.pool.get().await?)
				.await?;

				if status != 0 {
					diesel::update(user::table.filter(user::id.eq(claims.id as i64)))
						.set(user::tracks.eq(user::tracks - 1))
						.execute(connection)
						.await?;
				}

				Ok(status)
			}
			.scope_boxed()
		})
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	if status == 0 {
		return Ok((
			StatusCode::NOT_FOUND,
			Json(super::error::ErrorMessage {
				success: false,
				message: "unknown track".to_string(),
				origin: "source",
			}),
		)
			.into_response());
	}

	Ok(StatusCode::OK.into_response())
}
