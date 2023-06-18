pub mod calc;

use std::{collections::HashMap, fs::File, sync::Arc, time::Duration};

use moka::future::{Cache, CacheBuilder};
use once_cell::sync::Lazy;
use reqwest::Url;
use serde::Deserialize;

use crate::http::HTTP;

pub type Prices = HashMap<String, f64>;

#[derive(Debug, Deserialize)]
pub struct ItemData {
	pub id: String,
	#[serde(default)]
	pub category: ItemCategory,
	pub upgrades: Option<Vec<Vec<Upgrade>>>,
	pub prestige: Option<Prestige>,
}

#[derive(Debug, Deserialize)]
pub struct Prestige {
	#[serde(rename = "item_id")]
	pub id: String,
	pub costs: Vec<Upgrade>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Upgrade {
	Essence {
		#[serde(alias = "essence_type")]
		id: String,
		#[serde(rename = "amount")]
		count: u32,
	},
	Item {
		#[serde(rename = "item_id")]
		id: String,
		#[serde(rename = "amount")]
		count: u32,
	},
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemCategory {
	#[default]
	None,
	ReforgeStone,
	Sword,
	Boots,
	Belt,
	Necklace,
	Gloves,
	Helmet,
	Chestplate,
	Accessory,
	Cloak,
	Leggings,
	Axe,
	Hoe,
	Cosmetic,
	PetItem,
	Bait,
	FishingRod,
	Portal,
	Bow,
	Wand,
	DungeonPass,
	Arrow,
	Spade,
	Pickaxe,
	Deployable,
	Drill,
	Shears,
	Bracelet,
	Gauntlet,
	Longsword,
	TravelScroll,
	ArrowPoison,
	FishingWeapon,
}

pub static ITEMS: Lazy<Arc<HashMap<String, ItemData>>> = Lazy::new(|| {
	let path = std::path::Path::new("assets/item_data.json");
	let file = File::open(path).unwrap();

	let items: Vec<ItemData> = serde_json::from_reader(file).unwrap();

	Arc::new(
		items
			.into_iter()
			.map(|item| (item.id.clone(), item))
			.collect(),
	)
});

pub static PRICES_CACHE: Lazy<Cache<(), Arc<Prices>>> = Lazy::new(|| {
	CacheBuilder::new(1)
		.time_to_idle(Duration::from_secs(60 * 15))
		.time_to_live(Duration::from_secs(60 * 15))
		.build()
});

pub static URL: Lazy<Url> = Lazy::new(|| {
	Url::parse("https://raw.githubusercontent.com/SkyHelperBot/Prices/main/prices.json").unwrap()
});

async fn prices_raw() -> Result<Arc<HashMap<String, f64>>, crate::Error> {
	let response = HTTP.get(URL.clone()).send().await?;
	let prices = response.json::<HashMap<String, f64>>().await?;

	Ok(Arc::new(
		prices
			.into_iter()
			.map(|(mut k, v)| {
				k.make_ascii_uppercase();
				(k, v)
			})
			.collect(),
	))
}

/// # Errors
/// Will return an error if the request fails or if the response is not valid JSON.
pub async fn prices() -> Result<Arc<HashMap<String, f64>>, Arc<crate::Error>> {
	let prices = PRICES_CACHE.try_get_with((), prices_raw()).await?;

	Ok(prices)
}
