use std::borrow::Cow;

use api::{canvas, player::Player};
use futures::StreamExt;
use once_cell::sync::Lazy;
use poise::serenity_prelude::CreateAttachment;
use redis::AsyncCommands;
use translate::{context, Context, Error};
use uuid::Uuid;

use crate::util;

static LEADERBOARDS: Lazy<Vec<api::leaderboard::Leaderboard>> = Lazy::new(|| {
	let ctx = context::Context::external(crate::DATA.get().unwrap());
	let mut leaderboards = api::Data::leaderboards(&ctx);

	leaderboards.sort_by(|a, b| a.display_name.cmp(&b.display_name));
	leaderboards
});

struct RedisUuid(Uuid);

impl redis::FromRedisValue for RedisUuid {
	fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
		match v {
			redis::Value::Data(data) => Ok(Self(Uuid::from_slice(data).map_err(|_| {
				redis::RedisError::from((
					redis::ErrorKind::TypeError,
					"expected slice of length 16",
				))
			})?)),
			_ => Err(redis::RedisError::from((
				redis::ErrorKind::TypeError,
				"unexpected type",
			))),
		}
	}
}

#[allow(clippy::unused_async)]
async fn autocomplete_board(
	_ctx: Context<'_>,
	partial: &str,
) -> impl Iterator<Item = poise::AutocompleteChoice<usize>> {
	let mut lower = partial.replace(' ', "");

	lower.make_ascii_lowercase();

	LEADERBOARDS
		.iter()
		.enumerate()
		.filter_map(|(value, board)| {
			if !board.display_name_lower.contains(&lower) {
				return None;
			}

			Some(poise::AutocompleteChoice {
				name: board.display_name.clone(),
				value,
			})
		})
		.take(10)
		.collect::<Vec<_>>()
		.into_iter()
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::cast_sign_loss)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn leaderboard(
	ctx: Context<'_>,
	#[autocomplete = "autocomplete_board"] board: usize,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
	#[min = 1] page: Option<isize>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let ctx = &context::Context::from_poise(&ctx);

	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let leaderboard = {
		let Some(leaderboard) = LEADERBOARDS.get(board) else {
			return Err(Error::LeaderboardNotFound(board));
		};

		leaderboard
	};

	let key = api::leaderboard::encode(&leaderboard.kind);

	let rank = if let Some(player) = player {
		let uuid = util::parse_uuid(Some(player.as_str()));
		let player = util::get_player_from_input(ctx, uuid, Some(player)).await?;

		ctx.data()
			.redis()
			.zrevrank(&key, player.uuid.as_bytes())
			.await
			.map_err(api::Error::from)?
	} else {
		page.unwrap_or(1) * 10 - 10
	};

	let top: Vec<RedisUuid> = ctx
		.data()
		.redis()
		.zrevrange(&key, rank, rank + 9)
		.await
		.map_err(api::Error::from)?;
	let top = top.into_iter().map(|r| r.0).collect::<Vec<_>>();

	let players = futures::stream::iter(
		top.into_iter()
			.map(Player::from_uuid_unchecked)
			.map(|p| p.get_data_owned(ctx)),
	)
	.buffered(10)
	.filter_map(|r| async { r.ok() })
	.collect::<Vec<_>>()
	.await;

	let png: Cow<_> = {
		use api::leaderboard::Kind::*;
		use api::player::stats::*;

		let rank = rank as usize;

		let mut surface = match leaderboard.kind {
			Arcade(mode, kind) => arcade::Arcade::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Arena(mode, kind) => arena::Arena::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			BedWars(mode, kind) => bed_wars::BedWars::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			BlitzSg(mode, kind) => blitz_sg::BlitzSg::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			BuildBattle(mode, kind) => build_battle::BuildBattle::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			CopsAndCrims(mode, kind) => cops_and_crims::CopsAndCrims::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Duels(mode, kind) => duels::Duels::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Fishing(mode, kind) => fishing::Fishing::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			MegaWalls(mode, kind) => mega_walls::MegaWalls::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			MurderMystery(mode, kind) => murder_mystery::MurderMystery::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Paintball(mode, kind) => paintball::Paintball::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Pit(mode, kind) => pit::Pit::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Quake(mode, kind) => quake::Quake::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			SkyWars(mode, kind) => sky_wars::SkyWars::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			SmashHeroes(mode, kind) => smash_heroes::SmashHeroes::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			SpeedUhc(mode, kind) => speed_uhc::SpeedUhc::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			TntGames(mode, kind) => tnt_games::TntGames::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			TurboKartRacers(mode, kind) => turbo_kart_racers::TurboKartRacers::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Uhc(mode, kind) => uhc::Uhc::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			VampireZ(mode, kind) => vampire_z::VampireZ::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Walls(mode, kind) => walls::Walls::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			Warlords(mode, kind) => warlords::Warlords::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
			WoolWars(mode, kind) => wool_wars::WoolWars::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				&leaderboard,
				family,
				background,
			),
		}?;

		canvas::to_png(&mut surface).into()
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}
