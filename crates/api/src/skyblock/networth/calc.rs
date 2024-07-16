pub mod worth {
	pub const ENRICHMENT: f64 = 0.5;
	pub const FARMING_FOR_DUMMIES: f64 = 0.5;
	pub const WOOD_SINGULARITY: f64 = 0.5;
	pub const ART_OF_WAR: f64 = 0.6;
	pub const FUMING_POTATO_BOOK: f64 = 0.6;
	pub const GEMSTONE_SLOTS: f64 = 0.6;
	pub const RUNES: f64 = 0.6;
	pub const TUNED_TRANSMISSION: f64 = 0.7;
	pub const ESSENCE: f64 = 0.75;
	pub const SILEX: f64 = 0.75;
	pub const ART_OF_PEACE: f64 = 0.8;
	pub const MANA_DISINTEGRATOR: f64 = 0.8;
	pub const RECOMB: f64 = 0.8;
	pub const THUNDER_IN_A_BOTTLE: f64 = 0.8;
	pub const ENCHANTS: f64 = 0.85;
	pub const SHENS_AUCTION_PRICE: f64 = 0.85;
	pub const DYE: f64 = 0.9;
	pub const GEMSTONE_CHAMBERS: f64 = 0.9;
	pub const ATTRIBUTES: f64 = 1.0;
	pub const DRILL_PART: f64 = 1.0;
	pub const ETHERWARP: f64 = 1.0;
	pub const MASTER_STAR: f64 = 1.0;
	pub const GEMSTONE: f64 = 1.0;
	pub const HOT_POTATO_BOOK: f64 = 1.0;
	pub const NECRON_BLADE_SCROLL: f64 = 1.0;
	pub const PRESTIGE_ITEM: f64 = 1.0;
	pub const REFORGE: f64 = 1.0;
	pub const WINNING_BID: f64 = 1.0;
	pub const PET_CANDY: f64 = 0.65;
	pub const PET_ITEM: f64 = 1.0;

	pub const COUNTER_STRIKE: f64 = 0.2;
	pub const BIG_BRAIN: f64 = 0.35;
	pub const ULTIMATE_INFERNO: f64 = 0.35;
	pub const OVERLOAD: f64 = 0.35;
	pub const ULTIMATE_SOUL_EATER: f64 = 0.35;
	pub const ULTIMATE_FATAL_TEMPO: f64 = 0.65;
}

use minecraft::text::{parse::minecraft_text, Text};
use std::borrow::Cow;

use crate::{
	nbt::inventory::{Attribute, Enchantment, Inventory, Item, Rune},
	skyblock::{member::Member, modifier::Modifier, prestige::PRESTIGES},
};

use super::{ItemCategory, Prices, Upgrade, ITEMS};

pub const TOP: usize = 3;

pub const ENRICHMENTS: &[&str] = &[
	"TALISMAN_ENRICHMENT_CRITICAL_CHANCE",
	"TALISMAN_ENRICHMENT_CRITICAL_DAMAGE",
	"TALISMAN_ENRICHMENT_DEFENSE",
	"TALISMAN_ENRICHMENT_HEALTH",
	"TALISMAN_ENRICHMENT_INTELLIGENCE",
	"TALISMAN_ENRICHMENT_MAGIC_FIND",
	"TALISMAN_ENRICHMENT_WALK_SPEED",
	"TALISMAN_ENRICHMENT_STRENGTH",
	"TALISMAN_ENRICHMENT_ATTACK_SPEED",
	"TALISMAN_ENRICHMENT_FEROCITY",
	"TALISMAN_ENRICHMENT_SEA_CREATURE_CHANCE",
];

#[derive(Debug)]
pub struct Networth {
	pub value: f64,
	pub categories: Vec<Category>,
}

#[derive(Debug)]
pub enum CategoryKind {
	Armour,
	Item,
	Pet,
	Accessory,
	Essence,
	Sack,
}

#[derive(Debug, Default)]
pub struct Category {
	pub kind: Option<CategoryKind>,
	pub value: f64,
	pub items: Vec<Item>,
}

impl CategoryKind {
	#[must_use]
	pub fn as_text(&self) -> &'static [Text<'static>] {
		use CategoryKind::*;

		match self {
			Armour => {
				const TEXT: [Text; 1] = minecraft_text("§fArmour");
				&TEXT
			}
			Item => {
				const TEXT: [Text; 1] = minecraft_text("§fItems");
				&TEXT
			}
			Pet => {
				const TEXT: [Text; 1] = minecraft_text("§fPets");
				&TEXT
			}
			Accessory => {
				const TEXT: [Text; 1] = minecraft_text("§fAccessories");
				&TEXT
			}
			Essence => {
				const TEXT: [Text; 1] = minecraft_text("§fEssence");
				&TEXT
			}
			Sack => {
				const TEXT: [Text; 1] = minecraft_text("§fSack");
				&TEXT
			}
		}
	}
}

impl Category {
	#[must_use]
	#[inline]
	pub fn empty(kind: CategoryKind) -> Self {
		Self {
			kind: Some(kind),
			value: 0.,
			items: vec![],
		}
	}
}

impl Member {
	/// Calculates the networth of a `SkyBlock` member.
	/// It does not include the value of their purse or island bank.
	#[must_use]
	#[allow(clippy::too_many_lines)]
	pub fn networth(&self, prices: &Prices) -> Networth {
		let pets = self.pets.as_ref().map_or(
			Category {
				kind: Some(CategoryKind::Pet),
				value: 0.,
				items: vec![],
			},
			|p| {
				let mut pets = p
					.iter()
					.map(|p| (p, p.worth(prices).unwrap_or_default()))
					.collect::<Vec<_>>();

				pets.sort_by(|a, b| b.1.total_cmp(&a.1));

				let value = if pets.len() > TOP {
					pets.drain(TOP..).map(|(_, w)| w).sum::<f64>()
				} else {
					0.
				} + pets.iter().map(|(_, w)| w).sum::<f64>();

				Category {
					kind: Some(CategoryKind::Pet),
					value,
					items: pets.into_iter().map(|p| p.0.clone().into()).collect(),
				}
			},
		);

		let sacks = {
			let value = self
				.sacks
				.iter()
				.map(|s| {
					prices
						.get(&s.0)
						.map(|w| *w * f64::from(s.1))
						.unwrap_or_default()
				})
				.sum::<f64>();

			Category {
				kind: Some(CategoryKind::Sack),
				value,
				items: vec![],
			}
		};

		let armour = {
			let mut items = Vec::new();

			if let Some(wardrobe) = self.wardrobe.as_ref() {
				items.extend(wardrobe.items.iter().filter_map(Option::as_ref));
			}

			if let Some(armour) = self.armour.as_ref() {
				items.extend(armour.items.iter().filter_map(Option::as_ref));
			}

			if let Some(equipment) = self.equipment.as_ref() {
				items.extend(equipment.items.iter().filter_map(Option::as_ref));
			}

			Inventory::<false>::worth_many(items, prices, CategoryKind::Armour)
		};

		let items = {
			let mut items = Vec::new();

			if let Some(inventory) = self.inventory.as_ref() {
				items.extend(inventory.items.iter().filter_map(Option::as_ref));
			}

			if let Some(ender_chest) = self.ender_chest.as_ref() {
				items.extend(ender_chest.items.iter().filter_map(Option::as_ref));
			}

			if let Some(quiver) = self.quiver.as_ref() {
				items.extend(quiver.items.iter().filter_map(Option::as_ref));
			}

			if let Some(fishing_bag) = self.fishing_bag.as_ref() {
				items.extend(fishing_bag.items.iter().filter_map(Option::as_ref));
			}

			if let Some(potion_bag) = self.potion_bag.as_ref() {
				items.extend(potion_bag.items.iter().filter_map(Option::as_ref));
			}

			if let Some(candy) = self.candy.as_ref() {
				items.extend(candy.items.iter().filter_map(Option::as_ref));
			}

			if let Some(vault) = self.vault.as_ref() {
				items.extend(vault.items.iter().filter_map(Option::as_ref));
			}

			self.backpack.iter().for_each(|b| {
				items.extend(b.items.iter().filter_map(Option::as_ref));
			});

			Inventory::<false>::worth_many(items, prices, CategoryKind::Item)
		};

		let categories = vec![
			self.essence.worth(prices),
			sacks,
			pets,
			armour,
			items,
			self.talisman_bag.as_ref().map_or_else(
				|| Category::empty(CategoryKind::Accessory),
				|b| b.worth(prices, CategoryKind::Accessory),
			),
		];

		Networth {
			value: categories.iter().map(|c| c.value).sum(),
			categories,
		}
	}
}

impl<const HOTBAR: bool> Inventory<HOTBAR> {
	#[must_use]
	pub fn worth(&self, prices: &Prices, kind: CategoryKind) -> Category {
		let mut items = self
			.items
			.iter()
			.filter_map(|i| i.as_ref().map(|i| (i, i.worth(prices).unwrap_or_default())))
			.collect::<Vec<_>>();

		items.sort_by(|a, b| b.1.total_cmp(&a.1));

		let value = if items.len() > TOP {
			items.drain(TOP..).map(|(_, w)| w).sum::<f64>()
		} else {
			0.
		} + items.iter().map(|(_, w)| w).sum::<f64>();

		Category {
			value,
			kind: Some(kind),
			items: items.into_iter().map(|i| i.0.clone()).collect(),
		}
	}

	#[must_use]
	pub fn worth_many(items: Vec<&Item>, prices: &Prices, kind: CategoryKind) -> Category {
		let mut items = items
			.into_iter()
			.map(|i| (i, i.worth(prices).unwrap_or_default()))
			.collect::<Vec<_>>();

		items.sort_by(|a, b| b.1.total_cmp(&a.1));

		let value = if items.len() > TOP {
			items.drain(TOP..).map(|(_, w)| w).sum::<f64>()
		} else {
			0.
		} + items.iter().map(|(_, w)| w).sum::<f64>();

		Category {
			value,
			kind: Some(kind),
			items: items.into_iter().map(|i| i.0.clone()).collect(),
		}
	}
}

impl Upgrade {
	#[must_use]
	pub fn worth(&self, prices: &Prices) -> Option<f64> {
		match self {
			Self::Essence { id, count, .. } => {
				let worth = prices.get(id).copied()?;
				let price = worth * f64::from(*count);

				Some(price * worth::ESSENCE)
			}
			Self::Item { id, count, .. } => {
				let worth = prices.get(id).copied()?;
				let price = worth * f64::from(*count);

				Some(price)
			}
		}
	}
}

impl Enchantment {
	#[must_use]
	pub fn worth(&self, prices: &Prices) -> Option<f64> {
		let id = format!("ENCHANTMENT_{}_{}", self.id, self.level);

		prices.get(&id).copied()
	}
}

impl Attribute {
	#[must_use]
	pub fn worth(&self, prices: &Prices) -> Option<f64> {
		if self.tier == 1 {
			return None;
		}

		let mut worth = prices.get(&self.id()).copied().unwrap_or_default();
		let base = self.base_worth(prices).unwrap_or_default();

		if base < worth {
			worth = base;
		}

		let mut split = self.id.split('_');

		if let (Some(prefix), Some(tier), Some(piece)) = (split.next(), split.next(), split.next())
		{
			if (prefix == "HOT" || prefix == "FIERY" || prefix == "BURNING" || prefix == "INFERNAL")
				&& (tier == "AURORA"
					|| tier == "CRIMSON"
					|| tier == "TERROR"
					|| tier == "HOLLOW"
					|| tier == "FERVOR")
			{
				let price = match piece {
					"HELMET" => prices.get(&format!("KUUDRA_HELMET_{}", self.id)).copied(),
					"CHESTPLATE" => prices
						.get(&format!("KUUDRA_CHESTPLATE_{}", self.id))
						.copied(),
					"LEGGINGS" => prices.get(&format!("KUUDRA_LEGGINGS_{}", self.id)).copied(),
					"BOOTS" => prices.get(&format!("KUUDRA_BOOTS_{}", self.id)).copied(),
					_ => None,
				}
				.unwrap_or_default();

				if price < worth {
					worth = price;
				}
			}
		};

		let shards = 2_u32.pow(u32::from(self.tier) - 1) - 1;

		Some(worth * f64::from(shards) * worth::ATTRIBUTES)
	}

	#[must_use]
	#[inline]
	pub fn id(&self) -> String {
		format!("ATTRIBUTE_SHARD_{}", self.id)
	}

	#[must_use]
	pub fn base_worth(&self, prices: &Prices) -> Option<f64> {
		prices
			.get(match self.id.as_str() {
				"GLOWSTONE_GAUNTLET" | "VANQUISHED_GLOWSTONE_GAUNTLET" => "GLOWSTONE_GAUNTLET",
				"BLAZE_BELT" | "VANQUISHED_BLAZE_BELT" => "BLAZE_BELT",
				"MAGMA_NECKLACE" | "VANQUISHED_MAGMA_NECKLACE" => "MAGMA_NECKLACE",
				"MAGMA_ROD" | "INFERNO_ROD" | "HELLFIRE_ROD" => "MAGMA_ROD",
				_ => return None,
			})
			.copied()
	}
}

impl Rune {
	#[must_use]
	pub fn worth(&self, prices: &Prices) -> Option<f64> {
		prices.get(&self.id()).copied()
	}

	#[must_use]
	#[inline]
	pub fn id(&self) -> String {
		format!("RUNE_{}_{}", self.id, self.tier)
	}
}

impl Item {
	#[allow(clippy::too_many_lines)]
	#[must_use]
	pub fn worth(&self, prices: &Prices) -> Option<f64> {
		if let Some(ref pet) = self.pet {
			return pet.worth(prices);
		}

		let data = ITEMS.get(&self.id);
		let id = self.id(prices);

		let worth = prices.get(id.as_ref()).copied().unwrap_or_default();

		let mut price = worth;

		if self.skin.is_some() {
			let worth = prices.get(&self.id).copied().unwrap_or_default();

			price = price.max(worth);
		}

		if let Some(worth) = self.price
			&& price == 0.
		{
			price = worth * 0.85;
		}

		if worth == 0.
			&& let Some(prestiges) = PRESTIGES.get(id.as_ref())
		{
			for data in prestiges.iter().filter_map(|p| ITEMS.get(*p)) {
				if let Some(ref upgrades) = data.upgrades {
					for upgrades in upgrades {
						for upgrade in upgrades {
							price += upgrade.worth(prices).unwrap_or_default();
						}
					}
				}

				if let Some(ref prestige) = data.prestige {
					for upgrade in &prestige.costs {
						price += upgrade.worth(prices).unwrap_or_default();
					}
				}
			}
		}

		if let Some(worth) = self.price
			&& self.auction
		{
			let worth = worth * worth::SHENS_AUCTION_PRICE;

			price = price.max(worth);
		}

		if id == "MIDAS_STAFF" || id == "MIDAS_SWORD" {
			if let Some(bid) = self.winning_bid {
				let max_bid = if id == "MIDAS_SWORD" {
					50_000_000.
				} else {
					100_000_000.
				};

				if bid >= max_bid {
					price = price.max(
						prices
							.get(if id == "MIDAS_SWORD" {
								"MIDAS_SWORD_50M"
							} else {
								"MIDAS_STAFF_100M"
							})
							.copied()
							.unwrap_or_default(),
					);
				} else {
					price = bid * worth::WINNING_BID;
				}
			}
		}

		if !self.enchantments.is_empty() {
			if id == "ENCHANTED_BOOK" {
				if self.enchantments.len() == 1 {
					price = self.enchantments[0].worth(prices).unwrap_or_default();
				} else {
					price = self
						.enchantments
						.iter()
						.map(|e| e.worth(prices).unwrap_or_default())
						.sum();
				}
			} else {
				for enchantment in &self.enchantments {
					if enchantment.id == "SCAVENGER" && enchantment.level == 5 {
						continue;
					}

					let value = if enchantment.id == "EXPERTISE"
						|| enchantment.id == "COMPACT"
						|| enchantment.id == "CULTIVATING"
						|| enchantment.id == "CHAMPION"
						|| enchantment.id == "HECATOMB"
					{
						1
					} else {
						enchantment.level
					};

					if value > 5 && enchantment.id == "EFFICIENCY" && id != "PROMISING_SPADE" {
						let worth = prices.get("sil_ex").copied().unwrap_or_default()
							* f64::from(value - if id == "STONK_PICKAXE" { 6 } else { 5 })
							* worth::SILEX;

						price += worth;
					}

					price += enchantment.worth(prices).unwrap_or_default()
						* match enchantment.id.as_str() {
							"COUNTER_STRIKE" => worth::COUNTER_STRIKE,
							"BIG_BRAIN" => worth::BIG_BRAIN,
							"ULTIMATE_INFERNO" => worth::ULTIMATE_INFERNO,
							"OVERLOAD" => worth::OVERLOAD,
							"ULTIMATE_SOUL_EATER" => worth::ULTIMATE_SOUL_EATER,
							"ULTIMATE_FATAL_TEMPO" => worth::ULTIMATE_FATAL_TEMPO,
							_ => worth::ENCHANTS,
						};
				}
			}
		}

		price += self
			.attributes
			.iter()
			.map(|a| a.worth(prices).unwrap_or_default())
			.sum::<f64>();

		if let Some(count) = self.wood_singularity {
			price += prices.get("WOOD_SINGULARITY").copied().unwrap_or_default()
				* f64::from(count)
				* worth::WOOD_SINGULARITY;
		}

		if let Some(count) = self.tuned_transmission {
			price += prices
				.get("TRANSMISSION_TUNER")
				.copied()
				.unwrap_or_default()
				* f64::from(count)
				* worth::TUNED_TRANSMISSION;
		}

		if let Some(count) = self.mana_disintegrator {
			price += prices
				.get("MANA_DISINTEGRATOR")
				.copied()
				.unwrap_or_default()
				* f64::from(count)
				* worth::MANA_DISINTEGRATOR;
		}

		if let Some(count) = self.thunder_charge
			&& id == "PULSE_RING"
		{
			price += prices
				.get("THUNDER_IN_A_BOTTLE")
				.copied()
				.unwrap_or_default()
				* f64::from(count)
				* worth::THUNDER_IN_A_BOTTLE;
		}

		if let Some(count) = self.hot_potato {
			if count > 10 {
				price += prices
					.get("FUMING_POTATO_BOOK")
					.copied()
					.unwrap_or_default()
					* f64::from(count - 10)
					* worth::FUMING_POTATO_BOOK;

				price += prices.get("HOT_POTATO_BOOK").copied().unwrap_or_default()
					* f64::from(10) * worth::HOT_POTATO_BOOK;
			} else {
				price += prices.get("HOT_POTATO_BOOK").copied().unwrap_or_default()
					* f64::from(count)
					* worth::HOT_POTATO_BOOK;
			}
		}

		if let Some(ref id) = self.dye {
			price += prices.get(id).copied().unwrap_or_default() * worth::DYE;
		}

		if let Some(count) = self.art_of_war {
			price += prices.get("THE_ART_OF_WAR").copied().unwrap_or_default()
				* f64::from(count)
				* worth::ART_OF_WAR;
		}

		if let Some(count) = self.art_of_peace {
			price += prices.get("THE_ART_OF_PEACE").copied().unwrap_or_default()
				* f64::from(count)
				* worth::ART_OF_PEACE;
		}

		if let Some(count) = self.farming_for_dummies {
			price += prices
				.get("FARMING_FOR_DUMMIES")
				.copied()
				.unwrap_or_default()
				* f64::from(count)
				* worth::FARMING_FOR_DUMMIES;
		}

		if self.talisman_enrichment.is_some() {
			let worth = ENRICHMENTS
				.iter()
				.map(|e| prices.get(*e).copied().unwrap_or_default())
				.fold(f64::INFINITY, f64::min);

			if worth.is_finite() {
				price += worth * worth::ENRICHMENT;
			}
		}

		if let Some(ref rune) = self.rune
			&& !id.starts_with("RUNE")
		{
			price += rune.worth(prices).unwrap_or_default() * worth::RUNES;
		}

		if matches!(self.rarity_upgrades, Some(1..))
			&& (!self.enchantments.is_empty()
				|| data.map(|d| {
					matches!(
						d.category,
						ItemCategory::Accessory
							| ItemCategory::Necklace
							| ItemCategory::Gloves
							| ItemCategory::Bracelet
							| ItemCategory::Belt | ItemCategory::Cloak
					)
				}) == Some(true)
				|| matches!(
					id.as_ref(),
					"DIVIAN_HELMET"
						| "DIVIAN_CHESTPLATE"
						| "DIVIAN_LEGGINGS"
						| "DIVIAN_BOOTS" | "FERMENTO_HELMET"
						| "FERMENTO_CHESTPLATE"
						| "FERMENTO_LEGGINGS"
						| "FERMENTO_BOOTS"
				)) {
			price += prices
				.get("RECOMBOBULATOR_3000")
				.copied()
				.unwrap_or_default()
				* if id == "BONE_BOOMERANG" {
					worth::RECOMB * 0.5
				} else {
					worth::RECOMB
				};
		}

		// TODO: gemstones
		// TODO: stars
		// TODO: upgrade costs

		if let Some(id) = self.modifier.as_ref().and_then(Modifier::reforge_id)
			&& data.map(|d| d.category != ItemCategory::Accessory) != Some(false)
		{
			price += prices.get(id).copied().unwrap_or_default() * worth::REFORGE;
		}

		for id in &self.scrolls {
			price += prices.get(id).copied().unwrap_or_default() * worth::NECRON_BLADE_SCROLL;
		}

		if let Some(ref id) = self.drill.upgrade_module {
			price += prices.get(id).copied().unwrap_or_default() * worth::DRILL_PART;
		}

		if let Some(ref id) = self.drill.fuel_tank {
			price += prices.get(id).copied().unwrap_or_default() * worth::DRILL_PART;
		}

		if let Some(ref id) = self.drill.engine {
			price += prices.get(id).copied().unwrap_or_default() * worth::DRILL_PART;
		}

		if let Some(1..) = self.ethermerge {
			price +=
				prices.get("ETHERWARP_CONDUIT").copied().unwrap_or_default() * worth::ETHERWARP;
		}

		for year in &self.cake_years {
			price += prices
				.get(&format!("NEW_YEAR_CAKE_{year}"))
				.copied()
				.unwrap_or_default();
		}

		Some(price * f64::from(self.count.max(1)))
	}

	#[must_use]
	#[inline]
	pub fn id(&self, prices: &Prices) -> Cow<str> {
		if let Some(ref rune) = self.rune
			&& self.id == "RUNE"
		{
			return rune.id().into();
		}

		if let Some(cake) = self.cake
			&& self.id == "NEW_YEAR_CAKE"
		{
			return format!("NEW_YEAR_CAKE_{cake}").into();
		}

		if let Some(ref colour) = self.party_hat.colour
			&& self.id.starts_with("PARTY_HAT_CRAB")
		{
			return format!("{}_{}", self.id, colour).into();
		}

		if self.id == "DCTR_SPACE_HELM" && self.edition.is_some() {
			return Cow::Borrowed("DCTR_SPACE_HELM_EDITIONED");
		}

		if self.shiny == Some(true) {
			let id = format!("{}_SHINY", self.id);

			if prices.contains_key(&id) {
				return id.into();
			}
		}

		if let Some(ref skin) = self.skin {
			let id = format!("{}_SKINNED_{skin}", self.id);

			if prices.contains_key(&id) {
				return Cow::Owned(id);
			}
		}

		if let Some(ref emoji) = self.party_hat.emoji
			&& self.id == "PARTY_HAT_SLOTH"
		{
			let id = format!("{}_{emoji}", self.id);

			if prices.contains_key(&id) {
				return Cow::Owned(id);
			}
		}

		Cow::Borrowed(&self.id)
	}
}
