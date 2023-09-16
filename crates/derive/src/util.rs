use std::borrow::Cow;

pub(crate) fn ident(s: &str) -> syn::Ident {
	syn::Ident::new(s, proc_macro2::Span::call_site())
}

pub(crate) fn get_tr_with_fallback<'t>(
	tr: Option<&'t str>,
	ident: Option<&syn::Ident>,
) -> Cow<'t, str> {
	if let Some(tr) = tr {
		tr.into()
	} else if let Some(ident) = ident {
		let name = ident.to_string().replace('_', "-");

		name.into()
	} else {
		panic!("get_tr_with_fallback called with no tr or ident")
	}
}

pub(crate) fn crate_ident(name: &str) -> syn::Ident {
	let crate_ = proc_macro_crate::crate_name(name).expect("crate not found");

	match crate_ {
		proc_macro_crate::FoundCrate::Itself => ident("crate"),
		proc_macro_crate::FoundCrate::Name(name) => ident(&name),
	}
}
