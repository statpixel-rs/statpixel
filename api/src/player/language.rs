use std::borrow::Cow;

use serde::Deserialize;
use translate::context::Context;

use crate::canvas::label::ToFormatted;

#[derive(Deserialize, bincode::Encode, bincode::Decode, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
	#[default]
	English,
	ChineseSimplified,
	ChineseTraditional,
	Czech,
	Danish,
	Dutch,
	Finnish,
	French,
	German,
	Greek,
	Hungarian,
	Italian,
	Japanese,
	Korean,
	Norwegian,
	Pirate,
	Polish,
	PortugueseBr,
	PortuguesePt,
	Romanian,
	Russian,
	Spanish,
	Swedish,
	Turkish,
	Ukrainian,
	#[serde(other)]
	Unknown,
}

impl ToFormatted for Language {
	fn to_formatted_label<'t, 'c: 't>(&'t self, _ctx: &'c Context<'c>) -> Cow<'t, str> {
		use Language::*;

		match self {
			English => "English".into(),
			ChineseSimplified => "Chinese Simplified".into(),
			ChineseTraditional => "Chinese Traditional".into(),
			Czech => "Czech".into(),
			Danish => "Danish".into(),
			Dutch => "Dutch".into(),
			Finnish => "Finnish".into(),
			French => "French".into(),
			German => "German".into(),
			Greek => "Greek".into(),
			Hungarian => "Hungarian".into(),
			Italian => "Italian".into(),
			Japanese => "Japanese".into(),
			Korean => "Korean".into(),
			Norwegian => "Norwegian".into(),
			Pirate => "Pirate".into(),
			Polish => "Polish".into(),
			PortugueseBr => "Portuguese (BR)".into(),
			PortuguesePt => "Portuguese (PT)".into(),
			Romanian => "Romanian".into(),
			Russian => "Russian".into(),
			Spanish => "Spanish".into(),
			Swedish => "Swedish".into(),
			Turkish => "Turkish".into(),
			Ukrainian => "Ukrainian".into(),
			Unknown => "Unknown".into(),
		}
	}
}
