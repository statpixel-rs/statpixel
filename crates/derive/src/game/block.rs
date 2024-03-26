use minecraft::paint::Paint;
use quote::{quote, ToTokens};
use std::borrow::Cow;

use crate::util::{crate_ident, get_tr_with_fallback};

use super::{
	key::{Access, Key, Side},
	label::Percent,
	mode::Mode,
	prelude::{Field, FieldGroup, FieldKind},
	structs::{GameFieldReceiver, ModeFieldData, OverallFieldData},
};

#[derive(Debug, Clone)]
pub struct Block<'a> {
	pub ident: proc_macro2::TokenStream,
	pub div: Option<proc_macro2::TokenStream>,
	pub tr: Cow<'a, str>,
	pub tr_top: Cow<'a, str>,
	pub tr_bottom: Option<Cow<'a, str>>,
	pub paint: &'a Paint,
	pub kind: FieldKind,
	pub measurable: bool,
	pub skip_modes: &'a [String],
	pub is_root: bool,
	pub is_mode: bool,
	pub skip_chart: bool,
}

impl ToTokens for Block<'_> {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let id = self.var_id();

		tokens.extend_one(quote!(#id));
	}
}

impl Block<'_> {
	fn _shape(&self, value: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
		let api = crate_ident("api");
		let tr: proc_macro2::TokenStream = self.as_tr();
		let paint = self.paint;

		quote! {
			.push_checked(
				&#api::canvas::shape::Bubble,
				#api::canvas::body::Body::from_bubble(
					ctx,
					family,
					&#value,
					&#tr,
					#paint,
				),
			)
		}
	}

	/// Returns a `push_checked` chained call for a `Canvas`
	pub fn shape(&self, mode: &Mode<'_>) -> Option<proc_macro2::TokenStream> {
		Some(self._shape(self.value(Side::None, Access::Mode(mode))?))
	}

	/// Returns a `push_checked` chained call for a `Canvas`
	pub fn shape_sum(&self, modes: &[Mode<'_>]) -> Option<proc_macro2::TokenStream> {
		Some(self._shape(self.value_sum(Side::None, modes, Access::None)?))
	}

	pub fn shape_diff(&self, mode: &Mode<'_>) -> Option<proc_macro2::TokenStream> {
		Some(self._shape(self.diff_fmt(mode)?))
	}

	pub fn shape_diff_sum(&self, modes: &[Mode<'_>]) -> Option<proc_macro2::TokenStream> {
		Some(self._shape(self.diff_fmt_sum(modes)?))
	}

	fn skip_mode(&self, mode: &Mode<'_>) -> bool {
		self.skip_modes.contains(&mode.id().to_string())
			|| (!self.is_mode
				&& mode
					.blocks()
					.iter()
					.any(|b| b.id().to_string().eq(&self.id().to_string())))
	}

	fn _condensed(&self, value: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
		let minecraft = crate_ident("minecraft");

		let tr = self.as_tr();
		let paint = self.paint;

		quote! {
			#minecraft::text::Text {
				text: #tr.as_ref(),
				paint: #minecraft::paint::Paint::White,
				..Default::default()
			},
			#minecraft::text::Text {
				text: ": ",
				paint: #minecraft::paint::Paint::White,
				..Default::default()
			},
			#minecraft::text::Text {
				text: #value.as_ref(),
				paint: #paint,
				..Default::default()
			},
			#minecraft::text::Text::NEW_LINE,
		}
	}

	pub fn condensed_shape(&self, mode: &Mode<'_>) -> Option<proc_macro2::TokenStream> {
		self.value_fmt(Access::Mode(mode))
			.map(|value| self._condensed(value))
	}

	pub fn condensed_shape_sum(&self, modes: &[Mode<'_>]) -> Option<proc_macro2::TokenStream> {
		self.value_fmt_sum(Side::None, modes, Access::None)
			.map(|value| self._condensed(value))
	}

	pub fn condensed_shape_diff(&self, mode: &Mode<'_>) -> Option<proc_macro2::TokenStream> {
		self.diff_fmt(mode).map(|value| self._condensed(value))
	}

	pub fn condensed_shape_diff_sum(&self, modes: &[Mode<'_>]) -> Option<proc_macro2::TokenStream> {
		self.diff_fmt_sum(modes).map(|value| self._condensed(value))
	}
}

impl Field for Block<'_> {
	fn key<'m>(&self, side: Side, access: Access<'m>) -> Option<Key<'m>> {
		if matches!(access, Access::ModeDiff(..) | Access::ModeSumDiff(..) | Access::NoneDiff if !self.is_measurable())
		{
			None
		} else if self.is_root {
			Some(side.into_data())
		} else if let Access::ModeSum(mode) = access {
			if self.skip_mode(mode) {
				return None;
			}

			Some(side.into_game(Some(mode)))
		} else if let Access::ModeSumDiff(mode) = access {
			if self.skip_mode(mode) {
				return None;
			}

			Some(side.into_game(Some(mode)))
		} else if let Access::Mode(mode) = access {
			if self.skip_mode(mode) {
				return None;
			}

			Some(side.into_stats())
		} else if let Access::ModeDiff(mode) = access {
			if self.skip_mode(mode) {
				return None;
			}

			Some(side.into_stats())
		} else {
			None
		}
	}

	fn var_id(&self) -> Cow<proc_macro2::TokenStream> {
		if self.div().is_none() && !self.is_root {
			Cow::Borrowed(self.id())
		} else {
			Cow::Owned(syn::parse_str(&self.tr().replace('-', "_")).unwrap())
		}
	}

	fn tr(&self) -> Cow<str> {
		self.tr.clone()
	}

	fn tr_top(&self) -> Cow<str> {
		self.tr_top.clone()
	}

	fn tr_bottom(&self) -> Option<Cow<str>> {
		self.tr_bottom.clone()
	}

	fn id(&self) -> &proc_macro2::TokenStream {
		&self.ident
	}

	fn div(&self) -> Option<&proc_macro2::TokenStream> {
		self.div.as_ref()
	}

	fn kind(&self) -> &FieldKind {
		&self.kind
	}

	fn is_measurable(&self) -> bool {
		self.measurable
	}

	fn skip_chart(&self) -> bool {
		self.skip_chart || self.div().is_some()
	}
}

impl<'a> TryFrom<&'a GameFieldReceiver> for Block<'a> {
	type Error = &'static str;

	fn try_from(_: &'a GameFieldReceiver) -> Result<Self, Self::Error> {
		Err("`Block`s cannot currently be created from `GameFieldReceiver`s")
	}
}

impl<'a> TryFrom<&'a OverallFieldData> for Block<'a> {
	type Error = &'static str;

	fn try_from(value: &'a OverallFieldData) -> Result<Self, Self::Error> {
		let path = value.path.as_ref().map_or_else(
			|| quote!(),
			|p| {
				if p.is_empty() {
					return quote!();
				}

				let mut tokens =
					syn::parse_str::<proc_macro2::TokenStream>(p).expect("a valid path");

				tokens.extend_one(quote!(.));
				tokens
			},
		);

		Ok(Self {
			ident: {
				let id = &value.ident;
				quote!(#path #id)
			},
			div: value.div.as_ref().map(|id| {
				let id = &id;
				quote!(#path #id)
			}),
			tr: get_tr_with_fallback(value.tr.as_deref(), Some(&value.ident)),
			tr_top: get_tr_with_fallback(None, Some(&value.ident)),
			tr_bottom: value
				.div
				.as_ref()
				.map(|id| get_tr_with_fallback(None, Some(id))),
			paint: &value.colour,
			kind: if let Some(ref id) = value.percent {
				FieldKind::Percent(Percent::from(id.as_str()))
			} else if value.min.is_present() {
				FieldKind::Min
			} else {
				FieldKind::None
			},
			is_root: value.path.is_some(),
			measurable: !value.nominal.is_present(),
			skip_modes: &value.skip_mode,
			is_mode: false,
			skip_chart: value.skip_chart.is_present(),
		})
	}
}

impl<'a> From<&'a ModeFieldData> for Block<'a> {
	fn from(value: &'a ModeFieldData) -> Self {
		let path = value.path.as_ref().map_or_else(
			|| quote!(),
			|p| {
				if p.is_empty() {
					return quote!();
				}

				let mut tokens =
					syn::parse_str::<proc_macro2::TokenStream>(p).expect("a valid path");

				tokens.extend_one(quote!(.));
				tokens
			},
		);

		Self {
			ident: {
				let id = &value.ident;
				quote!(#path #id)
			},
			div: value.div.as_ref().map(|id| {
				let id = &id;
				quote!(#path #id)
			}),
			tr: get_tr_with_fallback(value.tr.as_deref(), Some(&value.ident)),
			tr_top: get_tr_with_fallback(None, Some(&value.ident)),
			tr_bottom: value
				.div
				.as_ref()
				.map(|id| get_tr_with_fallback(None, Some(id))),
			paint: &value.colour,
			kind: if let Some(ref id) = value.percent {
				FieldKind::Percent(Percent::from(id.as_str()))
			} else {
				FieldKind::None
			},
			measurable: true,
			skip_modes: &[],
			is_root: value.path.is_some(),
			is_mode: true,
			skip_chart: value.skip_chart.is_present(),
		}
	}
}
