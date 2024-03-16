use chrono::{DateTime, Utc};
use database::schema::{boost, track, user};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use translate::{context, tr, tr_fmt};

use crate::{
	util::{self, success_embed},
	Context, Error,
};

const MAX_TRACKS_PER_GUILD: i64 = 100;

/// Tracks a Minecraft account and submits their data to a channel.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	default_member_permissions = "MANAGE_GUILD"
)]
pub async fn track(
	ctx: Context<'_>,
	#[max_length = 36]
	#[autocomplete = "crate::commands::autocomplete_username"]
	player: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let author = ctx.author();
	let guild_id = ctx.guild_id();
	let channel_id = ctx.channel_id();

	let uuid = util::parse_uuid(player.as_deref());
	let ctx = &context::Context::from_poise(&ctx);
	let player = util::get_player_with_username_from_input(ctx, uuid, player).await?;

	let mut connection = ctx.connection().await?;

	connection
		.transaction::<(), Error, _>(|connection| {
			async move {
				match diesel::insert_into(track::table)
					.values((
						track::user_id.eq(author.id.get() as i64),
						track::uuid.eq(player.uuid),
						track::guild_id.eq(guild_id.map(|g| g.get() as i64)),
						track::channel_id.eq(channel_id.get() as i64),
					))
					.execute(connection)
					.await
				{
					Err(diesel::result::Error::DatabaseError(
						diesel::result::DatabaseErrorKind::UniqueViolation,
						_,
					)) => {
						return Err(Error::TrackAlreadyExists);
					}
					Err(e) => return Err(e.into()),
					_ => {}
				};

				if let Some(guild_id) = guild_id {
					let is_premium: i64 = boost::table
						.filter(boost::guild_id.eq(guild_id.get() as i64))
						.count()
						.get_result(connection)
						.await?;

					if is_premium == 0 {
						let tracks: i64 = track::table
							.filter(track::guild_id.eq(guild_id.get() as i64))
							.count()
							.get_result(connection)
							.await?;

						if tracks > MAX_TRACKS_PER_GUILD {
							return Err(Error::GuildTrackLimitReached(MAX_TRACKS_PER_GUILD));
						}
					}
				}

				let (tracks, max_tracks, premium_until) = diesel::insert_into(user::table)
					.values((user::id.eq(author.id.get() as i64), user::tracks.eq(1)))
					.on_conflict(user::id)
					.do_update()
					.set(user::tracks.eq(user::tracks + 1))
					.returning((user::tracks, user::max_tracks, user::premium_until))
					.get_result::<(i16, i16, Option<DateTime<Utc>>)>(connection)
					.await?;

				if let Some(premium_until) = premium_until
					&& premium_until < Utc::now()
					&& tracks > max_tracks
				{
					return Err(Error::UserTrackLimitReached(max_tracks));
				}

				Ok(())
			}
			.scope_boxed()
		})
		.await?;

	ctx.send(
		success_embed(
			tr_fmt!(ctx, "track-success-title", name: player.username.unwrap()),
			tr(ctx, "track-success-description"),
		)
		.content(crate::tip::random(ctx)),
	)
	.await?;

	Ok(())
}
