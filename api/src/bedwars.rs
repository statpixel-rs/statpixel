use crate::{image::Image, shape, skyblock::materials::MATERIALS};

impl HotbarItem {
	pub fn into_slot(&self) -> shape::Slot {
		let id = match self {
			Self::Melee => "GOLD_SWORD",
			Self::Tools => "IRON_PICKAXE",
			Self::Blocks => "HARD_CLAY",
			Self::Utility => "TNT",
			Self::Ranged => "BOW",
			Self::Potions => "BREWING_STAND",
			Self::Tracker => "COMPASS",
			Self::None => return shape::Slot(None, 1),
		};

		shape::Slot(MATERIALS.get(id).map(Image::image), 1)
	}
}

impl ShopItem {
	pub fn into_slot(&self) -> shape::Slot {
		let (id, count) = match self {
			Self::Wool => ("WOOL", 16),
			Self::HardenedClay => ("HARD_CLAY", 16),
			Self::BlastProofGlass => ("GLASS", 4),
			Self::EndStone => ("ENDER_STONE", 12),
			Self::Ladder => ("LADDER", 8),
			Self::OakWoodPlanks => ("WOOD", 16),
			Self::Obsidian => ("OBSIDIAN", 4),
			Self::StoneSword => ("STONE_SWORD", 1),
			Self::IronSword => ("IRON_SWORD", 1),
			Self::DiamondSword => ("DIAMOND_SWORD", 1),
			Self::StickKnockbackI => ("STICK", 1),
			Self::ChainmailBoots => ("CHAINMAIL_BOOTS", 1),
			Self::IronBoots => ("IRON_BOOTS", 1),
			Self::DiamondBoots => ("DIAMOND_BOOTS", 1),
			Self::WoodenPickaxe => ("WOOD_PICKAXE", 1),
			Self::WoodenAxe => ("WOOD_AXE", 1),
			Self::Shears => ("SHEARS", 1),
			Self::Arrow => ("ARROW", 6),
			Self::Bow => ("BOW", 1),
			Self::BowPowerI => ("HURRICANE_BOW", 1),
			Self::BowPowerIPunchI => ("DEATH_BOW_STANDBY", 1),
			Self::SpeedIIPotion45Seconds => ("SPEED_POTION", 1),
			Self::JumpVPotion45Seconds => ("JUMP_POTION", 1),
			Self::InvisibilityPotion30Seconds => ("INVISIBILITY_POTION", 1),
			Self::GoldenApple => ("GOLDEN_APPLE", 1),
			Self::Bedbug => ("SNOW_BALL", 1),
			Self::DreamDefender => ("SPAWN_EGG", 1),
			Self::Fireball => ("FIRE_CHARGE", 1),
			Self::Tnt => ("TNT", 1),
			Self::EnderPearl => ("ENDER_PEARL", 1),
			Self::WaterBucket => ("WATER_BUCKET", 1),
			Self::BridgeEgg => ("EGG", 1),
			Self::MagicMilk => ("MILK_BUCKET", 1),
			Self::Sponge => ("SPONGE", 4),
			Self::CompactPopUpTower => ("TRAPPED_CHEST", 1),
			Self::None => return shape::Slot(None, 1),
		};

		shape::Slot(MATERIALS.get(id).map(Image::image), count)
	}
}
