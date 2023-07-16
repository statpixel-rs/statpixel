use std::fmt;

use serde::{
	de::{self, MapAccess, Visitor},
	Deserialize, Deserializer,
};

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, PartialEq, Default)]
pub struct Socials {
	pub discord: Option<String>,
}

impl<'de> Deserialize<'de> for Socials {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize, Default)]
		#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
		struct Links {
			discord: Option<String>,
		}

		struct ContainerVisitor;

		impl<'de> Visitor<'de> for ContainerVisitor {
			type Value = Socials;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("struct Container")
			}

			fn visit_map<V>(self, mut map: V) -> Result<Socials, V::Error>
			where
				V: MapAccess<'de>,
			{
				let mut links = None::<Links>;

				while let Some(key) = map.next_key()? {
					match key {
						"links" => {
							if links.is_some() {
								return Err(de::Error::duplicate_field("links"));
							}

							links = Some(map.next_value()?);
						}
						_ => {
							let _ = map.next_value::<serde::de::IgnoredAny>()?;
						}
					}
				}

				let links = links.unwrap_or_default();

				Ok(Socials {
					discord: links.discord,
				})
			}
		}

		const FIELDS: &[&str] = &["links"];
		deserializer.deserialize_struct("Container", FIELDS, ContainerVisitor)
	}
}
