use serde::Deserialize;

use super::networth::{
	calc::{Category, CategoryKind},
	Prices,
};

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Essence {
	#[serde(rename = "essence_undead")]
	pub undead: u32,
	#[serde(rename = "essence_dragon")]
	pub dragon: u32,
	#[serde(rename = "essence_gold")]
	pub gold: u32,
	#[serde(rename = "essence_diamond")]
	pub diamond: u32,
	#[serde(rename = "essence_wither")]
	pub wither: u32,
	#[serde(rename = "essence_spider")]
	pub spider: u32,
	#[serde(rename = "essence_ice")]
	pub ice: u32,
	#[serde(rename = "essence_crimson")]
	pub crimson: u32,
}

impl Essence {
	#[must_use]
	pub fn worth(&self, prices: &Prices) -> Category {
		Category {
			kind: Some(CategoryKind::Essence),
			items: vec![],
			value: prices
				.get("ESSENCE_UNDEAD")
				.map(|w| *w * f64::from(self.undead))
				.unwrap_or_default()
				+ prices
					.get("ESSENCE_DRAGON")
					.map(|w| *w * f64::from(self.dragon))
					.unwrap_or_default()
				+ prices
					.get("ESSENCE_GOLD")
					.map(|w| *w * f64::from(self.gold))
					.unwrap_or_default()
				+ prices
					.get("ESSENCE_DIAMOND")
					.map(|w| *w * f64::from(self.diamond))
					.unwrap_or_default()
				+ prices
					.get("ESSENCE_WITHER")
					.map(|w| *w * f64::from(self.wither))
					.unwrap_or_default()
				+ prices
					.get("ESSENCE_SPIDER")
					.map(|w| *w * f64::from(self.spider))
					.unwrap_or_default()
				+ prices
					.get("ESSENCE_ICE")
					.map(|w| *w * f64::from(self.ice))
					.unwrap_or_default()
				+ prices
					.get("ESSENCE_CRIMSON")
					.map(|w| *w * f64::from(self.crimson))
					.unwrap_or_default(),
		}
	}
}
