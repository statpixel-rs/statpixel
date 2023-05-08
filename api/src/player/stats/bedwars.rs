use macros::Game;
use serde::Deserialize;

// Todo: paint extras field (7 info lines)
// in the mode, so we can use mode-related data in the fields
// instead of just fields in the overall struct

#[derive(Deserialize, Default, Debug, Clone, Game)]
#[serde(default)]
#[game(
	path = "bed_wars",
	pretty = "§c§lBed§d§lWars",
	calc = "minecraft::calc::bedwars",
	field(ident = "wins", colour = "green"),
	field(ident = "losses", colour = "red"),
	field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	field(ident = "final_kills", colour = "green"),
	field(ident = "final_deaths", colour = "red"),
	field(
		tr = "fkdr",
		ident = "final_kills",
		div = "final_deaths",
		colour = "gold"
	),
	field(ident = "beds_broken", colour = "green"),
	field(ident = "beds_lost", colour = "red"),
	field(tr = "bblr", ident = "beds_broken", div = "beds_lost", colour = "gold"),
	label(ident = "iron_collected", colour = "gray"),
	label(ident = "gold_collected", colour = "gold"),
	label(ident = "emerald_collected", colour = "dark_green"),
	label(ident = "diamond_collected", colour = "aqua"),
	label(ident = "items_purchased", colour = "red")
)]
pub struct BedWars {
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
	#[game(xp)]
	pub xp: u64,

	#[serde(flatten)]
	#[game(mode(hypixel = "bedwars_eight_one", tr = "Solo"))]
	pub solo: Solo,
	#[serde(flatten)]
	#[game(mode(hypixel = "bedwars_eight_two", tr = "Double"))]
	pub double: Double,
	#[serde(flatten)]
	#[game(mode(hypixel = "bedwars_four_three", tr = "Three"))]
	pub three: Three,
	#[serde(flatten)]
	#[game(mode(hypixel = "bedwars_four_four", tr = "Four"))]
	pub four: Four,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Solo {
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
pub struct Double {
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
pub struct Three {
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
pub struct Four {
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
