use macros::Game;

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default, Game)]
#[game(
	path = "turbo_kart_racers",
	pretty = "§a§lKart Racers",
	plain = "Kart Racers",
	field(ident = "bronze_trophies", colour = "red"),
	field(ident = "silver_trophies", colour = "gray"),
	field(ident = "gold_trophies", colour = "gold")
)]
#[serde(default)]
pub struct TurboKartRacers {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[game(label(colour = "aqua"))]
	pub box_pickups: u32,
	#[serde(rename = "coins_picked_up")]
	#[game(label(colour = "blue"))]
	pub coin_pickups: u32,
	#[game(label(colour = "yellow"), nominal)]
	pub grand_prix: bool,
	#[serde(rename = "show_win_prefix")]
	#[game(label(colour = "light_purple"), nominal)]
	pub show_prefix: bool,
	#[game(label(colour = "green"))]
	pub wins: u32,
	#[game(label(colour = "red"))]
	pub laps_completed: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub normal: Normal,
}

#[derive(serde::Deserialize, serde::Serialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Normal {
	#[serde(rename = "bronze_trophy")]
	pub bronze_trophies: u32,
	#[serde(rename = "silver_trophy")]
	pub silver_trophies: u32,
	#[serde(rename = "gold_trophy")]
	pub gold_trophies: u32,
}
