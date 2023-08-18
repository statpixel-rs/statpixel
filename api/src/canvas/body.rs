use std::borrow::Cow;

use minecraft::{paint::Paint, style::MinecraftFont, text::Text};
use skia_safe::{
	textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextAlign},
	FontMgr,
};
use translate::{context, tr};

use crate::{
	canvas::label::ToFormatted,
	game::{mode::Mode, r#type::Type},
	player::status::Session,
};

#[derive(Debug)]
pub struct Body {
	paragraph: ParagraphBuilder,
	first: bool,
	size: f32,
}

impl Body {
	#[inline]
	#[must_use]
	pub fn empty() -> Paragraph {
		ParagraphBuilder::new(&ParagraphStyle::new(), FontCollection::new()).build()
	}

	#[inline]
	#[must_use]
	pub fn new(size: f32, align: impl Into<Option<TextAlign>>) -> Self {
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

		Self {
			paragraph: ParagraphBuilder::new(&style, font),
			first: true,
			size,
		}
	}

	#[inline]
	#[must_use]
	pub fn extend_owned<'t>(mut self, text: impl IntoIterator<Item = Text<'t>>) -> Self {
		self.first = false;

		for blob in text {
			let style = blob.get_style(blob.paint, self.size);

			self.paragraph.push_style(&style);
			self.paragraph.add_text(blob.text);
		}

		self
	}

	#[inline]
	#[must_use]
	pub fn extend<'t>(mut self, text: impl IntoIterator<Item = &'t Text<'t>>) -> Self {
		self.first = false;

		for blob in text {
			let style = blob.get_style(blob.paint, self.size);

			self.paragraph.push_style(&style);
			self.paragraph.add_text(blob.text);
		}

		self
	}

	#[inline]
	#[must_use]
	pub fn append(mut self, blob: Text) -> Self {
		let style = blob.get_style(blob.paint, self.size);

		self.first = false;
		self.paragraph.push_style(&style);
		self.paragraph.add_text(blob.text);
		self
	}

	#[must_use]
	pub fn append_item(self, label: &str, value: &str, colour: &Paint) -> Self {
		if self.first {
			self
		} else {
			self.append(Text::NEW_LINE)
		}
		.extend(&[
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
	pub fn from_bubble_small(
		ctx: &context::Context<'_>,
		value: &impl ToFormatted,
		label: &str,
		paint: Paint,
	) -> Paragraph {
		Self::new(25., TextAlign::Center)
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
					text: &value.to_formatted(ctx),
					paint,
					font: MinecraftFont::Normal,
					size: None,
				},
			])
			.build()
	}

	#[must_use]
	pub fn from_bubble(
		ctx: &context::Context<'_>,
		value: &impl ToFormatted,
		label: &str,
		paint: Paint,
	) -> Paragraph {
		Self::new(40., TextAlign::Center)
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
					text: &value.to_formatted(ctx),
					paint,
					font: MinecraftFont::Normal,
					size: None,
				},
			])
			.build()
	}

	#[must_use]
	#[allow(clippy::needless_pass_by_value)]
	pub fn from_bubble_cow(value: Cow<str>, label: &str, paint: Paint) -> Paragraph {
		Self::new(40., TextAlign::Center)
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
					text: value.as_ref(),
					paint,
					font: MinecraftFont::Normal,
					size: None,
				},
			])
			.build()
	}

	#[must_use]
	pub fn from_status(ctx: &context::Context<'_>, session: &Session) -> Paragraph {
		if session.online {
			let mode = session.game_mode.as_deref().map(Mode::from);

			Self::new(18., TextAlign::Center)
				.extend(&[
					Text {
						text: &tr(ctx, "online"),
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
						text: mode.as_ref().map_or("Unknown", Mode::as_clean_name),
						paint: Paint::Aqua,
						..Default::default()
					},
				])
				.build()
		} else {
			Self::new(25., TextAlign::Center).build()
		}
	}

	pub fn build_slice(
		slice: &[Text],
		size: f32,
		align: impl Into<Option<TextAlign>>,
	) -> Paragraph {
		Self::new(size, align).extend(slice).build()
	}

	#[inline]
	#[must_use]
	pub fn build(mut self) -> Paragraph {
		self.paragraph.build()
	}
}
