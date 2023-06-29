use serde::{Deserialize, Deserializer};

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub(crate) fn f32_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f32 = Deserialize::deserialize(deserializer)?;

	Ok(s as u32)
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn f32_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f32 = Deserialize::deserialize(deserializer)?;

	Ok(s as i32)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub(crate) fn f64_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>,
{
	let s: f64 = Deserialize::deserialize(deserializer)?;

	Ok(s as u64)
}
