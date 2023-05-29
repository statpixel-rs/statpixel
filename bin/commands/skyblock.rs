use std::borrow::Cow;

use api::{
	canvas::{self, chart},
	player::Player,
	skyblock::{profile::TransactionAction, NAMES},
};
use chrono::Utc;
use minecraft::{
	calc::{network, sky_block},
	minecraft_text,
	paint::Paint,
	text::Text,
};
use poise::serenity_prelude::AttachmentType;
use translate::{tr, Context, Error};

const LABEL: [Text; 2] = minecraft_text!("§b§lSky§a§lBlock");

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

#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
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
	ctx.defer().await?;

	let (_format, player, mut data, session) = crate::get_data!(ctx, uuid, username);
	let profiles = data.stats.sky_block.profiles;

	// clear the profiles so that we can continue to use Data
	data.stats.sky_block.profiles = vec![];

	let Some(profile) = (match profile {
		Some(profile) => profiles.into_iter().find(|p| p.name == profile),
		None => profiles.into_iter().next(),
	}) else {
		return Err(Error::Custom("No profile found.".to_string()));
	};

	let mut profile = Player::get_skyblock_profile(profile.id).await?;

	let Some(member) = profile.members.remove(&player.uuid) else {
		return Err(Error::Custom(format!("`{}` is not a member of the provided profile.", player.username)));
	};

	let png = {
		let mut surface = canvas::create_surface(4);

		canvas::header::apply_name(&mut surface, &data);
		canvas::header::apply_status(ctx, &mut surface, &session);
		canvas::game::apply_label(
			&mut surface,
			&[
				LABEL[0],
				LABEL[1],
				Text {
					text: " (",
					paint: Paint::White,
					..Default::default()
				},
				Text {
					text: tr!(ctx, "member-profile").as_ref(),
					paint: Paint::White,
					..Default::default()
				},
				Text {
					text: ")",
					paint: Paint::White,
					..Default::default()
				},
			],
		);

		let level = network::get_level(data.xp);

		canvas::game::apply_data(
			ctx,
			&mut surface,
			&network::get_level_format(level),
			network::get_level_progress(data.xp),
			network::get_curr_level_xp(data.xp),
			network::get_level_xp(data.xp),
			&network::get_colours(level),
		);

		canvas::sidebar::item(
			ctx,
			&mut surface,
			&(tr!(ctx, "coins"), member.coin_purse, Paint::Gold),
			0,
		);

		canvas::sidebar::item(
			ctx,
			&mut surface,
			&(
				tr!(ctx, "fairy-souls"),
				member.fairy_souls_collected,
				Paint::Aqua,
			),
			1,
		);

		canvas::sidebar::item(
			ctx,
			&mut surface,
			&(
				tr!(ctx, "fairy-exchanges"),
				member.fairy_exchanges,
				Paint::LightPurple,
			),
			2,
		);

		canvas::sidebar::item(
			ctx,
			&mut surface,
			&(
				tr!(ctx, "fishing-treasure"),
				member.fishing_treasure_caught,
				Paint::Blue,
			),
			3,
		);

		canvas::sidebar::item(
			ctx,
			&mut surface,
			&(
				tr!(ctx, "zones-visited"),
				member.zones_visited,
				Paint::Green,
			),
			4,
		);

		canvas::sidebar::item(
			ctx,
			&mut surface,
			&(
				tr!(ctx, "generators-crafted"),
				member.generators_crafted,
				Paint::White,
			),
			5,
		);

		canvas::sidebar::item(
			ctx,
			&mut surface,
			&(
				tr!(ctx, "highest-crit"),
				member.stats.highest_critical_damage,
				Paint::Red,
			),
			6,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.farming),
			tr!(ctx, "farming").as_ref(),
			Paint::Gold,
			0,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.mining),
			tr!(ctx, "mining").as_ref(),
			Paint::Gray,
			1,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.combat),
			tr!(ctx, "combat").as_ref(),
			Paint::Gray,
			2,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.foraging),
			tr!(ctx, "foraging").as_ref(),
			Paint::Green,
			3,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.fishing),
			tr!(ctx, "fishing").as_ref(),
			Paint::White,
			4,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.enchanting),
			tr!(ctx, "enchanting").as_ref(),
			Paint::DarkPurple,
			5,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.alchemy),
			tr!(ctx, "alchemy").as_ref(),
			Paint::Yellow,
			6,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.taming),
			tr!(ctx, "taming").as_ref(),
			Paint::Gray,
			7,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_dungeoneering(member.dungeons.types.catacombs.experience),
			tr!(ctx, "dungeoneering").as_ref(),
			Paint::Gray,
			8,
		);
		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_general(member.skills.carpentry),
			tr!(ctx, "carpentry").as_ref(),
			Paint::Red,
			9,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_runecrafting(member.skills.runecrafting),
			tr!(ctx, "runecrafting").as_ref(),
			Paint::LightPurple,
			10,
		);

		canvas::game::bubble(
			ctx,
			&mut surface,
			sky_block::skills::get_level_social(member.skills.social),
			tr!(ctx, "social").as_ref(),
			Paint::Green,
			11,
		);

		canvas::to_png(&mut surface).into()
	};

	ctx.send(move |m| {
		m.attachment(AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".to_string(),
		})
	})
	.await?;

	Ok(())
}

#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
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
	ctx.defer().await?;

	let (_format, _player, mut data, _session) = crate::get_data!(ctx, uuid, username);
	let profiles = data.stats.sky_block.profiles;

	// clear the profiles so that we can continue to use Data
	data.stats.sky_block.profiles = vec![];

	let Some(profile) = (match profile {
		Some(profile) => profiles.into_iter().find(|p| p.name == profile),
		None => profiles.into_iter().next(),
	}) else {
		return Err(Error::Custom("No profile found.".to_string()));
	};

	let profile = Player::get_skyblock_profile(profile.id).await?;

	let mut bank = profile.banking;
	let mut lower = u64::MAX;
	// The upper bound should be at least 100
	let mut upper = 100u64;

	// overwrite the bank transactions and replace the "change" by the total at that time
	for transaction in bank.transactions.iter_mut().rev() {
		match transaction.action {
			TransactionAction::Withdraw => bank.balance += transaction.amount,
			TransactionAction::Deposit => bank.balance -= transaction.amount,
		}

		transaction.amount = bank.balance;

		if transaction.amount < lower {
			lower = transaction.amount;
		}

		if transaction.amount > upper {
			upper = transaction.amount;
		}
	}

	let first = bank
		.transactions
		.first()
		.map_or_else(Utc::now, |t| t.timestamp);

	let last = bank
		.transactions
		.last()
		.map_or_else(Utc::now, |t| t.timestamp);

	let png = {
		let mut buffer = chart::u64::create(
			ctx,
			vec![(
				tr!(ctx, "bank-balance"),
				bank.transactions
					.iter()
					.map(|t| (t.timestamp, t.amount))
					.collect(),
			)],
			first..last,
			(lower * 7 / 8)..(upper * 8 / 7),
			Some(Paint::Gold),
		)?;

		let mut surface = chart::canvas(&mut buffer)?;

		chart::apply_title(
			ctx,
			&mut surface,
			&data,
			&[Text {
				text: tr!(ctx, "island-bank-balance").as_ref(),
				paint: Paint::Gold,
				..Default::default()
			}],
		);
		chart::round_corners(&mut surface);

		Cow::Owned(
			surface
				.image_snapshot()
				.encode_to_data(skia_safe::EncodedImageFormat::PNG)
				.unwrap()
				.to_vec(),
		)
	};

	ctx.send(move |m| {
		m.attachment(AttachmentType::Bytes {
			data: png,
			filename: "canvas.png".to_string(),
		})
	})
	.await?;

	Ok(())
}

#[allow(clippy::unused_async)]
#[poise::command(
	slash_command,
	required_bot_permissions = "ATTACH_FILES",
	subcommands("profile", "bank")
)]
pub async fn skyblock(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
