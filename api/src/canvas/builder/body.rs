use minecraft::{paint::Paint, style::MinecraftFont, text::Text};
use skia_safe::{
	textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextAlign},
	FontMgr,
};
use translate::{tr, Context};

use crate::{
	canvas::label::ToFormatted,
	game::{mode::Mode, r#type::Type},
	player::status::Session,
};

#[derive(Debug)]
pub struct Body<'t> {
	text: Vec<Text<'t>>,
	first: bool,
}

impl<'t> Default for Body<'t> {
	fn default() -> Self {
		Self {
			text: vec![],
			first: true,
		}
	}
}

impl<'t> Body<'t> {
	#[inline]
	#[must_use]
	pub fn extend(mut self, text: &[Text<'t>]) -> Self {
		self.first = false;
		self.text.extend_from_slice(text);
		self
	}

	#[inline]
	#[must_use]
	pub fn append(mut self, text: Text<'t>) -> Self {
		self.first = false;
		self.text.push(text);
		self
	}

	#[must_use]
	pub fn append_item(mut self, label: &'t str, value: &'t str, colour: &'t Paint) -> Self {
		if !self.first {
			self.text.push(Text::NEW_LINE);
		}

		self.extend(&[
			Text {
				text: "• ",
				paint: *colour,
				..Default::default()
			},
			Text {
				text: label,
				paint: Paint::White,
				..Default::default()
			},
			Text {
				text: ": ",
				paint: Paint::White,
				..Default::default()
			},
			Text {
				text: value,
				paint: *colour,
				..Default::default()
			},
		])
	}

	#[must_use]
	pub fn from_bubble(
		ctx: Context<'_>,
		value: &impl ToFormatted,
		label: &str,
		paint: Paint,
	) -> Paragraph {
		Self::default()
			.extend(&[
				Text {
					text: label,
					paint,
					font: MinecraftFont::Normal,
					size: Some(20.),
				},
				Text {
					text: "\n",
					size: Some(20.),
					..Default::default()
				},
				Text {
					text: &value.to_formatted_label(ctx),
					paint,
					font: MinecraftFont::Normal,
					size: None,
				},
			])
			.build(40., TextAlign::Center)
	}

	#[must_use]
	pub fn from_status(ctx: Context<'_>, session: &Session) -> Paragraph {
		if session.online {
			Self::default()
				.extend(&[
					Text {
						text: &tr!(ctx, "online"),
						paint: Paint::Green,
						..Default::default()
					},
					Text {
						text: "\n",
						..Default::default()
					},
					Text {
						text: session.game_type.unwrap_or(Type::Lobby).as_clean_name(),
						paint: Paint::Gray,
						..Default::default()
					},
					Text {
						text: "\n",
						..Default::default()
					},
					Text {
						text: session
							.game_mode
							.as_deref()
							.and_then(|m| Mode::try_from(m).ok())
							.map_or("Unknown", |m| m.as_clean_name()),
						paint: Paint::Aqua,
						..Default::default()
					},
				])
				.build(18., TextAlign::Center)
		} else {
			Self::default()
				.extend(&[Text {
					text: &tr!(ctx, "offline"),
					paint: minecraft::paint::Paint::DarkGray,
					..Default::default()
				}])
				.build(25., TextAlign::Center)
		}
	}

	pub fn build(self, size: f32, align: impl Into<Option<TextAlign>>) -> Paragraph {
		let style = {
			let mut style = ParagraphStyle::new();

			style.set_text_align(align.into().unwrap_or(TextAlign::Left));
			style
		};

		let font = {
			let mut manager = FontCollection::new();

			manager.set_default_font_manager(FontMgr::new(), "Minecraft");
			manager
		};

		let mut builder = ParagraphBuilder::new(&style, font);

		for blob in self.text {
			let style = blob.get_style(blob.paint, size);

			builder.push_style(&style);
			builder.add_text(blob.text);
		}

		builder.build()
	}
}
