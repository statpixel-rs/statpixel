#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "cops_and_crims",
		pretty = "§9§lCops §f§land §c§lCrims",
		plain = "Cops and Crims",
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
		field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
		field(ident = "assists", colour = "green"),
		field(ident = "cop_kills", colour = "red"),
		field(ident = "criminal_kills", colour = "gold")
	)
)]
#[serde(default)]
pub struct CopsAndCrims {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,
	#[cfg_attr(feature = "game", game(label(colour = "aqua")))]
	pub shots_fired: u32,
	#[cfg_attr(feature = "game", game(label(colour = "red")))]
	pub headshot_kills: u32,
	#[cfg_attr(feature = "game", game(label(colour = "green")))]
	pub grenade_kills: u32,
	#[cfg_attr(feature = "game", game(label(colour = "red")))]
	pub bombs_defused: u32,
	#[cfg_attr(feature = "game", game(label(colour = "green")))]
	pub bombs_planted: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub defusal: Defusal,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub gun_game: GunGame,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub deathmatch: Deathmatch,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Defusal {
	#[serde(rename = "game_wins")]
	pub wins: u32,
	#[serde(rename = "game_plays")]
	pub games: u32,
	pub kills: u32,
	pub deaths: u32,
	pub assists: u32,
	pub cop_kills: u32,
	pub criminal_kills: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct GunGame {
	#[serde(rename = "game_wins_gungame")]
	pub wins: u32,
	#[serde(rename = "game_plays_gungame")]
	pub games: u32,
	#[serde(rename = "kills_gungame")]
	pub kills: u32,
	#[serde(rename = "deaths_gungame")]
	pub deaths: u32,
	#[serde(rename = "assists_gungame")]
	pub assists: u32,
	#[serde(rename = "cop_kills_gungame")]
	pub cop_kills: u32,
	#[serde(rename = "criminal_kills_gungame")]
	pub criminal_kills: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Deathmatch {
	#[serde(rename = "game_wins_deathmatch")]
	pub wins: u32,
	#[serde(rename = "game_plays_deathmatch")]
	pub games: u32,
	#[serde(rename = "kills_deathmatch")]
	pub kills: u32,
	#[serde(rename = "deaths_deathmatch")]
	pub deaths: u32,
	#[serde(rename = "assists_deathmatch")]
	pub assists: u32,
	#[serde(rename = "cop_kills_deathmatch")]
	pub cop_kills: u32,
	#[serde(rename = "criminal_kills_deathmatch")]
	pub criminal_kills: u32,
}
