use proc_macro2::TokenStream;
use quote::quote;

macro_rules! ident {
	($id: literal) => {
		::syn::Ident::new($id, ::proc_macro2::Span::call_site())
	};
}

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

pub(crate) fn get_percent_ident_for_type(ty: syn::Type) -> syn::Ident {
	let type_id = match ty {
		syn::Type::Path(p) => p.path.segments.first().unwrap().ident.to_string(),
		_ => panic!("Unknown type for percent: {ty:?}"),
	};

	if type_id == "u32" {
		ident!("PercentU32")
	} else if type_id == "u64" {
		ident!("PercentU64")
	} else if type_id == "i32" {
		ident!("PercentI32")
	} else if type_id == "i64" {
		ident!("PercentI64")
	} else {
		panic!("Unknown type_id for percent: {type_id}");
	}
}

pub(crate) fn get_percent_ident_for_str(type_id: &str) -> syn::Ident {
	if type_id == "u32" {
		ident!("PercentU32")
	} else if type_id == "u64" {
		ident!("PercentU64")
	} else if type_id == "i32" {
		ident!("PercentI32")
	} else if type_id == "i64" {
		ident!("PercentI64")
	} else {
		panic!("Unknown type str for percent: {type_id:?}",);
	}
}
