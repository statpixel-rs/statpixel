use std::backtrace::Backtrace;
use std::borrow::Cow;

use darling::{FromField, FromMeta};
use minecraft::paint::Paint;
use quote::quote;

use super::{
	block::Block,
	key::{Access, Side},
	label::Label,
	mode::Mode,
	prelude::{Field, FieldGroup},
	GameInputReceiver,
};

#[derive(Debug, FromField)]
#[darling(attributes(game))]
pub(crate) struct GameFieldReceiver {
	/// Get the ident of the field. For fields in tuple or newtype structs or
	/// enum bodies, this can be `None`.
	pub ident: Option<syn::Ident>,

	/// This magic field name pulls the type from the input.
	pub ty: syn::Type,

	/// Field data
	pub label: Option<GameLabel>,

	/// Mode data
	pub mode: Option<ModeData>,

	/// Field that stores player xp
	pub xp: darling::util::Flag,
	pub nominal: darling::util::Flag,
}

#[derive(Debug, FromMeta)]
pub(crate) struct GameLabel {
	#[darling(default)]
	pub colour: Paint,

	/// The translation key of the label.
	/// Defaults to the ident with underscores replaced with dashes.
	pub tr: Option<String>,

	pub div: Option<syn::Ident>,

	pub percent: darling::util::Flag,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ModeData {
	pub skip_overall: darling::util::Flag,
	/// The Hypixel mode name, for use in automatically
	/// selecting the current mode a player is playing.
	pub hypixel: Option<String>,
	pub calc: Option<syn::Path>,
	/// A path to a function that computes the XP for the mode given the game data
	pub xp: Option<syn::Path>,
	pub tr: Option<String>,

	#[darling(multiple)]
	pub field: Vec<ModeFieldData>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ModeFieldData {
	pub ident: syn::Ident,
	pub div: Option<syn::Ident>,

	pub tr: Option<String>,

	#[darling(default)]
	pub colour: Paint,

	pub percent: Option<String>,
	pub path: Option<String>,
	pub skip_chart: darling::util::Flag,
}

#[derive(Debug, FromMeta)]
pub(crate) struct OverallFieldData {
	pub ident: syn::Ident,

	pub div: Option<syn::Ident>,

	pub tr: Option<String>,

	#[darling(default)]
	pub colour: Paint,

	pub percent: Option<String>,
	pub min: darling::util::Flag,

	pub path: Option<String>,

	pub nominal: darling::util::Flag,
	pub skip_chart: darling::util::Flag,
	/// A list of modes to skip. These should be equal to the
	/// key used for their data in the game data struct.
	#[darling(multiple)]
	pub skip_mode: Vec<String>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct InfoFieldData {
	pub ident: syn::Ident,

	pub div: Option<syn::Ident>,

	pub tr: Option<String>,

	#[darling(default)]
	pub colour: Paint,

	pub percent: Option<String>,

	pub path: Option<String>,
	pub nominal: darling::util::Flag,
}

impl GameInputReceiver {
	pub fn overall_modes(&self) -> Vec<Mode> {
		self.data
			.as_ref()
			.take_struct()
			.expect("GameInputReceiver::modes expected data as a struct")
			.fields
			.into_iter()
			.filter_map(|f| {
				if let Some(ref mode) = f.mode && !mode.skip_overall.is_present() { f.try_into().ok() } else { None }
			})
			.collect::<Vec<_>>()
	}

	pub fn modes(&self) -> Vec<Mode> {
		self.data
			.as_ref()
			.take_struct()
			.expect("GameInputReceiver::modes expected data as a struct")
			.fields
			.into_iter()
			.filter_map(|f| f.try_into().ok())
			.collect::<Vec<_>>()
	}
}

impl FieldGroup for GameInputReceiver {
	fn xp(
		&self,
		side: Side,
		game_stats_ident: &proc_macro2::TokenStream,
		mode: Option<&syn::Ident>,
	) -> Option<Cow<proc_macro2::TokenStream>> {
		if let Some(path) = self.xp_local.as_ref() {
			let path_tokens = syn::parse_str::<proc_macro2::TokenStream>(path)
				.expect("could not parse xp_local into a TokenStream");

			if let Some(mode) = mode {
				let key = side.into_data();

				Some(Cow::Owned(
					quote!(#key.stats.#game_stats_ident.#mode.#path_tokens),
				))
			} else {
				let modes = self.overall_modes();

				Some(Cow::Owned(
					self.blocks()
						.iter()
						.find(|b| b.id().to_string().eq(path))
						.expect("could not find block with id matching xp_local")
						.value_sum(side, &modes, Access::None)
						.unwrap_or_else(|| {
							panic!(
								"could not compute xp field with xp_local {}",
								Backtrace::capture()
							)
						}),
				))
			}
		} else if let Some(xp) = self.xp.as_ref() {
			let key = side.into_data();
			let path: proc_macro2::TokenStream =
				syn::parse_str(xp).expect("could not parse xp into a TokenStream");

			Some(Cow::Owned(quote!(#key.stats.#path)))
		} else {
			let key = side.into_data();
			let id = self
				.data
				.as_ref()
				.take_struct()
				.expect("FieldGroup::xp expected data as a struct")
				.fields
				.iter()
				.find(|f| f.xp.is_present())
				.map_or_else(
					|| quote!(#key.xp),
					|f| {
						let id = f.ident.as_ref().expect("xp field must have an ident");
						quote!(#key.stats.#game_stats_ident.#id)
					},
				);

			Some(Cow::Owned(id))
		}
	}

	fn blocks(&self) -> Cow<Vec<Block>> {
		let mut fields: Vec<_> = self
			.data
			.as_ref()
			.take_struct()
			.expect("FieldGroup::field_values expected data as a struct")
			.fields
			.into_iter()
			.filter_map(|f| f.try_into().ok())
			.collect();

		fields.extend(self.field.iter().filter_map(|f| f.try_into().ok()));
		Cow::Owned(fields)
	}

	fn labels(&self) -> Cow<Vec<Label>> {
		let mut labels: Vec<_> = self
			.data
			.as_ref()
			.take_struct()
			.expect("FieldGroup::fields expected data as a struct")
			.fields
			.into_iter()
			.filter_map(|f| f.try_into().ok())
			.collect();

		labels.extend(self.label.iter().filter_map(|f| f.try_into().ok()));
		Cow::Owned(labels)
	}
}
