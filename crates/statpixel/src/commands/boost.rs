use chrono::{DateTime, Utc};
use database::schema::{boost, user};
use diesel::ExpressionMethods;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use translate::{context, tr, tr_fmt};

use crate::{util::success_embed, Context, Error};

/// Adds a boost to a guild if you're StatPixel+.
#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn boost(ctx: Context<'_>) -> Result<(), Error> {
	ctx.defer().await?;

	let Some(guild) = ctx.partial_guild().await else {
		return Err(Error::NotInAGuild);
	};

	let author = ctx.author();
	let ctx = &context::Context::from_poise(&ctx);
	let mut connection = ctx.connection().await?;

	connection
		.transaction::<(), Error, _>(|connection| {
			async move {
				match diesel::insert_into(boost::table)
					.values((
						boost::user_id.eq(author.id.get() as i64),
						boost::guild_id.eq(guild.id.get() as i64),
					))
					.execute(connection)
					.await
				{
					Err(diesel::result::Error::DatabaseError(
						diesel::result::DatabaseErrorKind::UniqueViolation,
						_,
					)) => {
						return Err(Error::BoostAlreadyExists);
					}
					Err(e) => return Err(e.into()),
					_ => {}
				};

				let (boosts, max_boosts, premium_until) = diesel::insert_into(user::table)
					.values((user::id.eq(author.id.get() as i64), user::boosts.eq(1)))
					.on_conflict(user::id)
					.do_update()
					.set(user::boosts.eq(user::boosts + 1))
					.returning((user::boosts, user::max_boosts, user::premium_until))
					.get_result::<(i16, i16, Option<DateTime<Utc>>)>(connection)
					.await?;

				if let Some(premium_until) = premium_until && premium_until < Utc::now() {
					return Err(Error::NotPremium);
				}

				if boosts > max_boosts {
					return Err(Error::BoostLimitReached(max_boosts));
				}

				Ok(())
			}
			.scope_boxed()
		})
		.await?;

	ctx.send(
		success_embed(
			tr_fmt!(&ctx, "boost-success-title", name: guild.name),
			tr(&ctx, "boost-success-description"),
		)
		.content(crate::tip::random(&ctx)),
	)
	.await?;

	Ok(())
}
