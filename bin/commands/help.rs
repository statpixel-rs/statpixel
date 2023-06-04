use crate::Error;
use translate::{tr, Context};

const TITLE: &str = concat!("StatPixel | v", env!("CARGO_PKG_VERSION"));

/// Shows the help menu.
#[poise::command(on_error = "crate::util::error_handler", slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
	ctx.send(|m| {
		m.embed(|e| {
			e.colour(crate::EMBED_COLOUR)
				.title(TITLE)
				.field(
					tr!(ctx, "help-general"),
					tr!(ctx, "help-general-description"),
					false,
				)
				.field(
					tr!(ctx, "help-display"),
					tr!(ctx, "help-display-description"),
					false,
				)
				.field(
					tr!(ctx, "help-link"),
					tr!(ctx, "help-link-description"),
					false,
				)
				.field(
					tr!(ctx, "help-snapshot"),
					tr!(ctx, "help-snapshot-description"),
					false,
				)
				.field(
					tr!(ctx, "help-history"),
					tr!(ctx, "help-history-description"),
					false,
				)
		})
	})
	.await?;

	Ok(())
}
