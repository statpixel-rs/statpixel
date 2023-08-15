use std::sync::Arc;

use axum::{
	extract::State,
	headers::{
		authorization::{self, Credentials},
		Authorization,
	},
	http::{HeaderValue, StatusCode},
	response::IntoResponse,
	Json, TypedHeader,
};
use axum_extra::extract::WithRejection;
use chrono::{Duration, Utc};
use database::schema::user;
use diesel::{
	dsl::sql,
	sql_types::{Nullable, Timestamptz},
	ExpressionMethods,
};
use diesel_async::RunQueryDsl;
use serde::Deserialize;

pub struct Plain(pub String);

impl Credentials for Plain {
	const SCHEME: &'static str = "Plain";

	fn decode(value: &HeaderValue) -> Option<Self> {
		value.to_str().ok().map(|s| Self(s.to_owned()))
	}

	fn encode(&self) -> HeaderValue {
		self.0.parse().unwrap()
	}
}

#[derive(Deserialize)]
pub struct Vote {
	pub user: String,
	#[serde(rename = "isWeekend")]
	pub is_weekend: bool,
}

const SECRET: &str = dotenvy_macro::dotenv!("TOPGG_SECRET");

pub async fn add_vote(
	State(state): State<Arc<super::Data>>,
	TypedHeader(bearer): TypedHeader<Authorization<authorization::Bearer>>,
	WithRejection(Json(vote), _): super::extract::Json<Vote>,
) -> Result<impl IntoResponse, StatusCode> {
	if bearer.token() != SECRET {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let Ok(id) = vote.user.parse::<u64>() else {
		return Err(StatusCode::BAD_REQUEST);
	};

	diesel::insert_into(user::table)
		.values((
			user::id.eq(id as i64),
			user::votes.eq(1),
			user::premium_until.eq(Utc::now() + Duration::days(3)),
		))
		.on_conflict(user::id)
		.do_update()
		.set((
			user::votes.eq(user::votes + if vote.is_weekend { 2 } else { 1 }),
			user::premium_until.eq(
				sql::<Nullable<Timestamptz>>("CASE WHEN premium_until IS NULL THEN NULL WHEN premium_until < NOW() THEN NOW() + INTERVAL '3 days' ELSE premium_until + INTERVAL '3 days' END")
			),
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
