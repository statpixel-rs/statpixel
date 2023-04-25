use once_cell::sync::Lazy;
use skia_safe::{Font, Typeface};

pub static FONT_NORMAL: Lazy<Font> = Lazy::new(|| {
	Font::from_typeface(
		Typeface::from_name("Minecraft", skia_safe::FontStyle::normal()).unwrap(),
		None,
	)
});

pub static FONT_BOLD: Lazy<Font> = Lazy::new(|| {
	Font::from_typeface(
		Typeface::from_name("Minecraft", skia_safe::FontStyle::bold()).unwrap(),
		None,
	)
});

pub static FONT_ITALIC: Lazy<Font> = Lazy::new(|| {
	Font::from_typeface(
		Typeface::from_name("Minecraft", skia_safe::FontStyle::italic()).unwrap(),
		None,
	)
});

pub static FONT_BOLD_ITALIC: Lazy<Font> = Lazy::new(|| {
	Font::from_typeface(
		Typeface::from_name("Minecraft", skia_safe::FontStyle::bold_italic()).unwrap(),
		None,
	)
});
