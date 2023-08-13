use darling::{ast, FromDeriveInput, FromVariant};
use quote::{quote, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(), supports(enum_unit, enum_tuple))]
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

		let fields = data.as_ref().take_enum().expect("should be an enum");

		let get_tr = fields.iter().map(|f| {
			let ident = &f.ident;
			let ty = if f.fields.fields.is_empty() {
				quote!(#ident)
			} else {
				quote!(#ident (..))
			};
			let ty_str = ident.to_string();

			quote! {
				Self::#ty => #ty_str,
			}
		});

		tokens.extend(quote! {
			impl #generics #ident #generics {
				pub fn tr(&self) -> &'static str {
					match self {
						#(#get_tr)*
					}
				}
			}
		});
	}
}

#[derive(Debug, FromVariant)]
#[darling(attributes())]
pub(crate) struct FieldReceiver {
	pub ident: syn::Ident,
	pub fields: ast::Fields<()>,
}
