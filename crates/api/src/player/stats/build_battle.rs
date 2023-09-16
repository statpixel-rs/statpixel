#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "build_battle",
		pretty = "§d§lBuild Battle",
		plain = "Build Battle",
		calc = "minecraft::calc::build_battle",
		field(ident = "wins", colour = "green")
	)
)]
#[serde(default)]
pub struct BuildBattle {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,
	#[cfg_attr(feature = "game", game(label(colour = "yellow"), xp))]
	pub score: u32,
	#[serde(rename = "solo_most_points")]
	#[cfg_attr(feature = "game", game(label(colour = "light_purple")))]
	pub most_points_solo: u32,
	#[serde(rename = "teams_most_points")]
	#[cfg_attr(feature = "game", game(label(colour = "dark_purple")))]
	pub most_points_team: u32,
	#[serde(rename = "total_votes")]
	#[cfg_attr(feature = "game", game(label(colour = "aqua")))]
	pub votes: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub solo_normal: SoloNormal,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub solo_pro: SoloPro,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub teams_normal: TeamNormal,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub guess_the_build: GuessTheBuild,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloNormal {
	#[serde(rename = "wins_solo_normal")]
	pub wins: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloPro {
	#[serde(rename = "wins_solo_pro")]
	pub wins: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct TeamNormal {
	#[serde(rename = "wins_teams_normal")]
	pub wins: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct GuessTheBuild {
	#[serde(rename = "wins_guess_the_build")]
	pub wins: u32,
}
