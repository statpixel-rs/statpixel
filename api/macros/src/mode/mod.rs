use darling::{ast, FromDeriveInput, FromField, FromMeta};
use minecraft::paint::Paint;
use quote::{quote, ToTokens};

use crate::{
	sum,
	tokens::{get_percent_ident_for_str, get_percent_ident_for_type, get_tr_with_fallback},
};

macro_rules! ident {
	($id: expr) => {
		::syn::Ident::new($id, ::proc_macro2::Span::call_site())
	};
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(mode), supports(struct_named))]
pub(crate) struct ModeInputReceiver {
	pub ident: syn::Ident,
	pub generics: syn::Generics,
	pub data: ast::Data<(), ModeFieldReceiver>,
	#[darling(multiple)]
	pub field: Vec<ModeFieldData>,
}

impl ToTokens for ModeInputReceiver {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let ModeInputReceiver {
			ident,
			generics,
			data,
			field: field_data,
		} = self;

		let fields = data
			.as_ref()
			.take_struct()
			.expect("should be a named struct")
			.fields;

		let valid_fields = fields
			.iter()
			.filter_map(|data| data.field.as_ref().map(|field| (data, field)))
			.collect::<Vec<_>>();

		let apply_items_mode = valid_fields.iter().map(|(data, field)| {
			let ident = data.ident.as_ref().unwrap();
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let percent = field.percent == Some(true);
			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				if percent {
					let value = sum::div_u32_single_field(&quote!(self), ident, div);
					let struct_name = get_percent_ident_for_type(data.ty.clone());

					return quote! {
						.push_checked(
							&crate::canvas::shape::Bubble,
							crate::canvas::body::Body::from_bubble(
								ctx,
								&crate::extras::percent::#struct_name (#value),
								&::translate::tr!(ctx, #tr),
								#colour,
							),
						)
					};
				} else {
					sum::div_f32_single_field(&quote!(self), ident, div)
				}
			} else {
				quote! { self.#ident }
			};

			quote! {
				.push_checked(
					&crate::canvas::shape::Bubble,
					crate::canvas::body::Body::from_bubble(
						ctx,
						&#value,
						&::translate::tr!(ctx, #tr),
						#colour,
					),
				)
			}
		});

		let apply_field_items_mode = field_data.iter().map(|field| {
			let mut split = field.ident.split('.');
			let (ident, parent) = match (split.next_back(), split.next()) {
				(Some(ident), Some(first)) => (ident!(ident), {
					let first = syn::Ident::new(first, proc_macro2::Span::call_site());

					let rest = split.map(|p| {
						let ident = syn::Ident::new(p, proc_macro2::Span::call_site());

						quote! { .#ident }
					});

					quote! { data.stats.#first #(#rest)* }
				}),
				_ => (ident!(&field.ident), quote!(self)),
			};
			let ident = &ident;

			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::div_u32_single_field(&parent, ident, div);

					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						.push_checked(
							&crate::canvas::shape::Bubble,
							crate::canvas::body::Body::from_bubble(
								ctx,
								&crate::extras::percent::#struct_name (#value),
								&::translate::tr!(ctx, #tr),
								#colour,
							),
						)
					};
				} else {
					sum::div_f32_single_field(&parent, ident, div)
				}
			} else {
				quote! { #parent .#ident }
			};

			quote! {
				.push_checked(
					&crate::canvas::shape::Bubble,
					crate::canvas::body::Body::from_bubble(
						ctx,
						&#value,
						&::translate::tr!(ctx, #tr),
						#colour,
					),
				)
			}
		});

		let apply_embed_field_items_mode = field_data.iter().map(|field| {
			let mut split = field.ident.split('.');
			let (ident, parent) = match (split.next_back(), split.next()) {
				(Some(ident), Some(first)) => (ident!(ident), {
					let first =
						syn::Ident::new(first, proc_macro2::Span::call_site());

					let rest = split.map(|p| {
						let ident = syn::Ident::new(p, proc_macro2::Span::call_site());

						quote! { .#ident }
					});

					quote! { data.stats.#first #(#rest)* }
				}),
				_ => (ident!(&field.ident), quote!(self)),
			};
			let ident = &ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let value = if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::div_u32_single_field(&parent, ident, div);
					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						field.push_str(::translate::tr!(ctx, #tr).as_ref());
						field.push_str(": **");
						field.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&crate::extras::percent::#struct_name (#value), ctx).as_ref());
						field.push_str("**\n");
					};
				} else {
					sum::div_f32_single_field(&parent, ident, div)
				}
			} else {
				quote! { #parent.#ident }
			};

			quote! {
				field.push_str(::translate::tr!(ctx, #tr).as_ref());
				field.push_str(": **");
				field.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&#value, ctx).as_ref());
				field.push_str("**\n");
			}
		});

		let min_fields = valid_fields
			.iter()
			.map(|(field, _)| {
				let ident = field.ident.as_ref().unwrap();

				quote! {
					{
						let v: u32 = self.#ident.into();

						if v < min {
							min = v;
						}
					}
				}
			})
			.collect::<Vec<_>>();

		let max_fields = valid_fields
			.iter()
			.map(|(field, _)| {
				let ident = field.ident.as_ref().unwrap();

				quote! {
					{
						let v: u32 = self.#ident.into();

						if v > max {
							max = v;
						}
					}
				}
			})
			.collect::<Vec<_>>();

		tokens.extend(quote! {
			impl #ident #generics {
				pub fn apply_own_fields<'c>(
					&self,
					ctx: &::translate::context::Context<'_>,
					mut canvas: crate::canvas::Canvas<'c>,
					data: &'c crate::player::data::Data,
					session: &crate::player::status::Session,
					stats: &Stats,
				) -> crate::canvas::Canvas<'c> {
					canvas
					#(#apply_items_mode)*
					#(#apply_field_items_mode)*
				}

				pub fn min_own_fields(&self) -> u32 {
					let mut min = ::std::u32::MAX;

					#(#min_fields)*

					min
				}

				pub fn max_own_fields(&self) -> u32 {
					let mut max = ::std::u32::MIN;

					#(#max_fields)*

					max
				}

				#[allow(clippy::ptr_arg)]
				pub fn embed_own_fields(
					&self,
					ctx: &::translate::context::Context<'_>,
					field: &mut ::std::string::String,
					data: &crate::player::data::Data,
					stats: &Stats,
				) {
					#(#apply_embed_field_items_mode)*
				}
			}
		});
	}
}

#[derive(Debug, FromField)]
#[darling(attributes(mode))]
pub(crate) struct ModeFieldReceiver {
	pub ty: syn::Type,

	/// Get the ident of the field. For fields in tuple or newtype structs or
	/// enum bodies, this can be `None`.
	pub ident: Option<syn::Ident>,

	/// Field data
	pub field: Option<ModeField>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ModeField {
	#[darling(default)]
	colour: Paint,

	/// The translation key of the label.
	/// Defaults to the ident with underscores replaced with dashes.
	tr: Option<String>,

	div: Option<syn::Ident>,

	percent: Option<bool>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ModeFieldData {
	ident: String,
	div: Option<syn::Ident>,

	tr: Option<String>,

	#[darling(default)]
	colour: Paint,

	percent: Option<String>,
}
