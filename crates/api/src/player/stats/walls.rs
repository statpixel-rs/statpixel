use extra::meters::Meters;

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "walls",
		pretty = "§e§lThe Walls",
		plain = "The Walls",
		field(ident = "wins", colour = "green"),
		field(ident = "losses", colour = "red"),
		field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
		field(ident = "kills", colour = "green"),
		field(ident = "deaths", colour = "red"),
		field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
		field(ident = "blocks_placed", colour = "green"),
		field(ident = "blocks_broken", colour = "red"),
		field(ident = "activations", colour = "gold"),
		label(
			tr = "bow-accuracy",
			ident = "arrows_hit",
			div = "arrows_shot",
			colour = "gold",
			percent = "u32"
		),
		label(ident = "distance_walked", colour = "blue"),
		label(ident = "distance_fallen", colour = "red"),
		label(ident = "iron_broken", colour = "gray")
	)
)]
#[serde(default)]
pub struct Walls {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode()))]
	pub standard: Standard,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Standard {
	#[serde(rename = "wins_standard")]
	pub wins: u32,
	#[serde(rename = "losses_standard")]
	pub losses: u32,
	#[serde(rename = "kills_standard")]
	pub kills: u32,
	#[serde(rename = "deaths_standard")]
	pub deaths: u32,
	#[serde(rename = "arrows_fired_standard")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_standard")]
	pub arrows_hit: u32,
	#[serde(rename = "blocks_broken_standard")]
	pub blocks_broken: u32,
	#[serde(rename = "blocks_placed_standard")]
	pub blocks_placed: u32,
	#[serde(rename = "activations_standard")]
	pub activations: u32,
	#[serde(rename = "meters_walked_standard")]
	pub distance_walked: Meters,
	#[serde(rename = "meters_fallen_standard")]
	pub distance_fallen: Meters,
	#[serde(rename = "iron_ore_broken_standard")]
	pub iron_broken: u32,
}
