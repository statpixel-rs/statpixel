use minecraft::paint::Paint;
use quote::{quote, ToTokens};
use std::borrow::Cow;

use crate::util::{crate_ident, get_tr_with_fallback, ident};

use super::{
	key::{Access, Key, Side},
	mode::Mode,
	prelude::{Field, FieldKind},
	structs::{GameFieldReceiver, InfoFieldData},
};

#[derive(Debug, Clone)]
pub enum Percent {
	U32,
	U64,
	I32,
	I64,
}

pub struct Collection<'a> {
	pub labels: Cow<'a, Vec<Label<'a>>>,
}

impl Collection<'_> {
	/// Returns a `push_right_start` chained call for a `Canvas`
	pub fn shape(&self, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		let api = crate_ident("api");
		let labels = self.labels.iter().filter_map(|label| label.item(mode));

		quote! {
			.push_right_start(
				&#api ::canvas::shape::Sidebar,
				#api ::canvas::body::Body::new(17., None, family)
					#(#labels)*
					.build()
			)
		}
	}

	pub fn shape_sum(&self, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		let api = crate_ident("api");
		let labels = self.labels.iter().filter_map(|label| label.item_sum(modes));

		quote! {
			.push_right_start(
				&#api ::canvas::shape::Sidebar,
				#api ::canvas::body::Body::new(17., None, family)
					#(#labels)*
					.build()
			)
		}
	}

	pub fn shape_diff(&self, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		let api = crate_ident("api");
		let labels = self.labels.iter().filter_map(|label| label.item_diff(mode));

		quote! {
			.push_right_start(
				&#api::canvas::shape::Sidebar,
				#api::canvas::body::Body::new(17., None, family)
					#(#labels)*
					.build()
			)
		}
	}

	pub fn shape_diff_sum(&self, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		let api = crate_ident("api");
		let labels = self
			.labels
			.iter()
			.filter_map(|label| label.item_diff_sum(modes));

		quote! {
			.push_right_start(
				&#api ::canvas::shape::Sidebar,
				#api ::canvas::body::Body::new(17., None, family)
					#(#labels)*
					.build()
			)
		}
	}
}

#[derive(Debug, Clone)]
pub struct Label<'a> {
	pub ident: proc_macro2::TokenStream,
	pub div: Option<proc_macro2::TokenStream>,
	pub tr: Cow<'a, str>,
	pub paint: &'a Paint,
	pub kind: FieldKind,
	pub is_static: bool,
	pub is_root: bool,
	pub measurable: bool,
}

impl ToTokens for Label<'_> {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let id = self.var_id();

		tokens.extend_one(quote!(#id));
	}
}

impl Label<'_> {
	/// Returns an `append_item` call for a `Body`
	pub fn item(&self, mode: &Mode<'_>) -> Option<proc_macro2::TokenStream> {
		let value = self.value_fmt(Access::Mode(mode))?;
		let tr = self.as_tr();
		let paint = self.paint;

		Some(quote! {
			.append_item(
				&#tr,
				&#value,
				&#paint
			)
		})
	}

	pub fn item_sum(&self, modes: &[Mode<'_>]) -> Option<proc_macro2::TokenStream> {
		let value = self.value_fmt_sum(Side::None, modes, Access::None)?;
		let tr = self.as_tr();
		let paint = self.paint;

		Some(quote! {
			.append_item(
				&#tr,
				&#value,
				&#paint
			)
		})
	}

	pub fn item_diff(&self, mode: &Mode<'_>) -> Option<proc_macro2::TokenStream> {
		let value = self.diff_fmt(mode)?;
		let tr = self.as_tr();
		let paint = self.paint;

		Some(quote! {
			.append_item(
				&#tr,
				&#value,
				&#paint
			)
		})
	}

	pub fn item_diff_sum(&self, modes: &[Mode<'_>]) -> Option<proc_macro2::TokenStream> {
		let value = self.diff_fmt_sum(modes)?;
		let tr = self.as_tr();
		let paint = self.paint;

		Some(quote! {
			.append_item(
				&#tr,
				&#value,
				&#paint
			)
		})
	}
}

impl From<&str> for Percent {
	fn from(value: &str) -> Self {
		match value {
			"u32" => Self::U32,
			"u64" => Self::U64,
			"i32" => Self::I32,
			"i64" => Self::I64,
			_ => panic!("Unknown type str for percent: {value:?}",),
		}
	}
}

impl ToTokens for Percent {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let api = crate_ident("api");

		let id = ident(match self {
			Self::U32 => "PercentU32",
			Self::U64 => "PercentU64",
			Self::I32 => "PercentI32",
			Self::I64 => "PercentI64",
		});

		tokens.extend(quote! {
			#api ::percent::#id
		});
	}
}

impl Field for Label<'_> {
	fn key<'m>(&self, side: Side, access: Access<'m>) -> Option<Key<'m>> {
		if matches!(access, Access::ModeDiff(..) | Access::ModeSumDiff(..) | Access::NoneDiff if !self.is_measurable())
		{
			None
		} else if self.is_root {
			Some(side.into_data())
		} else if let Access::ModeSum(mode) = access {
			Some(side.into_game(Some(mode)))
		} else if let Access::ModeSumDiff(mode) = access {
			Some(side.into_game(Some(mode)))
		} else if self.is_static {
			Some(side.into_game(None))
		} else if matches!(access, Access::Mode(..) | Access::ModeDiff(..)) {
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

	fn paint(&self) -> &Paint {
		self.paint
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
		false
	}
}

impl<'a> TryFrom<&'a GameFieldReceiver> for Label<'a> {
	type Error = &'static str;

	fn try_from(value: &'a GameFieldReceiver) -> Result<Self, Self::Error> {
		let ident = value.ident.as_ref().ok_or("missing ident")?;
		let label = value.label.as_ref().ok_or("missing label")?;

		Ok(Self {
			ident: quote!(#ident),
			div: label.div.as_ref().map(|id| {
				let id = &id;
				quote!(#id)
			}),
			tr: get_tr_with_fallback(label.tr.as_deref(), Some(ident)),
			paint: &label.colour,
			kind: if label.percent.is_present() {
				FieldKind::Percent(Percent::U32)
			} else {
				FieldKind::None
			},
			is_static: true,
			is_root: false,
			measurable: !value.nominal.is_present(),
		})
	}
}

impl<'a> From<&'a InfoFieldData> for Label<'a> {
	fn from(value: &'a InfoFieldData) -> Self {
		let path = value.path.as_ref().map_or_else(
			|| quote!(),
			|p| {
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
			paint: &value.colour,
			kind: if let Some(ref id) = value.percent {
				FieldKind::Percent(Percent::from(id.as_str()))
			} else {
				FieldKind::None
			},
			is_static: false,
			is_root: value.path.is_some(),
			measurable: !value.nominal.is_present(),
		}
	}
}
