use std::collections::HashMap;

use chrono::{DateTime, Utc};
use database::{
	schema::{bazaar, bazaar_item},
	PostgresPool,
};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use futures::StreamExt;
use once_cell::sync::Lazy;
use reqwest::{Method, Request, Url};
use serde::Deserialize;
use tracing::{error, info};

use crate::http::HTTP;

static BAZAAR_URL: Lazy<Url> =
	Lazy::new(|| Url::parse("https://api.hypixel.net/v2/skyblock/bazaar").unwrap());

#[derive(Debug, Deserialize)]
pub struct Product {
	pub quick_status: QuickStatus,
}

#[derive(Debug, Deserialize)]
pub struct QuickStatus {
	#[serde(rename = "sellPrice")]
	pub sell_price: f64,
	#[serde(rename = "sellVolume")]
	pub sell_volume: i32,
	#[serde(rename = "sellOrders")]
	pub sell_orders: i32,
	#[serde(rename = "buyPrice")]
	pub buy_price: f64,
	#[serde(rename = "buyVolume")]
	pub buy_volume: i32,
	#[serde(rename = "buyOrders")]
	pub buy_orders: i32,
}

#[derive(Debug, Deserialize)]
pub struct Response {
	#[serde(with = "crate::de::vec_map")]
	pub products: Vec<(String, Product)>,
	#[serde(rename = "lastUpdated", with = "chrono::serde::ts_milliseconds")]
	pub updated_at: DateTime<Utc>,
}

/// # Panics
/// Panics if a database query fails. This is useful since it is only done once on startup.
pub async fn get_all_item_identifiers(pool: &PostgresPool) -> HashMap<String, i16> {
	bazaar_item::table
		.select((bazaar_item::name, bazaar_item::id))
		.load(&mut pool.get().await.unwrap())
		.await
		.unwrap()
		.into_iter()
		.collect()
}

/// # Errors
/// Returns an error if a database query fails or if an HTTP request fails.
pub async fn update<S: std::hash::BuildHasher>(
	pool: &PostgresPool,
	identifiers: &mut HashMap<String, i16, S>,
) -> Result<(), crate::Error> {
	let req = Request::new(Method::GET, BAZAAR_URL.clone());
	let Response {
		products,
		updated_at,
	} = HTTP
		.perform_bare(req)
		.await?
		.error_for_status()?
		.json::<Response>()
		.await?;

	diesel::delete(bazaar::table.filter(
		bazaar::created_at.lt(Utc::now() - chrono::Duration::try_days(14).unwrap_or_default()),
	))
	.execute(&mut pool.get().await?)
	.await?;

	let insert = tokio::sync::Mutex::new(vec![]);

	futures::stream::iter(products.into_iter().map(|(name, product)| {
		let identifiers = &identifiers;
		let pool = &pool;
		let insert = &insert;

		async move {
			let id = if let Some(id) = identifiers.get(&name) {
				*id
			} else {
				// Since is it guaranteed that `products` contains unique names, we can safely
				// create a new item without checking if it is already being inserted.
				let id = diesel::insert_into(bazaar_item::table)
					.values((bazaar_item::name.eq(&name),))
					.returning(bazaar_item::id)
					.get_result::<i16>(&mut pool.get().await?)
					.await?;

				info!(name = name, id = id, "inserting new bazaar item");
				insert.lock().await.push((name, id));

				id
			};

			diesel::insert_into(bazaar::table)
				.values((
					bazaar::item_id.eq(id),
					bazaar::sell_price.eq(product.quick_status.sell_price),
					bazaar::sell_volume.eq(product.quick_status.sell_volume),
					bazaar::sell_orders.eq(product.quick_status.sell_orders),
					bazaar::buy_price.eq(product.quick_status.buy_price),
					bazaar::buy_volume.eq(product.quick_status.buy_volume),
					bazaar::buy_orders.eq(product.quick_status.buy_orders),
					bazaar::created_at.eq(updated_at),
				))
				.execute(&mut pool.get().await?)
				.await?;

			Ok::<_, translate::Error>(())
		}
	}))
	.buffer_unordered(10)
	.collect::<Vec<_>>()
	.await
	.into_iter()
	.for_each(|r| {
		if let Err(e) = r {
			error!("error updating bazaar: {}", e);
		}
	});

	identifiers.extend(insert.into_inner());

	Ok(())
}
