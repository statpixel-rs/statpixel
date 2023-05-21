use proc_macro2::TokenStream;
use quote::quote;

use crate::tokens::{get_percent_ident_for_str, get_percent_ident_for_type, get_tr_with_fallback};

use super::{GameFieldReceiver, GameLabel, InfoFieldData};

pub(crate) fn parse_str_to_dot_path(path: &str) -> TokenStream {
	if path.contains('.') {
		let mut parts = path.split('.');
		let first = syn::Ident::new(parts.next().unwrap(), proc_macro2::Span::call_site());

		let rest = parts.map(|p| {
			let ident = syn::Ident::new(p, proc_macro2::Span::call_site());

			quote! { .#ident }
		});

		quote! { #first #(#rest)* }
	} else {
		let ident = syn::Ident::new(path, proc_macro2::Span::call_site());

		quote! { #ident }
	}
}

pub(crate) fn map_game_field_to_extras_value(
	(field, label): &(&GameFieldReceiver, &GameLabel),
) -> TokenStream {
	let name = field.ident.as_ref();
	let tr = get_tr_with_fallback(label.tr.as_deref(), name);

	let colour = &label.colour;
	let percent = label.percent == Some(true);

	let value = if let Some(div) = label.div.as_ref() {
		quote! { ::std::cmp::min(100, stats.#name * 100 / if stats.#div == 0 { 1 } else { stats.#div }) }
	} else {
		quote! { stats.#name }
	};

	if percent {
		let struct_name = get_percent_ident_for_type(field.ty.clone());

		quote! {
			crate::canvas::sidebar::item(
				ctx,
				surface,
				&(
					::translate::tr!(ctx, #tr),
					::std::boxed::Box::new(crate::extras::percent::#struct_name (#value)),
					#colour,
				),
				idx
			);

			idx += 1;
		}
	} else {
		quote! {
			crate::canvas::sidebar::item(
				ctx,
				surface,
				&(
					::translate::tr!(ctx, #tr),
					::std::boxed::Box::new(#value),
					#colour,
				),
				idx
			);

			idx += 1;
		}
	}
}

pub(crate) fn map_info_field_to_extras_value(info: &InfoFieldData) -> TokenStream {
	let name = &info.ident;
	let tr = get_tr_with_fallback(info.tr.as_deref(), Some(name));

	let colour = &info.colour;

	let value = if let Some(path) = info.path.as_ref() {
		let path = parse_str_to_dot_path(path);

		quote! { player.stats.#path.#name }
	} else if let Some(div) = info.div.as_ref() {
		quote! { ::std::cmp::min(100, self.#name * 100 / if self.#div == 0 { 1 } else { self.#div }) }
	} else {
		quote! { self.#name }
	};

	if let Some(ty) = info.percent.as_ref() {
		let struct_name = get_percent_ident_for_str(ty);

		quote! {
			crate::canvas::sidebar::item(
				ctx,
				surface,
				&(
					::translate::tr!(ctx, #tr),
					::std::boxed::Box::new(crate::extras::percent::#struct_name (#value)),
					#colour,
				),
				idx
			);

			idx += 1;
		}
	} else {
		quote! {
			crate::canvas::sidebar::item(
				ctx,
				surface,
				&(
					::translate::tr!(ctx, #tr),
					::std::boxed::Box::new(#value),
					#colour,
				),
				idx
			);

			idx += 1;
		}
	}
}
