use crate::player::stats::arena;

#[derive(bincode::Encode, bincode::Decode, Default)]
pub struct Arena {
	pub coins: i32,
	pub magical_chests: u32,
	pub magical_keys: u32,
	pub rating: u32,

	pub solo: Data,
	pub double: Data,
	pub four: Data,
}

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct Data {
	pub wins: u32,
	pub losses: u32,
	pub kills: u32,
	pub deaths: u32,
}

macro_rules! impl_from_data {
	($name: ty) => {
		impl From<Data> for $name {
			fn from(value: Data) -> Self {
				Self {
					wins: value.wins,
					losses: value.losses,
					kills: value.kills,
					deaths: value.deaths,
					win_streak: 0,
				}
			}
		}
	};
}

impl_from_data!(arena::Solo);
impl_from_data!(arena::Double);
impl_from_data!(arena::Four);

impl From<Arena> for arena::Arena {
	fn from(value: Arena) -> Self {
		Self {
			coins: value.coins,
			magical_chests: value.magical_chests,
			magical_keys: value.magical_keys,
			rating: value.rating,
			solo: value.solo.into(),
			double: value.double.into(),
			four: value.four.into(),
		}
	}
}
