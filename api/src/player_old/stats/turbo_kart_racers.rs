use crate::player::stats::turbo_kart_racers;

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct TurboKartRacers {
	pub coins: i32,
	pub box_pickups: u32,
	pub coin_pickups: u32,
	pub grand_prix: bool,
	pub show_prefix: bool,
	pub wins: u32,
	pub normal: turbo_kart_racers::Normal,
}

impl From<TurboKartRacers> for turbo_kart_racers::TurboKartRacers {
	fn from(value: TurboKartRacers) -> Self {
		Self {
			coins: value.coins,
			box_pickups: value.box_pickups,
			coin_pickups: value.coin_pickups,
			grand_prix: value.grand_prix,
			show_prefix: value.show_prefix,
			wins: value.wins,
			normal: value.normal,
			..turbo_kart_racers::TurboKartRacers::default()
		}
	}
}
