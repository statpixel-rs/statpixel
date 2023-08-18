use macros::Game;

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default, Game)]
#[serde(default)]
#[game(
	path = "bed_wars",
	pretty = "§c§lBed §f§lWars",
	plain = "Bed Wars",
	calc = "minecraft::calc::bed_wars",
	field(ident = "wins", colour = "green"),
	field(ident = "losses", colour = "red"),
	field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
	field(ident = "final_kills", colour = "green"),
	field(ident = "final_deaths", colour = "red"),
	field(
		tr = "fkdr",
		ident = "final_kills",
		div = "final_deaths",
		colour = "gold"
	),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
	field(ident = "beds_broken", colour = "green"),
	field(ident = "beds_lost", colour = "red"),
	field(tr = "bblr", ident = "beds_broken", div = "beds_lost", colour = "gold"),
	label(ident = "iron_collected", colour = "gray"),
	label(ident = "gold_collected", colour = "gold"),
	label(ident = "diamond_collected", colour = "aqua"),
	label(ident = "emerald_collected", colour = "dark_green"),
	label(ident = "items_purchased", colour = "red")
)]
pub struct BedWars {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[serde(
		rename(deserialize = "bedwars_boxes"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	#[game(label(colour = "yellow"))]
	pub loot_chests: u32,
	#[serde(
		rename(deserialize = "Experience"),
		deserialize_with = "crate::de::from::f64_to_u64"
	)]
	#[game(xp)]
	pub xp: u64,
	#[serde(rename = "winstreak")]
	pub win_streak: u32,

	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_ONE"))]
	pub solo: Solo,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_TWO"))]
	pub double: Double,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_THREE"))]
	pub three: Three,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_FOUR"))]
	pub four: Four,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_ONE_RUSH", skip_overall))]
	pub solo_rush: SoloRush,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_TWO_RUSH", skip_overall))]
	pub double_rush: DoubleRush,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_FOUR_RUSH", skip_overall))]
	pub four_rush: FourRush,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_ONE_ULTIMATE", skip_overall))]
	pub solo_ultimate: SoloUltimate,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_TWO_ULTIMATE", skip_overall))]
	pub double_ultimate: DoubleUltimate,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_FOUR_ULTIMATE", skip_overall))]
	pub four_ultimate: FourUltimate,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_CASTLE", skip_overall))]
	pub castle: Castle,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_TWO_LUCKY", skip_overall))]
	pub double_lucky: DoubleLucky,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_FOUR_LUCKY", skip_overall))]
	pub four_lucky: FourLucky,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_TWO_VOIDLESS", skip_overall))]
	pub double_voidless: DoubleVoidless,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_FOUR_VOIDLESS", skip_overall))]
	pub four_voidless: FourVoidless,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_TWO_ARMED", skip_overall))]
	pub double_armed: DoubleArmed,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_FOUR_ARMED", skip_overall))]
	pub four_armed: FourArmed,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_TWO_UNDERWORLD", skip_overall))]
	pub double_underworld: DoubleUnderworld,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_FOUR_UNDERWORLD", skip_overall))]
	pub four_underworld: FourUnderworld,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_EIGHT_TWO_SWAP", skip_overall))]
	pub double_swap: DoubleSwap,
	#[serde(flatten)]
	#[game(mode(hypixel = "BEDWARS_FOUR_FOUR_SWAP", skip_overall))]
	pub four_swap: FourSwap,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
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
	#[serde(rename = "eight_one_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
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
	#[serde(rename = "eight_two_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
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
	#[serde(rename = "four_three_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
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
	#[serde(rename = "four_four_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloRush {
	#[serde(rename = "eight_one_rush_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_one_rush_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_one_rush_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_one_rush_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_one_rush_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_one_rush_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_one_rush_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_one_rush_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_one_rush_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_one_rush_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_one_rush_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_one_rush_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_one_rush_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_one_rush_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleRush {
	#[serde(rename = "eight_two_rush_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_rush_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_rush_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_rush_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_two_rush_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_two_rush_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_two_rush_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_two_rush_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_two_rush_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_two_rush_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_two_rush_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_two_rush_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_two_rush_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_two_rush_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct FourRush {
	#[serde(rename = "four_four_rush_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_rush_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_rush_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_rush_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_four_rush_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_four_rush_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_four_rush_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_four_rush_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_four_rush_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_four_rush_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_four_rush_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_four_rush_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_four_rush_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "four_four_rush_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloUltimate {
	#[serde(rename = "eight_one_ultimate_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_one_ultimate_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_one_ultimate_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_one_ultimate_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_one_ultimate_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_one_ultimate_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_one_ultimate_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_one_ultimate_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_one_ultimate_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_one_ultimate_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_one_ultimate_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_one_ultimate_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_one_ultimate_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_one_ultimate_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleUltimate {
	#[serde(rename = "eight_two_ultimate_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_ultimate_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_ultimate_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_ultimate_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_two_ultimate_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_two_ultimate_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_two_ultimate_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_two_ultimate_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_two_ultimate_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_two_ultimate_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_two_ultimate_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_two_ultimate_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_two_ultimate_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_two_ultimate_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct FourUltimate {
	#[serde(rename = "four_four_ultimate_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_ultimate_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_ultimate_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_ultimate_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_four_ultimate_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_four_ultimate_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_four_ultimate_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_four_ultimate_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_four_ultimate_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_four_ultimate_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_four_ultimate_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_four_ultimate_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_four_ultimate_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "four_four_ultimate_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Castle {
	#[serde(rename = "castle_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "castle_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "castle_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "castle_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "castle_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "castle_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "castle_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "castle_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "castle_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "castle_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "castle_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "castle_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "castle_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "castle_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleLucky {
	#[serde(rename = "eight_two_lucky_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_lucky_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_lucky_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_lucky_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_two_lucky_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_two_lucky_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_two_lucky_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_two_lucky_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_two_lucky_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_two_lucky_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_two_lucky_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_two_lucky_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_two_lucky_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_two_lucky_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct FourLucky {
	#[serde(rename = "four_four_lucky_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_lucky_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_lucky_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_lucky_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_four_lucky_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_four_lucky_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_four_lucky_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_four_lucky_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_four_lucky_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_four_lucky_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_four_lucky_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_four_lucky_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_four_lucky_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "four_four_lucky_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleVoidless {
	#[serde(rename = "eight_two_voidless_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_voidless_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_voidless_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_voidless_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_two_voidless_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_two_voidless_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_two_voidless_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_two_voidless_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_two_voidless_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_two_voidless_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_two_voidless_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_two_voidless_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_two_voidless_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_two_voidless_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct FourVoidless {
	#[serde(rename = "four_four_voidless_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_voidless_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_voidless_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_voidless_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_four_voidless_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_four_voidless_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_four_voidless_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_four_voidless_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_four_voidless_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_four_voidless_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_four_voidless_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_four_voidless_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_four_voidless_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "four_four_voidless_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleArmed {
	#[serde(rename = "eight_two_armed_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_armed_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_armed_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_armed_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_two_armed_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_two_armed_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_two_armed_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_two_armed_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_two_armed_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_two_armed_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_two_armed_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_two_armed_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_two_armed_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_two_armed_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct FourArmed {
	#[serde(rename = "four_four_armed_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_armed_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_armed_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_armed_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_four_armed_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_four_armed_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_four_armed_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_four_armed_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_four_armed_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_four_armed_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_four_armed_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_four_armed_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_four_armed_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "four_four_armed_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleUnderworld {
	#[serde(rename = "eight_two_underworld_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_underworld_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_underworld_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_underworld_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_two_underworld_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_two_underworld_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_two_underworld_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_two_underworld_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_two_underworld_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_two_underworld_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_two_underworld_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_two_underworld_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_two_underworld_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_two_underworld_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct FourUnderworld {
	#[serde(rename = "four_four_underworld_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_underworld_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_underworld_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_underworld_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_four_underworld_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_four_underworld_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_four_underworld_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_four_underworld_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_four_underworld_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_four_underworld_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_four_underworld_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_four_underworld_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_four_underworld_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "four_four_underworld_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleSwap {
	#[serde(rename = "eight_two_swap_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "eight_two_swap_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "eight_two_swap_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "eight_two_swap_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "eight_two_swap_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "eight_two_swap_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "eight_two_swap_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "eight_two_swap_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "eight_two_swap_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "eight_two_swap_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "eight_two_swap_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "eight_two_swap_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "eight_two_swap_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "eight_two_swap_winstreak")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct FourSwap {
	#[serde(rename = "four_four_swap_wins_bedwars")]
	pub wins: u32,
	#[serde(rename = "four_four_swap_losses_bedwars")]
	pub losses: u32,
	#[serde(rename = "four_four_swap_kills_bedwars")]
	pub kills: u32,
	#[serde(rename = "four_four_swap_deaths_bedwars")]
	pub deaths: u32,
	#[serde(rename = "four_four_swap_final_kills_bedwars")]
	pub final_kills: u32,
	#[serde(rename = "four_four_swap_final_deaths_bedwars")]
	pub final_deaths: u32,
	#[serde(rename = "four_four_swap_beds_broken_bedwars")]
	pub beds_broken: u32,
	#[serde(rename = "four_four_swap_beds_lost_bedwars")]
	pub beds_lost: u32,
	#[serde(rename = "four_four_swap_iron_resources_collected_bedwars")]
	pub iron_collected: u32,
	#[serde(rename = "four_four_swap_gold_resources_collected_bedwars")]
	pub gold_collected: u32,
	#[serde(rename = "four_four_swap_diamond_resources_collected_bedwars")]
	pub diamond_collected: u32,
	#[serde(rename = "four_four_swap_emerald_resources_collected_bedwars")]
	pub emerald_collected: u32,
	#[serde(rename = "four_four_swap_items_purchased_bedwars")]
	pub items_purchased: u32,
	#[serde(rename = "four_four_swap_winstreak")]
	pub win_streak: u32,
}
