use macros::{Game, Mode};
use serde::{Deserialize, Serialize};

use crate::seconds::Seconds;

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq)]
#[serde(default)]
#[game(
	path = "blitz_sg",
	pretty = "§c§lBlitz",
	field(ident = "wins", colour = "green"),
	field(ident = "games_played", colour = "red"),
	field(
		tr = "wr",
		ident = "wins",
		div = "games_played",
		colour = "gold",
		percent
	),
	field(ident = "damage_dealt", colour = "green"),
	field(ident = "damage_taken", colour = "red"),
	field(
		tr = "ddtr",
		ident = "damage_dealt",
		div = "damage_taken",
		colour = "gold"
	),
	label(ident = "potions_drunk", colour = "green"),
	label(ident = "chests_opened", colour = "red"),
	label(ident = "time_played", colour = "gold"),
	label(
		colour = "red",
		ident = "arrows_hit",
		div = "arrows_shot",
		percent,
		tr = "bow-accuracy"
	)
)]
pub struct BlitzSg {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub armorer: Armorer,
	#[serde(flatten)]
	#[game(mode())]
	pub scout: Scout,
	#[serde(flatten)]
	#[game(mode())]
	pub speleologist: Speleologist,
	#[serde(flatten)]
	#[game(mode())]
	pub random: Random,
	#[serde(flatten)]
	#[game(mode())]
	pub rogue: Rogue,
	#[serde(flatten)]
	#[game(mode())]
	pub rambo: Rambo,
	#[serde(flatten)]
	#[game(mode())]
	pub troll: Troll,
	#[serde(flatten)]
	#[game(mode())]
	pub horsetamer: HorseTamer,
	#[serde(flatten)]
	#[game(mode())]
	pub wolftamer: WolfTamer,
	#[serde(flatten)]
	#[game(mode())]
	pub warrior: Warrior,
	#[serde(flatten)]
	#[game(mode())]
	pub phoenix: Phoenix,
	#[serde(flatten)]
	#[game(mode())]
	pub donkeytamer: DonkeyTamer,
	#[serde(flatten)]
	#[game(mode())]
	pub ranger: Ranger,
	#[serde(flatten)]
	#[game(mode())]
	pub archer: Archer,
	#[serde(flatten)]
	#[game(mode())]
	pub necromancer: Necromancer,
	#[serde(flatten)]
	#[game(mode())]
	pub meatmaster: Meatmaster,
	#[serde(flatten)]
	#[game(mode())]
	pub tim: Tim,
	#[serde(flatten)]
	#[game(mode())]
	pub pigman: Pigman,
	#[serde(flatten)]
	#[game(mode())]
	pub creepertamer: CreeperTamer,
	#[serde(flatten)]
	#[game(mode())]
	pub florist: Florist,
	#[serde(flatten)]
	#[game(mode())]
	pub warlock: Warlock,
	#[serde(flatten)]
	#[game(mode())]
	pub milkman: Milkman,
	#[serde(flatten)]
	#[game(mode())]
	pub astronaut: Astronaut,
	#[serde(flatten)]
	#[game(mode())]
	pub blaze: Blaze,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Armorer {
	#[serde(rename = "wins_armorer")]
	pub wins: u32,
	#[serde(rename = "games_played_armorer")]
	pub games_played: u32,
	#[serde(rename = "damage_armorer")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_armorer")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_armorer")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_armorer")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_armorer")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_armorer")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_armorer")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Scout {
	#[serde(rename = "wins_scout")]
	pub wins: u32,
	#[serde(rename = "games_played_scout")]
	pub games_played: u32,
	#[serde(rename = "damage_scout")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_scout")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_scout")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_scout")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_scout")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_scout")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_scout")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Speleologist {
	#[serde(rename = "wins_speleologist")]
	pub wins: u32,
	#[serde(rename = "games_played_speleologist")]
	pub games_played: u32,
	#[serde(rename = "damage_speleologist")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_speleologist")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_speleologist")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_speleologist")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_speleologist")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_speleologist")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_speleologist")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Random {
	#[serde(rename = "wins_random")]
	pub wins: u32,
	#[serde(rename = "games_played_random")]
	pub games_played: u32,
	#[serde(rename = "damage_random")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_random")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_random")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_random")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_random")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_random")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_random")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Rogue {
	#[serde(rename = "wins_rogue")]
	pub wins: u32,
	#[serde(rename = "games_played_rogue")]
	pub games_played: u32,
	#[serde(rename = "damage_rogue")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_rogue")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_rogue")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_rogue")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_rogue")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_rogue")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_rogue")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Rambo {
	#[serde(rename = "wins_rambo")]
	pub wins: u32,
	#[serde(rename = "games_played_rambo")]
	pub games_played: u32,
	#[serde(rename = "damage_rambo")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_rambo")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_rambo")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_rambo")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_rambo")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_rambo")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_rambo")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Troll {
	#[serde(rename = "wins_troll")]
	pub wins: u32,
	#[serde(rename = "games_played_troll")]
	pub games_played: u32,
	#[serde(rename = "damage_troll")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_troll")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_troll")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_troll")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_troll")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_troll")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_troll")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct HorseTamer {
	#[serde(rename = "wins_horsetamer")]
	pub wins: u32,
	#[serde(rename = "games_played_horsetamer")]
	pub games_played: u32,
	#[serde(rename = "damage_horsetamer")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_horsetamer")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_horsetamer")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_horsetamer")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_horsetamer")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_horsetamer")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_horsetamer")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct WolfTamer {
	#[serde(rename = "wins_wolftamer")]
	pub wins: u32,
	#[serde(rename = "games_played_wolftamer")]
	pub games_played: u32,
	#[serde(rename = "damage_wolftamer")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_wolftamer")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_wolftamer")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_wolftamer")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_wolftamer")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_wolftamer")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_wolftamer")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Warrior {
	#[serde(rename = "wins_warrior")]
	pub wins: u32,
	#[serde(rename = "games_played_warrior")]
	pub games_played: u32,
	#[serde(rename = "damage_warrior")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_warrior")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_warrior")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_warrior")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_warrior")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_warrior")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_warrior")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Phoenix {
	#[serde(rename = "wins_phoenix")]
	pub wins: u32,
	#[serde(rename = "games_played_phoenix")]
	pub games_played: u32,
	#[serde(rename = "damage_phoenix")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_phoenix")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_phoenix")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_phoenix")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_phoenix")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_phoenix")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_phoenix")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct DonkeyTamer {
	#[serde(rename = "wins_donkeytamer")]
	pub wins: u32,
	#[serde(rename = "games_played_donkeytamer")]
	pub games_played: u32,
	#[serde(rename = "damage_donkeytamer")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_donkeytamer")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_donkeytamer")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_donkeytamer")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_donkeytamer")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_donkeytamer")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_donkeytamer")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Ranger {
	#[serde(rename = "wins_ranger")]
	pub wins: u32,
	#[serde(rename = "games_played_ranger")]
	pub games_played: u32,
	#[serde(rename = "damage_ranger")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_ranger")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_ranger")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_ranger")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_ranger")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_ranger")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_ranger")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Archer {
	#[serde(rename = "wins_archer")]
	pub wins: u32,
	#[serde(rename = "games_played_archer")]
	pub games_played: u32,
	#[serde(rename = "damage_archer")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_archer")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_archer")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_archer")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_archer")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_archer")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_archer")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Necromancer {
	#[serde(rename = "wins_necromancer")]
	pub wins: u32,
	#[serde(rename = "games_played_necromancer")]
	pub games_played: u32,
	#[serde(rename = "damage_necromancer")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_necromancer")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_necromancer")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_necromancer")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_necromancer")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_necromancer")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_necromancer")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Meatmaster {
	#[serde(rename = "wins_meatmaster")]
	pub wins: u32,
	#[serde(rename = "games_played_meatmaster")]
	pub games_played: u32,
	#[serde(rename = "damage_meatmaster")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_meatmaster")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_meatmaster")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_meatmaster")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_meatmaster")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_meatmaster")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_meatmaster")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Tim {
	#[serde(rename = "wins_tim")]
	pub wins: u32,
	#[serde(rename = "games_played_tim")]
	pub games_played: u32,
	#[serde(rename = "damage_tim")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_tim")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_tim")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_tim")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_tim")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_tim")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_tim")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Pigman {
	#[serde(rename = "wins_pigman")]
	pub wins: u32,
	#[serde(rename = "games_played_pigman")]
	pub games_played: u32,
	#[serde(rename = "damage_pigman")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_pigman")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_pigman")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_pigman")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_pigman")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_pigman")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_pigman")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct CreeperTamer {
	#[serde(rename = "wins_creepertamer")]
	pub wins: u32,
	#[serde(rename = "games_played_creepertamer")]
	pub games_played: u32,
	#[serde(rename = "damage_creepertamer")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_creepertamer")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_creepertamer")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_creepertamer")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_creepertamer")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_creepertamer")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_creepertamer")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Florist {
	#[serde(rename = "wins_florist")]
	pub wins: u32,
	#[serde(rename = "games_played_florist")]
	pub games_played: u32,
	#[serde(rename = "damage_florist")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_florist")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_florist")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_florist")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_florist")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_florist")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_florist")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Warlock {
	#[serde(rename = "wins_warlock")]
	pub wins: u32,
	#[serde(rename = "games_played_warlock")]
	pub games_played: u32,
	#[serde(rename = "damage_warlock")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_warlock")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_warlock")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_warlock")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_warlock")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_warlock")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_warlock")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Milkman {
	#[serde(rename = "wins_milkman")]
	pub wins: u32,
	#[serde(rename = "games_played_milkman")]
	pub games_played: u32,
	#[serde(rename = "damage_milkman")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_milkman")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_milkman")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_milkman")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_milkman")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_milkman")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_milkman")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Astronaut {
	#[serde(rename = "wins_astronaut")]
	pub wins: u32,
	#[serde(rename = "games_played_astronaut")]
	pub games_played: u32,
	#[serde(rename = "damage_astronaut")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_astronaut")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_astronaut")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_astronaut")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_astronaut")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_astronaut")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_astronaut")]
	pub arrows_hit: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Mode)]
#[serde(default)]
pub struct Blaze {
	#[serde(rename = "wins_blaze")]
	pub wins: u32,
	#[serde(rename = "games_played_blaze")]
	pub games_played: u32,
	#[serde(rename = "damage_blaze")]
	pub damage_dealt: u32,
	#[serde(rename = "damage_taken_blaze")]
	pub damage_taken: u32,
	#[serde(rename = "potions_drunk_blaze")]
	pub potions_drunk: u32,
	#[serde(rename = "chests_opened_blaze")]
	pub chests_opened: u32,
	#[serde(rename = "time_played_blaze")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_fired_blaze")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_blaze")]
	pub arrows_hit: u32,
}
