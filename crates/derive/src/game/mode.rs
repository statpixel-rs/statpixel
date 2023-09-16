use std::borrow::Cow;

use quote::{quote, ToTokens};

use super::{block::Block, key::Side, prelude::FieldGroup, structs::GameFieldReceiver};

#[derive(Debug)]
pub struct Mode<'a> {
	pub ident: &'a syn::Ident,
	pub ty: &'a syn::Type,
	pub xp: Option<&'a syn::Path>,
	pub calc: Option<&'a syn::Path>,
	pub tr: Cow<'a, str>,
	pub name: Option<&'a str>,

	pub blocks: Vec<Block<'a>>,
}

impl ToTokens for Mode<'_> {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let id = self.id();

		tokens.extend_one(quote!(#id));
	}
}

impl Mode<'_> {
	#[inline]
	pub fn id(&self) -> &syn::Ident {
		self.ident
	}

	#[inline]
	pub fn ty(&self) -> &syn::Type {
		self.ty
	}

	#[inline]
	pub fn tr(&self) -> Cow<str> {
		self.tr.clone()
	}

	#[inline]
	pub fn name(&self) -> Option<&str> {
		self.name
	}
}

impl FieldGroup for Mode<'_> {
	#[inline]
	fn blocks(&self) -> Cow<Vec<Block>> {
		Cow::Borrowed(&self.blocks)
	}

	#[inline]
	fn xp(
		&self,
		side: Side,
		game_stats_ident: &proc_macro2::TokenStream,
		_: Option<&syn::Ident>,
	) -> Option<Cow<proc_macro2::TokenStream>> {
		let key = side.into_data();

		self.xp
			.as_ref()
			.map(|func| Cow::Owned(quote!(#func(&#key.stats.#game_stats_ident))))
	}

	#[inline]
	fn labels(&self) -> Cow<Vec<super::label::Label>> {
		Cow::Owned(vec![])
	}
}

impl<'a> TryFrom<&'a GameFieldReceiver> for Mode<'a> {
	type Error = &'static str;

	fn try_from(value: &'a GameFieldReceiver) -> Result<Self, Self::Error> {
		let mode = value.mode.as_ref().ok_or("missing mode")?;
		let ident = value.ident.as_ref().ok_or("missing ident")?;

		Ok(Self {
			ident,
			ty: &value.ty,
			xp: mode.xp.as_ref(),
			calc: mode.calc.as_ref(),
			tr: if let Some(ref tr) = mode.tr {
				tr.into()
			} else {
				let ty = &value.ty;
				quote!(#ty).to_string().into()
			},
			blocks: mode.field.iter().map(Block::from).collect(),
			name: mode.hypixel.as_deref(),
		})
	}
}
