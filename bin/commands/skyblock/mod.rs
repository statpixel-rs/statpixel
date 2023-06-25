pub mod run;

use api::skyblock::NAMES;
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

	run::auctions(ctx, username, uuid).await
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

	run::profile(ctx, username, profile, uuid).await
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

	run::bank(ctx, username, profile, uuid).await
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

	run::networth(ctx, username, profile, uuid).await
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

	run::pets(ctx, username, profile, uuid).await
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

			run::$fn(ctx, username, profile, uuid).await
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
	)
)]
pub async fn skyblock(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
