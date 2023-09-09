use std::borrow::Cow;
use std::collections::HashMap;

use api::{
	canvas::{self, body::Body, shape, Canvas},
	player::Player,
};
use database::schema::leaderboard;
use diesel::{
	dsl::sql, expression::SqlLiteral, sql_types::Integer, BoolExpressionMethods, ExpressionMethods,
	QueryDsl,
};
use diesel_async::RunQueryDsl;
use futures::StreamExt;
use minecraft::{
	paint::Paint,
	text::{parse::minecraft_string, Text},
};
use once_cell::sync::Lazy;
use poise::serenity_prelude::CreateAttachment;
use skia_safe::textlayout::TextAlign;
use translate::{context, has_tr, tr, Context, Error};
use uuid::Uuid;

use crate::util;

#[derive(serde::Deserialize, Debug)]
struct LeaderboardRaw {
	#[serde(deserialize_with = "hypixel::game::r#type::de_from_name")]
	game: hypixel::game::r#type::Type,
	mode: Option<String>,
	path: String,
	tr: String,
}

struct Leaderboard {
	game: hypixel::game::r#type::Type,
	display_name: String,
	display_name_lower: String,
	name: String,
	path_select_sql: SqlLiteral<Integer>,
	path_order_sql: SqlLiteral<Integer>,
	path_filter_sql: diesel::dsl::IsNotNull<SqlLiteral<Integer>>,
}

static LEADERBOARDS: Lazy<HashMap<String, Leaderboard>> = Lazy::new(|| {
	let file = std::fs::File::open("include/leaderboards.json").unwrap();
	let leaderboards: Vec<LeaderboardRaw> = serde_json::from_reader(file).unwrap();
	let ctx = context::Context::external(crate::DATA.get().unwrap());

	leaderboards
		.into_iter()
		.filter_map(|l| {
			let mode = l.mode.and_then(|m| {
				if !has_tr(&ctx, &m) {
					return None;
				}

				Some(tr(&ctx, &m).into_owned())
			});

			if !has_tr(&ctx, &l.tr) {
				return None;
			}

			let name_tr = tr(&ctx, &l.tr);

			let name = format!(
				"{}{} ({})",
				if mode.is_some() { " " } else { "" },
				if let Some(mode) = mode.as_deref() {
					mode
				} else {
					""
				},
				name_tr
			);
			let display_name = format!("{}{}", l.game.as_clean_name(), name);

			let path_sql = sql::<diesel::sql_types::Integer>(&l.path);

			Some((
				display_name.clone(),
				Leaderboard {
					game: l.game,
					path_select_sql: sql::<Integer>(&format!("CAST({} AS INT)", &l.path)),
					path_filter_sql: path_sql.clone().is_not_null(),
					// TODO: use ascending order for stuff like fastest wins, etc.
					path_order_sql: sql::<Integer>(&format!("{} DESC", &l.path)),
					display_name_lower: display_name.to_ascii_lowercase(),
					display_name,
					name,
				},
			))
		})
		.collect()
});

#[allow(clippy::unused_async)]
async fn autocomplete_board(_ctx: Context<'_>, partial: &str) -> impl Iterator<Item = String> {
	let lower = partial.to_ascii_lowercase();

	Box::new(
		LEADERBOARDS
			.values()
			.filter_map(|board| {
				if !board.display_name_lower.contains(&lower) {
					return None;
				}

				Some(board.display_name.clone())
			})
			.take(10)
			.collect::<Vec<_>>()
			.into_iter(),
	)
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::cast_sign_loss)]
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "ATTACH_FILES"
)]
pub async fn leaderboard(
	ctx: Context<'_>,
	#[autocomplete = "autocomplete_board"] board: String,
	guild: Option<String>,
	player: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let ctx = &context::Context::from_poise(&ctx);

	let (_, family, background) = crate::util::get_image_options_from_input(ctx).await;
	let leaderboard = {
		let Some(leaderboard) = LEADERBOARDS.get(&board) else {
			return Err(Error::LeaderboardNotFound(board));
		};

		leaderboard
	};

	let skip = if let Some(player) = player && guild.is_none() {
		let uuid = util::parse_uuid(Some(player.as_str()));
		let player = util::get_player_from_input(ctx, uuid, Some(player)).await?;

		let value = leaderboard::table
			.filter(&leaderboard.path_filter_sql)
			.filter(leaderboard::uuid.eq(player.uuid))
			.select(&leaderboard.path_select_sql)
			.first::<i32>(&mut ctx.data().pool.get().await?)
			.await?;

		leaderboard::table
			.filter(&leaderboard.path_filter_sql)
			.filter(
				&leaderboard.path_select_sql.clone()
					.gt(value)
					.or(
						leaderboard.path_select_sql.clone().eq(value)
							.and(leaderboard::uuid.lt(player.uuid))
					)
			)
			.count()
			.get_result::<i64>(&mut ctx.data().pool.get().await?)
			.await?
	} else {
		0i64
	};

	let leaders = if let Some(guild) = guild {
		let guild_id = util::parse_uuid(Some(guild.as_str()));
		let guild = crate::commands::get_guild(ctx, Some(guild), None, None, guild_id).await?;

		leaderboard::table
			.filter(&leaderboard.path_filter_sql)
			.filter(leaderboard::uuid.eq_any(guild.members.iter().map(|m| m.uuid)))
			.select((leaderboard::uuid, &leaderboard.path_select_sql))
			.order((&leaderboard.path_order_sql, leaderboard::uuid.asc()))
			.limit(10)
			.get_results::<(Uuid, i32)>(&mut ctx.data().pool.get().await?)
			.await?
	} else {
		leaderboard::table
			.filter(&leaderboard.path_filter_sql)
			.select((leaderboard::uuid, &leaderboard.path_select_sql))
			.order((&leaderboard.path_order_sql, leaderboard::uuid.asc()))
			.offset(skip)
			.limit(10)
			.get_results::<(Uuid, i32)>(&mut ctx.data().pool.get().await?)
			.await?
	};

	let leader_uuids = leaders.iter().map(|(uuid, _)| *uuid).collect::<Vec<_>>();
	let leader_names = futures::stream::iter(
		leader_uuids
			.into_iter()
			.map(Player::from_uuid_unchecked)
			.map(|p| p.get_display_string_owned(ctx)),
	)
	.buffered(10)
	.filter_map(|r| async { r.ok() })
	.collect::<Vec<_>>()
	.await;

	let png: Cow<_> = {
		let mut canvas = Canvas::new(720., family).gap(7.).push_down(
			&shape::LeaderboardTitle,
			Body::new(24., TextAlign::Center, family)
				.extend(leaderboard.game.as_text())
				.extend(&[Text {
					text: &leaderboard.name,
					paint: Paint::White,
					..Default::default()
				}])
				.build(),
		);

		for (idx, (player, (_, value))) in leader_names.iter().zip(leaders).enumerate() {
			canvas = canvas
				.push_down_start(
					&shape::LeaderboardPlace,
					shape::LeaderboardPlace::from_usize(family, idx + 1),
				)
				.push_right(
					&shape::LeaderboardName,
					Body::build_slice(
						family,
						minecraft_string(player).collect::<Vec<_>>().as_slice(),
						20.,
						None,
					),
				)
				.push_right(
					&shape::LeaderboardValue,
					shape::LeaderboardValue::from_value(ctx, family, &(value as u32)),
				);
		}

		let mut surface = canvas.build(None, background).unwrap();

		canvas::to_png(&mut surface).into()
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}
