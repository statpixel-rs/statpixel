use std::{fmt, marker::PhantomData};

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

struct TupleVecMapVisitor<V> {
	marker: PhantomData<Vec<V>>,
}

impl<V> TupleVecMapVisitor<V> {
	fn new() -> Self {
		TupleVecMapVisitor {
			marker: PhantomData,
		}
	}
}

impl<'de, V> Visitor<'de> for TupleVecMapVisitor<V>
where
	V: Deserialize<'de>,
{
	type Value = Vec<V>;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a map")
	}

	#[inline]
	fn visit_unit<E>(self) -> Result<Vec<V>, E> {
		Ok(Vec::new())
	}

	#[inline]
	fn visit_map<T>(self, mut access: T) -> Result<Vec<V>, T::Error>
	where
		T: MapAccess<'de>,
	{
		let mut values = Vec::with_capacity(std::cmp::min(access.size_hint().unwrap_or(0), 4096));

		while let Some((key, value)) = access.next_entry()? {
			// ignore key
			let _: serde::de::IgnoredAny = key;

			values.push(value);
		}

		Ok(values)
	}
}

pub fn deserialize<'de, V, D>(deserializer: D) -> Result<Vec<V>, D::Error>
where
	D: Deserializer<'de>,
	V: Deserialize<'de>,
{
	deserializer.deserialize_map(TupleVecMapVisitor::new())
}
