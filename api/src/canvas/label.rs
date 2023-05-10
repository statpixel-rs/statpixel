use num_format::{Locale, ToFormattedString};

pub trait ToFormattedLabel {
	fn to_formatted_label(&self, locale: &Locale, percent: bool) -> String;
}

macro_rules! impl_to_formatted_label_for_int {
	($int:ty) => {
		impl ToFormattedLabel for $int
		where
			Self: ToFormattedString,
		{
			fn to_formatted_label(&self, locale: &Locale, percent: bool) -> String {
				if percent {
					format!("{}%", self.to_formatted_string(locale))
				} else {
					if *self < 1_000_000 {
						self.to_formatted_string(locale)
					} else {
						format!(
							"{}M",
							(*self as f32 / 1_000_000.).to_formatted_label(locale, percent)
						)
					}
				}
			}
		}
	};
}

macro_rules! impl_to_formatted_label_for_float {
	($float:ty) => {
		impl ToFormattedLabel for $float {
			fn to_formatted_label(&self, locale: &Locale, percent: bool) -> String {
				let sep = match locale {
					Locale::de | Locale::fr | Locale::it | Locale::es | Locale::pt => ",",
					_ => ".",
				};

				let mut string = format!("{:.2}{}", self, if percent { "%" } else { "" });

				if let Some(index) = string.find('.') {
					string.replace_range(index..index + 1, sep);
				}

				string
			}
		}
	};
}

impl_to_formatted_label_for_int!(i32);
impl_to_formatted_label_for_int!(i64);
impl_to_formatted_label_for_int!(i128);
impl_to_formatted_label_for_int!(isize);
impl_to_formatted_label_for_int!(u32);
impl_to_formatted_label_for_int!(u64);
impl_to_formatted_label_for_int!(u128);
impl_to_formatted_label_for_int!(usize);

impl_to_formatted_label_for_float!(f32);
impl_to_formatted_label_for_float!(f64);

impl ToFormattedLabel for String {
	fn to_formatted_label(&self, _: &Locale, _percent: bool) -> String {
		self.clone()
	}
}

impl ToFormattedLabel for &str {
	fn to_formatted_label(&self, _: &Locale, _percent: bool) -> String {
		self.to_string()
	}
}
