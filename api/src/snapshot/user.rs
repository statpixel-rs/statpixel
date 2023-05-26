use crate::player::data::Data;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use translate::Error;

/// # Errors
/// Returns an error if the data could not be encoded.
pub fn encode(data: &Data) -> Result<Vec<u8>, Error> {
	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

	bincode::encode_into_std_write(data, &mut encoder, bincode::config::standard())?;

	Ok(encoder.finish()?)
}

/// # Errors
/// Returns an error if the data could not be decoded.
pub fn decode(data: &[u8]) -> Result<Data, Error> {
	let mut decoder = ZlibDecoder::new(data);

	Ok(bincode::decode_from_std_read(
		&mut decoder,
		bincode::config::standard(),
	)?)
}
