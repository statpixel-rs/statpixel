use api::player::Player;
use database::schema;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use poise::serenity_prelude::User;
use uuid::Uuid;

use crate::{
	canvas::create_surface,
	util::{error_embed, success_embed},
	Context, Error,
};

async fn get_player_from_input(
	ctx: Context<'_>,
	author: &User,
	uuid_raw: Option<String>,
	username: Option<String>,
) -> Result<Player, Error> {
	match (
		uuid_raw.and_then(|uuid| Uuid::parse_str(&uuid).ok()),
		username,
	) {
		(Some(uuid), _) => Ok(Player::from_uuid(&uuid).await?),
		(_, Some(username)) => Ok(Player::from_username(&username).await?),
		(None, None) => {
			let uuid: Option<Uuid> = schema::users::table
				.filter(schema::users::id.eq(author.id.0 as i64))
				.select(schema::users::uuid)
				.get_result::<Option<Uuid>>(&mut ctx.data().pool.get()?)?;

			if let Some(uuid) = uuid {
				Ok(Player::from_uuid(&uuid).await?)
			} else {
				Err(Error::NotLinked)
			}
		}
	}
}

/// Shows the SkyWars stats of a player.
#[poise::command(slash_command)]
pub async fn skywars(
	ctx: Context<'_>,
	#[description = "Your Minecraft UUID"] uuid: Option<String>,
	#[description = "Your Minecraft username"] username: Option<String>,
) -> Result<(), Error> {
	let player = match get_player_from_input(ctx, ctx.author(), uuid, username).await {
		Ok(player) => player,
		Err(Error::NotLinked) => {
			ctx.send(|m| {
				error_embed(
					m,
					"Missing arguments",
					"Invalid UUID or username provided, and you are not linked.",
				)
			})
			.await?;

			return Ok(());
		}
		Err(e) => return Err(e),
	};

	let data = player.get_data().await?;
	let canvas = create_surface()
		.image_snapshot()
		.encode_to_data(skia_bindings::SkEncodedImageFormat::PNG)
		.unwrap();

	ctx.send(|m| {
		success_embed(
			m,
			"SkyWars",
			&format!("Nice, {} kills!", data.stats.sky_wars.solo_normal.kills),
		)
		.attachment(poise::serenity_prelude::AttachmentType::Bytes {
			data: canvas.as_bytes().into(),
			filename: "canvas.png".into(),
		})
	})
	.await?;

	Ok(())
}
