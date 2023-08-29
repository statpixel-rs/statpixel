use crate::context::{Context, Locale};

pub trait GetNumFormatLocale {
	fn get_num_format_locale(&self) -> num_format::Locale;
}

impl GetNumFormatLocale for Context<'_> {
	fn get_num_format_locale(&self) -> num_format::Locale {
		match self.locale().unwrap_or_default() {
			Locale::bg => num_format::Locale::bg,
			Locale::cs => num_format::Locale::cs,
			Locale::da => num_format::Locale::da,
			Locale::de => num_format::Locale::de,
			Locale::el => num_format::Locale::el,
			Locale::es_ES => num_format::Locale::es,
			Locale::fi => num_format::Locale::fi,
			Locale::fr => num_format::Locale::fr,
			Locale::hi => num_format::Locale::hi,
			Locale::hr => num_format::Locale::hr,
			Locale::hu => num_format::Locale::hu,
			Locale::it => num_format::Locale::it,
			Locale::ja => num_format::Locale::ja,
			Locale::ko => num_format::Locale::ko,
			Locale::lt => num_format::Locale::lt,
			Locale::nl => num_format::Locale::nl,
			Locale::no => num_format::Locale::nn,
			Locale::pl => num_format::Locale::pl,
			Locale::pt_BR => num_format::Locale::pt_PT,
			Locale::ro => num_format::Locale::ro,
			Locale::ru => num_format::Locale::ru,
			Locale::sv_SE => num_format::Locale::sv,
			Locale::th => num_format::Locale::th,
			Locale::tr => num_format::Locale::tr,
			Locale::uk => num_format::Locale::uk,
			Locale::zh_CN | Locale::zh_TW => num_format::Locale::zh,
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
			Locale::bg => chrono::Locale::bg_BG,
			Locale::cs => chrono::Locale::cs_CZ,
			Locale::da => chrono::Locale::da_DK,
			Locale::de => chrono::Locale::de_DE,
			Locale::el => chrono::Locale::el_GR,
			Locale::es_ES => chrono::Locale::es_ES,
			Locale::fi => chrono::Locale::fi_FI,
			Locale::fr => chrono::Locale::fr_FR,
			Locale::hi => chrono::Locale::hi_IN,
			Locale::hr => chrono::Locale::hr_HR,
			Locale::hu => chrono::Locale::hu_HU,
			Locale::it => chrono::Locale::it_IT,
			Locale::ja => chrono::Locale::ja_JP,
			Locale::ko => chrono::Locale::ko_KR,
			Locale::lt => chrono::Locale::lt_LT,
			Locale::nl => chrono::Locale::nl_NL,
			Locale::no => chrono::Locale::nb_NO,
			Locale::pl => chrono::Locale::pl_PL,
			Locale::pt_BR => chrono::Locale::pt_BR,
			Locale::ro => chrono::Locale::ro_RO,
			Locale::ru => chrono::Locale::ru_RU,
			Locale::sv_SE => chrono::Locale::sv_SE,
			Locale::th => chrono::Locale::th_TH,
			Locale::tr => chrono::Locale::tr_TR,
			Locale::uk => chrono::Locale::uk_UA,
			Locale::zh_CN => chrono::Locale::zh_CN,
			Locale::zh_TW => chrono::Locale::zh_TW,
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

#[cfg(feature = "error")]
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
