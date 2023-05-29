use serde::{Deserialize, Deserializer};
use uuid::Uuid;

#[derive(bincode::Encode, bincode::Decode, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct SkyBlock {
	#[serde(deserialize_with = "from_profile_map")]
	pub profiles: Vec<Profile>,
}

#[derive(bincode::Encode, bincode::Decode, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Profile {
	#[serde(rename = "profile_id")]
	#[bincode(with_serde)]
	pub id: Uuid,
	#[serde(rename = "cute_name")]
	pub name: String,
}

fn from_profile_map<'de, D>(deserializer: D) -> Result<Vec<Profile>, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = Vec<Profile>;

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a mapping of profiles to profile + profile type")
		}

		fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			let mut vec = Vec::with_capacity(map.size_hint().unwrap_or(0));

			while let Some((p, profile)) = map.next_entry()? {
				let _: &str = p;

				vec.push(profile);
			}

			Ok(vec)
		}
	}

	deserializer.deserialize_map(Visitor)
}
