// add api to delete one track, view all tracks

use std::sync::Arc;

use axum::{
	extract::{Path, State},
	http::StatusCode,
	response::IntoResponse,
	Json,
};
use axum_extra::extract::WithRejection;
use chrono::{DateTime, Utc};
use database::schema::{boost, user};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Boost {
	pub guild_id: u64,
	pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct BoostInput {
	pub guild_id: u64,
}

pub async fn get(
	State(state): State<Arc<super::Data>>,
	claims: super::auth::Claims,
) -> Result<impl IntoResponse, StatusCode> {
	let tracks: Vec<(i64, DateTime<Utc>)> = boost::table
		.filter(boost::user_id.eq(claims.id as i64))
		.select((boost::guild_id, boost::created_at))
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
			.map(|t| Boost {
				guild_id: t.0 as u64,
				created_at: t.1,
			})
			.collect::<Vec<_>>(),
	))
}

pub async fn delete(
	State(state): State<Arc<super::Data>>,
	claims: super::auth::Claims,
	WithRejection(Json(boost), _): super::extract::Json<BoostInput>,
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
					boost::table.filter(
						boost::user_id
							.eq(claims.id as i64)
							.and(boost::guild_id.eq(boost.guild_id as i64)),
					),
				)
				.execute(&mut state.pool.get().await?)
				.await?;

				if status != 0 {
					diesel::update(user::table.filter(user::id.eq(claims.id as i64)))
						.set(user::boosts.eq(user::boosts - 1))
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
				message: "unknown boost".to_string(),
				origin: "source",
			}),
		)
			.into_response());
	}

	Ok(StatusCode::OK.into_response())
}

pub async fn create(
	State(state): State<Arc<super::Data>>,
	claims: super::auth::Claims,
	Path(guild_id): Path<u64>,
) -> Result<impl IntoResponse, StatusCode> {
	diesel::insert_into(boost::table)
		.values((
			boost::user_id.eq(claims.id as i64),
			boost::guild_id.eq(guild_id as i64),
		))
		.execute(
			&mut state
				.pool
				.get()
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
		)
		.await
		.map_err(|_| StatusCode::CONFLICT)?;

	Ok(())
}
