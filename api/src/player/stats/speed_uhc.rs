#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "speed_uhc",
		pretty = "§e§lSpeed §6§lUHC",
		plain = "Speed UHC",
		field(ident = "wins", colour = "green"),
		field(ident = "losses", colour = "red"),
		field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
		field(ident = "kills", colour = "green"),
		field(ident = "deaths", colour = "red"),
		field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
		label(ident = "score", colour = "blue"),
		label(ident = "survived_players", colour = "red")
	)
)]
#[serde(default)]
pub struct SpeedUhc {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,
	#[cfg_attr(feature = "game", game(label(colour = "aqua")))]
	pub tears: u32,
	#[cfg_attr(feature = "game", game(label(colour = "yellow")))]
	pub win_streak: u32,
	pub blocks_placed: u32,
	pub arrows_shot: u32,
	#[cfg_attr(
		feature = "game",
		game(label(tr = "bow-accuracy", colour = "red", div = "arrows_shot", percent))
	)]
	pub arrows_hit: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub solo: Solo,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub team: Team,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Solo {
	#[serde(rename = "wins_solo_normal")]
	pub wins: u32,
	#[serde(rename = "losses_solo_normal")]
	pub losses: u32,
	#[serde(rename = "kills_solo_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_normal")]
	pub deaths: u32,
	#[serde(rename = "score_solo")]
	pub score: u32,
	#[serde(rename = "survived_players_solo")]
	pub survived_players: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Team {
	#[serde(rename = "wins_team_normal")]
	pub wins: u32,
	#[serde(rename = "losses_team_normal")]
	pub losses: u32,
	#[serde(rename = "kills_team_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_team_normal")]
	pub deaths: u32,
	#[serde(rename = "score_team")]
	pub score: u32,
	#[serde(rename = "survived_players_team")]
	pub survived_players: u32,
}
