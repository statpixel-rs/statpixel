pub mod image;
pub mod run;

use api::{command, Player};
use once_cell::sync::Lazy;
use translate::{context, Context, Error};
use uuid::Uuid;

use crate::util;

pub static LEADERBOARDS: Lazy<Vec<api::leaderboard::Leaderboard>> = Lazy::new(|| {
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
	#[min = 1]
	#[max = 3_000]
	page: Option<u32>,
	#[min = 1]
	#[max = 30_000]
	position: Option<u32>,
	#[min = 0.0] value: Option<f32>,
	order: Option<command::LeaderboardOrder>,
) -> Result<(), Error> {
	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);

	let leaderboard = {
		let Some(leaderboard) = LEADERBOARDS.get(board) else {
			return Err(Error::LeaderboardNotFound(board));
		};

		leaderboard
	};

	let player = if let Some(ref uuid) = uuid {
		Some(Player::from_uuid(uuid).await?)
	} else if let Some(username) = player.as_deref() {
		Some(Player::from_username(username).await?)
	} else {
		None
	};

	run::command(
		ctx,
		leaderboard,
		u32::try_from(board).unwrap_or(0),
		player
			.map(|p| command::LeaderboardInput::Player(p.uuid))
			.or_else(|| position.map(command::LeaderboardInput::Position))
			.or_else(|| page.map(|p| command::LeaderboardInput::Page(p - 1)))
			.or_else(|| value.map(command::LeaderboardInput::Value))
			.unwrap_or(command::LeaderboardInput::Page(0)),
		command::LeaderboardFilter::None,
		order.unwrap_or_default(),
	)
	.await
}
