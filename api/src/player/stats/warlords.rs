#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "warlords",
		pretty = "§b§lWarlords",
		plain = "Warlords",
		field(ident = "wins_blue", colour = "blue"),
		field(ident = "wins_red", colour = "red"),
		field(ident = "kills", colour = "gold")
	)
)]
#[serde(default)]
pub struct Warlords {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,
	#[serde(
		rename(deserialize = "damage"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	#[cfg_attr(feature = "game", game(label(colour = "green")))]
	pub damage_dealt: u32,
	#[serde(
		rename(deserialize = "damage_taken"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	#[cfg_attr(feature = "game", game(label(colour = "blue")))]
	pub damage_taken: u32,
	#[serde(
		rename(deserialize = "heal"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	#[cfg_attr(feature = "game", game(label(colour = "red")))]
	pub health_regenerated: u32,
	#[cfg_attr(feature = "game", game(label(colour = "aqua"), nominal))]
	pub hide_prestige: bool,
	#[cfg_attr(
		feature = "game",
		game(label(tr = "kdr", div = "deaths", colour = "gold"))
	)]
	pub kills: u32,
	pub deaths: u32,
	#[serde(rename = "mvp_count")]
	#[cfg_attr(feature = "game", game(label(colour = "yellow")))]
	pub mvps: u32,
	pub win_streak: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub capture_the_flag: CaptureTheFlag,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub domination: Domination,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub team_deathmatch: TeamDeathmatch,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct CaptureTheFlag {
	#[serde(rename = "wins_capturetheflag_blu")]
	pub wins_blue: u32,
	#[serde(rename = "wins_capturetheflag_red")]
	pub wins_red: u32,
	#[serde(rename = "kills_capturetheflag")]
	pub kills: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Domination {
	#[serde(rename = "wins_domination_blu")]
	pub wins_blue: u32,
	#[serde(rename = "wins_domination_red")]
	pub wins_red: u32,
	#[serde(rename = "kills_domination")]
	pub kills: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct TeamDeathmatch {
	#[serde(rename = "wins_teamdeathmatch_blu")]
	pub wins_blue: u32,
	#[serde(rename = "wins_teamdeathmatch_red")]
	pub wins_red: u32,
	#[serde(rename = "kills_teamdeathmatch")]
	pub kills: u32,
}
