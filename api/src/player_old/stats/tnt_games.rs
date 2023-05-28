use crate::player::stats::tnt_games;

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct TntGames {
	pub coins: i32,
	pub tnt_run: tnt_games::TntRun,
	pub pvp_run: tnt_games::PvpRun,
	pub bow_spleef: tnt_games::BowSpleef,
	pub wizard: tnt_games::Wizard,
}

impl From<TntGames> for tnt_games::TntGames {
	fn from(value: TntGames) -> Self {
		Self {
			coins: value.coins,
			tnt_run: value.tnt_run,
			pvp_run: value.pvp_run,
			bow_spleef: value.bow_spleef,
			wizard: value.wizard,
			tnt_tag: tnt_games::TntTag::default(),
		}
	}
}
