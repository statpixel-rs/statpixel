use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub enum Id {
	Command(super::command::Id),
	Builder(super::builder::Id),
}

/// # Errors
/// Returns [`crate::Error::IdentifierTooLong`] if the encoded string is longer than 100 characters
pub fn builder(id: super::builder::Id) -> crate::Result<String> {
	let string = encode(&Id::Builder(id));

	// Discord only allows custom_id to be 100 characters,
	// so deny it if its any longer regardless of what it
	// will be used for
	if string.len() > 100 {
		Err(translate::Error::IdentifierTooLong)
	} else {
		Ok(string)
	}
}

#[must_use]
pub fn command(id: super::command::Id) -> String {
	encode(&Id::Command(id))
}

/// # Panics
/// Panics if the data cannot be encoded
#[must_use]
pub fn encode(id: &Id) -> String {
	URL_SAFE_NO_PAD.encode(bitcode::encode(&id).unwrap())
}

#[must_use]
pub fn decode(id: &str) -> Option<Id> {
	bitcode::decode(URL_SAFE_NO_PAD.decode(id.as_bytes()).ok()?.as_slice()).ok()
}
