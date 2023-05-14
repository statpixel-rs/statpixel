#[allow(clippy::wildcard_imports)]
use api::player::{self, stats::*};

use chrono::{DateTime, Utc};
use database::schema::snapshot;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use translate::{Context, Error};

enum SnapshotStatus {
	Found((Box<player::data::Data>, DateTime<Utc>)),
	Inserted,
}

/// Gets the earliest snapshot of a given player within a timeframe.
fn get_snapshot(
	ctx: Context<'_>,
	player: &player::Player,
	timeframe: DateTime<Utc>,
) -> Result<Option<(player::data::Data, DateTime<Utc>)>, Error> {
	let result = snapshot::table
		.filter(snapshot::columns::created_at.ge(timeframe))
		.filter(snapshot::columns::uuid.eq(player.uuid))
		.select((snapshot::columns::data, snapshot::columns::created_at))
		.order(snapshot::columns::created_at.asc())
		.first::<(Vec<u8>, DateTime<Utc>)>(&mut ctx.data().pool.get()?);

	match result {
		Ok((data, created_at)) => Ok(Some((bson::from_slice(&data[..])?, created_at))),
		Err(diesel::NotFound) => Ok(None),
		Err(e) => Err(e.into()),
	}
}

fn get_or_insert_snapshot(
	ctx: Context<'_>,
	player: &player::Player,
	data: &player::data::Data,
	timeframe: DateTime<Utc>,
) -> Result<SnapshotStatus, Error> {
	// If a snapshot exists within the given timeframe, return it.
	if let Some(snapshot) = get_snapshot(ctx, player, timeframe)? {
		return Ok(SnapshotStatus::Found((Box::new(snapshot.0), snapshot.1)));
	}

	// Otherwise, insert the current data into the database.
	diesel::insert_into(snapshot::table)
		.values((
			snapshot::columns::uuid.eq(player.uuid),
			snapshot::columns::data.eq(bson::to_vec(data)?),
		))
		.execute(&mut ctx.data().pool.get()?)?;

	// And return nothing.
	Ok(SnapshotStatus::Inserted)
}

macro_rules! generate_command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			mode: Option<$mode>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let (player, mut data, session) = $crate::get_data!(ctx, uuid, username);
			let status =
				get_or_insert_snapshot(ctx, &player, &data, ::chrono::Utc::now() - ::chrono::Duration::days(1))?;

			let png: ::std::option::Option<::std::borrow::Cow<[u8]>> =
				if let SnapshotStatus::Found((ref snapshot, _)) = status {
					let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, mode);

					::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
				} else {
					::std::option::Option::None
				};

			let content = match status {
				SnapshotStatus::Found((_, created_at)) => format!(
					"Showing statistics change from <t:{}:f> to <t:{}:f>",
					created_at.timestamp(),
					::chrono::Utc::now().timestamp(),
				),
				SnapshotStatus::Inserted => format!(
					"No previous data found for **{}**, so it has been inserted.\nShowing statistics change from <t:{}:f> to <t:{}:f>",
					$crate::util::escape_username(&player.username),
					::chrono::Utc::now().timestamp(),
					::chrono::Utc::now().timestamp(),
				),
			};

			ctx.send(move |m| {
				if let ::std::option::Option::Some(png) = png {
					m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
						data: png,
						filename: "canvas.png".into(),
					});
				}

				m.content(content)
			})
			.await?;

			Ok(())
		}
	};
}

macro_rules! generate_large_command {
	($game: ty, $mode: ty, $fn: ident) => {
		async fn autocomplete_mode<'a>(
			ctx: $crate::Context<'a>,
			partial: &'a str,
		) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
			let partial = partial.to_ascii_lowercase();

			<$game>::autocomplete(ctx, partial).await
		}

		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16]
			#[autocomplete = "crate::commands::autocomplete_username"]
			username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let mode: ::std::option::Option<$mode> = mode.map(|m| m.into());
			let (player, mut data, session) = $crate::get_data!(ctx, uuid, username);
			let status =
				get_or_insert_snapshot(ctx, &player, &data, ::chrono::Utc::now() - ::chrono::Duration::days(1))?;

			let png: ::std::option::Option<::std::borrow::Cow<[u8]>> =
				if let SnapshotStatus::Found((ref snapshot, _)) = status {
					let mut surface = <$game>::canvas_diff(ctx, snapshot, &mut data, &session, mode);

					::std::option::Option::Some(::api::canvas::to_png(&mut surface).into())
				} else {
					::std::option::Option::None
				};

			let content = match status {
				SnapshotStatus::Found((_, created_at)) => format!(
					"Showing statistics change from <t:{}:f> to <t:{}:f>",
					created_at.timestamp(),
					::chrono::Utc::now().timestamp(),
				),
				SnapshotStatus::Inserted => format!(
					"No previous data found for **{}**, so it has been inserted.\nShowing statistics change from <t:{}:f> to <t:{}:f>",
					$crate::util::escape_username(&player.username),
					::chrono::Utc::now().timestamp(),
					::chrono::Utc::now().timestamp(),
				),
			};

			ctx.send(move |m| {
				if let ::std::option::Option::Some(png) = png {
					m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
						data: png,
						filename: "canvas.png".into(),
					});
				}

				m.content(content)
			})
			.await?;

			Ok(())
		}
	};
}

generate_command!(arcade::Arcade, arcade::ArcadeMode, arcade);
generate_command!(arena::Arena, arena::ArenaMode, arena);
generate_command!(bed_wars::BedWars, bed_wars::BedWarsMode, bedwars);
generate_command!(blitz_sg::BlitzSg, blitz_sg::BlitzSgMode, blitz);
generate_command!(
	build_battle::BuildBattle,
	build_battle::BuildBattleMode,
	buildbattle
);
generate_command!(
	cops_and_crims::CopsAndCrims,
	cops_and_crims::CopsAndCrimsMode,
	copsandcrims
);
generate_large_command!(duels::Duels, duels::DuelsMode, duels);
generate_command!(mega_walls::MegaWalls, mega_walls::MegaWallsMode, megawalls);
generate_command!(
	murder_mystery::MurderMystery,
	murder_mystery::MurderMysteryMode,
	murdermystery
);
generate_command!(paintball::Paintball, paintball::PaintballMode, paintball);
generate_command!(pit::Pit, pit::PitMode, pit);
generate_command!(quake::Quake, quake::QuakeMode, quake);
generate_command!(sky_wars::SkyWars, sky_wars::SkyWarsMode, skywars);
generate_command!(
	smash_heroes::SmashHeroes,
	smash_heroes::SmashHeroesMode,
	smash
);
generate_command!(speed_uhc::SpeedUhc, speed_uhc::SpeedUhcMode, speeduhc);
generate_command!(tnt_games::TntGames, tnt_games::TntGamesMode, tntgames);
generate_command!(
	turbo_kart_racers::TurboKartRacers,
	turbo_kart_racers::TurboKartRacersMode,
	turkokartracers
);
generate_command!(uhc::Uhc, uhc::UhcMode, uhc);
generate_command!(vampire_z::VampireZ, vampire_z::VampireZMode, vampirez);
generate_command!(walls::Walls, walls::WallsMode, walls);
generate_command!(warlords::Warlords, warlords::WarlordsMode, warlords);
generate_command!(wool_wars::WoolWars, wool_wars::WoolWarsMode, woolwars);

#[poise::command(
	slash_command,
	subcommands(
		"arcade",
		"arena",
		"bedwars",
		"blitz",
		"buildbattle",
		"copsandcrims",
		"duels",
		"megawalls",
		"murdermystery",
		"paintball",
		"pit",
		"quake",
		"skywars",
		"smash",
		"speeduhc",
		"tntgames",
		"turkokartracers",
		"uhc",
		"vampirez",
		"walls",
		"warlords",
		"woolwars"
	)
)]
#[allow(clippy::unused_async)]
pub async fn history(_ctx: Context<'_>) -> Result<(), Error> {
	Ok(())
}
