use crate::player::stats::warlords;

#[derive(bincode::Decode, bincode::Encode, Default)]
pub struct Warlords {
	pub coins: i32,
	pub damage_dealt: u32,
	pub damage_taken: u32,
	pub health_regenerated: u32,
	pub hide_prestige: bool,
	pub kills: u32,
	pub deaths: u32,
	pub mvps: u32,

	pub capture_the_flag: warlords::CaptureTheFlag,
	pub domination: warlords::Domination,
	pub team_deathmatch: warlords::TeamDeathmatch,
}

impl From<Warlords> for warlords::Warlords {
	fn from(value: Warlords) -> Self {
		Self {
			coins: value.coins,
			damage_dealt: value.damage_dealt,
			damage_taken: value.damage_taken,
			health_regenerated: value.health_regenerated,
			hide_prestige: value.hide_prestige,
			kills: value.kills,
			deaths: value.deaths,
			mvps: value.mvps,
			capture_the_flag: value.capture_the_flag,
			domination: value.domination,
			team_deathmatch: value.team_deathmatch,
			win_streak: 0,
		}
	}
}
