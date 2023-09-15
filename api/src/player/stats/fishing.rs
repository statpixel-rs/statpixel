#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
pub struct Outer {
	#[serde(default)]
	pub fishing: Wrapper,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
pub struct Wrapper {
	#[serde(default)]
	pub stats: Stats,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
pub struct Stats {
	#[serde(default)]
	pub permanent: Fishing,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "fishing.fishing.stats.permanent",
		pretty = "§b§lFishing",
		plain = "Fishing",
		field(ident = "fish", colour = "green"),
		field(ident = "junk", colour = "red"),
		field(ident = "treasure", colour = "gold"),
		label(ident = "xp", path = "", tr = "experience", colour = "yellow", nominal),
		label(ident = "karma", path = "", colour = "light_purple", nominal),
		label(ident = "rewards", path = "", colour = "gold", nominal),
		label(ident = "friend_requests", path = "", colour = "green", nominal),
		label(
			ident = "playtime",
			path = "",
			tr = "time-played",
			colour = "gold",
			nominal
		),
		label(ident = "first_login", path = "", colour = "aqua", nominal),
		label(ident = "last_login", path = "", colour = "blue", nominal),
	)
)]
#[serde(default)]
pub struct Fishing {
	#[cfg_attr(feature = "game", game(mode()))]
	pub water: Water,
	#[cfg_attr(feature = "game", game(mode()))]
	pub lava: Lava,
	#[cfg_attr(feature = "game", game(mode()))]
	pub ice: Ice,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Water {
	pub fish: u32,
	pub junk: u32,
	pub treasure: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Lava {
	pub fish: u32,
	pub junk: u32,
	pub treasure: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Ice {
	pub fish: u32,
	pub junk: u32,
	pub treasure: u32,
}
