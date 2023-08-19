use database::schema;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use minecraft::style::Family;
use translate::tr;

use crate::{format::Display, util::success_embed, Context, Error};

#[derive(poise::ChoiceParameter)]
pub enum Format {
	Image,
	ImageFaithful,
	ImageRoboto,
	Compact,
	Text,
}

/// Changes the way responses are displayed.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "EMBED_LINKS"
)]
pub async fn display(ctx: Context<'_>, format: Format) -> Result<(), Error> {
	let u = ctx.author();
	let (display, font) = match format {
		Format::Image => (Display::Image, Family::Minecraft),
		Format::ImageFaithful => (Display::Image, Family::Faithful),
		Format::ImageRoboto => (Display::Image, Family::Roboto),
		Format::Compact => (Display::Compact, Family::Minecraft),
		Format::Text => (Display::Text, Family::Minecraft),
	};

	diesel::insert_into(schema::user::table)
		.values((
			schema::user::display.eq(&display),
			schema::user::font.eq(&font),
			schema::user::id.eq(u.id.0.get() as i64),
		))
		.on_conflict(schema::user::id)
		.do_update()
		.set((
			schema::user::display.eq(&display),
			schema::user::font.eq(&font),
			schema::user::updated_at.eq(chrono::Utc::now()),
		))
		.execute(&mut ctx.data().pool.get().await?)
		.await?;

	ctx.send(success_embed(
		tr(&ctx, "display-changed"),
		match format {
			Format::Compact => tr(&ctx, "display-changed-compact-description"),
			Format::Text => tr(&ctx, "display-changed-text-description"),
			_ => tr(&ctx, "display-changed-image-description"),
		},
	))
	.await?;

	Ok(())
}
