use darling::{ast, FromDeriveInput, FromField};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, ToTokens};

/// Creates a new struct that holds the diff between two values of the same struct.
#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named, struct_tuple))]
pub(crate) struct DiffInputReceiver {
	pub ident: syn::Ident,
	pub generics: syn::Generics,
	pub data: ast::Data<(), DiffFieldReceiver>,
}

#[derive(Debug, FromField)]
pub(crate) struct DiffFieldReceiver {
	pub ident: Option<syn::Ident>,
}

impl ToTokens for DiffInputReceiver {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let DiffInputReceiver {
			ident,
			generics,
			data,
		} = self;

		let api = crate_name("api").expect("api should be in the workspace");
		let api = match api {
			FoundCrate::Itself => quote!(crate),
			FoundCrate::Name(name) => {
				let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
				quote!(#ident)
			}
		};

		let struct_ = data
			.as_ref()
			.take_struct()
			.expect("should be a named struct");

		if struct_.is_tuple() {
			let diff_fields = struct_.fields.iter().enumerate().map(|(idx, _)| {
				let idx = syn::Index::from(idx);

				quote! {
					self.#idx.diff(&other.#idx)
				}
			});

			tokens.extend(quote! {
				impl #generics #api ::canvas::diff::Diff for #ident #generics {
					fn diff(&self, other: &Self) -> Self {
						Self (#(#diff_fields),*)
					}
				}
			});
		} else if struct_.is_struct() {
			let diff_fields = struct_.fields.iter().map(|field| {
				let ident = field.ident.as_ref().unwrap();

				quote! {
					#ident: self.#ident.diff(&other.#ident)
				}
			});
			tokens.extend(quote! {
				impl #generics #api ::canvas::diff::Diff for #ident #generics {
					fn diff(&self, other: &Self) -> Self {
						Self {
							#(#diff_fields),*
						}
					}
				}
			});
		}
	}
}
