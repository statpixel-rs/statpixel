use std::iter::Peekable;

use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn sum_div_f32_fields<'id>(
	mut fields: impl Iterator<Item = &'id syn::Ident>,
	parent: Option<&TokenStream>,
	top: &syn::Ident,
	bottom: &syn::Ident,
) -> TokenStream {
	let first = match fields.next() {
		Some(first) => first,
		None => panic!("game::sum::sum_div_f32_field_stats must be called with at least one field"),
	};

	let parent_token = parent.map_or_else(|| quote! { #first }, |p| quote! { #p.#first });
	let second = match fields.next() {
		Some(second) => second,
		None => return div_f32_single_field(&parent_token, top, bottom),
	};

	let (rest_top, rest_bottom) = div_multi_fields_with_leading_plus(fields, parent, top, bottom);
	let parent = parent.map(|p| quote! { #p. }).unwrap_or(quote! {});

	quote! {
		{
			let sum_top = #parent #first.#top + #parent #second.#top #rest_top;
			let sum_bottom = #parent #first.#bottom + #parent #second.#bottom #rest_bottom;

			sum_top as f32 / if sum_bottom == 0 { 1. } else { sum_bottom as f32 }
		}
	}
}

pub(crate) fn sum_div_u32_fields<'id>(
	mut fields: impl Iterator<Item = &'id syn::Ident>,
	parent: Option<&TokenStream>,
	top: &syn::Ident,
	bottom: &syn::Ident,
) -> TokenStream {
	let first = match fields.next() {
		Some(first) => first,
		None => panic!("game::sum::sum_div_u32_field_stats must be called with at least one field"),
	};

	let parent_token = parent.map_or_else(|| quote! { #first }, |p| quote! { #p.#first });
	let second = match fields.next() {
		Some(second) => second,
		None => return div_u32_single_field(&parent_token, top, bottom),
	};

	let (rest_top, rest_bottom) = div_multi_fields_with_leading_plus(fields, parent, top, bottom);
	let parent = parent.map(|p| quote! { #p. }).unwrap_or(quote! {});

	quote! {
		{
			let sum_top = #parent #first.#top + #parent #second.#top #rest_top;
			let sum_bottom = #parent #first.#bottom + #parent #second.#bottom #rest_bottom;

			::std::cmp::min(100, sum_top * 100 / if sum_bottom == 0 { 1 } else { sum_bottom })
		}
	}
}

pub(crate) fn sum_fields<'id>(
	mut fields: Peekable<impl Iterator<Item = &'id syn::Ident>>,
	parent: Option<&syn::Ident>,
	key: &syn::Ident,
) -> TokenStream {
	let first = match fields.next() {
		Some(first) => first,
		None => panic!("game::sum::sum_field_stats must be called with at least one field"),
	};

	let parent_token = parent.map_or_else(|| quote! { #first }, |p| quote! { #p. #first });
	let second = match fields.next() {
		Some(second) => second,
		None => return sum_single_field(&parent_token, key),
	};

	let rest =
		sum_multi_fields_with_leading_plus(fields, parent.map(|p| quote! { #p }).as_ref(), key);
	let parent = parent.map(|p| quote! { #p. }).unwrap_or(quote! {});

	quote! {
		{ #parent #first.#key + #parent #second.#key #rest }
	}
}

/// Returns the sums of all fields as (top, bottom)
fn div_multi_fields_with_leading_plus<'id>(
	fields: impl Iterator<Item = &'id syn::Ident>,
	parent: Option<&TokenStream>,
	top: &syn::Ident,
	bottom: &syn::Ident,
) -> (TokenStream, TokenStream) {
	let parent = parent.map(|p| quote! { #p. }).unwrap_or(quote! {});
	let mut sum_top = TokenStream::new();
	let mut sum_bottom = TokenStream::new();

	for field in fields {
		sum_top.extend(quote! {
			+ #parent #field.#top
		});

		sum_bottom.extend(quote! {
			+ #parent #field.#bottom
		});
	}

	(sum_top, sum_bottom)
}

fn sum_multi_fields_with_leading_plus<'id>(
	fields: impl Iterator<Item = &'id syn::Ident>,
	parent: Option<&TokenStream>,
	key: &syn::Ident,
) -> TokenStream {
	let parent = parent.map(|p| quote! { #p. }).unwrap_or(quote! {});
	let mut stream = TokenStream::new();

	for field in fields {
		stream.extend(quote! {
			+ #parent #field.#key
		});
	}

	stream
}

pub fn div_f32_single_field(
	parent: &TokenStream,
	top: &syn::Ident,
	bottom: &syn::Ident,
) -> TokenStream {
	quote! {
		{
			let bottom = #parent.#bottom;

			#parent.#top as f32 / if bottom == 0 { 1. } else { bottom as f32 }
		}
	}
}

pub fn div_u32_single_field(
	parent: &TokenStream,
	top: &syn::Ident,
	bottom: &syn::Ident,
) -> TokenStream {
	quote! {
		{
			let bottom = #parent.#bottom;

			::std::cmp::min(100, #parent.#top * 100 / if bottom == 0 { 1 } else { bottom })
		}
	}
}

pub fn sum_single_field(parent: &TokenStream, key: &syn::Ident) -> TokenStream {
	quote! {
		#parent.#key
	}
}
