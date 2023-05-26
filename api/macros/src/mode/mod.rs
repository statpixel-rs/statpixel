use darling::{ast, FromDeriveInput, FromField, FromMeta};
use minecraft::paint::Paint;
use quote::{quote, ToTokens};

use crate::{
	sum,
	tokens::{get_percent_ident_for_str, get_percent_ident_for_type, get_tr_with_fallback},
};

macro_rules! ident {
	($id: literal) => {
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

		let outer_start_idx = fields.iter().filter(|data| data.field.is_some()).count();
		let valid_fields = fields
			.iter()
			.filter_map(|data| data.field.as_ref().map(|field| (data, field)))
			.collect::<Vec<_>>();

		let apply_items_mode = valid_fields.iter().enumerate().map(|(idx, (data, field))| {
			let ident = data.ident.as_ref().unwrap();
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let percent = field.percent == Some(true);
			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				if percent {
					let value = sum::div_u32_single_field(&ident!("self"), None, ident, div);
					let struct_name = get_percent_ident_for_type(data.ty.clone());

					return Some(quote! {
						crate::canvas::game::bubble(
							ctx,
							surface,
							crate::extras::percent::#struct_name (#value),
							&::translate::tr!(ctx, #tr),
							#colour,
							#idx + start_idx,
						);
					});
				} else {
					sum::div_f32_single_field(&ident!("self"), None, ident, div)
				}
			} else {
				quote! { self.#ident }
			};

			Some(quote! {
				crate::canvas::game::bubble(
					ctx,
					surface,
					#value,
					&::translate::tr!(ctx, #tr),
					#colour,
					#idx + start_idx,
				);
			})
		});

		let apply_field_items_mode = field_data.iter().enumerate().map(|(idx, field)| {
			let idx = idx + outer_start_idx;

			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::div_u32_single_field(&ident!("self"), None, ident, div);

					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						crate::canvas::game::bubble(
							ctx,
							surface,
							crate::extras::percent::#struct_name (#value),
							&::translate::tr!(ctx, #tr),
							#colour,
							#idx + start_idx,
						);
					};
				} else {
					sum::div_f32_single_field(&ident!("self"), None, ident, div)
				}
			} else {
				quote! { self.#ident }
			};

			quote! {
				crate::canvas::game::bubble(
					ctx,
					surface,
					#value,
					&::translate::tr!(ctx, #tr),
					#colour,
					#idx + start_idx,
				);
			}
		});

		let apply_embed_field_items_mode = field_data.iter().map(|field| {
			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let value = if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::div_u32_single_field(&ident!("self"), None, ident, div);

					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						field.push_str(::translate::tr!(ctx, #tr).as_ref());
						field.push_str(": **");
						field.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&crate::extras::percent::#struct_name (#value), ctx).as_ref());
						field.push_str("**\n");
					};
				} else {
					sum::div_f32_single_field(&ident!("self"), None, ident, div)
				}
			} else {
				quote! { self.#ident }
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

		let field_count = outer_start_idx as u8;

		tokens.extend(quote! {
			impl #ident #generics {
				pub fn apply_own_fields(
					&self,
					ctx: ::translate::Context<'_>,
					surface: &mut ::skia_safe::Surface,
					data: &crate::player::data::Data,
					session: &crate::player::status::Session,
					stats: &Stats,
					start_idx: usize,
				) {
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

				pub fn get_own_field_count() -> u8 {
					#field_count
				}

				#[allow(clippy::ptr_arg)]
				pub fn embed_own_fields(
					&self,
					ctx: ::translate::Context<'_>,
					field: &mut ::std::string::String,
					data: &crate::player::data::Data,
					session: &crate::player::status::Session,
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
	ident: syn::Ident,

	div: Option<syn::Ident>,

	tr: Option<String>,

	#[darling(default)]
	colour: Paint,

	percent: Option<String>,
}
