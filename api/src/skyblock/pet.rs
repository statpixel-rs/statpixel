use serde::Deserialize;

use super::networth::{calc::worth, Prices};
use crate::nbt::inventory::Item;

const TOTAL_XP: [u32; 200] = [
	100,
	210,
	330,
	460,
	605,
	765,
	940,
	1_130,
	1_340,
	1_570,
	1_820,
	2_095,
	2_395,
	2_725,
	3_085,
	3_485,
	3_925,
	4_415,
	4_955,
	5_555,
	6_215,
	6_945,
	7_745,
	8_625,
	9_585,
	10_635,
	11_785,
	13_045,
	14_425,
	15_935,
	17_585,
	19_385,
	21_345,
	23_475,
	25_785,
	28_285,
	30_985,
	33_905,
	37_065,
	40_485,
	44_185,
	48_185,
	52_535,
	57_285,
	62_485,
	68_185,
	74_485,
	81_485,
	89_285,
	97_985,
	107_685,
	118_485,
	130_485,
	143_785,
	158_485,
	174_685,
	192_485,
	211_985,
	233_285,
	256_485,
	281_685,
	309_085,
	338_885,
	371_285,
	406_485,
	444_685,
	486_085,
	530_885,
	579_285,
	631_485,
	687_685,
	748_085,
	812_885,
	882_285,
	956_485,
	1_035_685,
	1_120_385,
	1_211_085,
	1_308_285,
	1_412_485,
	1_524_185,
	1_643_885,
	1_772_085,
	1_909_285,
	2_055_985,
	2_212_685,
	2_380_385,
	2_560_085,
	2_752_785,
	2_959_485,
	3_181_185,
	3_418_885,
	3_673_585,
	3_946_285,
	4_237_985,
	4_549_685,
	4_883_385,
	5_241_085,
	5_624_785,
	6_036_485,
	6_478_185,
	6_954_885,
	7_471_585,
	8_033_285,
	8_644_985,
	9_311_685,
	10_038_385,
	10_830_085,
	11_691_785,
	12_628_485,
	13_645_185,
	14_746_885,
	15_938_585,
	17_225_285,
	18_611_985,
	20_108_685,
	21_725_385,
	23_472_085,
	25_358_785,
	25_358_785,
	25_358_786,
	27_245_486,
	29_132_186,
	31_018_886,
	32_905_586,
	34_792_286,
	36_678_986,
	38_565_686,
	40_452_386,
	42_339_086,
	44_225_786,
	46_112_486,
	47_999_186,
	49_885_886,
	51_772_586,
	53_659_286,
	55_545_986,
	57_432_686,
	59_319_386,
	61_206_086,
	63_092_786,
	64_979_486,
	66_866_186,
	68_752_886,
	70_639_586,
	72_526_286,
	74_412_986,
	76_299_686,
	78_186_386,
	80_073_086,
	81_959_786,
	83_846_486,
	85_733_186,
	87_619_886,
	89_506_586,
	91_393_286,
	93_279_986,
	95_166_686,
	97_053_386,
	98_940_086,
	100_826_786,
	102_713_486,
	104_600_186,
	106_486_886,
	108_373_586,
	110_260_286,
	112_146_986,
	114_033_686,
	115_920_386,
	117_807_086,
	119_693_786,
	121_580_486,
	123_467_186,
	125_353_886,
	127_240_586,
	129_127_286,
	131_013_986,
	132_900_686,
	134_787_386,
	136_674_086,
	138_560_786,
	140_447_486,
	142_334_186,
	144_220_886,
	146_107_586,
	147_994_286,
	149_880_986,
	151_767_686,
	153_654_386,
	155_541_086,
	157_427_786,
	159_314_486,
	161_201_186,
	163_087_886,
	164_974_586,
	166_861_286,
	168_747_986,
	170_634_686,
	172_521_386,
	174_408_086,
];

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Tier {
	Common,
	Uncommon,
	Rare,
	Epic,
	Legendary,
	Mythic,
	Divine,
	Special,
	VerySpecial,
}

impl Tier {
	#[must_use]
	pub fn upgrade(&self) -> Self {
		use Tier::*;

		match self {
			Common => Uncommon,
			Uncommon => Rare,
			Rare => Epic,
			Epic => Legendary,
			Legendary => Mythic,
			Mythic => Divine,
			Divine => Special,
			Special | VerySpecial => VerySpecial,
		}
	}

	#[must_use]
	pub fn level_offset(&self) -> usize {
		use Tier::*;

		match self {
			Common => 0,
			Uncommon => 6,
			Rare => 11,
			Epic => 16,
			Legendary | Mythic | Divine | Special | VerySpecial => 20,
		}
	}

	#[must_use]
	pub fn as_upper_str(&self) -> &'static str {
		use Tier::*;

		match self {
			Common => "COMMON",
			Uncommon => "UNCOMMON",
			Rare => "RARE",
			Epic => "EPIC",
			Legendary => "LEGENDARY",
			Mythic => "MYTHIC",
			Divine => "DIVINE",
			Special => "SPECIAL",
			VerySpecial => "VERY_SPECIAL",
		}
	}

	#[must_use]
	pub fn as_pretty_str(&self) -> &'static str {
		use Tier::*;

		match self {
			Common => "Common",
			Uncommon => "Uncom.",
			Rare => "Rare",
			Epic => "Epic",
			Legendary => "Leg.",
			Mythic => "Mythic",
			Divine => "Divine",
			Special => "Spec.",
			VerySpecial => "V. Spec.",
		}
	}

	#[must_use]
	pub fn colour(&self) -> &'static str {
		use Tier::*;

		match self {
			Common => "§7",
			Uncommon => "§a",
			Rare => "§9",
			Epic => "§5",
			Legendary => "§6§l",
			Mythic => "§d§l",
			Divine => "§c§l",
			Special => "§b§l",
			VerySpecial => "§e§l",
		}
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pet {
	#[serde(rename = "type")]
	pub id: String,
	#[serde(
		deserialize_with = "crate::player::stats::from_trunc_f32_to_u32",
		rename = "exp"
	)]
	pub xp: u32,
	pub tier: Tier,
	#[serde(rename = "heldItem")]
	pub item: Option<String>,
	pub skin: Option<String>,
	#[serde(rename = "candyUsed")]
	pub candy: u32,
}

impl From<Pet> for Item {
	fn from(value: Pet) -> Self {
		let name = value.name();

		Item::empty_wth_name(value.id, name)
	}
}

impl Pet {
	#[must_use]
	pub fn name(&self) -> String {
		let tier = self.tier.as_pretty_str();
		let level = self.level().0.to_string();
		let colour = self.tier.colour();

		let mut name =
			String::with_capacity(tier.len() + 1 + self.id.len() + colour.len() + 10 + level.len());

		name.push_str(colour);
		name.push_str(tier);
		name.push(' ');

		// Capitalize every letter after an underscore in the id
		let mut chars = self.id.chars();

		if let Some(first) = chars.next() {
			name.extend(first.to_uppercase());
		}

		while let Some(c) = chars.next() {
			if c == '_' {
				name.push(' ');

				if let Some(next) = chars.next() {
					name.push(next.to_ascii_uppercase());
				}
			} else {
				name.push(c.to_ascii_lowercase());
			}
		}

		name.push_str(" §8[§f");
		name.push_str(level.as_str());
		name.push_str("§8]");

		name
	}

	#[must_use]
	pub fn max_level(&self) -> usize {
		if self.id == "GOLDEN_DRAGON" {
			200
		} else {
			100
		}
	}

	#[must_use]
	pub fn try_upgrade(&self) -> Option<Tier> {
		if let Some(ref item) = self.item {
			if item == "pet_item_tier_boost"
				|| item == "pet_item_vampire_fang"
				|| item == "pet_item_toy_jerry"
			{
				self.tier.upgrade().into()
			} else {
				None
			}
		} else {
			None
		}
	}

	#[must_use]
	pub fn level(&self) -> (u8, u32) {
		let offset = self
			.try_upgrade()
			.as_ref()
			.unwrap_or(&self.tier)
			.level_offset();

		let max_level = TOTAL_XP.len().min(self.max_level() + offset);

		let level = TOTAL_XP[offset..max_level]
			.binary_search_by(|xp| xp.cmp(&self.xp))
			.map_or_else(|level| level, |level| level - 1)
			.min(self.max_level() - 1);

		let max_xp = TOTAL_XP[max_level - 1];

		#[allow(clippy::cast_possible_truncation)]
		(level as u8 + 1, max_xp)
	}

	#[must_use]
	pub fn key(&self, level: u8) -> Option<String> {
		let tier = self.tier.as_upper_str();
		let id = self.id.as_str();

		self.skin.as_ref().map(|s| match level {
			..100 => format!("LVL_1_{tier}_{id}_SKINNED_{s}"),
			100..200 => format!("LVL_100_{tier}_{id}_SKINNED_{s}"),
			_ => format!("LVL_200_{tier}_{id}_SKINNED_{s}"),
		})
	}

	#[must_use]
	pub fn base_key(&self, level: u8) -> String {
		let tier = self.tier.as_upper_str();
		let id = self.id.as_str();

		match level {
			..100 => format!("LVL_1_{tier}_{id}"),
			100..200 => format!("LVL_100_{tier}_{id}"),
			_ => format!("LVL_200_{tier}_{id}"),
		}
	}

	fn base_worth(&self, prices: &Prices, level: u8) -> f64 {
		self.key(level)
			.and_then(|k| prices.get(&k))
			.unwrap_or(&0.)
			.max(*prices.get(&self.base_key(level)).unwrap_or(&0.))
	}

	#[must_use]
	pub fn worth(&self, prices: &Prices) -> Option<f64> {
		let (level, max) = self.level();

		let mut price = match level {
			..100 => {
				let base = self.base_worth(prices, 1);
				let curr = self.base_worth(prices, 100);

				(curr - base) / f64::from(max) * f64::from(self.xp) + base
			}
			100 => self.base_worth(prices, 100),
			101..200 => {
				if level % 100 == 1 {
					self.base_worth(prices, 100)
				} else {
					let base = self.base_worth(prices, 100);
					let curr = self.base_worth(prices, 200);

					(curr - base) / f64::from(max) * f64::from(self.xp) + base
				}
			}
			200.. => self.base_worth(prices, 200),
		};

		if let Some(ref item) = self.item {
			price += prices
				.get(item)
				.map(|v| *v * worth::PET_ITEM)
				.unwrap_or_default();
		}

		if self.candy > 0
			&& self.id != "ENDER_DRAGON"
			&& self.id != "GOLDEN_DRAGON"
			&& self.id != "SCATHA"
		{
			let reduced = price * worth::PET_CANDY;

			if level == 100 {
				if price >= 5_000_000. {
					price = (price - 5_000_000.).max(reduced);
				}
			} else if price >= 2_500_000. {
				price = (price - 2_500_000.).max(reduced);
			}
		}

		Some(price)
	}
}
