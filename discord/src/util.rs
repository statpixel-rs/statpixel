use api::player::Player;
use database::schema;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use poise::serenity_prelude::User;
use poise::CreateReply;
use std::fmt::Display;
use uuid::Uuid;

use crate::{Context, Data, Error};

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
	username: Option<String>,
) -> Result<Player, Error> {
	match (
		uuid_raw.and_then(|uuid| Uuid::parse_str(&uuid).ok()),
		username,
	) {
		(Some(uuid), _) => Ok(Player::from_uuid(&uuid).await?),
		(_, Some(username)) => Ok(Player::from_username(&username).await?),
		(None, None) => {
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

pub async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
	eprintln!("Internal error: {error:?}");
}
