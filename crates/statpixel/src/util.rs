use api::{guild::Guild, player::Player};
use database::schema::{self, session};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;
use minecraft::{style::Family, username::Username};
use poise::{serenity_prelude as serenity, CreateReply};
use skia_safe::Color;
use std::sync::Arc;
use tracing::error;
use translate::{context, prelude::GetLocale, tr, tr_fmt, ApiError, Data};
use uuid::Uuid;

use crate::{context::Context, format, Error};

pub fn success_embed<S, D>(title: S, description: D) -> CreateReply
where
	S: Into<String>,
	D: Into<String>,
{
	CreateReply::new().embed(
		serenity::CreateEmbed::new()
			.title(title)
			.description(description)
			.colour(crate::EMBED_COLOUR),
	)
}

pub fn error_embed<S, D>(title: S, description: D) -> CreateReply
where
	S: Into<String>,
	D: Into<String>,
{
	CreateReply::new().embed(
		serenity::CreateEmbed::new()
			.title(title)
			.description(description)
			.colour(crate::EMBED_COLOUR_ERROR),
	)
}

pub fn deprecated_interaction(ctx: &impl GetLocale) -> CreateReply {
	CreateReply::new().embed(
		serenity::CreateEmbed::new()
			.title(tr(ctx, "deprecated-interaction"))
			.description(tr(ctx, "deprecated-interaction-description"))
			.colour(crate::EMBED_COLOUR_ERROR),
	)
}

pub fn invalid_identifier(ctx: &impl GetLocale) -> CreateReply {
	CreateReply::new().embed(
		serenity::CreateEmbed::new()
			.title(tr(ctx, "invalid-identifier"))
			.description(tr(ctx, "invalid-identifier-description"))
			.colour(crate::EMBED_COLOUR_ERROR),
	)
}

pub async fn get_image_options_from_input(
	ctx: &Context<'_>,
) -> (format::Display, Family, Option<Color>) {
	let Ok(mut connection) = ctx.data().pool.get().await else {
		return (format::Display::default(), Family::default(), None);
	};

	let Some(author) = ctx.author() else {
		return (format::Display::default(), Family::default(), None);
	};

	let result = schema::user::table
		.filter(schema::user::id.eq(author.id.get() as i64))
		.select((
			schema::user::display,
			schema::user::font,
			schema::user::colour,
		))
		.get_result::<(format::Display, Family, Option<i32>)>(&mut connection)
		.await;

	match result {
		#[allow(clippy::cast_sign_loss)]
		Ok((display, family, colour)) => (display, family, colour.map(|c| (c as u32).into())),
		Err(_) => (format::Display::default(), Family::default(), None),
	}
}

pub fn parse_uuid(uuid: Option<&str>) -> Option<Uuid> {
	uuid.and_then(|u| Uuid::parse_str(u).ok())
}

pub async fn get_player_from_input(
	ctx: &Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<Player, Error> {
	match (
		uuid,
		username
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username,
	) {
		(Some(id), _, _) => {
			let session = session::table
				.filter(session::id.eq(id))
				.select((session::uuid, session::snapshot_id))
				.get_result::<(Uuid, i64)>(&mut ctx.data().pool.get().await?)
				.await
				.optional()?;

			Ok(if let Some((player_uuid, snapshot_id)) = session {
				Player {
					uuid: player_uuid,
					username: None,
					session: Some((id, snapshot_id)),
				}
			} else {
				Player::from_uuid(&id).await?
			})
		}
		(_, _, Some(name)) if name.starts_with('#') => {
			let session = session::table
				.filter(
					session::user_id.eq(ctx.author().ok_or(Error::NotImplemented)?.id.get() as i64),
				)
				.filter(session::name.eq(&name[1..]))
				.select((session::uuid, session::id, session::snapshot_id))
				.get_result::<(Uuid, Uuid, i64)>(&mut ctx.data().pool.get().await?)
				.await?;

			let mut player = Player::from_uuid(&session.0).await?;

			player.session = Some((session.1, session.2));
			Ok(player)
		}
		(_, Some(username), _) => Ok(Player::from_username(username.as_str()).await?),
		(_, None, Some(username)) => Err(Error::InvalidUsername(username)),
		_ => {
			let uuid = schema::user::table
				.filter(schema::user::id.eq(ctx.author().ok_or(Error::NotLinked)?.id.get() as i64))
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
	ctx: &Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<Player, Error> {
	match (
		uuid,
		username
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username,
	) {
		(Some(id), _, _) => {
			let session = session::table
				.filter(session::id.eq(id))
				.select((session::uuid, session::snapshot_id))
				.get_result::<(Uuid, i64)>(&mut ctx.data().pool.get().await?)
				.await
				.optional()?;

			Ok(if let Some((player_uuid, snapshot_id)) = session {
				let mut player = Player::from_uuid(&player_uuid).await?;

				player.session = Some((id, snapshot_id));
				player
			} else {
				Player::from_uuid(&id).await?
			})
		}
		(_, _, Some(name)) if name.starts_with('#') => {
			let session = session::table
				.filter(
					session::user_id.eq(ctx.author().ok_or(Error::NotImplemented)?.id.get() as i64),
				)
				.filter(session::name.eq(&name[1..]))
				.select((session::uuid, session::id, session::snapshot_id))
				.get_result::<(Uuid, Uuid, i64)>(&mut ctx.data().pool.get().await?)
				.await?;

			let mut player = Player::from_uuid(&session.0).await?;

			player.session = Some((session.1, session.2));
			Ok(player)
		}
		(_, Some(username), _) => Ok(Player::from_username(username.as_str()).await?),
		(_, None, Some(username)) => Err(Error::InvalidUsername(username)),
		_ => {
			let uuid = schema::user::table
				.filter(schema::user::id.eq(ctx.author().ok_or(Error::NotLinked)?.id.get() as i64))
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

pub async fn get_guild_with_member_opt_from_input(
	ctx: &Context<'_>,
	name: Option<String>,
	uuid: Option<Uuid>,
	username: Option<String>,
	guild_id: Option<Uuid>,
) -> Result<(Arc<Guild>, Option<Player>), Error> {
	match (
		guild_id,
		name,
		uuid,
		username
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username,
	) {
		(Some(uuid), _, _, _, _) => Ok((Guild::from_uuid(uuid).await?, None)),
		(_, Some(name), _, _, _) => Ok((Guild::from_name(&name).await?, None)),
		(_, _, Some(uuid), _, _) => Ok((
			Guild::from_member_uuid(uuid).await?,
			Some(Player::from_uuid_unchecked(uuid)),
		)),
		(_, _, _, Some(username), _) => {
			let player = Player::from_username(username.as_str()).await?;

			Ok((Guild::from_member_uuid(player.uuid).await?, Some(player)))
		}
		(_, _, _, None, Some(username)) => Err(Error::InvalidUsername(username)),
		_ => {
			let uuid = schema::user::table
				.filter(schema::user::id.eq(ctx.author().ok_or(Error::NotLinked)?.id.get() as i64))
				.select(schema::user::uuid)
				.get_result::<Option<Uuid>>(&mut ctx.data().pool.get().await?)
				.await;

			if let Ok(Some(uuid)) = uuid {
				Ok((
					Guild::from_member_uuid(uuid).await?,
					Some(Player::from_uuid_unchecked(uuid)),
				))
			} else {
				Err(Error::NotLinked)
			}
		}
	}
}

pub async fn get_guild_from_input(
	ctx: &Context<'_>,
	name: Option<String>,
	uuid: Option<Uuid>,
	username: Option<String>,
	guild_id: Option<Uuid>,
) -> Result<Arc<Guild>, Error> {
	match (
		guild_id,
		name,
		uuid,
		username
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username,
	) {
		(Some(uuid), _, _, _, _) => Ok(Guild::from_uuid(uuid).await?),
		(_, Some(name), _, _, _) => Ok(Guild::from_name(&name).await?),
		(_, _, Some(uuid), _, _) => Ok(Guild::from_member_uuid(uuid).await?),
		(_, _, _, Some(username), _) => {
			let player = Player::from_username(username.as_str()).await?;

			Ok(Guild::from_member_uuid(player.uuid).await?)
		}
		(_, _, _, None, Some(username)) => Err(Error::InvalidUsername(username)),
		_ => {
			let uuid = schema::user::table
				.filter(schema::user::id.eq(ctx.author().ok_or(Error::NotLinked)?.id.get() as i64))
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

pub async fn get_guild_with_member_from_input(
	ctx: &Context<'_>,
	uuid: Option<Uuid>,
	username: Option<String>,
) -> Result<(Arc<Guild>, Player), Error> {
	match (
		uuid,
		username
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username,
	) {
		(Some(uuid), _, _) => Ok((
			Guild::from_member_uuid(uuid).await?,
			Player::from_uuid_unchecked(uuid),
		)),
		(_, Some(username), _) => {
			let player = Player::from_username(username.as_str()).await?;

			Ok((Guild::from_member_uuid(player.uuid).await?, player))
		}
		(_, None, Some(username)) => Err(Error::InvalidUsername(username)),
		_ => {
			let uuid = schema::user::table
				.filter(schema::user::id.eq(ctx.author().ok_or(Error::NotLinked)?.id.get() as i64))
				.select(schema::user::uuid)
				.get_result::<Option<Uuid>>(&mut ctx.data().pool.get().await?)
				.await;

			if let Ok(Some(uuid)) = uuid {
				Ok((
					Guild::from_member_uuid(uuid).await?,
					Player::from_uuid_unchecked(uuid),
				))
			} else {
				Err(Error::NotLinked)
			}
		}
	}
}

#[allow(clippy::too_many_lines)]
pub async fn error(ctx: &context::Context<'_>, error: Error) {
	let content = match error {
		Error::ApiRaw(err) => match err {
			ApiError::PlayerNotFound(ref name) => {
				tr_fmt!(ctx, "error-player-not-found", name: name.as_str())
			}
			ApiError::SnapshotNotFound(ref name) => {
				tr_fmt!(ctx, "error-snapshot-not-found", name: name.as_str())
			}
			ApiError::ProfileNotFound(ref profile, ref name) => {
				tr_fmt!(ctx, "error-profile-not-found", profile: profile.as_str(), name: name.as_str())
			}
			ApiError::UuidNotFound(ref uuid) => {
				tr_fmt!(ctx, "error-player-uuid-not-found", uuid: uuid.to_string())
			}
			ApiError::UsernameNotFound(ref name) => {
				tr_fmt!(ctx, "error-player-username-not-found", name: name.as_str())
			}
			ApiError::GuildByMemberUuidNotFound(ref uuid) => {
				tr_fmt!(ctx, "error-guild-by-member-uuid-not-found", uuid: uuid.to_string())
			}
			ApiError::GuildByMemberUsernameNotFound(ref name) => {
				tr_fmt!(ctx, "error-guild-by-member-username-not-found", name: name.as_str())
			}
			ApiError::GuildNotFound(ref name) => {
				tr_fmt!(ctx, "error-guild-not-found", name: name.as_str())
			}
			ref error => {
				error!(error = ?error, "internal error");
				tr(ctx, "error-internal")
			}
		},
		Error::Api(err) => match *err {
			ApiError::PlayerNotFound(ref name) => {
				tr_fmt!(ctx, "error-player-not-found", name: name.as_str())
			}
			ApiError::SnapshotNotFound(ref name) => {
				tr_fmt!(ctx, "error-snapshot-not-found", name: name.as_str())
			}
			ApiError::ProfileNotFound(ref profile, ref name) => {
				tr_fmt!(ctx, "error-profile-not-found", profile: profile.as_str(), name: name.as_str())
			}
			ApiError::UuidNotFound(ref uuid) => {
				tr_fmt!(ctx, "error-player-uuid-not-found", uuid: uuid.to_string())
			}
			ApiError::UsernameNotFound(ref name) => {
				tr_fmt!(ctx, "error-player-username-not-found", name: name.as_str())
			}
			ApiError::GuildByMemberUuidNotFound(ref uuid) => {
				tr_fmt!(ctx, "error-guild-by-member-uuid-not-found", uuid: uuid.to_string())
			}
			ApiError::GuildByMemberUsernameNotFound(ref name) => {
				tr_fmt!(ctx, "error-guild-by-member-username-not-found", name: name.as_str())
			}
			ApiError::GuildNotFound(ref name) => {
				tr_fmt!(ctx, "error-guild-not-found", name: name.as_str())
			}
			ref error => {
				error!(error = ?error, "internal error");
				tr(ctx, "error-internal")
			}
		},
		Error::SessionNotFound => tr(ctx, "error-session-not-found"),
		Error::SessionAlreadyExists => tr(ctx, "error-session-already-exists"),
		Error::NotLinked => tr(ctx, "error-not-linked"),
		Error::InvalidUuid(ref uuid) => {
			tr_fmt!(ctx, "error-invalid-uuid", uuid: uuid.to_string())
		}
		Error::InvalidUsername(ref name) => {
			tr_fmt!(ctx, "error-invalid-username", name: name.as_str())
		}
		Error::MemberPlayerNotFound(ref name) => {
			tr_fmt!(ctx, "error-member-player-not-found", name: name.as_str())
		}
		Error::SkyBlockProfileNotFound(ref name) => {
			tr_fmt!(ctx, "error-skyblock-profile-not-found", name: name.as_str())
		}
		Error::LeaderboardNotFound(ref name) => {
			tr_fmt!(ctx, "error-leaderboard-not-found", name: name)
		}
		Error::LeaderboardPlayerNotFound => tr(ctx, "error-leaderboard-player-not-found"),
		Error::IdentifierTooLong => tr(ctx, "error-identifier-too-long"),
		Error::UserTrackLimitReached(ref limit) => {
			tr_fmt!(ctx, "error-user-track-limit-reached", limit: limit)
		}
		Error::GuildTrackLimitReached(ref limit) => {
			tr_fmt!(ctx, "error-guild-track-limit-reached", limit: limit)
		}
		Error::TrackAlreadyExists => tr(ctx, "error-track-already-exists"),
		Error::BoostLimitReached(ref limit) => {
			tr_fmt!(ctx, "error-boost-limit-reached", limit: limit)
		}
		Error::NotPremium => tr(ctx, "error-not-premium"),
		Error::NotInAGuild => tr(ctx, "error-not-in-a-guild"),
		Error::BoostAlreadyExists => tr(ctx, "error-boost-already-exists"),
		Error::TimeParse(error) => match error {
			humantime::DurationError::InvalidCharacter(position) => {
				tr_fmt!(ctx, "error-time-invalid-character", position: position)
			}
			humantime::DurationError::NumberExpected(position) => {
				tr_fmt!(ctx, "error-time-expected-number", position: position)
			}
			humantime::DurationError::UnknownUnit {
				start: position,
				unit,
				value,
				..
			} => {
				tr_fmt!(ctx, "error-time-unknown-unit", position: position, unit: unit, value: value)
			}
			humantime::DurationError::NumberOverflow => tr(ctx, "error-time-overflow"),
			humantime::DurationError::Empty => tr(ctx, "error-time-empty"),
		},
		ref error => {
			error!(error = ?error, "internal error");
			tr(ctx, "error-internal")
		}
	};

	if let Err(e) = ctx
		.reply(poise::CreateReply::new().content(content).ephemeral(true))
		.await
	{
		error!(e = ?e, "failed to send error message");
	}
}

pub async fn error_handler(e: poise::FrameworkError<'_, Data, Error>) {
	if let poise::FrameworkError::Command { error: e, ctx } = e {
		let ctx = &context::Context::from_poise(&ctx);

		error(ctx, e).await;
	} else {
		error!(error = ?e, "non-command internal error");
	}
}
