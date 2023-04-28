use database::schema;
use diesel::{ExpressionMethods, RunQueryDsl};
use uuid::Uuid;

use crate::{
	util::{error_embed, escape_username, success_embed},
	Context, Error,
};

/// Links your Discord account to a Minecraft account.
#[poise::command(slash_command)]
pub async fn link(
	ctx: Context<'_>,
	#[description = "Your Minecraft UUID"]
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
	#[description = "Your Minecraft username"]
	#[max_length = 16]
	username: Option<String>,
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
			success_embed(
				m,
				"Linking successful",
				&format!(
					"Your Discord account is now linked to the Minecraft account **{}**.",
					escape_username(&player.username)
				),
			)
		})
		.await?;
	} else {
		ctx.send(|m| {
			error_embed(
				m,
				"Linking failed",
				&match (uuid, username) {
					(Some(uuid), _) => {
						format!("The UUID `{uuid}` does not belong to a Minecraft account.")
					}
					(_, Some(username)) => format!(
						"The username **{}** does not belong to a Minecraft account.",
						escape_username(&username)
					),
					(None, None) => "You must provide a valid UUID or a username.".to_string(),
				},
			)
		})
		.await?;
	}

	Ok(())
}
