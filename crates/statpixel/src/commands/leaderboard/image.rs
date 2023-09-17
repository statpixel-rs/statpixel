use std::borrow::Cow;

use api::{canvas, command, player::Player};
use futures::StreamExt;
use redis::AsyncCommands;
use translate::{context, Error};

use super::RedisUuid;

const ITEMS_PER_PAGE: isize = 10;

#[allow(clippy::too_many_lines)]
pub async fn command(
	ctx: &context::Context<'_>,
	leaderboard: &api::leaderboard::Leaderboard,
	input: command::LeaderboardInput,
	_filter: command::LeaderboardFilter,
	order: command::LeaderboardOrder,
	family: minecraft::style::Family,
	background: Option<skia_safe::Color>,
) -> Result<(Cow<'static, [u8]>, u32), Error> {
	let key = api::leaderboard::encode(&leaderboard.kind);

	let rank = match input {
		command::LeaderboardInput::Page(page) => page as isize * ITEMS_PER_PAGE,
		command::LeaderboardInput::Player(uuid) => {
			let rank: Option<isize> = match order {
				command::LeaderboardOrder::Descending => ctx
					.data()
					.redis()
					.zrevrank(&key, uuid.as_bytes())
					.await
					.map_err(api::Error::from)?,
				command::LeaderboardOrder::Ascending => ctx
					.data()
					.redis()
					.zrank(&key, uuid.as_bytes())
					.await
					.map_err(api::Error::from)?,
			};

			if let Some(rank) = rank {
				rank / ITEMS_PER_PAGE * ITEMS_PER_PAGE
			} else {
				return Err(Error::LeaderboardPlayerNotFound);
			}
		}
		command::LeaderboardInput::Position(position) => {
			(position as isize - 1) / ITEMS_PER_PAGE * ITEMS_PER_PAGE
		}
		command::LeaderboardInput::Value(value) => {
			let rank: isize = match order {
				command::LeaderboardOrder::Descending => ctx
					.data()
					.redis()
					.zcount(&key, value, f64::INFINITY)
					.await
					.map_err(api::Error::from)?,
				command::LeaderboardOrder::Ascending => ctx
					.data()
					.redis()
					.zcount(&key, f64::NEG_INFINITY, value)
					.await
					.map_err(api::Error::from)?,
			};

			rank / ITEMS_PER_PAGE * ITEMS_PER_PAGE
		}
	};

	let top: Vec<RedisUuid> = match order {
		command::LeaderboardOrder::Descending => ctx
			.data()
			.redis()
			.zrevrange(&key, rank, rank + 9)
			.await
			.map_err(api::Error::from)?,
		command::LeaderboardOrder::Ascending => ctx
			.data()
			.redis()
			.zrange(&key, rank, rank + 9)
			.await
			.map_err(api::Error::from)?,
	};

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

	let mut surface = {
		#[allow(clippy::enum_glob_use)]
		#[allow(clippy::items_after_statements)]
		use api::leaderboard::Kind::*;
		use api::player::stats::*;

		#[allow(clippy::cast_sign_loss)]
		let rank = rank as usize;

		match leaderboard.kind {
			Arcade(mode, kind) => arcade::Arcade::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Arena(mode, kind) => arena::Arena::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			BedWars(mode, kind) => bed_wars::BedWars::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			BlitzSg(mode, kind) => blitz_sg::BlitzSg::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			BuildBattle(mode, kind) => build_battle::BuildBattle::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			CopsAndCrims(mode, kind) => cops_and_crims::CopsAndCrims::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Duels(mode, kind) => duels::Duels::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Fishing(mode, kind) => fishing::Fishing::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			MegaWalls(mode, kind) => mega_walls::MegaWalls::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			MurderMystery(mode, kind) => murder_mystery::MurderMystery::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Paintball(mode, kind) => paintball::Paintball::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Pit(mode, kind) => pit::Pit::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Quake(mode, kind) => quake::Quake::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			SkyWars(mode, kind) => sky_wars::SkyWars::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			SmashHeroes(mode, kind) => smash_heroes::SmashHeroes::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			SpeedUhc(mode, kind) => speed_uhc::SpeedUhc::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			TntGames(mode, kind) => tnt_games::TntGames::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			TurboKartRacers(mode, kind) => turbo_kart_racers::TurboKartRacers::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Uhc(mode, kind) => uhc::Uhc::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			VampireZ(mode, kind) => vampire_z::VampireZ::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Walls(mode, kind) => walls::Walls::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			Warlords(mode, kind) => warlords::Warlords::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
			WoolWars(mode, kind) => wool_wars::WoolWars::leaderboard(
				ctx,
				rank,
				&players,
				&mode,
				&kind,
				leaderboard,
				family,
				background,
			),
		}?
	};

	Ok((
		canvas::to_png(&mut surface).into(),
		(rank / ITEMS_PER_PAGE).try_into().unwrap_or(0),
	))
}
