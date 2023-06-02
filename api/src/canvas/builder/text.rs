use minecraft::text::{parse::minecraft_string, Text};

use crate::player::data::Data;

#[must_use]
pub fn from_data<'u>(data: &'u Data, username: &'u str) -> Vec<Text<'u>> {
	let rank = data.get_rank();

	// TODO: Allocate entire vec size immediately
	let mut text = if let Some(text) = rank.get_text() {
		text.to_vec()
	} else if let Some(prefix) = data.prefix.as_ref() {
		minecraft_string(prefix).by_ref().collect()
	} else {
		vec![]
	};

	if !text.is_empty() {
		text.reserve_exact(2);
		text.push(Text {
			text: " ",
			..Default::default()
		});
	}

	text.push(Text {
		text: username,
		paint: rank.get_username_paint(),
		..Default::default()
	});

	text
}
