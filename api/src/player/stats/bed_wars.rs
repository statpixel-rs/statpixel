use crate::{
	image::Image, milliseconds::MillisecondsOption, shape, skyblock::materials::MATERIALS,
};
use macros::Game;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(rename_all = "PascalCase")]
pub enum HotbarItem {
	Melee,
	Tools,
	Blocks,
	Utility,
	Ranged,
	Potions,
	Tracker,
	#[serde(rename = "null")]
	#[default]
	None,
}

impl HotbarItem {
	pub fn into_slot(&self) -> shape::Slot {
		let id = match self {
			Self::Melee => "GOLD_SWORD",
			Self::Tools => "IRON_PICKAXE",
			Self::Blocks => "HARD_CLAY",
			Self::Utility => "TNT",
			Self::Ranged => "BOW",
			Self::Potions => "BREWING_STAND",
			Self::Tracker => "COMPASS",
			Self::None => return shape::Slot(None, 1),
		};

		shape::Slot(MATERIALS.get(id).map(Image::image), 1)
	}
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(rename_all = "snake_case")]
pub enum ShopItem {
	Wool,
	HardenedClay,
	#[serde(rename = "blast-proof_glass")]
	BlastProofGlass,
	EndStone,
	Ladder,
	OakWoodPlanks,
	Obsidian,
	StoneSword,
	IronSword,
	DiamondSword,
	#[serde(rename = "stick_(knockback_i)")]
	StickKnockbackI,
	ChainmailBoots,
	IronBoots,
	DiamondBoots,
	WoodenPickaxe,
	WoodenAxe,
	Shears,
	Arrow,
	Bow,
	#[serde(rename = "bow_(power_i)")]
	BowPowerI,
	#[serde(rename = "bow_(power_i__punch_i)")]
	BowPowerIPunchI,
	#[serde(rename = "speed_ii_potion_(45_seconds)")]
	SpeedIIPotion45Seconds,
	#[serde(rename = "jump_v_potion_(45_seconds)")]
	JumpVPotion45Seconds,
	#[serde(rename = "invisibility_potion_(30_seconds)")]
	InvisibilityPotion30Seconds,
	GoldenApple,
	Bedbug,
	DreamDefender,
	Fireball,
	Tnt,
	EnderPearl,
	WaterBucket,
	BridgeEgg,
	MagicMilk,
	Sponge,
	#[serde(rename = "compact_pop-up_tower")]
	CompactPopUpTower,
	#[serde(rename = "null")]
	#[default]
	None,
}

impl ShopItem {
	pub fn into_slot(&self) -> shape::Slot {
		let (id, count) = match self {
			Self::Wool => ("WOOL", 16),
			Self::HardenedClay => ("HARD_CLAY", 16),
			Self::BlastProofGlass => ("GLASS", 4),
			Self::EndStone => ("ENDER_STONE", 12),
			Self::Ladder => ("LADDER", 8),
			Self::OakWoodPlanks => ("WOOD", 16),
			Self::Obsidian => ("OBSIDIAN", 4),
			Self::StoneSword => ("STONE_SWORD", 1),
			Self::IronSword => ("IRON_SWORD", 1),
			Self::DiamondSword => ("DIAMOND_SWORD", 1),
			Self::StickKnockbackI => ("STICK", 1),
			Self::ChainmailBoots => ("CHAINMAIL_BOOTS", 1),
			Self::IronBoots => ("IRON_BOOTS", 1),
			Self::DiamondBoots => ("DIAMOND_BOOTS", 1),
			Self::WoodenPickaxe => ("WOOD_PICKAXE", 1),
			Self::WoodenAxe => ("WOOD_AXE", 1),
			Self::Shears => ("SHEARS", 1),
			Self::Arrow => ("ARROW", 6),
			Self::Bow => ("BOW", 1),
			Self::BowPowerI => ("HURRICANE_BOW", 1),
			Self::BowPowerIPunchI => ("DEATH_BOW_STANDBY", 1),
			Self::SpeedIIPotion45Seconds => ("SPEED_POTION", 1),
			Self::JumpVPotion45Seconds => ("JUMP_POTION", 1),
			Self::InvisibilityPotion30Seconds => ("INVISIBILITY_POTION", 1),
			Self::GoldenApple => ("GOLDEN_APPLE", 1),
			Self::Bedbug => ("SNOW_BALL", 1),
			Self::DreamDefender => ("SPAWN_EGG", 1),
			Self::Fireball => ("FIRE_CHARGE", 1),
			Self::Tnt => ("TNT", 1),
			Self::EnderPearl => ("ENDER_PEARL", 1),
			Self::WaterBucket => ("WATER_BUCKET", 1),
			Self::BridgeEgg => ("EGG", 1),
			Self::MagicMilk => ("MILK_BUCKET", 1),
			Self::Sponge => ("SPONGE", 4),
			Self::CompactPopUpTower => ("TRAPPED_CHEST", 1),
			Self::None => return shape::Slot(None, 1),
		};

		shape::Slot(MATERIALS.get(id).map(Image::image), count)
	}
}

fn from_comma_separated<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
	D: Deserializer<'de>,
	T: for<'d> Deserialize<'d> + Default + DeserializeOwned,
{
	let s: String = Deserialize::deserialize(deserializer)?;

	// TODO: Avoid the allocation by either writing this manually or figuring out how to do it
	// by reference with serde
	Ok(s.split(',')
		.map(|s| {
			serde_json::from_value(serde_json::Value::String(s.to_string())).unwrap_or_default()
		})
		.collect())
}

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

	pub practice: Practice,
	#[serde(rename = "favorite_slots", deserialize_with = "from_comma_separated")]
	pub hotbar: Vec<HotbarItem>,
	#[serde(rename = "favourites_2", deserialize_with = "from_comma_separated")]
	pub shop: Vec<ShopItem>,

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
pub struct Practice {
	pub mlg: PracticeMode,
	pub bridging: PracticeMode,
	#[serde(rename = "fireball_jumping")]
	pub fireball: PracticeMode,
	#[serde(rename = "pearl_clutching")]
	pub pearl: PracticeMode,
	pub records: PracticeRecords,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default, Debug)]
#[serde(default)]
pub struct PracticeRecords {
	#[serde(rename = "bridging_distance_30:elevation_NONE:angle_STRAIGHT:")]
	pub straight_none_30: MillisecondsOption,
	#[serde(rename = "bridging_distance_50:elevation_NONE:angle_STRAIGHT:")]
	pub straight_none_50: MillisecondsOption,
	#[serde(rename = "bridging_distance_100:elevation_NONE:angle_STRAIGHT:")]
	pub straight_none_100: MillisecondsOption,
	#[serde(rename = "bridging_distance_30:elevation_SLIGHT:angle_DIAGONAL:")]
	pub straight_slight_30: MillisecondsOption,
	#[serde(rename = "bridging_distance_50:elevation_SLIGHT:angle_DIAGONAL:")]
	pub straight_slight_50: MillisecondsOption,
	#[serde(rename = "bridging_distance_100:elevation_SLIGHT:angle_DIAGONAL:")]
	pub straight_slight_100: MillisecondsOption,
	#[serde(rename = "bridging_distance_30:elevation_STAIRCASE:angle_STRAIGHT:")]
	pub straight_staircase_30: MillisecondsOption,
	#[serde(rename = "bridging_distance_50:elevation_STAIRCASE:angle_STRAIGHT:")]
	pub straight_staircase_50: MillisecondsOption,
	#[serde(rename = "bridging_distance_100:elevation_STAIRCASE:angle_STRAIGHT:")]
	pub straight_staircase_100: MillisecondsOption,
	#[serde(rename = "bridging_distance_30:elevation_NONE:angle_DIAGONAL:")]
	pub diagonal_none_30: MillisecondsOption,
	#[serde(rename = "bridging_distance_50:elevation_NONE:angle_DIAGONAL:")]
	pub diagonal_none_50: MillisecondsOption,
	#[serde(rename = "bridging_distance_100:elevation_NONE:angle_DIAGONAL:")]
	pub diagonal_none_100: MillisecondsOption,
	#[serde(rename = "bridging_distance_30:elevation_SLIGHT:angle_DIAGONAL:")]
	pub diagonal_slight_30: MillisecondsOption,
	#[serde(rename = "bridging_distance_50:elevation_SLIGHT:angle_DIAGONAL:")]
	pub diagonal_slight_50: MillisecondsOption,
	#[serde(rename = "bridging_distance_100:elevation_SLIGHT:angle_DIAGONAL:")]
	pub diagonal_slight_100: MillisecondsOption,
	#[serde(rename = "bridging_distance_30:elevation_STAIRCASE:angle_DIAGONAL:")]
	pub diagonal_staircase_30: MillisecondsOption,
	#[serde(rename = "bridging_distance_50:elevation_STAIRCASE:angle_DIAGONAL:")]
	pub diagonal_staircase_50: MillisecondsOption,
	#[serde(rename = "bridging_distance_100:elevation_STAIRCASE:angle_DIAGONAL:")]
	pub diagonal_staircase_100: MillisecondsOption,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct PracticeMode {
	#[serde(rename = "successful_attempts")]
	pub successes: u32,
	#[serde(rename = "failed_attempts")]
	pub failures: u32,
	pub blocks_placed: u32,
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
