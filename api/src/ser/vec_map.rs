use std::{fmt, marker::PhantomData};

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

struct TupleVecMapVisitor<K, V> {
	marker: PhantomData<Vec<(K, V)>>,
}

impl<K, V> TupleVecMapVisitor<K, V> {
	fn new() -> Self {
		TupleVecMapVisitor {
			marker: PhantomData,
		}
	}
}

impl<'de, K, V> Visitor<'de> for TupleVecMapVisitor<K, V>
where
	K: Deserialize<'de>,
	V: Deserialize<'de>,
{
	type Value = Vec<(K, V)>;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a map")
	}

	#[inline]
	fn visit_unit<E>(self) -> Result<Vec<(K, V)>, E> {
		Ok(Vec::new())
	}

	#[inline]
	fn visit_map<T>(self, mut access: T) -> Result<Vec<(K, V)>, T::Error>
	where
		T: MapAccess<'de>,
	{
		let mut values = Vec::with_capacity(std::cmp::min(access.size_hint().unwrap_or(0), 4096));

		while let Some((key, value)) = access.next_entry()? {
			values.push((key, value));
		}

		Ok(values)
	}
}

pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<Vec<(K, V)>, D::Error>
where
	D: Deserializer<'de>,
	K: Deserialize<'de>,
	V: Deserialize<'de>,
{
	deserializer.deserialize_map(TupleVecMapVisitor::new())
}
