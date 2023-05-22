use diesel::{
	backend::Backend,
	deserialize::FromSql,
	pg::Pg,
	serialize::{self, Output, ToSql},
	sql_types::SmallInt,
	AsExpression, FromSqlRow,
};

#[derive(poise::ChoiceParameter, Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = SmallInt)]
pub enum Display {
	Image,
	Compact,
	Text,
}

impl FromSql<SmallInt, Pg> for Display {
	fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
		Ok(match i16::from_sql(bytes)? {
			0 => Display::Image,
			1 => Display::Compact,
			2 => Display::Text,
			_ => unreachable!(),
		})
	}
}

impl<Db> ToSql<SmallInt, Db> for Display
where
	Db: Backend,
	i16: ToSql<SmallInt, Db>,
{
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Db>) -> serialize::Result {
		match self {
			Display::Image => 0.to_sql(out),
			Display::Compact => 1.to_sql(out),
			Display::Text => 2.to_sql(out),
		}
	}
}
