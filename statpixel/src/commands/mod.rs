use std::{borrow::Cow, sync::Arc};

use api::{
	guild::Guild,
	image::Image,
	player::{self, data::Data, status::Session, Player},
};
use database::{extend::lower, schema};
use diesel::{ExpressionMethods, QueryDsl, TextExpressionMethods};
use diesel_async::RunQueryDsl;
use translate::{context, Context, Error};
use uuid::Uuid;

use crate::util;

pub mod about;
pub mod background;
pub mod builder;
pub mod display;
pub mod execute;
pub mod from;
pub mod games;
pub mod guild;
pub mod help;
pub mod history;
pub mod leaderboard;
pub mod link;
pub mod network;
pub mod project;
pub mod recent;
pub mod skyblock;
pub mod snapshot;
pub mod unlink;
pub mod winstreaks;

#[allow(clippy::unused_async)]
pub async fn autocomplete_username(
	ctx: Context<'_>,
	partial: &str,
) -> Box<dyn Iterator<Item = String> + Send> {
	tracing::debug!("Autocompleting username `{partial}`");

	if let Ok(mut connection) = ctx.data().pool.get().await {
		if partial.is_empty() || partial.contains('%') {
			let result = schema::autocomplete::table
				.order(schema::autocomplete::searches.desc())
				.limit(10)
				.select(schema::autocomplete::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return Box::new(result.into_iter());
			}
		} else {
			let result = schema::autocomplete::table
				.filter(
					lower(schema::autocomplete::name)
						.like(format!("{}%", partial.to_ascii_lowercase())),
				)
				.order(schema::autocomplete::searches.desc())
				.limit(9)
				.select(schema::autocomplete::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return Box::new(std::iter::once(partial.to_string()).chain(result.into_iter()));
			}
		}
	}

	Box::new(std::iter::once(partial.to_string()))
}

#[allow(clippy::unused_async)]
pub async fn autocomplete_guild_name(
	ctx: Context<'_>,
	partial: &str,
) -> Box<dyn Iterator<Item = String> + Send> {
	tracing::debug!("Autocompleting guild name `{partial}`");

	if let Ok(mut connection) = ctx.data().pool.get().await {
		if partial.is_empty() || partial.contains('%') {
			let result = schema::guild_autocomplete::table
				.filter(schema::guild_autocomplete::name.is_not_null())
				.order(schema::guild_autocomplete::xp.desc())
				.limit(10)
				.select(schema::guild_autocomplete::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return Box::new(result.into_iter());
			}
		} else {
			let result = schema::guild_autocomplete::table
				.filter(
					lower(schema::guild_autocomplete::name)
						.like(format!("{}%", partial.to_ascii_lowercase())),
				)
				.order(schema::guild_autocomplete::xp.desc())
				.limit(9)
				.select(schema::guild_autocomplete::name)
				.get_results::<String>(&mut connection)
				.await;

			if let Ok(result) = result {
				return Box::new(std::iter::once(partial.to_string()).chain(result.into_iter()));
			}
		}
	}

	Box::new(std::iter::once(partial.to_string()))
}

pub async fn get_guild(
	ctx: &context::Context<'_>,
	name: Option<String>,
	uuid: Option<Uuid>,
	username: Option<String>,
	guild_id: Option<Uuid>,
) -> Result<Arc<Guild>, Error> {
	let (guild, _) = tokio::join!(
		util::get_guild_from_input(ctx, name, uuid, username, guild_id),
		ctx.defer(),
	);

	guild
}

pub async fn get_guild_with_member(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<(Arc<Guild>, Player), Error> {
	let (result, _) = tokio::join!(
		util::get_guild_with_member_from_input(ctx, uuid, username),
		ctx.defer(),
	);

	result
}

pub async fn get_player_data_session_skin_suffix(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<
	(
		Player,
		Arc<Data>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let (result, _) = tokio::join!(
		player_data_session_skin_suffix(ctx, uuid, username),
		ctx.defer(),
	);

	result
}

pub async fn get_player_data_guild_session_skin_suffix(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<
	(
		Player,
		Arc<Data>,
		Option<Arc<Guild>>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let (result, _) = tokio::join!(
		player_data_guild_session_skin_suffix(ctx, uuid, username),
		ctx.defer(),
	);

	result
}

pub async fn get_player_data_games_session_skin_suffix(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<
	(
		Player,
		Arc<Data>,
		Arc<Vec<player::games::Game>>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let (result, _) = tokio::join!(
		player_data_games_session_skin_suffix(ctx, uuid, username),
		ctx.defer(),
	);

	result
}

pub async fn from_player_data_session_skin_suffix(
	ctx: &context::Context<'_>,
	player: &Player,
) -> Result<
	(
		Arc<Data>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let (data, session, skin, suffix) = tokio::join!(
		player.get_data(),
		player.get_session(),
		player.get_skin(),
		player.get_suffix(ctx),
	);

	let data = data?;
	let session = session?;

	Ok((data, session, skin, suffix))
}

pub async fn from_player_data_guild_session_skin_suffix(
	ctx: &context::Context<'_>,
	player: &Player,
) -> Result<
	(
		Arc<Data>,
		Option<Arc<Guild>>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let (data, guild, session, skin, suffix) = tokio::join!(
		player.get_data(),
		player.get_guild(),
		player.get_session(),
		player.get_skin(),
		player.get_suffix(ctx),
	);

	let data = data?;
	let session = session?;

	Ok((data, guild, session, skin, suffix))
}

pub async fn from_player_data_games_session_skin_suffix(
	ctx: &context::Context<'_>,
	player: &Player,
) -> Result<
	(
		Arc<Data>,
		Arc<Vec<player::games::Game>>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let (data, games, session, skin, suffix) = tokio::join!(
		player.get_data(),
		player.get_games(),
		player.get_session(),
		player.get_skin(),
		player.get_suffix(ctx),
	);

	let data = data?;
	let session = session?;
	let games = games?;

	Ok((data, games, session, skin, suffix))
}

async fn player_data_games_session_skin_suffix(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<
	(
		Player,
		Arc<Data>,
		Arc<Vec<player::games::Game>>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let player = util::get_player_from_input(ctx, uuid, username).await?;
	let (data, games, session, skin, suffix) =
		from_player_data_games_session_skin_suffix(ctx, &player).await?;

	Ok((player, data, games, session, skin, suffix))
}

async fn player_data_session_skin_suffix(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<
	(
		Player,
		Arc<Data>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let player = util::get_player_from_input(ctx, uuid, username).await?;
	let (data, session, skin, suffix) = from_player_data_session_skin_suffix(ctx, &player).await?;

	Ok((player, data, session, skin, suffix))
}

async fn player_data_guild_session_skin_suffix(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<
	(
		Player,
		Arc<Data>,
		Option<Arc<Guild>>,
		Arc<Session>,
		Cow<'static, Image<'static>>,
		Option<String>,
	),
	Error,
> {
	let player = util::get_player_from_input(ctx, uuid, username).await?;
	let (data, guild, session, skin, suffix) =
		from_player_data_guild_session_skin_suffix(ctx, &player).await?;

	Ok((player, data, guild, session, skin, suffix))
}

pub async fn get_player_data(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<(Player, Arc<Data>), Error> {
	let (result, _) = tokio::join!(player_data(ctx, uuid, username), ctx.defer());

	result
}

async fn player_data(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<(Player, Arc<Data>), Error> {
	let player = util::get_player_from_input(ctx, uuid, username).await?;
	let data = player.get_data().await?;

	Ok((player, data))
}

pub async fn get_player_session(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<(Player, Arc<Session>), Error> {
	let (result, _) = tokio::join!(player_session(ctx, uuid, username), ctx.defer());

	result
}

pub async fn get_player_username_session(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<(Player, Arc<Session>), Error> {
	let (result, _) = tokio::join!(player_username_session(ctx, uuid, username), ctx.defer());

	result
}

async fn player_session(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<(Player, Arc<Session>), Error> {
	let player = util::get_player_from_input(ctx, uuid, username).await?;
	let session = player.get_session().await?;

	Ok((player, session))
}

async fn player_username_session(
	ctx: &context::Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<(Player, Arc<Session>), Error> {
	let player = util::get_player_with_username_from_input(ctx, uuid, username).await?;
	let session = player.get_session().await?;

	Ok((player, session))
}
