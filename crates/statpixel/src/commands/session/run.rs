use std::borrow::Cow;

use api::{command::Id, id, Player};
use chrono::{DateTime, Utc};
use database::schema::session;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::{CreateActionRow, CreateButton, CreateEmbed};
use translate::{context, tr, tr_fmt};

use crate::Error;

pub async fn list(
	ctx: &context::Context<'_>,
	player: Option<Player>,
	page: u32,
) -> Result<(), Error> {
	let sessions: Vec<(uuid::Uuid, Option<String>, uuid::Uuid, DateTime<Utc>)> =
		if let Some(ref player) = player {
			session::table
				.select((
					session::id,
					session::name,
					session::uuid,
					session::created_at,
				))
				.filter(session::uuid.eq(player.uuid))
				.order(session::created_at.desc())
				.limit(10)
				.offset(i64::from(page) * 10)
				.get_results(&mut ctx.connection().await?)
				.await?
		} else {
			session::table
				.select((
					session::id,
					session::name,
					session::uuid,
					session::created_at,
				))
				.filter(session::user_id.eq(ctx.author().unwrap().id.get() as i64))
				.order(session::created_at.desc())
				.limit(10)
				.offset(i64::from(page) * 10)
				.get_results(&mut ctx.connection().await?)
				.await?
		};

	let embed = CreateEmbed::new()
		.title(tr_fmt!(ctx, "session-list-title", page: page + 1))
		.colour(crate::EMBED_COLOUR)
		.description(if sessions.is_empty() {
			tr(ctx, "session-list-empty")
		} else {
			Cow::Owned(
				sessions
					.into_iter()
					.map(|(id, name, uuid, created_at)| {
						format!(
							"- [**`{}`**](https://namemc.com/profile/{}) - <t:{}:R>",
							name.unwrap_or_else(|| id.to_string()),
							uuid,
							created_at.timestamp()
						)
					})
					.intersperse("\n".to_string())
					.collect::<String>(),
			)
		});

	let mut components = vec![];

	if page > 0 {
		components.push(
			CreateButton::new(id::command(Id::SessionPage {
				uuid: player.as_ref().map(|p| p.uuid),
				page: page - 1,
			}))
			.emoji(crate::emoji::ARROW_LEFT),
		);
	}

	components.push(
		CreateButton::new(id::command(Id::SessionPage {
			uuid: player.as_ref().map(|p| p.uuid),
			page: page + 1,
		}))
		.emoji(crate::emoji::ARROW_RIGHT),
	);

	ctx.send(
		poise::CreateReply::new()
			.embed(embed)
			.components(vec![CreateActionRow::Buttons(components)]),
	)
	.await?;

	Ok(())
}
