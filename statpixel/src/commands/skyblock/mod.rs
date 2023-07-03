pub mod image;
pub mod run;

use api::skyblock::NAMES;
use database::{extend::lower, schema::bazaar_item};
use diesel::{ExpressionMethods, QueryDsl, TextExpressionMethods};
use diesel_async::RunQueryDsl;
use translate::{context, Context, Error};

use crate::util;

#[allow(clippy::unused_async)]
async fn autocomplete_profile(_ctx: Context<'_>, partial: &str) -> impl Iterator<Item = String> {
	let lower = partial.to_ascii_lowercase();

	NAMES
		.iter()
		.filter(|n| n.to_ascii_lowercase().starts_with(&lower))
		.take(10)
		.map(|s| (*s).to_string())
		.collect::<Vec<_>>()
		.into_iter()
}

async fn autocomplete_product(
	ctx: Context<'_>,
	partial: &str,
) -> impl Iterator<Item = String> + Send {
	tracing::debug!("Autocompleting username `{partial}`");

	if let Ok(mut connection) = ctx.data().pool.get().await {
		if partial.is_empty() || partial.contains('%') {
			let result: Result<_, _> = bazaar_item::table
				.order(bazaar_item::name.asc())
				.limit(10)
				.select(bazaar_item::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return result.into_iter();
			}
		} else {
			let result = bazaar_item::table
				.filter(lower(bazaar_item::name).like(format!("{}%", partial.to_ascii_lowercase())))
				.order(bazaar_item::name.asc())
				.limit(9)
				.select(bazaar_item::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return result.into_iter();
			}
		}
	}

	vec![].into_iter()
}

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn auctions(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(uuid)?;
	let ctx = &context::Context::from_poise(&ctx);

	run::auctions(ctx, username, uuid, None).await
}

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn profile(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[autocomplete = "autocomplete_profile"] profile: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(uuid)?;
	let ctx = &context::Context::from_poise(&ctx);

	run::profile(ctx, username, profile, uuid, None).await
}

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn bank(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[autocomplete = "autocomplete_profile"] profile: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(uuid)?;
	let ctx = &context::Context::from_poise(&ctx);

	run::bank(ctx, username, profile, uuid, None).await
}

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn networth(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[autocomplete = "autocomplete_profile"] profile: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(uuid)?;
	let ctx = &context::Context::from_poise(&ctx);

	run::networth(ctx, username, profile, uuid, None).await
}

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn pets(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[autocomplete = "autocomplete_profile"] profile: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(uuid)?;
	let ctx = &context::Context::from_poise(&ctx);

	run::pets(ctx, username, profile, uuid, None).await
}

#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn bazaar(
	ctx: Context<'_>,
	#[min_length = 1]
	#[autocomplete = "autocomplete_product"]
	mut product: String,
) -> Result<(), Error> {
	product.make_ascii_uppercase();

	let ctx = &context::Context::from_poise(&ctx);

	run::bazaar(ctx, product).await
}

macro_rules! inventory_command {
	($fn: ident, $key: ident) => {
		#[poise::command(
			on_error = "crate::util::error_handler",
			slash_command,
			required_bot_permissions = "ATTACH_FILES"
		)]
		pub async fn $fn(
			ctx: Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<String>,
			#[autocomplete = "autocomplete_profile"] profile: Option<String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<String>,
		) -> Result<(), Error> {
			let uuid = util::parse_uuid(uuid)?;
			let ctx = &context::Context::from_poise(&ctx);

			run::$fn(ctx, username, profile, uuid, None).await
		}
	};
}

inventory_command!(inventory, inventory);
inventory_command!(enderchest, ender_chest);
inventory_command!(talisman, talisman_bag);
inventory_command!(quiver, quiver);
inventory_command!(fishing, fishing_bag);
inventory_command!(potions, potion_bag);
inventory_command!(equipment, equipment);
inventory_command!(wardrobe, wardrobe);
inventory_command!(candy, candy);
inventory_command!(vault, vault);

#[allow(clippy::unused_async)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES",
	subcommands(
		"profile",
		"bank",
		"auctions",
		"inventory",
		"enderchest",
		"talisman",
		"quiver",
		"fishing",
		"potions",
		"equipment",
		"wardrobe",
		"candy",
		"vault",
		"pets",
		"networth",
		"bazaar"
	)
)]
pub async fn skyblock(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
