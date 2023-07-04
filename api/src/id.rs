use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub enum Id {
	Command(super::command::Id),
	Builder(super::builder::Id),
}

#[must_use]
pub fn builder(id: super::builder::Id) -> String {
	encode(Id::Builder(id))
}

#[must_use]
pub fn command(id: super::command::Id) -> String {
	encode(Id::Command(id))
}

/// # Panics
/// Panics if the data cannot be encoded
#[must_use]
pub fn encode(id: Id) -> String {
	let bytes = bincode::encode_to_vec(id, bincode::config::standard()).unwrap();
	URL_SAFE_NO_PAD.encode(bytes)
}

#[must_use]
pub fn decode(id: &str) -> Option<Id> {
	let bytes = URL_SAFE_NO_PAD.decode(id.as_bytes()).ok()?;

	bincode::decode_from_slice(&bytes, bincode::config::standard())
		.map(|o| o.0)
		.ok()
}
