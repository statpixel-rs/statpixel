use std::fmt;

use serde::de::Visitor;
use serde::Deserializer;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
	D: Deserializer<'de>,
{
	struct F64StringVisitor;

	impl<'de> Visitor<'de> for F64StringVisitor {
		type Value = Option<f64>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("a string")
		}

		#[inline]
		fn visit_str<E>(self, value: &str) -> Result<Option<f64>, E>
		where
			E: serde::de::Error,
		{
			Some(value.parse().map_err(serde::de::Error::custom)).transpose()
		}
	}

	deserializer.deserialize_str(F64StringVisitor)
}
