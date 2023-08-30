use quote::{quote, ToTokens};

use super::mode::Mode;

#[derive(Debug, Clone, Copy)]
pub enum Access<'m> {
	None,
	NoneDiff,
	Mode(&'m Mode<'m>),
	ModeDiff(&'m Mode<'m>),
	ModeSum(&'m Mode<'m>),
	ModeSumDiff(&'m Mode<'m>),
}

impl<'m> Access<'m> {
	pub fn with_mode(self, mode: &'m Mode<'m>) -> Access<'m> {
		match self {
			Self::None => Self::ModeSum(mode),
			Self::NoneDiff => Self::ModeSumDiff(mode),
			_ => panic!("cannot call with_mode on {:?}", self),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
	None,
	Lhs,
	Rhs,
}

impl Side {
	pub fn into_game<'m>(self, mode: Option<&'m Mode<'m>>) -> Key<'m> {
		match self {
			Self::None => Key::Game(mode),
			Self::Lhs => Key::GameLhs(mode),
			Self::Rhs => Key::GameRhs(mode),
		}
	}

	pub fn into_data(self) -> Key<'static> {
		match self {
			Self::None => Key::Data,
			Self::Lhs => Key::DataLhs,
			Self::Rhs => Key::DataRhs,
		}
	}

	pub fn into_stats(self) -> Key<'static> {
		match self {
			Self::None => Key::Stats,
			Self::Lhs => Key::StatsLhs,
			Self::Rhs => Key::StatsRhs,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Key<'a> {
	Stats,
	StatsLhs,
	StatsRhs,
	Game(Option<&'a Mode<'a>>),
	GameLhs(Option<&'a Mode<'a>>),
	GameRhs(Option<&'a Mode<'a>>),
	Data,
	DataLhs,
	DataRhs,
}

impl ToTokens for Key<'_> {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let t = match self {
			Key::Stats => quote!(stats),
			Key::StatsLhs => quote!(stats_lhs),
			Key::StatsRhs => quote!(stats_rhs),
			Key::Game(Some(mode)) => quote!(game.#mode),
			Key::GameLhs(Some(mode)) => quote!(game_lhs.#mode),
			Key::GameRhs(Some(mode)) => quote!(game_rhs.#mode),
			Key::Game(None) => quote!(game),
			Key::GameLhs(None) => quote!(game_lhs),
			Key::GameRhs(None) => quote!(game_rhs),
			Key::Data => quote!(data),
			Key::DataLhs => quote!(data_lhs),
			Key::DataRhs => quote!(data_rhs),
		};

		tokens.extend(t);
	}
}
