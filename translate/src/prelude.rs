use crate::context::{Context, Locale};

pub trait GetNumFormatLocale {
	fn get_num_format_locale(&self) -> num_format::Locale;
}

impl GetNumFormatLocale for Context<'_> {
	fn get_num_format_locale(&self) -> num_format::Locale {
		match self.locale().unwrap_or_default() {
			Locale::de => num_format::Locale::de,
			Locale::es_ES => num_format::Locale::es,
			Locale::fr => num_format::Locale::fr,
			Locale::ja => num_format::Locale::ja,
			Locale::ru => num_format::Locale::ru,
			_ => num_format::Locale::en,
		}
	}
}

pub trait GetChronoLocale {
	fn get_chrono_locale(&self) -> chrono::Locale;
}

impl GetChronoLocale for Context<'_> {
	fn get_chrono_locale(&self) -> chrono::Locale {
		match self.locale().unwrap_or_default() {
			Locale::de => chrono::Locale::de_DE,
			Locale::es_ES => chrono::Locale::es_ES,
			Locale::fr => chrono::Locale::fr_FR,
			Locale::ja => chrono::Locale::ja_JP,
			Locale::ru => chrono::Locale::ru_RU,
			_ => chrono::Locale::en_US,
		}
	}
}

pub trait GetLocale {
	fn locale(&self) -> Option<&Locale>;
	fn data(&self) -> &crate::Data;
}

impl GetLocale for &'_ Context<'_> {
	fn locale(&self) -> Option<&Locale> {
		(*self).locale()
	}

	fn data(&self) -> &crate::Data {
		(*self).data()
	}
}

impl GetLocale for Context<'_> {
	fn locale(&self) -> Option<&Locale> {
		self.locale()
	}

	fn data(&self) -> &crate::Data {
		self.data()
	}
}

impl GetLocale for crate::Context<'_> {
	fn locale(&self) -> Option<&Locale> {
		Some(match (*self).locale() {
			Some("de") => &Locale::de,
			Some("es-ES") => &Locale::es_ES,
			Some("fr") => &Locale::fr,
			Some("ja") => &Locale::ja,
			Some("ru") => &Locale::ru,
			_ => &Locale::en_US,
		})
	}

	fn data(&self) -> &crate::Data {
		(*self).data()
	}
}
