pub mod inventory;
pub mod item;

use base64::{engine::general_purpose::STANDARD, Engine};
use flate2::read::GzDecoder;
use serde::{Deserialize, Deserializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("invalid base64")]
	InvalidBase64,
	#[error("invalid nbt")]
	InvalidNbt,
}

#[derive(Deserialize)]
struct Data {
	data: String,
}

/// # Errors
/// [`Error::InvalidBase64`] if the input is not valid base64
/// [`Error::InvalidNbt`] if the decoded + gzipped data is not valid nbt data
pub fn from_data<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: Deserialize<'de>,
{
	let d: Data = Deserialize::deserialize(deserializer)?;

	let bytes = STANDARD
		.decode(d.data.as_str())
		.map_err(|_| serde::de::Error::custom("invalid base64 string"))?;
	let result = GzDecoder::new(&bytes[..]);

	fastnbt::from_reader(result).map_err(serde::de::Error::custom)
}
