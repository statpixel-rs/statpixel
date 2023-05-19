use api::{guild::Guild, player::Player};
use database::schema;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use minecraft::username::Username;
use poise::serenity_prelude::User;
use poise::CreateReply;
use std::fmt::Display;
use uuid::Uuid;

use crate::{Context, Error};

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

pub async fn get_player_from_input(
	ctx: Context<'_>,
	author: &User,
	uuid_raw: Option<String>,
	username_raw: Option<String>,
) -> Result<Player, Error> {
	match (
		uuid_raw
			.as_ref()
			.and_then(|uuid| Uuid::parse_str(uuid).ok()),
		uuid_raw,
		username_raw
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username_raw,
	) {
		(Some(uuid), _, _, _) => Ok(Player::from_uuid(&uuid).await?),
		(_, _, Some(username), _) => Ok(Player::from_username(username.as_str()).await?),
		(None, Some(uuid), _, _) => Err(Error::InvalidUuid(uuid)),
		(_, _, None, Some(username)) => Err(Error::InvalidUsername(username)),
		(None, _, None, _) => {
			let uuid: Option<Uuid> = schema::users::table
				.filter(schema::users::id.eq(author.id.0 as i64))
				.select(schema::users::uuid)
				.get_result::<Option<Uuid>>(&mut ctx.data().pool.get()?)?;

			if let Some(uuid) = uuid {
				Ok(Player::from_uuid(&uuid).await?)
			} else {
				Err(Error::NotLinked)
			}
		}
	}
}

pub async fn get_guild_from_input(
	ctx: Context<'_>,
	author: &User,
	name_raw: Option<String>,
	uuid_raw: Option<String>,
	username_raw: Option<String>,
) -> Result<Guild, Error> {
	match (
		name_raw,
		uuid_raw
			.as_ref()
			.and_then(|uuid| Uuid::parse_str(uuid).ok()),
		uuid_raw,
		username_raw
			.as_ref()
			.and_then(|username| Username::try_from_str(username).ok()),
		username_raw,
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
			let uuid: Option<Uuid> = schema::users::table
				.filter(schema::users::id.eq(author.id.0 as i64))
				.select(schema::users::uuid)
				.get_result::<Option<Uuid>>(&mut ctx.data().pool.get()?)?;

			if let Some(uuid) = uuid {
				Ok(Guild::from_member_uuid(uuid).await?)
			} else {
				Err(Error::NotLinked)
			}
		}
	}
}
