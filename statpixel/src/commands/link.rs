use database::schema;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use poise::serenity_prelude as serenity;
use translate::{context, tr, tr_fmt};
use uuid::Uuid;

use crate::{
	util::{error_embed, escape_username, success_embed},
	Context, Error,
};

/// Links your Discord account to a Minecraft account.
#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn link(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let ctx = &context::Context::from_poise(&ctx);

	let (player, uuid, username) = match (uuid.and_then(|u| Uuid::parse_str(&u).ok()), username) {
		(r @ Some(uuid), _) => (api::player::Player::from_uuid(&uuid).await, r, None),
		(_, Some(username)) => (
			api::player::Player::from_username(&username).await,
			None,
			Some(username),
		),
		(None, None) => {
			ctx.send(
				poise::CreateReply::new()
					.content(crate::tip::random(ctx))
					.embed(
						serenity::CreateEmbed::new()
							.title(tr(ctx, "linking-failed"))
							.description(tr(ctx, "linking-failed-description"))
							.colour(crate::EMBED_COLOUR),
					),
			)
			.await?;

			return Ok(());
		}
	};

	if let Ok(player) = player {
		diesel::insert_into(schema::user::table)
			.values((
				schema::user::id.eq(ctx.author().unwrap().id.0.get() as i64),
				schema::user::uuid.eq(player.uuid),
			))
			.on_conflict(schema::user::id)
			.do_update()
			.set((
				schema::user::uuid.eq(player.uuid),
				schema::user::updated_at.eq(chrono::Utc::now()),
			))
			.execute(&mut ctx.data().pool.get().await?)
			.await?;

		ctx.send(success_embed(
			tr(ctx, "linking-succeeded"),
			tr_fmt!(ctx, "linking-succeeded-description", name: escape_username(player.username.as_deref().unwrap())),
		))
		.await?;
	} else {
		ctx.send(error_embed(
			tr(ctx, "linking-failed"),
			match (uuid, username) {
				(Some(uuid), _) => {
					tr_fmt!(ctx, "linking-failed-uuid-description", uuid: uuid.to_string())
				}
				(_, Some(username)) => tr_fmt!(
					ctx,
					"linking-failed-username-description",
					name: escape_username(&username)
				),
				(None, None) => tr(ctx, "linking-failed-description"),
			},
		))
		.await?;
	}

	Ok(())
}
