use api::command;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateAttachment;
use translate::{context, tr_fmt, Error};

pub async fn command(
	ctx: &context::Context<'_>,
	leaderboard: &api::leaderboard::Leaderboard,
	board: u32,
	input: command::LeaderboardInput,
	filter: command::LeaderboardFilter,
	order: command::LeaderboardOrder,
) -> Result<(), Error> {
	ctx.defer().await?;

	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let id = api::id::command(api::command::Id::Leaderboard {
		board,
		input,
		filter,
		order,
	});

	let (png, page) =
		super::image::command(ctx, leaderboard, input, filter, order, family, background).await?;

	let mut buttons = vec![];

	if page > 0 {
		buttons.push(
			serenity::CreateButton::new(api::id::command(api::command::Id::Leaderboard {
				board,
				filter,
				order,
				input: api::command::LeaderboardInput::Page(page - 1),
			}))
			.emoji(serenity::ReactionType::Unicode("⬅️".to_string()))
			.style(serenity::ButtonStyle::Primary),
		);
	}

	buttons.push(
		serenity::CreateButton::new(api::id::command(api::command::Id::Leaderboard {
			board,
			filter,
			order,
			input: api::command::LeaderboardInput::Page(page + 1),
		}))
		.emoji(serenity::ReactionType::Unicode("➡️".to_string()))
		.style(serenity::ButtonStyle::Primary),
	);

	let row = serenity::CreateActionRow::Buttons(buttons);

	ctx.send(
		poise::CreateReply::new()
			.content(format!(
				"{}\n{}",
				tr_fmt!(ctx, "identifier", identifier: id),
				crate::tip::random(ctx),
			))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME))
			.components(vec![row]),
	)
	.await?;

	Ok(())
}
