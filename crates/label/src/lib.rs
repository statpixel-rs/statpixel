use std::borrow::Cow;

use chrono::{DateTime, Utc};
use minecraft::colour::Colour;
use num_format::ToFormattedString;
use pure_rust_locales::locale_match;
use translate::{
	context::Context,
	prelude::{GetChronoLocale, GetNumFormatLocale},
	tr,
};

pub trait ToFormatted {
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str>;
}

impl<T> ToFormatted for &'_ T
where
	T: ToFormatted + Copy,
{
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
		(*self).to_formatted(ctx)
	}
}

macro_rules! impl_to_formatted_for_int {
	($int:ty) => {
		impl ToFormatted for $int
		where
			Self: ToFormattedString,
		{
			fn to_formatted<'t, 'c: 't>(
				&'t self,
				ctx: &'c Context<'c>,
			) -> ::std::borrow::Cow<'t, str> {
				let locale = ctx.get_num_format_locale();

				::std::borrow::Cow::Owned(if *self >= 0 {
					if *self < 1_000_000 {
						self.to_formatted_string(&locale)
					} else if *self < 1_000_000_000 {
						format!("{}M", (*self as f32 / 1_000_000.).to_formatted(ctx))
					} else {
						format!("{}B", (*self as f32 / 1_000_000_000.).to_formatted(ctx))
					}
				} else {
					if *self < -1_000_000 {
						(-*self).to_formatted_string(&locale)
					} else if *self < 1_000_000_000 {
						format!("-{}M", (-*self as f32 / 1_000_000.).to_formatted(ctx))
					} else {
						format!("-{}B", (-*self as f32 / 1_000_000_000.).to_formatted(ctx))
					}
				})
			}
		}
	};
}

macro_rules! impl_to_formatted_for_uint {
	($int:ty) => {
		impl ToFormatted for $int
		where
			Self: ToFormattedString,
		{
			fn to_formatted<'t, 'c: 't>(
				&'t self,
				ctx: &'c Context<'c>,
			) -> ::std::borrow::Cow<'t, str> {
				let locale = ctx.get_num_format_locale();

				::std::borrow::Cow::Owned(if *self < 1_000_000 {
					self.to_formatted_string(&locale)
				} else if *self < 1_000_000_000 {
					format!("{}M", (*self as f32 / 1_000_000.).to_formatted(ctx))
				} else {
					format!("{}B", (*self as f32 / 1_000_000_000.).to_formatted(ctx))
				})
			}
		}
	};
}

#[allow(clippy::cast_precision_loss)]
impl ToFormatted for u64
where
	Self: ToFormattedString,
{
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> ::std::borrow::Cow<'t, str> {
		let locale = ctx.get_num_format_locale();

		::std::borrow::Cow::Owned(if *self < 1_000_000 {
			self.to_formatted_string(&locale)
		} else if *self < 1_000_000_000 {
			format!("{}M", (*self as f32 / 1_000_000.).to_formatted(ctx))
		} else if *self < 1_000_000_000_000 {
			format!("{}B", (*self as f64 / 1_000_000_000.).to_formatted(ctx))
		} else {
			format!("{}T", (*self as f64 / 1_000_000_000_000.).to_formatted(ctx))
		})
	}
}

#[allow(clippy::cast_precision_loss)]
impl ToFormatted for i64
where
	Self: ToFormattedString,
{
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> ::std::borrow::Cow<'t, str> {
		let locale = ctx.get_num_format_locale();

		::std::borrow::Cow::Owned(if *self >= 0 {
			if *self < 1_000_000 {
				self.to_formatted_string(&locale)
			} else if *self < 1_000_000_000 {
				format!("{}M", (*self as f32 / 1_000_000.).to_formatted(ctx))
			} else if *self < 1_000_000_000_000 {
				format!("{}B", (*self as f64 / 1_000_000_000.).to_formatted(ctx))
			} else {
				format!("{}T", (*self as f64 / 1_000_000_000_000.).to_formatted(ctx))
			}
		} else if *self < -1_000_000 {
			(-*self).to_formatted_string(&locale)
		} else if *self < 1_000_000_000 {
			format!("-{}M", (-*self as f32 / 1_000_000.).to_formatted(ctx))
		} else if *self < 1_000_000_000_000 {
			format!("-{}B", (-*self as f64 / 1_000_000_000.).to_formatted(ctx))
		} else {
			format!(
				"-{}T",
				(-*self as f64 / 1_000_000_000_000.).to_formatted(ctx)
			)
		})
	}
}

impl ToFormatted for f64 {
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> ::std::borrow::Cow<'t, str> {
		let locale = ctx.get_num_format_locale();
		let sep = match locale {
			num_format::Locale::de
			| num_format::Locale::fr
			| num_format::Locale::it
			| num_format::Locale::es
			| num_format::Locale::pt => ",",
			_ => ".",
		};

		if *self < 1_000. {
			let rounded = (*self * 100.).trunc() / 100.;
			let mut string = format!("{rounded:.2}");

			if sep != "." {
				let len = string.len();

				string.replace_range(len - 3..len - 2, sep);
			}

			Cow::Owned(string)
		} else {
			#[allow(clippy::cast_possible_truncation)]
			#[allow(clippy::cast_sign_loss)]
			(*self as u64).to_formatted(ctx).into_owned().into()
		}
	}
}

impl ToFormatted for f32 {
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> ::std::borrow::Cow<'t, str> {
		let locale = ctx.get_num_format_locale();
		let sep = match locale {
			num_format::Locale::de
			| num_format::Locale::fr
			| num_format::Locale::it
			| num_format::Locale::es
			| num_format::Locale::pt => ",",
			_ => ".",
		};

		if *self < 1_000. {
			let rounded = (*self * 100.).trunc() / 100.;
			let mut string = format!("{rounded:.2}");

			if sep != "." {
				let len = string.len();

				string.replace_range(len - 3..len - 2, sep);
			}

			Cow::Owned(string)
		} else {
			#[allow(clippy::cast_possible_truncation)]
			#[allow(clippy::cast_sign_loss)]
			(*self as u32).to_formatted(ctx).into_owned().into()
		}
	}
}

impl ToFormatted for u8
where
	Self: ToFormattedString,
{
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
		let locale = ctx.get_num_format_locale();

		Cow::Owned(self.to_formatted_string(&locale))
	}
}

impl_to_formatted_for_int!(i32);
impl_to_formatted_for_int!(i128);
impl_to_formatted_for_int!(isize);

impl_to_formatted_for_uint!(u32);
impl_to_formatted_for_uint!(u128);
impl_to_formatted_for_uint!(usize);

impl ToFormatted for String {
	fn to_formatted<'t, 'c: 't>(&'t self, _ctx: &'c Context<'c>) -> Cow<'t, str> {
		Cow::Borrowed(self)
	}
}

impl ToFormatted for &str {
	fn to_formatted<'t, 'c: 't>(&'t self, _ctx: &'c Context<'c>) -> Cow<'t, str> {
		Cow::Borrowed(self)
	}
}

impl ToFormatted for bool {
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
		if *self {
			tr(ctx, "yes")
		} else {
			tr(ctx, "no")
		}
	}
}

impl ToFormatted for Colour {
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
		match self {
			Colour::Black => tr(ctx, "black"),
			Colour::DarkBlue => tr(ctx, "dark-blue"),
			Colour::DarkGreen => tr(ctx, "dark-green"),
			Colour::DarkAqua => tr(ctx, "dark-aqua"),
			Colour::DarkRed => tr(ctx, "dark-red"),
			Colour::DarkPurple => tr(ctx, "dark-purple"),
			Colour::Gold => tr(ctx, "gold"),
			Colour::Gray => tr(ctx, "gray"),
			Colour::DarkGray => tr(ctx, "dark-gray"),
			Colour::Blue => tr(ctx, "blue"),
			Colour::Green => tr(ctx, "green"),
			Colour::Aqua => tr(ctx, "aqua"),
			Colour::Red => tr(ctx, "red"),
			Colour::LightPurple => tr(ctx, "light-purple"),
			Colour::Yellow => tr(ctx, "yellow"),
			Colour::White => tr(ctx, "white"),
		}
	}
}

impl<T> ToFormatted for Option<T>
where
	T: ToFormatted,
{
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
		match self {
			Some(value) => value.to_formatted(ctx),
			None => tr(ctx, "none"),
		}
	}
}

impl ToFormatted for DateTime<Utc> {
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'static, str> {
		let locale = ctx.get_chrono_locale();
		let fmt = locale_match!(locale => LC_TIME::D_FMT);

		Cow::Owned(self.format_localized(fmt, locale).to_string())
	}
}

impl ToFormatted for Box<dyn ToFormatted> {
	fn to_formatted<'t, 'c: 't>(&'t self, ctx: &'c Context<'c>) -> Cow<'t, str> {
		self.as_ref().to_formatted(ctx)
	}
}
