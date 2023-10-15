use std::fmt::Debug;

use base64::Engine;
use serde::Deserializer;

pub(crate) fn json<'de, 'a, D, T: serde::de::DeserializeOwned + Debug>(
	deserializer: D,
) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
{
	let s: String = serde::Deserialize::deserialize(deserializer)?;
	let bytes = base64::engine::general_purpose::STANDARD
		.decode(s)
		.map_err(serde::de::Error::custom)?;

	serde_json::from_slice(&bytes).map_err(serde::de::Error::custom)
}
