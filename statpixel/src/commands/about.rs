use crate::Error;
use api::canvas::label::ToFormatted;
use database::schema;
use diesel::{dsl::count_distinct, QueryDsl};
use diesel_async::RunQueryDsl;
use translate::{context, tr, Context};

use poise::serenity_prelude as serenity;

const TITLE: &str = concat!("StatPixel | v", env!("CARGO_PKG_VERSION"));

#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
	let ctx = &context::Context::from_poise(&ctx);
	let (guilds, profiles, users, snapshots) = {
		let connection = &mut ctx.data().pool.get().await?;

		let users = schema::usage::table
			.select(count_distinct(schema::usage::user_id))
			.get_result::<i64>(connection)
			.await?;

		let snapshots = schema::snapshot::table
			.count()
			.get_result::<i64>(connection)
			.await?;

		let profiles = schema::user::table
			.count()
			.get_result::<i64>(connection)
			.await?;

		let guilds = crate::GUILDS.read().await.len();

		(guilds, profiles, users, snapshots)
	};

	ctx.send(
		poise::CreateReply::new().embed(
			serenity::CreateEmbed::new()
				.colour(crate::EMBED_COLOUR)
				.thumbnail("https://cdn.discordapp.com/avatars/718687348883193916/63ca5ae665b90ce664e449039651129d.png?size=256")
				.title(TITLE)
				.description(tr(ctx, "about-description"))
				.field(tr(ctx, "author"), "@walfakoot (GoogleSites#7707)", true)
				.field(tr(ctx, "guilds"), guilds.to_formatted(ctx), true)
				.field("\u{200b}", "\u{200b}", true)
				.field(tr(ctx, "profiles"), profiles.to_formatted(ctx), true)
				.field(tr(ctx, "users"), users.to_formatted(ctx), true)
				.field(
					tr(ctx, "snapshots"),
					snapshots.to_formatted(ctx),
					true,
				),
		),
	)
	.await?;

	Ok(())
}
