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
