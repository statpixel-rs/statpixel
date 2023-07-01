use crate::player::stats::bed_wars;

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct BedWars {
	pub coins: i32,
	pub loot_chests: u32,
	pub xp: u64,

	pub solo: Data,
	pub double: Data,
	pub three: Data,
	pub four: Data,
	pub solo_rush: Data,
	pub double_rush: Data,
	pub four_rush: Data,
	pub solo_ultimate: Data,
	pub double_ultimate: Data,
	pub four_ultimate: Data,
	pub castle: Data,
	pub double_lucky: Data,
	pub four_lucky: Data,
	pub double_voidless: Data,
	pub four_voidless: Data,
	pub double_armed: Data,
	pub four_armed: Data,
	pub double_underworld: Data,
	pub four_underworld: Data,
	pub double_swap: Data,
	pub four_swap: Data,
}

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct Data {
	pub wins: u32,
	pub losses: u32,
	pub kills: u32,
	pub deaths: u32,
	pub final_kills: u32,
	pub final_deaths: u32,
	pub beds_broken: u32,
	pub beds_lost: u32,
	pub iron_collected: u32,
	pub gold_collected: u32,
	pub diamond_collected: u32,
	pub emerald_collected: u32,
	pub items_purchased: u32,
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
					final_kills: value.final_kills,
					final_deaths: value.final_deaths,
					beds_broken: value.beds_broken,
					beds_lost: value.beds_lost,
					iron_collected: value.iron_collected,
					gold_collected: value.gold_collected,
					diamond_collected: value.diamond_collected,
					emerald_collected: value.emerald_collected,
					items_purchased: value.items_purchased,
					win_streak: 0,
				}
			}
		}
	};
}

impl_from_data!(bed_wars::Solo);
impl_from_data!(bed_wars::Double);
impl_from_data!(bed_wars::Three);
impl_from_data!(bed_wars::Four);
impl_from_data!(bed_wars::SoloRush);
impl_from_data!(bed_wars::DoubleRush);
impl_from_data!(bed_wars::FourRush);
impl_from_data!(bed_wars::SoloUltimate);
impl_from_data!(bed_wars::DoubleUltimate);
impl_from_data!(bed_wars::FourUltimate);
impl_from_data!(bed_wars::Castle);
impl_from_data!(bed_wars::DoubleLucky);
impl_from_data!(bed_wars::FourLucky);
impl_from_data!(bed_wars::DoubleVoidless);
impl_from_data!(bed_wars::FourVoidless);
impl_from_data!(bed_wars::DoubleArmed);
impl_from_data!(bed_wars::FourArmed);
impl_from_data!(bed_wars::DoubleUnderworld);
impl_from_data!(bed_wars::FourUnderworld);
impl_from_data!(bed_wars::DoubleSwap);
impl_from_data!(bed_wars::FourSwap);

impl From<BedWars> for bed_wars::BedWars {
	fn from(value: BedWars) -> Self {
		Self {
			coins: value.coins,
			loot_chests: value.loot_chests,
			xp: value.xp,
			win_streak: 0,
			solo: value.solo.into(),
			double: value.double.into(),
			three: value.three.into(),
			four: value.four.into(),
			solo_rush: value.solo_rush.into(),
			double_rush: value.double_rush.into(),
			four_rush: value.four_rush.into(),
			solo_ultimate: value.solo_ultimate.into(),
			double_ultimate: value.double_ultimate.into(),
			four_ultimate: value.four_ultimate.into(),
			castle: value.castle.into(),
			double_lucky: value.double_lucky.into(),
			four_lucky: value.four_lucky.into(),
			double_voidless: value.double_voidless.into(),
			four_voidless: value.four_voidless.into(),
			double_armed: value.double_armed.into(),
			four_armed: value.four_armed.into(),
			double_underworld: value.double_underworld.into(),
			four_underworld: value.four_underworld.into(),
			double_swap: value.double_swap.into(),
			four_swap: value.four_swap.into(),
		}
	}
}
