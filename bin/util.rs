use api::{guild::Guild, player::Player};
use database::schema;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use minecraft::username::Username;
use poise::CreateReply;
use std::{fmt::Display, sync::Arc};
use tracing::error;
use translate::{tr, tr_fmt, ApiError, Data};
use uuid::Uuid;

use crate::{format, Context, Error};

pub fn success_embed<'a, 'b, S>(
	reply: &'b mut CreateReply<'a>,
	title: S,
	description: S,
) -> &'b mut CreateReply<'a>
where
	S: Into<String> + Display,
{
	reply.embed(|e| {
		e.title(title)
			.description(description)
			.colour(crate::EMBED_COLOUR)
	});

	reply
}

pub fn error_embed<'a, 'b, S>(
	reply: &'b mut CreateReply<'a>,
	title: S,
	description: S,
) -> &'b mut CreateReply<'a>
where
	S: Display,
{
	reply.embed(|e| {
		e.title(title)
			.description(description)
			.colour(crate::EMBED_COLOUR_ERROR)
	});

	reply
}

pub fn escape_username(username: &str) -> String {
	username.replace('_', "\\_")
}

pub async fn get_format_from_input(ctx: Context<'_>) -> format::Display {
	let Ok(mut connection) = ctx.data().pool.get().await else {
		return format::Display::default();
	};

	let result = schema::user::table
		.filter(schema::user::id.eq(ctx.author().id.0 as i64))
		.select(schema::user::display)
		.get_result::<format::Display>(&mut connection)
		.await;

	result.unwrap_or_default()
}

pub async fn get_player_from_input(
	ctx: Context<'_>,
	uuid: Option<String>,
	username: Option<String>,
) -> Result<Player, Error> {
	match (
		uuid.as_ref().and_then(|uuid| Uuid::parse_str(uuid).ok()),
		uuid,
		username
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username,
	) {
		(Some(uuid), _, _, _) => Ok(Player::from_uuid(&uuid).await?),
		(_, _, Some(username), _) => Ok(Player::from_username(username.as_str()).await?),
		(None, Some(uuid), _, _) => Err(Error::InvalidUuid(uuid)),
		(_, _, None, Some(username)) => Err(Error::InvalidUsername(username)),
		(None, _, None, _) => {
			let uuid = schema::user::table
				.filter(schema::user::id.eq(ctx.author().id.0 as i64))
				.select(schema::user::uuid)
				.get_result::<Option<Uuid>>(&mut ctx.data().pool.get().await?)
				.await;

			if let Ok(Some(uuid)) = uuid {
				Ok(Player::from_uuid_unchecked(uuid))
			} else {
				Err(Error::NotLinked)
			}
		}
	}
}

pub async fn get_player_with_username_from_input(
	ctx: Context<'_>,
	uuid: Option<String>,
	username: Option<String>,
) -> Result<Player, Error> {
	match (
		uuid.as_ref().and_then(|uuid| Uuid::parse_str(uuid).ok()),
		uuid,
		username
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username,
	) {
		(Some(uuid), _, _, _) => Ok(Player::from_uuid(&uuid).await?),
		(_, _, Some(username), _) => Ok(Player::from_username(username.as_str()).await?),
		(None, Some(uuid), _, _) => Err(Error::InvalidUuid(uuid)),
		(_, _, None, Some(username)) => Err(Error::InvalidUsername(username)),
		(None, _, None, _) => {
			let uuid = schema::user::table
				.filter(schema::user::id.eq(ctx.author().id.0 as i64))
				.select(schema::user::uuid)
				.get_result::<Option<Uuid>>(&mut ctx.data().pool.get().await?)
				.await;

			if let Ok(Some(uuid)) = uuid {
				Ok(Player::from_uuid(&uuid).await?)
			} else {
				Err(Error::NotLinked)
			}
		}
	}
}

pub async fn get_guild_from_input(
	ctx: Context<'_>,
	name: Option<String>,
	uuid: Option<String>,
	username: Option<String>,
) -> Result<Arc<Guild>, Error> {
	match (
		name,
		uuid.as_ref().and_then(|uuid| Uuid::parse_str(uuid).ok()),
		uuid,
		username
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username,
	) {
		(Some(name), _, _, _, _) => Ok(Guild::from_name(&name).await?),
		(_, Some(uuid), _, _, _) => Ok(Guild::from_member_uuid(uuid).await?),
		(_, _, _, Some(username), _) => {
			let player = Player::from_username(username.as_str()).await?;

			Ok(Guild::from_member_uuid(player.uuid).await?)
		}
		(_, None, Some(uuid), _, _) => Err(Error::InvalidUuid(uuid)),
		(_, _, _, None, Some(username)) => Err(Error::InvalidUsername(username)),
		(_, None, _, None, _) => {
			let uuid = schema::user::table
				.filter(schema::user::id.eq(ctx.author().id.0 as i64))
				.select(schema::user::uuid)
				.get_result::<Option<Uuid>>(&mut ctx.data().pool.get().await?)
				.await;

			if let Ok(Some(uuid)) = uuid {
				Ok(Guild::from_member_uuid(uuid).await?)
			} else {
				Err(Error::NotLinked)
			}
		}
	}
}

pub async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
	if let poise::FrameworkError::Command { error, ctx } = error {
		let content = match error {
			Error::Api(err) => match *err {
				ApiError::PlayerNotFound(ref name) => {
					tr_fmt!(ctx, "error-player-not-found", name: format!("`{}`", name))
				}
				ApiError::SessionNotFound(ref name) => {
					tr_fmt!(ctx, "error-session-not-found", name: format!("`{}`", name))
				}
				ApiError::ProfileNotFound(ref profile, ref name) => {
					tr_fmt!(ctx, "error-profile-not-found", profile: format!("`{}`", profile), name: format!("`{}`", name))
				}
				ApiError::UuidNotFound(ref uuid) => {
					tr_fmt!(ctx, "error-player-uuid-not-found", uuid: format!("`{}`", uuid))
				}
				ApiError::UsernameNotFound(ref name) => {
					tr_fmt!(ctx, "error-player-username-not-found", name: format!("`{}`", name))
				}
				ApiError::GuildByMemberUuidNotFound(ref uuid) => {
					tr_fmt!(ctx, "error-guild-by-member-uuid-not-found", uuid: format!("`{}`", uuid))
				}
				ApiError::GuildByMemberUsernameNotFound(ref name) => {
					tr_fmt!(ctx, "error-guild-by-member-username-not-found", name: format!("`{}`", name))
				}
				ApiError::GuildNotFound(ref name) => {
					tr_fmt!(ctx, "error-guild-not-found", name: format!("`{}`", name))
				}
				ref error => {
					error!(error = ?error, "internal error");
					tr!(ctx, "error-internal")
				}
			},
			Error::NotLinked => {
				tr!(ctx, "error-not-linked")
			}
			Error::InvalidUuid(ref uuid) => {
				tr_fmt!(ctx, "error-invalid-uuid", uuid: format!("`{}`", uuid))
			}
			Error::InvalidUsername(ref name) => {
				tr_fmt!(ctx, "error-invalid-username", name: format!("`{}`", name))
			}
			Error::MemberPlayerNotFound(ref name) => {
				tr_fmt!(ctx, "error-member-player-not-found", name: format!("`{}`", name))
			}
			Error::SkyBlockProfileNotFound(ref name) => {
				tr_fmt!(ctx, "error-skyblock-profile-not-found", name: format!("`{}`", name))
			}
			Error::PlayerSnapshotNotFound(ref name) => {
				tr_fmt!(ctx, "error-player-snapshot-not-found", name: format!("`{}`", name))
			}
			Error::LeaderboardNotFound(ref name) => {
				tr_fmt!(ctx, "error-leaderboard-not-found", name: format!("`{}`", name))
			}
			ref error => {
				error!(error = ?error, "internal error");
				tr!(ctx, "error-internal")
			}
		};

		if let Err(e) = ctx
			.send(|m| {
				m.content(content);
				m
			})
			.await
		{
			error!(e = ?e, "failed to send error message");
		}
	} else {
		error!(error = ?error, "non-command internal error");
	}
}
