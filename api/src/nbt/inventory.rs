use serde::{Deserialize, Deserializer};

use crate::skyblock::{modifier::Modifier, pet::Pet};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Inventory<const HOTBAR: bool = false> {
	#[serde(deserialize_with = "nested_items::<_, HOTBAR>", rename = "i")]
	pub items: Vec<Option<Item>>,
}

#[derive(Clone, Debug, Default)]
pub struct Item {
	pub id: String,
	pub name: String,
	pub count: u8,
	pub damage: u16,
	pub party_hat: PartyHat,
	pub skin: Option<String>,
	pub pet: Option<Pet>,
	pub rune: Option<Rune>,
	pub price: Option<f64>,
	pub cake: Option<u16>,
	pub cake_years: Vec<u16>,
	pub edition: Option<u32>,
	pub shiny: Option<bool>,
	pub auction: bool,
	pub winning_bid: Option<f64>,
	pub enchantments: Vec<Enchantment>,
	pub attributes: Vec<Attribute>,
	pub modifier: Option<Modifier>,
	pub drill: Drill,
	pub scrolls: Vec<String>,

	pub wood_singularity: Option<u32>,
	pub tuned_transmission: Option<u32>,
	pub mana_disintegrator: Option<u32>,
	pub thunder_charge: Option<u32>,
	pub hot_potato: Option<u32>,
	pub dye: Option<String>,
	pub art_of_war: Option<u32>,
	pub art_of_peace: Option<u32>,
	pub farming_for_dummies: Option<u32>,
	pub talisman_enrichment: Option<String>,
	pub rarity_upgrades: Option<u32>,
	pub ethermerge: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct Drill {
	pub upgrade_module: Option<String>,
	pub fuel_tank: Option<String>,
	pub engine: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Attribute {
	pub id: String,
	pub tier: u8,
}

#[derive(Clone, Debug)]
pub struct Enchantment {
	pub id: String,
	pub level: u8,
}

#[derive(Clone, Debug, Default)]
pub struct PartyHat {
	pub emoji: Option<String>,
	pub colour: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Rune {
	pub id: String,
	pub tier: u8,
}

impl TryFrom<raw::Item> for Item {
	type Error = ();

	fn try_from(item: raw::Item) -> Result<Self, Self::Error> {
		item.tag
			.and_then(|t| {
				t.extra.and_then(|mut e| {
					e.id.map(|id| Self {
						id,
						name: t.display.name,
						count: item.count,
						damage: item.damage,
						pet: e.pet,
						skin: e.skin,
						rune: if e.runes.is_empty() {
							None
						} else {
							Some(e.runes.swap_remove(0))
						}
						.map(|(id, tier)| Rune { id, tier }),
						price: e.price,
						party_hat: PartyHat {
							emoji: e.party_hat_emoji,
							colour: e.party_hat_color,
						},
						cake: e.cake,
						edition: e.edition,
						shiny: e.shiny,
						auction: e.bid.is_some(),
						winning_bid: e.winning_bid,
						enchantments: e
							.enchantments
							.into_iter()
							.map(|(id, level)| Enchantment { id, level })
							.collect(),
						attributes: e
							.attributes
							.into_iter()
							.map(|(id, tier)| Attribute { id, tier })
							.collect(),
						wood_singularity: e.wood_singularity,
						tuned_transmission: e.tuned_transmission,
						mana_disintegrator: e.mana_disintegrator,
						thunder_charge: e.thunder_charge,
						hot_potato: e.hot_potato,
						dye: e.dye,
						art_of_war: e.art_of_war,
						art_of_peace: e.art_of_peace,
						farming_for_dummies: e.farming_for_dummies,
						talisman_enrichment: e.talisman_enrichment,
						rarity_upgrades: e.rarity_upgrades,
						modifier: e.modifier,
						drill: Drill {
							upgrade_module: e.drill_part_upgrade_module,
							fuel_tank: e.drill_part_fuel_tank,
							engine: e.drill_part_engine,
						},
						scrolls: e.scrolls,
						ethermerge: e.ethermerge,
						cake_years: e.cake_years,
					})
				})
			})
			.ok_or(())
	}
}

impl Item {
	#[must_use]
	pub fn empty(id: String) -> Self {
		Self {
			id,
			count: 1,
			auction: false,
			..Item::default()
		}
	}

	#[must_use]
	pub fn empty_wth_name(id: String, name: String) -> Self {
		Self {
			id,
			name,
			count: 1,
			auction: false,
			..Item::default()
		}
	}
}

pub mod raw {
	use super::{Modifier, Pet};
	use serde::Deserialize;

	#[derive(Deserialize)]
	pub struct Item {
		pub tag: Option<Tag>,
		#[serde(rename = "Count", default)]
		pub count: u8,
		#[serde(rename = "Damage", default)]
		pub damage: u16,
	}

	#[derive(Deserialize)]
	pub struct Tag {
		#[serde(rename = "ExtraAttributes")]
		pub extra: Option<ExtraAttributes>,
		pub display: Display,
	}

	#[derive(Deserialize)]
	pub struct Display {
		#[serde(rename = "Name")]
		pub name: String,
	}

	#[derive(Deserialize)]
	pub struct ExtraAttributes {
		pub id: Option<String>,
		pub pet: Option<Pet>,
		pub skin: Option<String>,
		pub party_hat_emoji: Option<String>,
		pub party_hat_color: Option<String>,
		#[serde(with = "crate::ser::f64_string_opt", default)]
		pub price: Option<f64>,
		#[serde(with = "crate::ser::vec_map", default)]
		pub runes: Vec<(String, u8)>,
		#[serde(rename = "new_years_cake")]
		pub cake: Option<u16>,
		#[serde(rename = "new_year_cake_bag_years", default)]
		pub cake_years: Vec<u16>,
		pub edition: Option<u32>,
		#[serde(rename = "is_shiny")]
		pub shiny: Option<bool>,
		pub bid: Option<f64>,
		pub winning_bid: Option<f64>,
		#[serde(with = "crate::ser::vec_map_uppercase", default)]
		pub enchantments: Vec<(String, u8)>,
		#[serde(with = "crate::ser::vec_map_uppercase", default)]
		pub attributes: Vec<(String, u8)>,
		pub modifier: Option<Modifier>,
		#[serde(rename = "ability_scroll", default)]
		pub scrolls: Vec<String>,
		pub drill_part_upgrade_module: Option<String>,
		pub drill_part_fuel_tank: Option<String>,
		pub drill_part_engine: Option<String>,

		#[serde(rename = "wood_singularity_count")]
		pub wood_singularity: Option<u32>,
		pub tuned_transmission: Option<u32>,
		#[serde(rename = "mana_disintegrator_count")]
		pub mana_disintegrator: Option<u32>,
		pub thunder_charge: Option<u32>,
		#[serde(rename = "hot_potato_count")]
		pub hot_potato: Option<u32>,
		#[serde(rename = "dye_item")]
		pub dye: Option<String>,
		#[serde(rename = "art_of_war_count")]
		pub art_of_war: Option<u32>,
		#[serde(rename = "artOfPeaceApplied")]
		pub art_of_peace: Option<u32>,
		#[serde(rename = "farming_for_dummies_count")]
		pub farming_for_dummies: Option<u32>,
		pub talisman_enrichment: Option<String>,
		pub rarity_upgrades: Option<u32>,
		pub ethermerge: Option<u32>,
		// pub auction: Option<?>
	}
}

fn nested_items<'de, D, const HOTBAR: bool>(deserializer: D) -> Result<Vec<Option<Item>>, D::Error>
where
	D: Deserializer<'de>,
{
	let mut items = Vec::<raw::Item>::deserialize(deserializer)?;

	if HOTBAR {
		// Move the first 9 items to be the last 9 items
		for _ in 0..9 {
			let item = items.remove(0);

			items.push(item);
		}
	}

	Ok(items.into_iter().map(|i| i.try_into().ok()).collect())
}
