use minecraft::{
	paint::Paint,
	text::{parse::minecraft_string, Text},
};

use hypixel::data::Data;

#[must_use]
pub fn from_data<'u>(data: &'u Data, username: &'u str, suffix: Option<&'u str>) -> Vec<Text<'u>> {
	let rank = data.get_rank();

	let mut text = if let Some(text) = rank.get_text() {
		// 2 for suffix, 1 for username, 1 for space since rank text is not empty
		let mut vec = Vec::with_capacity(text.len() + usize::from(suffix.is_some()) * 2 + 2);

		vec.extend(text);
		vec
	} else if let Some(prefix) = data.prefix.as_ref() {
		let mut vec = minecraft_string(prefix).by_ref().collect::<Vec<_>>();

		vec.reserve_exact(usize::from(suffix.is_some()) * 2 + 2);
		vec
	} else {
		Vec::with_capacity(usize::from(suffix.is_some()) * 2 + 1)
	};

	if !text.is_empty() {
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

	if let Some(suffix) = suffix {
		text.push(Text {
			text: " ",
			..Default::default()
		});

		text.push(Text {
			text: suffix,
			paint: Paint::Gold,
			..Default::default()
		});
	}

	text
}

#[must_use]
pub fn from_data_with_level<'u>(
	data: &'u Data,
	username: &'u str,
	suffix: Option<&'u str>,
	level: &'u str,
) -> Vec<Text<'u>> {
	let mut text = minecraft_string(level).collect::<Vec<_>>();

	text.push(Text {
		text: " ",
		..Default::default()
	});

	let rank = data.get_rank();

	if let Some(vec) = rank.get_text() {
		text.extend(vec);
		text.reserve_exact(usize::from(suffix.is_some()) * 2 + 2);
	} else if let Some(prefix) = data.prefix.as_ref() {
		text.extend(minecraft_string(prefix));
		text.reserve_exact(usize::from(suffix.is_some()) * 2 + 2);
	} else {
		text.reserve_exact(usize::from(suffix.is_some()) * 2 + 1);
	};

	if !text.is_empty() {
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

	if let Some(suffix) = suffix {
		text.push(Text {
			text: " ",
			..Default::default()
		});

		text.push(Text {
			text: suffix,
			paint: Paint::Gold,
			..Default::default()
		});
	}

	text
}
