mod mode;

pub(crate) use mode::impl_mode;

use crate::util::{crate_ident, ident};

use super::GameInputReceiver;

pub(crate) struct State<'t> {
	pub receiver: &'t GameInputReceiver,
	pub crates: Crates,
	pub idents: Idents,
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
			idents: Idents::new(
				&receiver.ident.to_string(),
				receiver.ident.clone(),
				calc,
				path_to_game,
			),
		}
	}
}

pub(crate) struct Idents {
	pub mode_enum: syn::Ident,
	pub kind_enum: syn::Ident,
	pub calc: syn::Path,
	pub game_ident: syn::Ident,
	pub path_to_game: proc_macro2::TokenStream,
}

impl Idents {
	pub fn new(
		id: &str,
		game_ident: syn::Ident,
		calc: syn::Path,
		path_to_game: proc_macro2::TokenStream,
	) -> Self {
		Self {
			mode_enum: ident(&format!("{}Mode", id)),
			kind_enum: ident(&format!("{}Kind", id)),
			calc,
			game_ident,
			path_to_game,
		}
	}
}

pub(crate) struct Crates {
	pub translate: syn::Ident,
	pub uuid: syn::Ident,
	pub poise: syn::Ident,
	pub chrono: syn::Ident,
	pub skia: syn::Ident,
	pub bincode: syn::Ident,
	pub bitcode: syn::Ident,
	pub minecraft: syn::Ident,
	pub serde: syn::Ident,
	pub hypixel: syn::Ident,
	pub api: syn::Ident,
	pub extra: syn::Ident,
	pub redis: syn::Ident,
}

impl Crates {
	pub fn new() -> Self {
		Self {
			translate: crate_ident("translate"),
			uuid: crate_ident("uuid"),
			poise: crate_ident("poise"),
			chrono: crate_ident("chrono"),
			skia: crate_ident("skia-safe"),
			bincode: crate_ident("bincode"),
			bitcode: crate_ident("bitcode"),
			minecraft: crate_ident("minecraft"),
			serde: crate_ident("serde"),
			hypixel: crate_ident("hypixel"),
			api: crate_ident("api"),
			extra: crate_ident("extra"),
			redis: crate_ident("redis"),
		}
	}
}
