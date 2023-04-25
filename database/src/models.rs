use diesel::prelude::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
	pub id: i64,
	pub uuid: Option<Uuid>,
	pub text: bool,
}

#[derive(Queryable)]
pub struct History {
	pub uuid: Uuid,
	pub game_id: i16,
	pub game_mode: i16,
	pub update_frequency: Option<i16>,
	pub created_at: chrono::DateTime<chrono::Utc>,
	pub updated_at: chrono::DateTime<chrono::Utc>,
}
