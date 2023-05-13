use macros::Game;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Game, PartialEq)]
#[game(
	path = "duels",
	pretty = "§b§lDuels",
	field(ident = "wins", colour = "green"),
	field(ident = "losses", colour = "red"),
	field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	label(
		colour = "red",
		ident = "arrows_hit",
		div = "arrows_shot",
		percent,
		tr = "bow-accuracy"
	),
	label(
		colour = "aqua",
		ident = "melee_hits",
		div = "melee_swings",
		percent,
		tr = "melee-accuracy"
	),
	label(
		colour = "green",
		ident = "health_regenerated",
		tr = "health-regenerated"
	),
	label(colour = "red", ident = "damage_dealt", tr = "damage-dealt")
)]
#[serde(default)]
pub struct Duels {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub uhc_solo: UhcSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub op_solo: OpSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub uhc_double: UhcDouble,
	#[serde(flatten)]
	#[game(mode())]
	pub bow_solo: BowSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub classic_solo: ClassicSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub op_double: OpDouble,
	#[serde(flatten)]
	#[game(mode())]
	pub uhc_four: UhcFour,
	#[serde(flatten)]
	#[game(mode())]
	pub sky_wars_double: SkyWarsDouble,
	#[serde(flatten)]
	#[game(mode())]
	pub sumo_solo: SumoSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub sky_wars_solo: SkyWarsSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub bridge_double_duel: BridgeDoubleDuel,
	#[serde(flatten)]
	#[game(mode())]
	pub bridge_four: BridgeFourDuel,
	#[serde(flatten)]
	#[game(mode())]
	pub bridge_solo: BridgeSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub bridge_three: BridgeThree,
	#[serde(flatten)]
	#[game(mode())]
	pub bridge_double: BridgeDouble,
	#[serde(flatten)]
	#[game(mode())]
	pub combo_solo: ComboSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub sumo_tournament: SumoTournament,
	#[serde(flatten)]
	#[game(mode())]
	pub sky_wars_tournament: SkyWarsTournament,
	#[serde(flatten)]
	#[game(mode())]
	pub uhc_meetup: UhcMeetup,
	#[serde(flatten)]
	#[game(mode())]
	pub potion_solo: PotionSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub blitz_solo: BlitzSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub bow_spleef_solo: BowSpleefSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub mega_walls_solo: MegaWallsSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub boxing_solo: BoxingSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub parkour: Parkour,
	#[serde(flatten)]
	#[game(mode())]
	pub arena_solo: ArenaSolo,
	#[serde(flatten)]
	#[game(mode())]
	pub capture_three: CaptureThree,
	#[serde(flatten)]
	#[game(mode())]
	pub bridge_three_duel: BridgeThreeDuel,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct UhcSolo {
	#[serde(rename = "uhc_duel_wins")]
	pub wins: u32,
	#[serde(rename = "uhc_duel_losses")]
	pub losses: u32,
	#[serde(rename = "uhc_duel_kills")]
	pub kills: u32,
	#[serde(rename = "uhc_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "uhc_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "uhc_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "uhc_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "uhc_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "uhc_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "uhc_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct OpSolo {
	#[serde(rename = "op_duel_wins")]
	pub wins: u32,
	#[serde(rename = "op_duel_losses")]
	pub losses: u32,
	#[serde(rename = "op_duel_kills")]
	pub kills: u32,
	#[serde(rename = "op_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "op_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "op_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "op_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "op_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "op_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "op_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct UhcDouble {
	#[serde(rename = "uhc_doubles_wins")]
	pub wins: u32,
	#[serde(rename = "uhc_doubles_losses")]
	pub losses: u32,
	#[serde(rename = "uhc_doubles_kills")]
	pub kills: u32,
	#[serde(rename = "uhc_doubles_deaths")]
	pub deaths: u32,
	#[serde(rename = "uhc_doubles_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "uhc_doubles_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "uhc_doubles_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "uhc_doubles_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "uhc_doubles_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "uhc_doubles_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BowSolo {
	#[serde(rename = "bow_duel_wins")]
	pub wins: u32,
	#[serde(rename = "bow_duel_losses")]
	pub losses: u32,
	#[serde(rename = "bow_duel_kills")]
	pub kills: u32,
	#[serde(rename = "bow_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "bow_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "bow_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "bow_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "bow_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "bow_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "bow_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ClassicSolo {
	#[serde(rename = "classic_duel_wins")]
	pub wins: u32,
	#[serde(rename = "classic_duel_losses")]
	pub losses: u32,
	#[serde(rename = "classic_duel_kills")]
	pub kills: u32,
	#[serde(rename = "classic_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "classic_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "classic_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "classic_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "classic_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "classic_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "classic_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct OpDouble {
	#[serde(rename = "op_doubles_wins")]
	pub wins: u32,
	#[serde(rename = "op_doubles_losses")]
	pub losses: u32,
	#[serde(rename = "op_doubles_kills")]
	pub kills: u32,
	#[serde(rename = "op_doubles_deaths")]
	pub deaths: u32,
	#[serde(rename = "op_doubles_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "op_doubles_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "op_doubles_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "op_doubles_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "op_doubles_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "op_doubles_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct UhcFour {
	#[serde(rename = "uhc_four_wins")]
	pub wins: u32,
	#[serde(rename = "uhc_four_losses")]
	pub losses: u32,
	#[serde(rename = "uhc_four_kills")]
	pub kills: u32,
	#[serde(rename = "uhc_four_deaths")]
	pub deaths: u32,
	#[serde(rename = "uhc_four_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "uhc_four_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "uhc_four_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "uhc_four_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "uhc_four_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "uhc_four_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SkyWarsDouble {
	#[serde(rename = "sw_doubles_wins")]
	pub wins: u32,
	#[serde(rename = "sw_doubles_losses")]
	pub losses: u32,
	#[serde(rename = "sw_doubles_kills")]
	pub kills: u32,
	#[serde(rename = "sw_doubles_deaths")]
	pub deaths: u32,
	#[serde(rename = "sw_doubles_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "sw_doubles_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "sw_doubles_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "sw_doubles_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "sw_doubles_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "sw_doubles_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SumoSolo {
	#[serde(rename = "sumo_duel_wins")]
	pub wins: u32,
	#[serde(rename = "sumo_duel_losses")]
	pub losses: u32,
	#[serde(rename = "sumo_duel_kills")]
	pub kills: u32,
	#[serde(rename = "sumo_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "sumo_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "sumo_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "sumo_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "sumo_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "sumo_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "sumo_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SkyWarsSolo {
	#[serde(rename = "sw_duel_wins")]
	pub wins: u32,
	#[serde(rename = "sw_duel_losses")]
	pub losses: u32,
	#[serde(rename = "sw_duel_kills")]
	pub kills: u32,
	#[serde(rename = "sw_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "sw_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "sw_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "sw_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "sw_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "sw_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "sw_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BridgeDoubleDuel {
	#[serde(rename = "bridge_doubles_wins")]
	pub wins: u32,
	#[serde(rename = "bridge_doubles_losses")]
	pub losses: u32,
	#[serde(rename = "bridge_doubles_kills")]
	pub kills: u32,
	#[serde(rename = "bridge_doubles_deaths")]
	pub deaths: u32,
	#[serde(rename = "bridge_doubles_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "bridge_doubles_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "bridge_doubles_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "bridge_doubles_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "bridge_doubles_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "bridge_doubles_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BridgeFourDuel {
	#[serde(rename = "bridge_four_wins")]
	pub wins: u32,
	#[serde(rename = "bridge_four_losses")]
	pub losses: u32,
	#[serde(rename = "bridge_four_kills")]
	pub kills: u32,
	#[serde(rename = "bridge_four_deaths")]
	pub deaths: u32,
	#[serde(rename = "bridge_four_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "bridge_four_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "bridge_four_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "bridge_four_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "bridge_four_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "bridge_four_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BridgeSolo {
	#[serde(rename = "bridge_duel_wins")]
	pub wins: u32,
	#[serde(rename = "bridge_duel_losses")]
	pub losses: u32,
	#[serde(rename = "bridge_duel_kills")]
	pub kills: u32,
	#[serde(rename = "bridge_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "bridge_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "bridge_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "bridge_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "bridge_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "bridge_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "bridge_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BridgeThree {
	#[serde(rename = "bridge_3v3v3v3_wins")]
	pub wins: u32,
	#[serde(rename = "bridge_3v3v3v3_losses")]
	pub losses: u32,
	#[serde(rename = "bridge_3v3v3v3_kills")]
	pub kills: u32,
	#[serde(rename = "bridge_3v3v3v3_deaths")]
	pub deaths: u32,
	#[serde(rename = "bridge_3v3v3v3_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "bridge_3v3v3v3_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "bridge_3v3v3v3_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "bridge_3v3v3v3_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "bridge_3v3v3v3_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "bridge_3v3v3v3_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BridgeDouble {
	#[serde(rename = "bridge_2v2v2v2_wins")]
	pub wins: u32,
	#[serde(rename = "bridge_2v2v2v2_losses")]
	pub losses: u32,
	#[serde(rename = "bridge_2v2v2v2_kills")]
	pub kills: u32,
	#[serde(rename = "bridge_2v2v2v2_deaths")]
	pub deaths: u32,
	#[serde(rename = "bridge_2v2v2v2_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "bridge_2v2v2v2_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "bridge_2v2v2v2_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "bridge_2v2v2v2_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "bridge_2v2v2v2_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "bridge_2v2v2v2_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ComboSolo {
	#[serde(rename = "combo_duel_wins")]
	pub wins: u32,
	#[serde(rename = "combo_duel_losses")]
	pub losses: u32,
	#[serde(rename = "combo_duel_kills")]
	pub kills: u32,
	#[serde(rename = "combo_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "combo_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "combo_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "combo_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "combo_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "combo_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "combo_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SumoTournament {
	#[serde(rename = "sumo_tournament_wins")]
	pub wins: u32,
	#[serde(rename = "sumo_tournament_losses")]
	pub losses: u32,
	#[serde(rename = "sumo_tournament_kills")]
	pub kills: u32,
	#[serde(rename = "sumo_tournament_deaths")]
	pub deaths: u32,
	#[serde(rename = "sumo_tournament_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "sumo_tournament_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "sumo_tournament_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "sumo_tournament_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "sumo_tournament_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "sumo_tournament_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct SkyWarsTournament {
	#[serde(rename = "sw_tournament_wins")]
	pub wins: u32,
	#[serde(rename = "sw_tournament_losses")]
	pub losses: u32,
	#[serde(rename = "sw_tournament_kills")]
	pub kills: u32,
	#[serde(rename = "sw_tournament_deaths")]
	pub deaths: u32,
	#[serde(rename = "sw_tournament_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "sw_tournament_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "sw_tournament_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "sw_tournament_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "sw_tournament_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "sw_tournament_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct UhcMeetup {
	#[serde(rename = "uhc_meetup_wins")]
	pub wins: u32,
	#[serde(rename = "uhc_meetup_losses")]
	pub losses: u32,
	#[serde(rename = "uhc_meetup_kills")]
	pub kills: u32,
	#[serde(rename = "uhc_meetup_deaths")]
	pub deaths: u32,
	#[serde(rename = "uhc_meetup_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "uhc_meetup_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "uhc_meetup_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "uhc_meetup_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "uhc_meetup_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "uhc_meetup_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct PotionSolo {
	#[serde(rename = "potion_duel_wins")]
	pub wins: u32,
	#[serde(rename = "potion_duel_losses")]
	pub losses: u32,
	#[serde(rename = "potion_duel_kills")]
	pub kills: u32,
	#[serde(rename = "potion_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "potion_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "potion_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "potion_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "potion_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "potion_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "potion_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BlitzSolo {
	#[serde(rename = "blitz_duel_wins")]
	pub wins: u32,
	#[serde(rename = "blitz_duel_losses")]
	pub losses: u32,
	#[serde(rename = "blitz_duel_kills")]
	pub kills: u32,
	#[serde(rename = "blitz_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "blitz_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "blitz_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "blitz_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "blitz_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "blitz_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "blitz_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BowSpleefSolo {
	#[serde(rename = "bowspleef_duel_wins")]
	pub wins: u32,
	#[serde(rename = "bowspleef_duel_losses")]
	pub losses: u32,
	#[serde(rename = "bowspleef_duel_kills")]
	pub kills: u32,
	#[serde(rename = "bowspleef_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "bowspleef_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "bowspleef_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "bowspleef_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "bowspleef_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "bowspleef_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "bowspleef_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct MegaWallsSolo {
	#[serde(rename = "mw_duel_wins")]
	pub wins: u32,
	#[serde(rename = "mw_duel_losses")]
	pub losses: u32,
	#[serde(rename = "mw_duel_kills")]
	pub kills: u32,
	#[serde(rename = "mw_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "mw_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "mw_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "mw_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "mw_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "mw_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "mw_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BoxingSolo {
	#[serde(rename = "boxing_duel_wins")]
	pub wins: u32,
	#[serde(rename = "boxing_duel_losses")]
	pub losses: u32,
	#[serde(rename = "boxing_duel_kills")]
	pub kills: u32,
	#[serde(rename = "boxing_duel_deaths")]
	pub deaths: u32,
	#[serde(rename = "boxing_duel_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "boxing_duel_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "boxing_duel_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "boxing_duel_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "boxing_duel_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "boxing_duel_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Parkour {
	#[serde(rename = "parkour_eight_wins")]
	pub wins: u32,
	#[serde(rename = "parkour_eight_losses")]
	pub losses: u32,
	#[serde(rename = "parkour_eight_kills")]
	pub kills: u32,
	#[serde(rename = "parkour_eight_deaths")]
	pub deaths: u32,
	#[serde(rename = "parkour_eight_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "parkour_eight_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "parkour_eight_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "parkour_eight_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "parkour_eight_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "parkour_eight_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ArenaSolo {
	#[serde(rename = "duel_arena_wins")]
	pub wins: u32,
	#[serde(rename = "duel_arena_losses")]
	pub losses: u32,
	#[serde(rename = "duel_arena_kills")]
	pub kills: u32,
	#[serde(rename = "duel_arena_deaths")]
	pub deaths: u32,
	#[serde(rename = "duel_arena_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "duel_arena_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "duel_arena_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "duel_arena_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "duel_arena_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "duel_arena_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct CaptureThree {
	#[serde(rename = "capture_threes_wins")]
	pub wins: u32,
	#[serde(rename = "capture_threes_losses")]
	pub losses: u32,
	#[serde(rename = "capture_threes_kills")]
	pub kills: u32,
	#[serde(rename = "capture_threes_deaths")]
	pub deaths: u32,
	#[serde(rename = "capture_threes_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "capture_threes_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "capture_threes_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "capture_threes_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "capture_threes_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "capture_threes_damage_dealt")]
	pub damage_dealt: u32,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct BridgeThreeDuel {
	#[serde(rename = "bridge_threes_wins")]
	pub wins: u32,
	#[serde(rename = "bridge_threes_losses")]
	pub losses: u32,
	#[serde(rename = "bridge_threes_kills")]
	pub kills: u32,
	#[serde(rename = "bridge_threes_deaths")]
	pub deaths: u32,
	#[serde(rename = "bridge_threes_melee_hits")]
	pub melee_hits: u32,
	#[serde(rename = "bridge_threes_melee_swings")]
	pub melee_swings: u32,
	#[serde(rename = "bridge_threes_bow_hits")]
	pub arrows_hit: u32,
	#[serde(rename = "bridge_threes_bow_shots")]
	pub arrows_shot: u32,
	#[serde(rename = "bridge_threes_health_regenerated")]
	pub health_regenerated: u32,
	#[serde(rename = "bridge_threes_damage_dealt")]
	pub damage_dealt: u32,
}
