use std::fmt;

use base64::{engine::general_purpose::STANDARD, Engine};
use flate2::read::GzDecoder;
use serde::de::{MapAccess, Visitor};
use serde::Deserializer;

use crate::nbt::inventory::Inventory;
use crate::nbt::Data;

struct TupleVecMapVisitor;

impl<'de> Visitor<'de> for TupleVecMapVisitor {
	type Value = Vec<Inventory>;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a map")
	}

	#[inline]
	fn visit_unit<E>(self) -> Result<Vec<Inventory>, E> {
		Ok(Vec::new())
	}

	#[inline]
	fn visit_map<T>(self, mut access: T) -> Result<Vec<Inventory>, T::Error>
	where
		T: MapAccess<'de>,
	{
		let mut values = Vec::with_capacity(std::cmp::min(access.size_hint().unwrap_or(0), 4096));

		while let Some((key, v)) = access.next_entry()? {
			let v: Data = v;
			let _: String = key;

			let bytes = STANDARD
				.decode(v.data.as_str())
				.map_err(|_| serde::de::Error::custom("invalid base64 string"))?;
			let result = GzDecoder::new(&bytes[..]);

			values.push(fastnbt::from_reader(result).map_err(serde::de::Error::custom)?);
		}

		Ok(values)
	}
}

/// # Errors
/// See serde for errors
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Inventory>, D::Error>
where
	D: Deserializer<'de>,
{
	deserializer.deserialize_map(TupleVecMapVisitor)
}
