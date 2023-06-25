use darling::{ast, FromDeriveInput, FromVariant};
use quote::{quote, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(), supports(enum_unit))]
pub(crate) struct GetTrInputReceiver {
	pub ident: syn::Ident,
	pub generics: syn::Generics,
	pub data: ast::Data<FieldReceiver, ()>,
}

impl ToTokens for GetTrInputReceiver {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let GetTrInputReceiver {
			ident,
			generics,
			data,
		} = self;

		let fields = data.as_ref().take_enum().expect("should be a named struct");

		let get_tr = fields.iter().map(|f| {
			let ty = &f.ident;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				Self::#ty => #ty_str,
			}
		});

		tokens.extend(quote! {
			impl #generics #ident #generics {
				pub fn get_tr(&self) -> &'static str {
					match self {
						#(#get_tr)*
					}
				}
			}
		});
	}
}

#[derive(Debug, FromVariant)]
#[darling(attributes(mode))]
pub(crate) struct FieldReceiver {
	pub ident: syn::Ident,
}
