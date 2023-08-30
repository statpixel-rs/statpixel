#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "uhc",
		pretty = "§e§lUHC §6§lChampions",
		plain = "UHC Champions",
		field(ident = "wins", colour = "green"),
		field(ident = "heads_eaten", colour = "red"),
		field(ident = "ultimates_crafted", colour = "gold"),
		field(ident = "kills", colour = "green"),
		field(ident = "deaths", colour = "red"),
		field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold")
	)
)]
#[serde(default)]
pub struct Uhc {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,
	#[cfg_attr(feature = "game", game(label(colour = "yellow")))]
	pub score: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub solo: Solo,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub team: Team,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub red_vs_blue: RedVsBlue,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub no_diamonds: NoDiamonds,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub vanilla_double: VanillaDouble,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub brawl: Brawl,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub solo_brawl: SoloBrawl,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub double_brawl: DoubleBrawl,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Solo {
	#[serde(
		rename(deserialize = "wins_solo"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub wins: u32,
	#[serde(rename = "heads_eaten_solo")]
	pub heads_eaten: u32,
	#[serde(rename = "ultimates_crafted_solo")]
	pub ultimates_crafted: u32,
	#[serde(
		rename(deserialize = "kills_solo"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub kills: u32,
	#[serde(rename = "deaths_solo")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Team {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	pub wins: u32,
	pub heads_eaten: u32,
	pub ultimates_crafted: u32,
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	pub kills: u32,
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct RedVsBlue {
	#[serde(
		rename(deserialize = "wins_red_vs_blue"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub wins: u32,
	#[serde(rename = "heads_eaten_red_vs_blue")]
	pub heads_eaten: u32,
	#[serde(rename = "ultimates_crafted_red_vs_blue")]
	pub ultimates_crafted: u32,
	#[serde(
		rename(deserialize = "kills_red_vs_blue"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub kills: u32,
	#[serde(rename = "deaths_red_vs_blue")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct NoDiamonds {
	#[serde(
		rename(deserialize = "wins_no_diamonds"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub wins: u32,
	#[serde(rename = "heads_eaten_no_diamonds")]
	pub heads_eaten: u32,
	#[serde(rename = "ultimates_crafted_no_diamonds")]
	pub ultimates_crafted: u32,
	#[serde(
		rename(deserialize = "kills_no_diamonds"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub kills: u32,
	#[serde(rename = "deaths_no_diamonds")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct VanillaDouble {
	#[serde(
		rename(deserialize = "wins_vanilla_doubles"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub wins: u32,
	#[serde(rename = "heads_eaten_vanilla_doubles")]
	pub heads_eaten: u32,
	#[serde(rename = "ultimates_crafted_vanilla_doubles")]
	pub ultimates_crafted: u32,
	#[serde(
		rename(deserialize = "kills_vanilla_doubles"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub kills: u32,
	#[serde(rename = "deaths_vanilla_doubles")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Brawl {
	#[serde(
		rename(deserialize = "wins_brawl"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub wins: u32,
	#[serde(rename = "heads_eaten_brawl")]
	pub heads_eaten: u32,
	#[serde(rename = "ultimates_crafted_brawl")]
	pub ultimates_crafted: u32,
	#[serde(
		rename(deserialize = "kills_brawl"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub kills: u32,
	#[serde(rename = "deaths_brawl")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloBrawl {
	#[serde(
		rename(deserialize = "wins_solo_brawl"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub wins: u32,
	#[serde(rename = "heads_eaten_solo_brawl")]
	pub heads_eaten: u32,
	#[serde(rename = "ultimates_crafted_solo_brawl")]
	pub ultimates_crafted: u32,
	#[serde(
		rename(deserialize = "kills_solo_brawl"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub kills: u32,
	#[serde(rename = "deaths_solo_brawl")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleBrawl {
	#[serde(
		rename(deserialize = "wins_duo_brawl"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub wins: u32,
	#[serde(rename = "heads_eaten_duo_brawl")]
	pub heads_eaten: u32,
	#[serde(rename = "ultimates_crafted_duo_brawl")]
	pub ultimates_crafted: u32,
	#[serde(
		rename(deserialize = "kills_duo_brawl"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	pub kills: u32,
	#[serde(rename = "deaths_duo_brawl")]
	pub deaths: u32,
}
