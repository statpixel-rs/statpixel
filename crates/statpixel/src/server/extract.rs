use axum_extra::extract::WithRejection;

pub type Json<T> = WithRejection<axum::extract::Json<T>, super::error::ServerError>;
pub type Query<T> = WithRejection<axum::extract::Query<T>, super::error::ServerError>;
