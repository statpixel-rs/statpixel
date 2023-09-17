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
		if page > 1 {
			buttons.push(
				serenity::CreateButton::new(api::id::command(api::command::Id::Leaderboard {
					board,
					filter,
					order,
					input: api::command::LeaderboardInput::Page(0),
				}))
				.emoji(crate::emoji::ARROW_START)
				.style(serenity::ButtonStyle::Secondary),
			);
		}

		buttons.push(
			serenity::CreateButton::new(api::id::command(api::command::Id::Leaderboard {
				board,
				filter,
				order,
				input: api::command::LeaderboardInput::Page(page - 1),
			}))
			.emoji(crate::emoji::ARROW_LEFT)
			.style(serenity::ButtonStyle::Secondary),
		);
	}

	buttons.push(
		serenity::CreateButton::new(api::id::command(api::command::Id::Leaderboard {
			board,
			filter,
			order,
			input: api::command::LeaderboardInput::Page(page + 1),
		}))
		.emoji(crate::emoji::ARROW_RIGHT)
		.style(serenity::ButtonStyle::Secondary),
	);

	if page < 2_999 {
		buttons.push(
			serenity::CreateButton::new(api::id::command(api::command::Id::Leaderboard {
				board,
				filter,
				order,
				input: api::command::LeaderboardInput::Page(3_000),
			}))
			.emoji(crate::emoji::ARROW_END)
			.style(serenity::ButtonStyle::Secondary),
		);
	}

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
