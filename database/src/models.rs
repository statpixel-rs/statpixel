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

#[derive(Queryable)]
pub struct Metric {
	pub id: i64,
	pub kind: MetricKind,
	pub created_at: chrono::DateTime<chrono::Utc>,
}

pub enum MetricKind {
	GuildJoin,
	GuildLeave,
	ProfileCreate,
	CommandRun,
}

impl From<i16> for MetricKind {
	fn from(kind: i16) -> Self {
		match kind {
			0 => Self::GuildJoin,
			1 => Self::GuildLeave,
			2 => Self::ProfileCreate,
			3 => Self::CommandRun,
			_ => unreachable!(),
		}
	}
}

impl From<MetricKind> for i16 {
	fn from(kind: MetricKind) -> Self {
		match kind {
			MetricKind::GuildJoin => 0,
			MetricKind::GuildLeave => 1,
			MetricKind::ProfileCreate => 2,
			MetricKind::CommandRun => 3,
		}
	}
}
