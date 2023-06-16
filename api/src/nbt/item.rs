use base64::{engine::general_purpose::STANDARD, Engine};
use flate2::read::GzDecoder;
use serde::{Deserialize, Deserializer};

mod item {
	use serde::Deserialize;

	#[derive(Deserialize, Debug)]
	pub struct Item {
		pub i: Vec<Wrapper>,
	}

	#[derive(Deserialize, Debug)]
	pub struct Wrapper {
		pub tag: Tag,
	}

	#[derive(Deserialize, Debug)]
	pub struct Tag {
		pub display: Display,
	}

	#[derive(Deserialize, Debug)]
	pub struct Display {
		#[serde(rename = "Name")]
		pub name: String,
	}
}

/// # Errors
/// See [`parse_from_encoded`]
pub fn parse_item_name(input: &str) -> Result<String, super::Error> {
	let item: item::Item = parse_from_encoded(input)?;
	let item = item.i.into_iter().next().ok_or(super::Error::InvalidNbt)?;

	Ok(item.tag.display.name)
}

/// # Errors
/// [`Error::InvalidBase64`] if the input is not valid base64
///
/// [`Error::InvalidNbt`] if the decoded + gzipped data is not valid nbt data
pub fn parse_from_encoded<'i, T>(input: &'i str) -> Result<T, super::Error>
where
	T: Deserialize<'i>,
{
	let bytes = STANDARD
		.decode(input)
		.map_err(|_| super::Error::InvalidBase64)?;
	let result = GzDecoder::new(&bytes[..]);

	fastnbt::from_reader(result).map_err(|_| super::Error::InvalidNbt)
}

/// # Errors
/// See [`parse_from_encoded`]
pub fn item_name_from_gzipped_base64<'de, D>(deserializer: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = String;

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a base64-encoded gzipped nbt string")
		}

		fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			parse_item_name(s).map_err(serde::de::Error::custom)
		}
	}

	deserializer.deserialize_str(Visitor)
}
