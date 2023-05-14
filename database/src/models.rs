use diesel::prelude::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
	pub id: i64,
	pub uuid: Option<Uuid>,
	pub text: bool,
}

#[derive(Queryable)]
pub struct Snapshot {
	pub id: i64,
	pub uuid: Uuid,
	pub data: Vec<u8>,

	pub created_at: chrono::DateTime<chrono::Utc>,
	pub updated_at: chrono::DateTime<chrono::Utc>,
}
