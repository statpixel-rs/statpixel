use database::schema;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use translate::tr;
use uuid::Uuid;

use crate::{
	util::{error_embed, success_embed},
	Context, Error,
};

/// Unlinks your Discord account from a Minecraft account.
#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn unlink(ctx: Context<'_>) -> Result<(), Error> {
	let removed = diesel::update(schema::user::table)
		.set((
			schema::user::uuid.eq::<Option<Uuid>>(None),
			schema::user::updated_at.eq(chrono::Utc::now()),
		))
		.filter(schema::user::id.eq(ctx.author().id.0.get() as i64))
		.filter(schema::user::uuid.is_not_null())
		.execute(&mut ctx.data().pool.get().await?)
		.await?;

	if removed > 0 {
		ctx.send(success_embed(
			tr(&ctx, "unlinking-succeeded"),
			tr(&ctx, "unlinking-succeeded-description"),
		))
		.await?;
	} else {
		ctx.send(error_embed(
			tr(&ctx, "unlinking-failed"),
			tr(&ctx, "unlinking-failed-description"),
		))
		.await?;
	}

	Ok(())
}
