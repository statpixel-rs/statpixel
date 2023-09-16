use std::sync::Arc;

use api::snapshot::user::decode;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use database::schema::{schedule, snapshot};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

pub async fn post(
	State(state): State<Arc<super::Data>>,
	body_bytes: axum::body::Bytes,
) -> Result<impl IntoResponse, StatusCode> {
	let data = decode(&body_bytes[..]).map_err(|_| StatusCode::BAD_REQUEST)?;
	let hash = fxhash::hash64(&body_bytes[..]) as i64;

	let prev_hash = diesel::update(
		schedule::table
			.filter(schedule::uuid.eq(data.uuid))
			.filter(schedule::vendor_update_at.lt(Utc::now() - Duration::minutes(15))),
	)
	.set((
		schedule::vendor_update_at.eq(Utc::now()),
		schedule::vendor_prev_hash.eq(schedule::vendor_hash),
		schedule::vendor_hash.eq(hash),
	))
	.returning(schedule::vendor_prev_hash)
	.get_result::<Option<i64>>(
		&mut state
			.pool
			.get()
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
	)
	.await
	.map_err(|_| StatusCode::NOT_FOUND)?;

	diesel::insert_into(snapshot::table)
		.values((
			snapshot::uuid.eq(data.uuid),
			snapshot::data.eq(&body_bytes[..]),
			snapshot::hash.eq(hash),
			snapshot::version.eq(api::player::VERSION),
			snapshot::trusted.eq(false),
			snapshot::did_update.eq(prev_hash.map(|h| h == hash) != Some(true)),
		))
		.execute(
			&mut state
				.pool
				.get()
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
		)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(())
}
