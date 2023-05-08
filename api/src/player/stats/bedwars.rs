use macros::Game;
use serde::Deserialize;

// Todo: paint extras field (7 info lines)
// in the mode, so we can use mode-related data in the fields
// instead of just fields in the overall struct

#[derive(Deserialize, Default, Debug, Clone, Game)]
#[serde(default)]
#[game(
	path = "bedwars",
	pretty = "§c§lBed§d§lWars",
	calc = "minecraft::calc::bedwars",
	fields(ident = "wins", colour = "green"),
	fields(ident = "losses", colour = "red"),
	fields(tr = "wlr", div = "wins", div = "losses", colour = "gold"),
	fields(ident = "kills", colour = "green"),
	fields(ident = "deaths", colour = "red"),
	fields(tr = "kdr", div = "kills", div = "deaths", colour = "gold"),
	fields(ident = "final_kills", colour = "green"),
	fields(ident = "final_deaths", colour = "red"),
	fields(
		tr = "fkdr",
		div = "final_kills",
		div = "final_deaths",
		colour = "gold"
	),
	fields(ident = "beds_broken", colour = "green"),
	fields(ident = "beds_lost", colour = "red"),
	fields(tr = "bblr", div = "beds_broken", div = "beds_lost", colour = "gold")
)]
pub struct Stats {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[serde(
		rename = "bedwars_boxes",
		deserialize_with = "super::from_trunc_f32_to_u32"
	)]
	#[game(label(colour = "yellow"))]
	pub loot_chests: u32,
	#[serde(
		rename = "Experience",
		deserialize_with = "super::from_trunc_f32_to_u64"
	)]
	pub xp: u64,

	#[serde(flatten)]
	pub solo: SoloStats,
	#[serde(flatten)]
	pub double: DoubleStats,
	#[serde(flatten)]
	pub three: ThreeStats,
	#[serde(flatten)]
	pub four: FourStats,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SoloStats {
	#[serde(rename = "eight_one_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_one_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_one_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_one_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_one_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_one_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_one_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_one_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_one_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_one_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_one_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_one_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_one_items_purchased_bedwars")]
	pub items_purchased: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct DoubleStats {
	#[serde(rename = "eight_two_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_two_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_two_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_two_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_two_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_two_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_two_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_two_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_two_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_two_items_purchased_bedwars")]
	pub items_purchased: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct ThreeStats {
	#[serde(rename = "four_three_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_three_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_three_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_three_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_three_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_three_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_three_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_three_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_three_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_three_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_three_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_three_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_three_items_purchased_bedwars")]
	pub items_purchased: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct FourStats {
	#[serde(rename = "four_four_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_four_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_four_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_four_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_four_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_four_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_four_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_four_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_four_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_four_items_purchased_bedwars")]
	pub items_purchased: u32,
}
