pub mod parse;
pub mod rank;

use skia_safe::{
	textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle},
	FontMgr, Rect, Surface,
};

use crate::{paint::Paint, style::MinecraftFont};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text<'t> {
	pub text: &'t str,
	pub font: MinecraftFont,
	pub paint: Paint,
	pub size: Option<f32>,
}

impl<'t> Text<'t> {
	#[must_use]
	pub fn get_style(&self, paint: Paint, default_size: f32) -> TextStyle {
		let size = self.size.unwrap_or(default_size);
		let mut style = self.font.get_style(paint, size);

		if self.font == MinecraftFont::Icon {
			style.set_font_size(size * 0.75);
			style.set_baseline_shift(0.);
		} else {
			style.set_font_size(size);
		}

		style.set_foreground_color(paint.into());

		style
	}
}

impl Default for Text<'_> {
	fn default() -> Self {
		Self {
			text: "",
			font: MinecraftFont::Normal,
			paint: Paint::White,
			size: None,
		}
	}
}

pub fn draw(
	surface: &mut Surface,
	text: &[Text<'_>],
	size: f32,
	rect: impl Into<Rect>,
	h_align: impl Into<Option<TextAlign>>,
	v_center: bool,
) {
	let style = {
		let mut style = ParagraphStyle::new();

		style.set_text_align(h_align.into().unwrap_or(TextAlign::Left));
		style
	};

	let mut paragraph = {
		let font = {
			let mut manager = FontCollection::new();

			manager.set_default_font_manager(FontMgr::new(), "Minecraft");
			manager
		};

		let mut builder = ParagraphBuilder::new(&style, font);

		for blob in text {
			let style = blob.get_style(blob.paint, size);

			builder.push_style(&style);
			builder.add_text(blob.text);
		}

		builder.build()
	};

	let rect: Rect = rect.into();

	paragraph.layout(rect.width());

	let point = (
		rect.left(),
		if v_center {
			rect.center_y() - paragraph.height() / 2.
		} else {
			rect.top()
		},
	);

	paragraph.paint(surface.canvas(), point);
}
