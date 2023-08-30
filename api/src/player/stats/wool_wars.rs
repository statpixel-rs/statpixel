#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Progression {
	#[serde(rename = "available_layers")]
	pub layers: u32,
	#[serde(
		rename(deserialize = "experience"),
		deserialize_with = "crate::de::from::f64_to_u64"
	)]
	pub xp: u64,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Outer {
	#[serde(rename = "wool_wars")]
	pub inner: Inner,
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	pub coins: u32,
	pub progression: Progression,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Inner {
	pub stats: WoolWars,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "wool_wars.inner.stats",
		pretty = "§f§lWool §7§lWars",
		plain = "Wool Wars",
		calc = "minecraft::calc::wool_wars",
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
		field(ident = "powerups_collected", colour = "red"),
		field(ident = "wool_placed", colour = "gold"),
		label(ident = "coins", path = "stats.wool_wars", colour = "gold"),
		label(
			ident = "layers",
			path = "stats.wool_wars.progression",
			colour = "blue"
		),
		xp = "wool_wars.progression.xp"
	)
)]
#[serde(default)]
pub struct WoolWars {
	#[cfg_attr(feature = "game", game(label(colour = "aqua")))]
	pub blocks_broken: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub normal: Normal,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Normal {
	pub wins: u32,
	#[serde(rename = "games_played")]
	pub games: u32,
	pub kills: u32,
	pub deaths: u32,
	pub assists: u32,
	#[serde(rename = "powerups_gotten")]
	pub powerups_collected: u32,
	pub wool_placed: u32,
}
