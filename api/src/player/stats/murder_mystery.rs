use extra::seconds;

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "murder_mystery",
		pretty = "§4§lMurder Mystery",
		plain = "Murder Mystery",
		field(ident = "wins", colour = "green"),
		field(ident = "games", colour = "red"),
		field(
			tr = "wr",
			ident = "wins",
			div = "games",
			colour = "gold",
			percent = "u32"
		),
		field(ident = "kills", colour = "green"),
		field(ident = "deaths", colour = "red"),
		field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold")
	)
)]
#[serde(default)]
pub struct MurderMystery {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,
	#[serde(
		rename(deserialize = "mm_chests"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	#[cfg_attr(feature = "game", game(label(colour = "yellow")))]
	pub loot_chests: u32,
	#[serde(rename = "total_time_survived_seconds")]
	#[cfg_attr(feature = "game", game(label(colour = "aqua")))]
	pub time_survived: seconds::Seconds,
	#[serde(rename = "murderer_wins")]
	#[cfg_attr(feature = "game", game(label(colour = "red")))]
	pub murderer_wins: u32,
	#[serde(rename = "detective_wins")]
	#[cfg_attr(feature = "game", game(label(colour = "green")))]
	pub detective_wins: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub assassins: Assassins,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub classic: Classic,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub double_up: DoubleUp,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub infection: Infection,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Assassins {
	#[serde(rename = "wins_MURDER_ASSASSINS")]
	pub wins: u32,
	#[serde(rename = "games_MURDER_ASSASSINS")]
	pub games: u32,
	#[serde(rename = "kills_MURDER_ASSASSINS")]
	pub kills: u32,
	#[serde(rename = "deaths_MURDER_ASSASSINS")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Classic {
	#[serde(rename = "wins_MURDER_CLASSIC")]
	pub wins: u32,
	#[serde(rename = "games_MURDER_CLASSIC")]
	pub games: u32,
	#[serde(rename = "kills_MURDER_CLASSIC")]
	pub kills: u32,
	#[serde(rename = "deaths_MURDER_CLASSIC")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct DoubleUp {
	#[serde(rename = "wins_MURDER_DOUBLE_UP")]
	pub wins: u32,
	#[serde(rename = "games_MURDER_DOUBLE_UP")]
	pub games: u32,
	#[serde(rename = "kills_MURDER_DOUBLE_UP")]
	pub kills: u32,
	#[serde(rename = "deaths_MURDER_DOUBLE_UP")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Infection {
	#[serde(rename = "wins_MURDER_INFECTION")]
	pub wins: u32,
	#[serde(rename = "games_MURDER_INFECTION")]
	pub games: u32,
	#[serde(rename = "kills_MURDER_INFECTION")]
	pub kills: u32,
	#[serde(rename = "deaths_MURDER_INFECTION")]
	pub deaths: u32,
}
