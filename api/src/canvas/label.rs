use std::borrow::Cow;

use minecraft::colour::Colour;
use num_format::{Locale, ToFormattedString};
use translate::{prelude::GetNumFormatLocale, tr, Context};

pub trait ToFormatted {
	fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: Context<'c>, percent: bool) -> Cow<'t, str>;
}

macro_rules! impl_to_formatted_label_for_int {
	($int:ty) => {
		impl ToFormatted for $int
		where
			Self: ToFormattedString,
		{
			fn to_formatted_label<'t, 'c: 't>(
				&'t self,
				ctx: Context<'c>,
				percent: bool,
			) -> ::std::borrow::Cow<'t, str> {
				let locale = ctx.get_num_format_locale();

				::std::borrow::Cow::Owned(if *self >= 0 {
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
				} else {
					if percent {
						format!("-{}%", (-*self).to_formatted_string(&locale))
					} else if *self < -1_000_000 {
						(-*self).to_formatted_string(&locale)
					} else if *self < 1_000_000_000 {
						format!(
							"-{}M",
							(-*self as f32 / 1_000_000.).to_formatted_label(ctx, percent)
						)
					} else {
						format!(
							"-{}B",
							(-*self as f32 / 1_000_000_000.).to_formatted_label(ctx, percent)
						)
					}
				})
			}
		}
	};
}

macro_rules! impl_to_formatted_label_for_uint {
	($int:ty) => {
		impl ToFormatted for $int
		where
			Self: ToFormattedString,
		{
			fn to_formatted_label<'t, 'c: 't>(
				&'t self,
				ctx: Context<'c>,
				percent: bool,
			) -> ::std::borrow::Cow<'t, str> {
				let locale = ctx.get_num_format_locale();

				::std::borrow::Cow::Owned(if percent {
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
				})
			}
		}
	};
}

macro_rules! impl_to_formatted_label_for_float {
	($float:ty) => {
		impl ToFormatted for $float {
			fn to_formatted_label<'t, 'c: 't>(
				&'t self,
				ctx: Context<'c>,
				percent: bool,
			) -> ::std::borrow::Cow<'t, str> {
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

				Cow::Owned(string)
			}
		}
	};
}

impl ToFormatted for u8
where
	Self: ToFormattedString,
{
	fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: Context<'c>, percent: bool) -> Cow<'t, str> {
		let locale = ctx.get_num_format_locale();

		Cow::Owned(if percent {
			format!("{}%", self.to_formatted_string(&locale))
		} else {
			self.to_formatted_string(&locale)
		})
	}
}

impl_to_formatted_label_for_int!(i32);
impl_to_formatted_label_for_int!(i64);
impl_to_formatted_label_for_int!(i128);
impl_to_formatted_label_for_int!(isize);

impl_to_formatted_label_for_uint!(u32);
impl_to_formatted_label_for_uint!(u64);
impl_to_formatted_label_for_uint!(u128);
impl_to_formatted_label_for_uint!(usize);

impl_to_formatted_label_for_float!(f32);
impl_to_formatted_label_for_float!(f64);

impl ToFormatted for String {
	fn to_formatted_label<'t, 'c: 't>(&'t self, _ctx: Context<'c>, _percent: bool) -> Cow<'t, str> {
		Cow::Borrowed(self)
	}
}

impl ToFormatted for &str {
	fn to_formatted_label<'t, 'c: 't>(&'t self, _ctx: Context<'c>, _percent: bool) -> Cow<'t, str> {
		Cow::Borrowed(self)
	}
}

impl ToFormatted for bool {
	fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: Context<'c>, _percent: bool) -> Cow<'t, str> {
		if *self {
			tr!(ctx, "yes")
		} else {
			tr!(ctx, "no")
		}
	}
}

impl ToFormatted for Colour {
	fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: Context<'c>, _percent: bool) -> Cow<'t, str> {
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

impl<T> ToFormatted for Option<T>
where
	T: ToFormatted,
{
	fn to_formatted_label<'t, 'c: 't>(&'t self, ctx: Context<'c>, percent: bool) -> Cow<'t, str> {
		match self {
			Some(value) => value.to_formatted_label(ctx, percent),
			None => tr!(ctx, "none"),
		}
	}
}
