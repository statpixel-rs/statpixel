use skia_safe::{
	textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign},
	FontMgr, Rect, Surface,
};

use crate::style::Family;

use super::Text;

pub fn draw(
	surface: &mut Surface,
	family: Family,
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
			let style = blob.get_style(family, blob.paint, size);

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
