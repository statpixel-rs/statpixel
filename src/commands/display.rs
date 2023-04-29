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
		diesel::insert_into(schema::users::table)
			.values((
				schema::users::text.eq(text),
				schema::users::id.eq(u.id.0 as i64),
			))
			.on_conflict(schema::users::id)
			.do_update()
			.set(schema::users::text.eq(text))
			.returning(schema::users::text)
			.get_result::<bool>(&mut ctx.data().pool.get()?)?
	} else {
		diesel::insert_into(schema::users::table)
			.values((
				schema::users::text.eq(true),
				schema::users::id.eq(u.id.0 as i64),
			))
			.on_conflict(schema::users::id)
			.do_update()
			.set(schema::users::text.eq(diesel::dsl::not(schema::users::text)))
			.returning(schema::users::text)
			.get_result::<bool>(&mut ctx.data().pool.get()?)?
	};

	ctx.send(|m| {
		success_embed(
			m,
			tr!(ctx, "display-changed"),
			match text {
				true => tr!(ctx, "display-changed-text-description"),
				false => tr!(ctx, "display-changed-image-description"),
			},
		)
	})
	.await?;

	Ok(())
}
