use std::{borrow::Cow, cmp::max};

use api::{
	canvas::{self, body::Body, chart, label::ToFormatted, Canvas},
	player::Player,
	skyblock::{profile::TransactionAction, NAMES},
};
use canvas::{shape, text};
use chrono::Utc;
use minecraft::{
	calc::{network, sky_block},
	paint::Paint,
	style::MinecraftFont,
	text::{
		parse::{minecraft_string, minecraft_text},
		Text,
	},
};
use poise::serenity_prelude::AttachmentType;
use skia_safe::textlayout::TextAlign;
use translate::{tr, Context, Error};

const LABEL: [Text; 2] = minecraft_text("§b§lSky§a§lBlock");

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
pub async fn auctions(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let (player, data, session, skin, suffix) = crate::get_all!(ctx, uuid, username);

	player.increase_searches(ctx).await?;

	let png = {
		let auctions = player.get_auctions().await?;
		let status = shape::Status(&session, skin.as_ref());
		let level = network::get_level(data.xp);
		let progress = shape::WideBubbleProgress(
			network::get_level_progress(data.xp),
			network::get_colours(level),
		);

		let mut canvas = Canvas::new(720.)
			.gap(7.)
			.push_down(
				&shape::Title,
				shape::Title::from_text(&text::from_data(&data, &data.username, suffix.as_deref())),
			)
			.push_down(
				&shape::Subtitle,
				shape::Subtitle::from_label(ctx, &LABEL, "player-auctions"),
			)
			.push_down(
				&progress,
				shape::WideBubbleProgress::from_level_progress(
					ctx,
					&network::get_level_format(level),
					&network::get_curr_level_xp(data.xp),
					&network::get_level_xp(data.xp),
				),
			)
			.push_right_start(&status, Body::from_status(ctx, &session));

		for auction in &auctions {
			let mut text = minecraft_string(&auction.item.name).collect::<Vec<_>>();
			let bid = max(auction.starting_bid, auction.highest_bid);
			let bid = bid.to_formatted_label(ctx);

			text.extend([
				Text {
					text: "\n",
					size: None,
					..Default::default()
				},
				Text {
					text: bid.as_ref(),
					paint: Paint::Gold,
					font: MinecraftFont::Normal,
					size: Some(30.),
				},
			]);

			canvas = canvas.push_checked(
				&shape::TallBubble,
				Body::build_slice(text.as_slice(), 23., TextAlign::Center),
			);
		}

		canvas::to_png(&mut canvas.build(None).unwrap()).into()
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

	let (player, mut data, session, skin, suffix) = crate::get_all!(ctx, uuid, username);
	let profiles = data.stats.sky_block.profiles;

	player.increase_searches(ctx).await?;

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
		let status = shape::Status(&session, skin.as_ref());
		let level = sky_block::get_level(member.leveling.xp);
		let progress = shape::WideBubbleProgress(
			sky_block::get_level_progress(member.leveling.xp),
			sky_block::get_colours(level),
		);

		let mut surface = Canvas::new(720.)
			.gap(7.)
			.push_down(
				&shape::Title,
				shape::Title::from_text(&text::from_data(&data, &data.username, suffix.as_deref())),
			)
			.push_down(
				&shape::Subtitle,
				shape::Subtitle::from_label(ctx, &LABEL, "member-profile"),
			)
			.push_down(
				&progress,
				shape::WideBubbleProgress::from_level_progress(
					ctx,
					&sky_block::get_level_format(level),
					&sky_block::get_curr_level_xp(member.leveling.xp),
					&sky_block::get_level_xp(member.leveling.xp),
				),
			)
			.push_right_start(
				&canvas::shape::Sidebar,
				canvas::body::Body::new(17., None)
					.append_item(
						&::translate::tr!(ctx, "coins"),
						&canvas::label::ToFormatted::to_formatted_label(&member.coin_purse, ctx),
						&Paint::Gold,
					)
					.append_item(
						&::translate::tr!(ctx, "fairy-souls"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fairy_souls_collected,
							ctx,
						),
						&Paint::Aqua,
					)
					.append_item(
						&::translate::tr!(ctx, "fairy-exchanges"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fairy_exchanges,
							ctx,
						),
						&Paint::LightPurple,
					)
					.append_item(
						&::translate::tr!(ctx, "fishing-treasure"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fishing_treasure_caught,
							ctx,
						),
						&Paint::Blue,
					)
					.append_item(
						&::translate::tr!(ctx, "zones-visited"),
						&canvas::label::ToFormatted::to_formatted_label(&member.zones_visited, ctx),
						&Paint::Green,
					)
					.append_item(
						&::translate::tr!(ctx, "generators-crafted"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.generators_crafted,
							ctx,
						),
						&Paint::White,
					)
					.append_item(
						&::translate::tr!(ctx, "highest-crit"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.stats.highest_critical_damage,
							ctx,
						),
						&Paint::Red,
					)
					.build(),
			)
			.push_right(&status, Body::from_status(ctx, &session))
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.farming),
					tr!(ctx, "farming").as_ref(),
					Paint::Gold,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.mining),
					tr!(ctx, "mining").as_ref(),
					Paint::Gray,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.combat),
					tr!(ctx, "combat").as_ref(),
					Paint::Gray,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.foraging),
					tr!(ctx, "foraging").as_ref(),
					Paint::Green,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.fishing),
					tr!(ctx, "fishing").as_ref(),
					Paint::White,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.enchanting),
					tr!(ctx, "enchanting").as_ref(),
					Paint::DarkPurple,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.alchemy),
					tr!(ctx, "alchemy").as_ref(),
					Paint::Yellow,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.taming),
					tr!(ctx, "taming").as_ref(),
					Paint::Gray,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_dungeoneering(
						member.dungeons.types.catacombs.experience,
					),
					tr!(ctx, "dungeoneering").as_ref(),
					Paint::Gray,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.carpentry),
					tr!(ctx, "carpentry").as_ref(),
					Paint::Red,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.runecrafting),
					tr!(ctx, "runecrafting").as_ref(),
					Paint::LightPurple,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.social),
					tr!(ctx, "social").as_ref(),
					Paint::Green,
				),
			)
			.build(None)
			.unwrap();

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

	let (player, mut data) = crate::get_data!(ctx, uuid, username);
	let profiles = data.stats.sky_block.profiles;

	player.increase_searches(ctx).await?;

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
	subcommands("profile", "bank", "auctions")
)]
pub async fn skyblock(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
