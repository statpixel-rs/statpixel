use database::schema;
use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;

use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn link(
	ctx: Context<'_>,
	#[description = "Your Minecraft UUID"] uuid: Option<String>,
	#[description = "Your Minecraft username"] username: Option<String>,
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
					e.title("Linking failed")
						.description("You must provide either a UUID or a username.")
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
			m.embed(|e| {
				e.title("Linking successful")
					.description(format!(
						"Your Discord account is now linked to the Minecraft account **{}**.",
						player.username.replace('_', "\\_")
					))
					.colour(crate::EMBED_COLOUR)
			})
		})
		.await?;
	} else {
		ctx.send(|m| {
			m.embed(|e| {
				e.title("Linking failed")
					.description(match (uuid, username) {
						(Some(uuid), _) => {
							format!("The UUID `{uuid}` does not belong to a Minecraft account.")
						}
						(_, Some(username)) => format!(
							"The username **{}** does not belong to a Minecraft account.",
							username.replace('_', "\\_")
						),
						(None, None) => "You must provide a valid UUID or a username.".to_string(),
					})
					.colour(crate::EMBED_COLOUR_ERROR)
			})
		})
		.await?;
	}

	Ok(())
}
