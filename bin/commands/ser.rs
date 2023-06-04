use std::io::{Read, Write};

use api::player::data::Data;
use flate2::Compression;
use translate::{Context, Error};

use crate::util::escape_username;

/// Shows the serialized size of a player's profile.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn ser(
	ctx: Context<'_>,
	#[max_length = 16] username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let (player, data) = crate::commands::get_player_data(ctx, uuid, username).await?;

	let ser = bincode::encode_to_vec(&data, bincode::config::standard()).unwrap();
	let serialized_bytes = ser.len();

	let mut c = flate2::write::ZlibEncoder::new(Vec::new(), Compression::default());

	c.write_all(&ser[..]).unwrap();

	let c = c.finish().unwrap();
	let compressed_bytes = c.len();

	let mut dec = flate2::read::ZlibDecoder::new(&c[..]);
	let mut d = Vec::new();

	dec.read_to_end(&mut d).unwrap();

	let (dec_data, _): (Data, _) =
		bincode::decode_from_slice(&d[..], bincode::config::standard()).unwrap();

	assert_eq!(
		data, dec_data,
		"deserialized data does not match serialized data"
	);

	ctx.send(|m| {
		m.content(format!(
			"**{}** profile size\n{serialized_bytes} serialized bytes ({compressed_bytes} compressed)",
			escape_username(&player.username),
		))
	})
	.await?;

	Ok(())
}
