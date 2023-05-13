use minecraft::colour::Colour;
use num_format::{Locale, ToFormattedString};
use translate::{prelude::GetNumFormatLocale, tr, Context};

pub trait ToFormatted {
	fn to_formatted_label(&self, ctx: Context<'_>, percent: bool) -> String;
}

macro_rules! impl_to_formatted_label_for_int {
	($int:ty) => {
		impl ToFormatted for $int
		where
			Self: ToFormattedString,
		{
			fn to_formatted_label(&self, ctx: Context<'_>, percent: bool) -> String {
				let locale = ctx.get_num_format_locale();

				if percent {
					format!("{}%", self.to_formatted_string(&locale))
				} else if *self < 1_000_000 {
					self.to_formatted_string(&locale)
				} else if *self < 1_000_000_000 {
					format!(
						"{}M",
						(*self as f32 / 1_000_000.).to_formatted_label(ctx, percent)
					)
				} else {
					format!(
						"{}B",
						(*self as f32 / 1_000_000_000.).to_formatted_label(ctx, percent)
					)
				}
			}
		}
	};
}

macro_rules! impl_to_formatted_label_for_float {
	($float:ty) => {
		impl ToFormatted for $float {
			fn to_formatted_label(&self, ctx: Context<'_>, percent: bool) -> String {
				let locale = ctx.get_num_format_locale();
				let sep = match locale {
					Locale::de | Locale::fr | Locale::it | Locale::es | Locale::pt => ",",
					_ => ".",
				};

				let mut string = format!("{:.2}{}", self, if percent { "%" } else { "" });

				if sep != "." {
					let len = string.len();

					string.replace_range(len - 3..len - 2, sep);
				}

				string
			}
		}
	};
}

impl ToFormatted for u8
where
	Self: ToFormattedString,
{
	fn to_formatted_label(&self, ctx: Context<'_>, percent: bool) -> String {
		let locale = ctx.get_num_format_locale();

		if percent {
			format!("{}%", self.to_formatted_string(&locale))
		} else {
			self.to_formatted_string(&locale)
		}
	}
}

impl_to_formatted_label_for_int!(u32);
impl_to_formatted_label_for_int!(u64);
impl_to_formatted_label_for_int!(u128);
impl_to_formatted_label_for_int!(usize);

impl_to_formatted_label_for_float!(f32);
impl_to_formatted_label_for_float!(f64);

impl ToFormatted for String {
	fn to_formatted_label(&self, _: Context<'_>, _percent: bool) -> String {
		self.clone()
	}
}

impl ToFormatted for &str {
	fn to_formatted_label(&self, _: Context<'_>, _percent: bool) -> String {
		(*self).to_string()
	}
}

impl ToFormatted for bool {
	fn to_formatted_label(&self, ctx: Context<'_>, _percent: bool) -> String {
		if *self {
			tr!(ctx, "yes")
		} else {
			tr!(ctx, "no")
		}
	}
}

impl ToFormatted for Colour {
	fn to_formatted_label(&self, ctx: Context<'_>, _percent: bool) -> String {
		match self {
			Colour::Black => tr!(ctx, "black"),
			Colour::DarkBlue => tr!(ctx, "dark-blue"),
			Colour::DarkGreen => tr!(ctx, "dark-green"),
			Colour::DarkAqua => tr!(ctx, "dark-aqua"),
			Colour::DarkRed => tr!(ctx, "dark-red"),
			Colour::DarkPurple => tr!(ctx, "dark-purple"),
			Colour::Gold => tr!(ctx, "gold"),
			Colour::Gray => tr!(ctx, "gray"),
			Colour::DarkGray => tr!(ctx, "dark-gray"),
			Colour::Blue => tr!(ctx, "blue"),
			Colour::Green => tr!(ctx, "green"),
			Colour::Aqua => tr!(ctx, "aqua"),
			Colour::Red => tr!(ctx, "red"),
			Colour::LightPurple => tr!(ctx, "light-purple"),
			Colour::Yellow => tr!(ctx, "yellow"),
			Colour::White => tr!(ctx, "white"),
		}
	}
}
