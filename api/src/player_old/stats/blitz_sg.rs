use crate::player::stats::blitz_sg;

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct BlitzSg {
	pub coins: i32,
	pub armorer: blitz_sg::Armorer,
	pub scout: blitz_sg::Scout,
	pub speleologist: blitz_sg::Speleologist,
	pub random: blitz_sg::Random,
	pub rogue: blitz_sg::Rogue,
	pub rambo: blitz_sg::Rambo,
	pub troll: blitz_sg::Troll,
	pub horsetamer: blitz_sg::HorseTamer,
	pub wolftamer: blitz_sg::WolfTamer,
	pub warrior: blitz_sg::Warrior,
	pub phoenix: blitz_sg::Phoenix,
	pub donkeytamer: blitz_sg::DonkeyTamer,
	pub ranger: blitz_sg::Ranger,
	pub archer: blitz_sg::Archer,
	pub necromancer: blitz_sg::Necromancer,
	pub meatmaster: blitz_sg::Meatmaster,
	pub tim: blitz_sg::Tim,
	pub pigman: blitz_sg::Pigman,
	pub creepertamer: blitz_sg::CreeperTamer,
	pub florist: blitz_sg::Florist,
	pub warlock: blitz_sg::Warlock,
	pub milkman: blitz_sg::Milkman,
	pub astronaut: blitz_sg::Astronaut,
	pub blaze: blitz_sg::Blaze,
}

impl From<BlitzSg> for blitz_sg::BlitzSg {
	fn from(value: BlitzSg) -> Self {
		Self {
			coins: value.coins,
			armorer: value.armorer,
			scout: value.scout,
			speleologist: value.speleologist,
			random: value.random,
			rogue: value.rogue,
			rambo: value.rambo,
			troll: value.troll,
			horsetamer: value.horsetamer,
			wolftamer: value.wolftamer,
			warrior: value.warrior,
			phoenix: value.phoenix,
			donkeytamer: value.donkeytamer,
			ranger: value.ranger,
			archer: value.archer,
			necromancer: value.necromancer,
			meatmaster: value.meatmaster,
			tim: value.tim,
			pigman: value.pigman,
			creepertamer: value.creepertamer,
			florist: value.florist,
			warlock: value.warlock,
			milkman: value.milkman,
			astronaut: value.astronaut,
			blaze: value.blaze,
			..blitz_sg::BlitzSg::default()
		}
	}
}
