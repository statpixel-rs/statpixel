#![feature(let_chains)]
#![feature(iter_intersperse)]
#![feature(extend_one)]

mod diff;
mod game;
mod get_tr;
mod mode_trait;

pub(crate) mod util;

use std::{fs::File, io::Write};

use diff::DiffInputReceiver;
use game::GameInputReceiver;
use get_tr::GetTrInputReceiver;

use darling::FromDeriveInput;
use mode_trait::ModeTraitInputReceiver;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_derive(Game, attributes(game))]
pub fn derive_game(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as syn::DeriveInput);
	let receiver = GameInputReceiver::from_derive_input(&input);

	match receiver {
		Ok(receiver) => {
			std::fs::create_dir_all("./target/generated/derive").unwrap();

			let mut w =
				File::create(format!("./target/generated/derive/{}.rs", receiver.ident)).unwrap();
			let id = receiver.ident.to_string();
			let tokens = receiver.into_token_stream();

			w.write_all(tokens.to_string().as_bytes()).unwrap();

			if id == "Arcade" {
				proc_macro2::TokenStream::new().into()
			} else {
				tokens.into()
			}
		}
		Err(error) => error.write_errors().into(),
	}
}

#[proc_macro_derive(ModeTrait, attributes(mode))]
pub fn derive_mode_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as syn::DeriveInput);
	let receiver = ModeTraitInputReceiver::from_derive_input(&input);

	match receiver {
		Ok(receiver) => receiver.into_token_stream().into(),
		Err(error) => error.write_errors().into(),
	}
}

#[proc_macro_derive(GetTr, attributes(tr))]
pub fn derive_get_tr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as syn::DeriveInput);
	let receiver = GetTrInputReceiver::from_derive_input(&input);

	match receiver {
		Ok(receiver) => receiver.into_token_stream().into(),
		Err(error) => error.write_errors().into(),
	}
}

#[proc_macro_derive(Diff)]
pub fn derive_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as syn::DeriveInput);
	let receiver = DiffInputReceiver::from_derive_input(&input);

	match receiver {
		Ok(receiver) => receiver.into_token_stream().into(),
		Err(error) => error.write_errors().into(),
	}
}
