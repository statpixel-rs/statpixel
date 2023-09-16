#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "smash_heroes",
		pretty = "§e§lSmash Heroes",
		plain = "Smash Heroes",
		field(ident = "wins", colour = "green"),
		field(ident = "losses", colour = "red"),
		field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
		field(ident = "kills", colour = "green"),
		field(ident = "deaths", colour = "red"),
		field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
		field(ident = "smasher", colour = "green"),
		field(ident = "smashed", colour = "red"),
		field(tr = "ssr", ident = "smasher", div = "smashed", colour = "gold")
	)
)]
#[serde(default)]
pub struct SmashHeroes {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,
	#[cfg_attr(feature = "game", game(label(colour = "yellow")))]
	pub win_streak: u32,
	#[serde(rename = "smash_level_total")]
	#[cfg_attr(feature = "game", game(label(colour = "blue")))]
	pub level: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub solo: Solo,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub double: Double,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub team: Team,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Solo {
	#[serde(rename = "wins_normal")]
	pub wins: u32,
	#[serde(rename = "losses_normal")]
	pub losses: u32,
	#[serde(rename = "kills_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_normal")]
	pub deaths: u32,
	#[serde(rename = "smasher_normal")]
	pub smasher: u32,
	#[serde(rename = "smashed_normal")]
	pub smashed: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Double {
	#[serde(rename = "wins_2v2")]
	pub wins: u32,
	#[serde(rename = "losses_2v2")]
	pub losses: u32,
	#[serde(rename = "kills_2v2")]
	pub kills: u32,
	#[serde(rename = "deaths_2v2")]
	pub deaths: u32,
	#[serde(rename = "smasher_2v2")]
	pub smasher: u32,
	#[serde(rename = "smashed_2v2")]
	pub smashed: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Team {
	#[serde(rename = "wins_teams")]
	pub wins: u32,
	#[serde(rename = "losses_teams")]
	pub losses: u32,
	#[serde(rename = "kills_teams")]
	pub kills: u32,
	#[serde(rename = "deaths_teams")]
	pub deaths: u32,
	#[serde(rename = "smasher_teams")]
	pub smasher: u32,
	#[serde(rename = "smashed_teams")]
	pub smashed: u32,
}
