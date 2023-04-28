use database::schema;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::{util::success_embed, Context, Error};

/// Changes the way responses are displayed.
#[poise::command(slash_command, required_bot_permissions = "EMBED_LINKS")]
pub async fn display(
	ctx: Context<'_>,
	#[description = "Whether to display responses as text"] text: Option<bool>,
) -> Result<(), Error> {
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
			"Display changed",
			match text {
				true => "Responses will now be sent as text.",
				false => "Responses will now be sent as images where applicable.",
			},
		)
	})
	.await?;

	Ok(())
}
