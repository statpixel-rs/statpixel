use std::{str::FromStr, sync::Arc, time::Duration};

use moka::future::{Cache, CacheBuilder};
use once_cell::sync::Lazy;
use reqwest::{Method, Request, Url};
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::{canvas::label::ToFormatted, game::r#type::Type, http::HTTP, player::stats::Stats};

pub static LEADERBOARD_CACHE: Lazy<Cache<(), Vec<Leaderboard>>> = Lazy::new(|| {
	CacheBuilder::new(1)
		.time_to_idle(Duration::from_secs(60 * 10))
		.time_to_live(Duration::from_secs(60 * 30))
		.build()
});

pub static URL: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/leaderboards").unwrap());

#[derive(Deserialize, Debug)]
pub struct Response {
	#[serde(deserialize_with = "from_leaderboard_map")]
	pub leaderboards: Vec<Leaderboard>,
}

#[derive(Deserialize, Debug, Clone)]
struct RawLeaderboard<'a> {
	path: String,
	prefix: &'a str,
	title: &'a str,
	leaders: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct Leaderboard {
	pub path: String,
	pub name: String,
	pub display_name: String,
	pub leaders: Vec<Uuid>,
	pub game: Type,
}

/// # Errors
/// Returns [`Error::Http`] if the request fails.
pub async fn get() -> Result<Vec<Leaderboard>, Arc<crate::Error>> {
	LEADERBOARD_CACHE.try_get_with((), get_raw()).await
}

async fn get_raw() -> Result<Vec<Leaderboard>, crate::Error> {
	let request = Request::new(Method::GET, URL.clone());
	let response = HTTP
		.perform_hypixel(request.into())
		.await?
		.json::<Response>()
		.await?;

	Ok(response.leaderboards)
}

fn from_leaderboard_map<'de, D>(deserializer: D) -> Result<Vec<Leaderboard>, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor(Vec<Leaderboard>);

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = Vec<Leaderboard>;

		fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.write_str("a mapping of games to their leaderboards")
		}

		fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
		where
			A: serde::de::MapAccess<'de>,
		{
			while let Some((game, boards)) = map.next_entry()? {
				let boards: Vec<RawLeaderboard> = boards;

				self.0.extend(
					boards
						.into_iter()
						.filter(|b| Stats::has_value(&game, b.path.as_str()))
						.map(|mut b| {
							// Only keep the first 10 leaders
							b.leaders.drain(10..);

							let display_name =
								format!("{} ({} {})", game.as_clean_name(), b.prefix, b.title);

							Leaderboard {
								path: b.path,
								display_name,
								name: format!("{} {}", b.prefix, b.title),
								leaders: b.leaders,
								game,
							}
						}),
				);
			}

			self.0.sort_by(|a, b| a.display_name.cmp(&b.display_name));
			self.0.shrink_to_fit();

			Ok(self.0)
		}
	}

	deserializer.deserialize_map(Visitor(vec![]))
}

impl Stats {
	#[must_use]
	pub fn has_value(game: &Type, path: &str) -> bool {
		match game {
			Type::BlitzSg => matches!(path, "wins" | "kills" | "wins_solo_normal" | "wins_teams"),
			Type::SpeedUhc => matches!(path, "kills_normal" | "wins_normal"),
			Type::Paintball | Type::Walls => matches!(path, "kills"),
			Type::MurderMystery => matches!(path, "wins" | "kills"),
			Type::SkyWars => matches!(path, "skywars_exp" | "wins" | "kills"),
			Type::BedWars => matches!(path, "bedwars_level" | "wins_new" | "final_kills_new"),
			Type::TurboKartRacers => matches!(path, "gold_trophy" | "laps_completed"),
			Type::WoolWars => matches!(path, "wool_wars.stats.wins" | "wool_wars.stats.kills"),
			Type::TntGames => matches!(
				path,
				"wins_tntrun" | "wins_pvprun" | "wins_capture" | "wins_tntag" | "wins_bowspleef"
			),
			Type::Uhc => matches!(path, "kills" | "wins"),
			Type::BuildBattle => matches!(path, "score" | "coins" | "wins"),
			Type::MegaWalls => matches!(path, "finalKills"),
			Type::VampireZ => matches!(path, "human_wins"),
			Type::Arcade => matches!(path, "coins"),
			Type::SmashHeroes => matches!(path, "smash_level_total" | "kills"),
			Type::Quake => matches!(path, "kills_total"),
			Type::CopsAndCrims => matches!(path, "normal_kills" | "deathmatch_kills"),
			Type::Warlords => matches!(
				path,
				"wins" | "wins_domination" | "wins_teamdeathmatch" | "wins_capturetheflag"
			),
			Type::Arena => matches!(
				path,
				"arena_rating_b" | "wins_1v1" | "wins_2v2" | "wins_4v4"
			),
			_ => false,
		}
	}

	#[must_use]
	#[allow(clippy::too_many_lines)]
	pub fn get_value(&self, game: &Type, path: &str) -> Option<Box<dyn ToFormatted>> {
		Some(Box::new(match game {
			Type::BlitzSg => match path {
				"wins" => self.blitz_sg.solo.wins + self.blitz_sg.team.wins,
				"kills" => self.blitz_sg.solo.kills + self.blitz_sg.team.kills,
				"wins_solo_normal" => self.blitz_sg.solo.wins,
				"wins_teams" => self.blitz_sg.team.wins,
				_ => return None,
			},
			Type::SpeedUhc => match path {
				"kills_normal" => self.speed_uhc.solo.kills + self.speed_uhc.team.kills,
				"wins_normal" => self.speed_uhc.solo.wins + self.speed_uhc.team.wins,
				_ => return None,
			},
			Type::Paintball => match path {
				"kills" => self.paintball.normal.kills,
				_ => return None,
			},
			Type::MurderMystery => match path {
				"wins" => {
					self.murder_mystery.assassins.wins
						+ self.murder_mystery.classic.wins
						+ self.murder_mystery.double_up.wins
						+ self.murder_mystery.infection.wins
				}
				"kills" => {
					self.murder_mystery.assassins.kills
						+ self.murder_mystery.classic.kills
						+ self.murder_mystery.double_up.kills
						+ self.murder_mystery.infection.kills
				}
				_ => return None,
			},
			Type::SkyWars => match path {
				"skywars_exp" => return Some(Box::new(self.sky_wars.xp)),
				"wins" => {
					self.sky_wars.solo_normal.wins
						+ self.sky_wars.solo_insane.wins
						+ self.sky_wars.team_normal.wins
						+ self.sky_wars.team_insane.wins
				}
				"kills" => {
					self.sky_wars.solo_normal.kills
						+ self.sky_wars.solo_insane.kills
						+ self.sky_wars.team_normal.kills
						+ self.sky_wars.team_insane.kills
				}
				_ => return None,
			},
			Type::BedWars => match path {
				"bedwars_level" => {
					return Some(Box::new(
						minecraft::calc::bed_wars::get_level(self.bed_wars.xp).0,
					))
				}
				"wins_new" => {
					self.bed_wars.solo.wins
						+ self.bed_wars.double.wins
						+ self.bed_wars.three.wins
						+ self.bed_wars.four.wins
				}
				"final_kills_new" => {
					self.bed_wars.solo.final_kills
						+ self.bed_wars.double.final_kills
						+ self.bed_wars.three.final_kills
						+ self.bed_wars.four.final_kills
				}
				_ => return None,
			},
			Type::TurboKartRacers => match path {
				"gold_trophy" => self.turbo_kart_racers.normal.gold_trophies,
				"laps_completed" => self.turbo_kart_racers.laps_completed,
				_ => return None,
			},
			Type::WoolWars => match path {
				"wool_wars.stats.wins" => self.wool_wars.inner.stats.normal.wins,
				"wool_wars.stats.kills" => self.wool_wars.inner.stats.normal.kills,
				_ => return None,
			},
			Type::TntGames => match path {
				"wins_tntrun" => self.tnt_games.tnt_run.wins,
				"wins_pvprun" => self.tnt_games.pvp_run.wins,
				"wins_capture" => self.tnt_games.wizard.wins,
				"wins_tntag" => self.tnt_games.tnt_tag.wins,
				"wins_bowspleef" => self.tnt_games.bow_spleef.wins,
				_ => return None,
			},
			Type::Uhc => {
				match path {
					"kills" => {
						self.uhc.brawl.kills
							+ self.uhc.double_brawl.kills
							+ self.uhc.no_diamonds.kills + self.uhc.red_vs_blue.kills
							+ self.uhc.solo.kills + self.uhc.solo_brawl.kills
							+ self.uhc.team.kills + self.uhc.vanilla_double.kills
					}
					"wins" => {
						self.uhc.brawl.wins
							+ self.uhc.double_brawl.wins + self.uhc.no_diamonds.wins
							+ self.uhc.red_vs_blue.wins + self.uhc.solo.wins
							+ self.uhc.solo_brawl.wins + self.uhc.team.wins
							+ self.uhc.vanilla_double.wins
					}
					_ => return None,
				}
			}
			Type::BuildBattle => match path {
				"score" => self.build_battle.score,
				"coins" => return Some(Box::new(self.build_battle.coins)),
				"wins" => {
					self.build_battle.guess_the_build.wins
						+ self.build_battle.solo_normal.wins
						+ self.build_battle.solo_pro.wins
						+ self.build_battle.teams_normal.wins
				}
				_ => return None,
			},
			Type::MegaWalls => match path {
				"finalKills" => {
					self.mega_walls.brawl.final_kills
						+ self.mega_walls.face_off.final_kills
						+ self.mega_walls.normal.final_kills
				}
				_ => return None,
			},
			Type::VampireZ => match path {
				"human_wins" => self.vampire_z.normal.human_wins,
				_ => return None,
			},
			Type::Arcade => match path {
				"coins" => return Some(Box::new(self.arcade.coins)),
				_ => return None,
			},
			Type::Walls => match path {
				"kills" => self.walls.standard.kills,
				_ => return None,
			},
			Type::SmashHeroes => match path {
				"smash_level_total" => self.smash_heroes.level,
				"kills" => {
					self.smash_heroes.solo.kills
						+ self.smash_heroes.team.kills
						+ self.smash_heroes.team.kills
				}
				_ => return None,
			},
			Type::Quake => match path {
				"kills_total" => {
					self.quake.solo.kills + self.quake.team.kills + self.quake.solo_tournament.kills
				}
				_ => return None,
			},
			Type::CopsAndCrims => match path {
				"normal_kills" => self.cops_and_crims.defusal.kills,
				"deathmatch_kills" => self.cops_and_crims.deathmatch.kills,
				_ => return None,
			},
			Type::Warlords => match path {
				"wins" => {
					self.warlords.capture_the_flag.wins_blue
						+ self.warlords.capture_the_flag.wins_red
						+ self.warlords.team_deathmatch.wins_blue
						+ self.warlords.team_deathmatch.wins_red
						+ self.warlords.domination.wins_blue
						+ self.warlords.domination.wins_red
				}
				"wins_domination" => {
					self.warlords.domination.wins_blue + self.warlords.domination.wins_red
				}
				"wins_teamdeathmatch" => {
					self.warlords.team_deathmatch.wins_blue + self.warlords.team_deathmatch.wins_red
				}
				"wins_capturetheflag" => {
					self.warlords.capture_the_flag.wins_blue
						+ self.warlords.capture_the_flag.wins_red
				}
				_ => return None,
			},
			Type::Arena => match path {
				"arena_rating_b" => self.arena.rating,
				"wins_1v1" => self.arena.solo.wins,
				"wins_2v2" => self.arena.double.wins,
				"wins_4v4" => self.arena.four.wins,
				_ => return None,
			},
			_ => return None,
		}))
	}
}
