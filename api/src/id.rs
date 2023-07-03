#[allow(clippy::wildcard_imports)]
use crate::player::stats::*;

use base64::Engine;
use macros::GetTr;
use poise::serenity_prelude as serenity;
use translate::{context, tr};
use uuid::Uuid;

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, GetTr)]
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

impl SkyBlockMode {
	#[allow(clippy::too_many_lines)]
	pub fn as_root(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		profile: Option<Uuid>,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String {
				options: vec![
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Auctions"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Auctions, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Bank"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Bank, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Candy"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Candy, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "EnderChest"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::EnderChest, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Equipment"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Equipment, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Fishing"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Fishing, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Inventory"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Inventory, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Networth"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Networth, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Pets"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Pets, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Potions"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Potions, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Profile"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Profile, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Quiver"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Quiver, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Talisman"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Talisman, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Vault"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Vault, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Wardrobe"),
						encode(Id::Root {
							kind: Mode::SkyBlock(Self::Wardrobe, profile),
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

	#[allow(clippy::too_many_lines)]
	pub fn as_snapshot(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		profile: Option<Uuid>,
		past: i64,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String {
				options: vec![
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Auctions"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Auctions, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Bank"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Bank, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Candy"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Candy, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "EnderChest"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::EnderChest, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Equipment"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Equipment, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Fishing"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Fishing, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Inventory"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Inventory, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Networth"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Networth, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Pets"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Pets, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Potions"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Potions, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Profile"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Profile, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Quiver"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Quiver, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Talisman"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Talisman, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Vault"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Vault, profile),
							uuid,
							past,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Wardrobe"),
						encode(Id::Snapshot {
							kind: Mode::SkyBlock(Self::Wardrobe, profile),
							uuid,
							past,
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

	#[allow(clippy::too_many_lines)]
	pub fn as_history(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		profile: Option<Uuid>,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String {
				options: vec![
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Auctions"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Auctions, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Bank"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Bank, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Candy"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Candy, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "EnderChest"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::EnderChest, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Equipment"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Equipment, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Fishing"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Fishing, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Inventory"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Inventory, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Networth"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Networth, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Pets"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Pets, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Potions"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Potions, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Profile"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Profile, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Quiver"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Quiver, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Talisman"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Talisman, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Vault"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Vault, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Wardrobe"),
						encode(Id::History {
							kind: Mode::SkyBlock(Self::Wardrobe, profile),
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

	#[allow(clippy::too_many_lines)]
	pub fn as_project(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		profile: Option<Uuid>,
		kind: SkyBlockKind,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String {
				options: vec![
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Auctions"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Auctions, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Bank"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Bank, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Candy"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Candy, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "EnderChest"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::EnderChest, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Equipment"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Equipment, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Fishing"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Fishing, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Inventory"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Inventory, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Networth"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Networth, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Pets"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Pets, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Potions"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Potions, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Profile"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Profile, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Quiver"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Quiver, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Talisman"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Talisman, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Vault"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Vault, kind, profile),
							uuid,
						}),
					),
					serenity::CreateSelectMenuOption::new(
						tr!(ctx, "Wardrobe"),
						encode(Id::Project {
							kind: ProjectMode::SkyBlock(Self::Wardrobe, kind, profile),
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

impl GuildMode {
	pub fn as_root(
		ctx: &context::Context<'_>,
		uuid: Uuid,
		limit: Option<usize>,
		past_nanos: Option<i64>,
		member: Option<Uuid>,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut options = Vec::with_capacity(if member.is_some() { 4 } else { 3 });

		options.push(serenity::CreateSelectMenuOption::new(
			tr!(ctx, "General"),
			encode(Id::Root {
				kind: Mode::Guild(Self::General, limit, past_nanos, member),
				uuid,
			}),
		));

		if member.is_some() {
			options.push(serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Member"),
				encode(Id::Root {
					kind: Mode::Guild(Self::Member, limit, past_nanos, member),
					uuid,
				}),
			));
		}

		options.extend([
			serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Members"),
				encode(Id::Root {
					kind: Mode::Guild(Self::Members, limit, past_nanos, member),
					uuid,
				}),
			),
			serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Top"),
				encode(Id::Root {
					kind: Mode::Guild(Self::Top, limit, past_nanos, member),
					uuid,
				}),
			),
		]);

		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String { options },
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
		past: i64,
		limit: Option<usize>,
		past_nanos: Option<i64>,
		member: Option<Uuid>,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut options = Vec::with_capacity(if member.is_some() { 4 } else { 3 });

		options.push(serenity::CreateSelectMenuOption::new(
			tr!(ctx, "General"),
			encode(Id::Snapshot {
				kind: Mode::Guild(Self::General, limit, past_nanos, member),
				uuid,
				past,
			}),
		));

		if member.is_some() {
			options.push(serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Member"),
				encode(Id::Snapshot {
					kind: Mode::Guild(Self::Member, limit, past_nanos, member),
					uuid,
					past,
				}),
			));
		}

		options.extend([
			serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Members"),
				encode(Id::Snapshot {
					kind: Mode::Guild(Self::Members, limit, past_nanos, member),
					uuid,
					past,
				}),
			),
			serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Top"),
				encode(Id::Snapshot {
					kind: Mode::Guild(Self::Top, limit, past_nanos, member),
					uuid,
					past,
				}),
			),
		]);

		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String { options },
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
		limit: Option<usize>,
		past_nanos: Option<i64>,
		member: Option<Uuid>,
		selected: Option<Self>,
	) -> serenity::CreateActionRow {
		let mut options = Vec::with_capacity(if member.is_some() { 4 } else { 3 });

		options.push(serenity::CreateSelectMenuOption::new(
			tr!(ctx, "General"),
			encode(Id::History {
				kind: Mode::Guild(Self::General, limit, past_nanos, member),
				uuid,
			}),
		));

		if member.is_some() {
			options.push(serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Member"),
				encode(Id::History {
					kind: Mode::Guild(Self::Member, limit, past_nanos, member),
					uuid,
				}),
			));
		}

		options.extend([
			serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Members"),
				encode(Id::History {
					kind: Mode::Guild(Self::Members, limit, past_nanos, member),
					uuid,
				}),
			),
			serenity::CreateSelectMenuOption::new(
				tr!(ctx, "Top"),
				encode(Id::History {
					kind: Mode::Guild(Self::Top, limit, past_nanos, member),
					uuid,
				}),
			),
		]);

		let mut menu = serenity::CreateSelectMenu::new(
			ctx.id().to_string(),
			serenity::CreateSelectMenuKind::String { options },
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

	// profile id
	SkyBlock(SkyBlockMode, #[bincode(with_serde)] Option<Uuid>),
	// `top.limit`, `top.past` as nanos
	Guild(
		GuildMode,
		Option<usize>,
		Option<i64>,
		#[bincode(with_serde)] Option<Uuid>,
	),
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
	SkyBlock(
		SkyBlockMode,
		SkyBlockKind,
		#[bincode(with_serde)] Option<Uuid>,
	),
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
		// duration in nanos
		past: i64,
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
