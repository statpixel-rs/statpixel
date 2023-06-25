#[allow(clippy::wildcard_imports)]
use crate::player::stats::*;

use crate::macros::ModeTrait;

use base64::Engine;
use chrono::{DateTime, Utc};
use macros::GetTr;
use poise::serenity_prelude as serenity;
use translate::{context, tr};
use uuid::Uuid;

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, ModeTrait)]
#[mode(kind = "SkyBlockKind", rename = "SkyBlock")]
pub enum SkyBlockMode {
	Auctions,
	Bank,
	Candy,
	EnderChest,
	Equipment,
	Fishing,
	Inventory,
	Networth,
	Pets,
	Potions,
	Profile,
	Quiver,
	Talisman,
	Vault,
	Wardrobe,
}

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, GetTr)]
pub enum GuildMode {
	General,
	Member,
	Members,
	Top,
}

impl GuildMode {
	pub fn as_root(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String {
				options: vec![
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "General"),
						encode(Id::Root {
							kind: Mode::Guild(Self::General),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Members"),
						encode(Id::Root {
							kind: Mode::Guild(Self::Members),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Top"),
						encode(Id::Root {
							kind: Mode::Guild(Self::Top),
							uuid,
						}),
					),
				],
			},
		)
		.min_values(1)
		.max_values(1);

		if let Some(selected) = selected {
			menu = menu.placeholder(tr!(ctx, selected.get_tr()));
		}

		serenity::CreateActionRow::SelectMenu(menu)
	}

	pub fn as_snapshot(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		from: DateTime<Utc>,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String {
				options: vec![
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "General"),
						encode(Id::Snapshot {
							kind: Mode::Guild(Self::General),
							uuid,
							from,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Members"),
						encode(Id::Snapshot {
							kind: Mode::Guild(Self::Members),
							uuid,
							from,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Top"),
						encode(Id::Snapshot {
							kind: Mode::Guild(Self::Top),
							uuid,
							from,
						}),
					),
				],
			},
		)
		.min_values(1)
		.max_values(1);

		if let Some(selected) = selected {
			menu = menu.placeholder(tr!(ctx, selected.get_tr()));
		}

		serenity::CreateActionRow::SelectMenu(menu)
	}

	pub fn as_history(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String {
				options: vec![
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "General"),
						encode(Id::History {
							kind: Mode::Guild(Self::General),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Members"),
						encode(Id::History {
							kind: Mode::Guild(Self::Members),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Top"),
						encode(Id::History {
							kind: Mode::Guild(Self::Top),
							uuid,
						}),
					),
				],
			},
		)
		.min_values(1)
		.max_values(1);

		if let Some(selected) = selected {
			menu = menu.placeholder(tr!(ctx, selected.get_tr()));
		}

		serenity::CreateActionRow::SelectMenu(menu)
	}

	pub fn as_project(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		kind: GuildKind,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String {
				options: vec![
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "General"),
						encode(Id::Project {
							kind: ProjectMode::Guild(Self::General, kind),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Members"),
						encode(Id::Project {
							kind: ProjectMode::Guild(Self::Members, kind),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Top"),
						encode(Id::Project {
							kind: ProjectMode::Guild(Self::Top, kind),
							uuid,
						}),
					),
				],
			},
		)
		.min_values(1)
		.max_values(1);

		if let Some(selected) = selected {
			menu = menu.placeholder(tr!(ctx, selected.get_tr()));
		}

		serenity::CreateActionRow::SelectMenu(menu)
	}
}

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, Default)]
pub enum GuildKind {
	#[default]
	None,
}

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, Default)]
pub enum SkyBlockKind {
	#[default]
	None,
}

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub enum Mode {
	Arcade(arcade::ArcadeMode),
	Arena(arena::ArenaMode),
	BedWars(bed_wars::BedWarsMode),
	BlitzSg(blitz_sg::BlitzSgMode),
	BuildBattle(build_battle::BuildBattleMode),
	CopsAndCrims(cops_and_crims::CopsAndCrimsMode),
	Duels(duels::DuelsMode),
	MegaWalls(mega_walls::MegaWallsMode),
	MurderMystery(murder_mystery::MurderMysteryMode),
	Paintball(paintball::PaintballMode),
	Pit(pit::PitMode),
	Quake(quake::QuakeMode),
	SkyWars(sky_wars::SkyWarsMode),
	SmashHeroes(smash_heroes::SmashHeroesMode),
	SpeedUhc(speed_uhc::SpeedUhcMode),
	TntGames(tnt_games::TntGamesMode),
	TurboKartRacers(turbo_kart_racers::TurboKartRacersMode),
	Uhc(uhc::UhcMode),
	VampireZ(vampire_z::VampireZMode),
	Walls(walls::WallsMode),
	Warlords(warlords::WarlordsMode),
	WoolWars(wool_wars::WoolWarsMode),

	SkyBlock(SkyBlockMode),
	Guild(GuildMode),
	Network,
}

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub enum ProjectMode {
	Arcade(arcade::ArcadeMode, arcade::ArcadeKind),
	Arena(arena::ArenaMode, arena::ArenaKind),
	BedWars(bed_wars::BedWarsMode, bed_wars::BedWarsKind),
	BlitzSg(blitz_sg::BlitzSgMode, blitz_sg::BlitzSgKind),
	BuildBattle(build_battle::BuildBattleMode, build_battle::BuildBattleKind),
	CopsAndCrims(
		cops_and_crims::CopsAndCrimsMode,
		cops_and_crims::CopsAndCrimsKind,
	),
	Duels(duels::DuelsMode, duels::DuelsKind),
	MegaWalls(mega_walls::MegaWallsMode, mega_walls::MegaWallsKind),
	MurderMystery(
		murder_mystery::MurderMysteryMode,
		murder_mystery::MurderMysteryKind,
	),
	Paintball(paintball::PaintballMode, paintball::PaintballKind),
	Pit(pit::PitMode, pit::PitKind),
	Quake(quake::QuakeMode, quake::QuakeKind),
	SkyWars(sky_wars::SkyWarsMode, sky_wars::SkyWarsKind),
	SmashHeroes(smash_heroes::SmashHeroesMode, smash_heroes::SmashHeroesKind),
	SpeedUhc(speed_uhc::SpeedUhcMode, speed_uhc::SpeedUhcKind),
	TntGames(tnt_games::TntGamesMode, tnt_games::TntGamesKind),
	TurboKartRacers(
		turbo_kart_racers::TurboKartRacersMode,
		turbo_kart_racers::TurboKartRacersKind,
	),
	Uhc(uhc::UhcMode, uhc::UhcKind),
	VampireZ(vampire_z::VampireZMode, vampire_z::VampireZKind),
	Walls(walls::WallsMode, walls::WallsKind),
	Warlords(warlords::WarlordsMode, warlords::WarlordsKind),
	WoolWars(wool_wars::WoolWarsMode, wool_wars::WoolWarsKind),

	Guild(GuildMode, GuildKind),
	SkyBlock(SkyBlockMode, SkyBlockKind),
}

/// The structure of a button's `custom_id`
#[derive(bincode::Encode, bincode::Decode, Debug)]
pub enum Id {
	/// A root command, like `/guild`, etc.
	Root {
		kind: Mode,
		#[bincode(with_serde)]
		uuid: Uuid,
	},
	/// A `/from`, `/daily`, `/weekly`, and `/monthly` command
	Snapshot {
		kind: Mode,
		#[bincode(with_serde)]
		uuid: Uuid,
		#[bincode(with_serde)]
		from: DateTime<Utc>,
	},
	/// A `/history` command
	History {
		kind: Mode,
		#[bincode(with_serde)]
		uuid: Uuid,
	},
	/// A `/project` command
	Project {
		kind: ProjectMode,
		#[bincode(with_serde)]
		uuid: Uuid,
	},
}

/// # Panics
/// Panics if the data cannot be encoded
#[must_use]
pub fn encode(id: Id) -> String {
	let bytes = bincode::encode_to_vec(id, bincode::config::standard()).unwrap();

	base64::engine::general_purpose::STANDARD_NO_PAD.encode(bytes)
}

#[must_use]
pub fn decode(id: &str) -> Option<Id> {
	let bytes = base64::engine::general_purpose::STANDARD_NO_PAD
		.decode(id.as_bytes())
		.ok()?;

	bincode::decode_from_slice(&bytes, bincode::config::standard())
		.map(|o| o.0)
		.ok()
}
