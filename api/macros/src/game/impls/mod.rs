mod mode;

pub(crate) use mode::impl_mode;

use crate::util::{crate_ident, ident};

use super::{prelude::FieldGroup, GameInputReceiver};

pub(crate) struct State<'t> {
	pub receiver: &'t GameInputReceiver,
	pub crates: Crates,
	pub idents: Idents,
	pub streams: Streams,
}

impl<'t> State<'t> {
	pub fn new(receiver: &'t GameInputReceiver) -> Self {
		let calc = syn::parse_quote!(::minecraft::calc::network);
		let calc = receiver.calc.clone().unwrap_or(calc);

		let path_to_game = syn::parse_str::<proc_macro2::TokenStream>(&receiver.path)
			.expect("path should be a valid path");

		Self {
			receiver,
			crates: Crates::new(),
			idents: Idents::new(&receiver.ident.to_string(), calc, path_to_game),
			streams: Streams::new(receiver),
		}
	}
}

pub(crate) struct Streams {
	pub blocks_sum: proc_macro2::TokenStream,
	pub blocks_diff_sum: proc_macro2::TokenStream,
	pub labels_sum: proc_macro2::TokenStream,
	pub labels_diff_sum: proc_macro2::TokenStream,
}

impl Streams {
	pub fn new(receiver: &GameInputReceiver) -> Self {
		let modes = receiver.overall_modes();

		Self {
			blocks_sum: receiver.block_shapes_sum(&modes),
			blocks_diff_sum: receiver.block_shapes_diff_sum(&modes),
			labels_sum: receiver.label_shapes_sum(&modes),
			labels_diff_sum: receiver.label_shapes_diff_sum(&modes),
		}
	}
}

pub(crate) struct Idents {
	pub mode_enum: syn::Ident,
	pub kind_enum: syn::Ident,
	pub calc: syn::Path,
	pub path_to_game: proc_macro2::TokenStream,
}

impl Idents {
	pub fn new(id: &str, calc: syn::Path, path_to_game: proc_macro2::TokenStream) -> Self {
		Self {
			mode_enum: ident(&format!("{}Mode", id)),
			kind_enum: ident(&format!("{}Kind", id)),
			calc,
			path_to_game,
		}
	}
}

pub(crate) struct Crates {
	pub api: syn::Ident,
	pub translate: syn::Ident,
	pub uuid: syn::Ident,
	pub poise: syn::Ident,
	pub chrono: syn::Ident,
	pub skia: syn::Ident,
	pub bincode: syn::Ident,
	pub bitcode: syn::Ident,
	pub minecraft: syn::Ident,
	pub serde: syn::Ident,
	pub futures: syn::Ident,
}

impl Crates {
	pub fn new() -> Self {
		Self {
			api: crate_ident("api"),
			translate: crate_ident("translate"),
			uuid: crate_ident("uuid"),
			poise: crate_ident("poise"),
			chrono: crate_ident("chrono"),
			skia: crate_ident("skia-safe"),
			bincode: crate_ident("bincode"),
			bitcode: crate_ident("bitcode"),
			minecraft: crate_ident("minecraft"),
			serde: crate_ident("serde"),
			futures: crate_ident("futures"),
		}
	}
}
