use extra::seconds::Seconds;

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "tnt_games",
		pretty = "§4§lTNT §c§lGames",
		plain = "TNT Games",
		field(ident = "wins", colour = "green")
	)
)]
#[serde(default)]
pub struct TntGames {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,

	#[serde(flatten)]
	#[cfg_attr(
		feature = "game",
		game(mode(
			field(colour = "red", ident = "deaths"),
			field(colour = "gold", ident = "record")
		))
	)]
	pub tnt_run: TntRun,
	#[serde(flatten)]
	#[cfg_attr(
		feature = "game",
		game(mode(
			field(colour = "red", ident = "double_jumps"),
			field(colour = "gold", ident = "record"),
			field(colour = "green", ident = "kills"),
			field(colour = "red", ident = "deaths"),
			field(tr = "kdr", colour = "gold", ident = "kills", div = "deaths")
		))
	)]
	pub pvp_run: PvpRun,
	#[serde(flatten)]
	#[cfg_attr(
		feature = "game",
		game(mode(
			field(colour = "red", ident = "deaths"),
			field(colour = "gold", ident = "tags")
		))
	)]
	pub bow_spleef: BowSpleef,
	#[serde(flatten)]
	#[cfg_attr(
		feature = "game",
		game(mode(
			field(colour = "red", ident = "air_time"),
			field(colour = "gold", ident = "points"),
			field(colour = "green", ident = "kills"),
			field(colour = "red", ident = "deaths"),
			field(tr = "kdr", colour = "gold", ident = "kills", div = "deaths")
		))
	)]
	pub wizard: Wizard,
	#[serde(flatten)]
	#[cfg_attr(
		feature = "game",
		game(mode(
			field(colour = "green", ident = "kills"),
			field(colour = "red", ident = "deaths"),
			field(tr = "kdr", colour = "gold", ident = "kills", div = "deaths")
		))
	)]
	pub tnt_tag: TntTag,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct TntRun {
	#[serde(rename = "wins_tntrun")]
	pub wins: u32,
	#[serde(rename = "deaths_tntrun")]
	pub deaths: u32,
	#[serde(rename = "record_tntrun")]
	pub record: Seconds,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct TntTag {
	#[serde(rename = "wins_tntag")]
	pub wins: u32,
	#[serde(rename = "kills_tntag")]
	pub kills: u32,
	#[serde(rename = "deaths_tntag")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct PvpRun {
	#[serde(rename = "wins_pvprun")]
	pub wins: u32,
	#[serde(rename = "new_pvprun_double_jumps")]
	pub double_jumps: u32,
	#[serde(rename = "record_pvprun")]
	pub record: Seconds,
	#[serde(rename = "kills_pvprun")]
	pub kills: u32,
	#[serde(rename = "deaths_pvprun")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct BowSpleef {
	#[serde(rename = "wins_bowspleef")]
	pub wins: u32,
	#[serde(rename = "deaths_bowspleef")]
	pub deaths: u32,
	#[serde(rename = "tags_bowspleef")]
	pub tags: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Wizard {
	#[serde(rename = "wins_capture")]
	pub wins: u32,
	#[serde(rename = "air_time_capture")]
	pub air_time: Seconds,
	#[serde(rename = "points_capture")]
	pub points: u32,
	#[serde(rename = "kills_capture")]
	pub kills: u32,
	#[serde(rename = "deaths_capture")]
	pub deaths: u32,
}
