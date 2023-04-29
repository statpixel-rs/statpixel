use database::schema;
use diesel::{ExpressionMethods, RunQueryDsl};
use translate::tr;
use uuid::Uuid;

use crate::{
	util::{error_embed, escape_username, success_embed},
	Context, Error,
};

/// Links your Discord account to a Minecraft account.
#[poise::command(slash_command)]
pub async fn link(
	ctx: Context<'_>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
	#[max_length = 16] username: Option<String>,
) -> Result<(), Error> {
	let (player, uuid, username) = match (uuid.and_then(|u| Uuid::parse_str(&u).ok()), username) {
		(r @ Some(uuid), _) => (api::player::Player::from_uuid(&uuid).await, r, None),
		(_, Some(username)) => (
			api::player::Player::from_username(&username).await,
			None,
			Some(username),
		),
		(None, None) => {
			ctx.send(|m| {
				m.embed(|e| {
					e.title(tr!(ctx, "linking-failed"))
						.description(tr!(ctx, "linking-failed-description"))
						.colour(crate::EMBED_COLOUR)
				})
			})
			.await?;

			return Ok(());
		}
	};

	if let Ok(player) = player {
		diesel::insert_into(schema::users::table)
			.values((
				schema::users::id.eq(ctx.author().id.0 as i64),
				schema::users::uuid.eq(player.uuid),
			))
			.on_conflict(schema::users::id)
			.do_update()
			.set(schema::users::uuid.eq(player.uuid))
			.execute(&mut ctx.data().pool.get()?)?;

		ctx.send(|m| {
			success_embed(
				m,
				tr!(ctx, "linking-succeeded"),
				tr!(ctx, "linking-succeeded-description", name: escape_username(&player.username)),
			)
		})
		.await?;
	} else {
		ctx.send(|m| {
			error_embed(
				m,
				tr!(ctx, "linking-failed"),
				match (uuid, username) {
					(Some(uuid), _) => {
						tr!(ctx, "linking-failed-uuid-description", uuid: uuid.to_string())
					}
					(_, Some(username)) => tr!(
						ctx,
						"linking-failed-username-description",
						name: escape_username(&username)
					),
					(None, None) => tr!(ctx, "linking-failed-description"),
				},
			)
		})
		.await?;
	}

	Ok(())
}
