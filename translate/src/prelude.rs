use std::str::FromStr;

use tracing::debug;

use crate::Context;

pub trait GetNumFormatLocale {
	fn get_num_format_locale(&self) -> num_format::Locale;
}

impl GetNumFormatLocale for Context<'_> {
	fn get_num_format_locale(&self) -> num_format::Locale {
		let result = num_format::Locale::from_str(self.locale().unwrap_or("en"))
			.unwrap_or(num_format::Locale::en);

		debug!("Using locale {:?} for {}", result, self.author().tag());

		result
	}
}

pub trait GetChronoLocale {
	fn get_chrono_locale(&self) -> chrono::Locale;
}

impl GetChronoLocale for Context<'_> {
	fn get_chrono_locale(&self) -> chrono::Locale {
		match self.locale().unwrap_or("en") {
			"de" => chrono::Locale::de_DE,
			"es" => chrono::Locale::es_ES,
			"fr" => chrono::Locale::fr_FR,
			"ja" => chrono::Locale::ja_JP,
			"ru" => chrono::Locale::ru_RU,
			_ => chrono::Locale::en_US,
		}
	}
}
