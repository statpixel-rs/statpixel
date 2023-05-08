use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn get_tr_with_fallback(tr: Option<&str>, ident: Option<&syn::Ident>) -> TokenStream {
	if let Some(tr) = tr {
		quote! { #tr }
	} else if let Some(ident) = ident {
		let name = ident.to_string().replace('_', "-");

		quote! { #name }
	} else {
		panic!("get_tr_with_fallback called with no tr or ident")
	}
}
