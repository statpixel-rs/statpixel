use database::schema;
use diesel::{ExpressionMethods, RunQueryDsl};
use translate::tr;

use crate::{util::success_embed, Context, Error};

/// Changes the way responses are displayed.
#[poise::command(slash_command, required_bot_permissions = "EMBED_LINKS")]
pub async fn display(ctx: Context<'_>, text: Option<bool>) -> Result<(), Error> {
	let u = ctx.author();

	// If they provide a value, use it. Otherwise, toggle the current value.
	let text = if let Some(text) = text {
		diesel::insert_into(schema::user::table)
			.values((
				schema::user::text.eq(text),
				schema::user::id.eq(u.id.0 as i64),
			))
			.on_conflict(schema::user::id)
			.do_update()
			.set(schema::user::text.eq(text))
			.returning(schema::user::text)
			.get_result::<bool>(&mut ctx.data().pool.get()?)?
	} else {
		diesel::insert_into(schema::user::table)
			.values((
				schema::user::text.eq(true),
				schema::user::id.eq(u.id.0 as i64),
			))
			.on_conflict(schema::user::id)
			.do_update()
			.set(schema::user::text.eq(diesel::dsl::not(schema::user::text)))
			.returning(schema::user::text)
			.get_result::<bool>(&mut ctx.data().pool.get()?)?
	};

	ctx.send(|m| {
		success_embed(
			m,
			tr!(ctx, "display-changed"),
			if text {
				tr!(ctx, "display-changed-text-description")
			} else {
				tr!(ctx, "display-changed-image-description")
			},
		)
	})
	.await?;

	Ok(())
}
