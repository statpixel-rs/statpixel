use extra::seconds::{Seconds, SecondsOption};

fn default_level_fmt() -> String {
	"§71".to_string()
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode)]
#[cfg_attr(
	feature = "game",
	derive(derive::Game),
	game(
		path = "sky_wars",
		pretty = "§b§lSky §f§lWars",
		plain = "Sky Wars",
		calc = "minecraft::calc::sky_wars",
		field(ident = "wins", colour = "green"),
		field(ident = "losses", colour = "red"),
		field(tr = "wlr", ident = "wins", div = "losses", colour = "gold"),
		field(ident = "kills", colour = "green"),
		field(ident = "deaths", colour = "red"),
		field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold"),
		field(
			ident = "time_played",
			colour = "green",
			skip_mode = "solo_insane",
			skip_mode = "team_insane",
			skip_chart
		),
		field(
			tr = "bow-accuracy",
			ident = "arrows_hit",
			div = "arrows_shot",
			percent = "u32",
			colour = "red",
			skip_mode = "solo_insane",
			skip_mode = "team_insane"
		),
		field(
			ident = "fastest_win",
			colour = "gold",
			min,
			skip_mode = "solo_insane",
			skip_mode = "team_insane",
			skip_chart
		)
	)
)]
#[serde(default)]
pub struct SkyWars {
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "gold")))]
	pub coins: u32,
	#[serde(
		rename(deserialize = "skywars_chests"),
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	#[cfg_attr(feature = "game", game(label(colour = "yellow")))]
	pub loot_chests: u32,
	#[cfg_attr(feature = "game", game(label(colour = "blue")))]
	pub opals: u32,
	#[cfg_attr(feature = "game", game(label(colour = "dark_purple")))]
	pub heads: u32,
	#[serde(deserialize_with = "crate::de::from::f32_to_u32")]
	#[cfg_attr(feature = "game", game(label(colour = "aqua")))]
	pub souls: u32,
	#[serde(
		rename = "cosmetic_tokens",
		deserialize_with = "crate::de::from::f32_to_u32"
	)]
	#[cfg_attr(feature = "game", game(label(colour = "dark_green")))]
	pub tokens: u32,
	#[serde(rename = "egg_thrown")]
	#[cfg_attr(feature = "game", game(label(colour = "yellow")))]
	pub eggs_thrown: u32,
	#[serde(rename(deserialize = "levelFormatted"), default = "default_level_fmt")]
	pub level_fmt: String,
	#[serde(
		rename(deserialize = "skywars_experience"),
		deserialize_with = "crate::de::from::f64_to_u64"
	)]
	#[cfg_attr(feature = "game", game(xp))]
	pub xp: u64,
	pub win_streak: u32,

	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode(hypixel = "solo_normal")))]
	pub solo_normal: SoloNormal,
	#[serde(flatten)]
	#[cfg_attr(
		feature = "game",
		game(mode(
			hypixel = "solo_insane",
			field(
				ident = "time_played",
				colour = "green",
				path = "stats.sky_wars.solo_normal",
				skip_chart
			),
			field(
				ident = "arrows_hit",
				colour = "red",
				div = "arrows_shot",
				percent = "u32",
				tr = "bow-accuracy",
				path = "stats.sky_wars.solo_normal"
			),
			field(
				ident = "fastest_win",
				colour = "gold",
				path = "stats.sky_wars.solo_normal",
				skip_chart
			)
		))
	)]
	pub solo_insane: SoloInsane,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode(hypixel = "teams_normal")))]
	pub team_normal: TeamNormal,
	#[serde(flatten)]
	#[cfg_attr(
		feature = "game",
		game(mode(
			hypixel = "teams_insane",
			field(
				ident = "time_played",
				colour = "green",
				path = "stats.sky_wars.solo_normal",
				skip_chart
			),
			field(
				ident = "arrows_hit",
				colour = "red",
				div = "arrows_shot",
				percent = "u32",
				tr = "bow-accuracy",
				path = "stats.sky_wars.solo_normal"
			),
			field(
				ident = "fastest_win",
				colour = "gold",
				path = "stats.sky_wars.solo_normal",
				skip_chart
			)
		))
	)]
	pub team_insane: TeamInsane,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode(hypixel = "mega_doubles")))]
	pub mega_double: MegaDouble,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode(hypixel = "mega_normal")))]
	pub mega_normal: MegaNormal,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode(hypixel = "ranked")))]
	pub ranked: Ranked,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode(hypixel = "solo_lab")))]
	pub solo_lab: SoloLab,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode(hypixel = "teams_lab")))]
	pub team_lab: TeamLab,
	#[serde(flatten)]
	#[cfg_attr(feature = "game", game(mode(hypixel = "solo_tourney")))]
	pub tourney: Tourney,
}

// We need to implement this manually for the level_fmt default
impl Default for SkyWars {
	fn default() -> Self {
		Self {
			level_fmt: default_level_fmt(),
			coins: u32::default(),
			loot_chests: u32::default(),
			opals: u32::default(),
			heads: u32::default(),
			souls: u32::default(),
			tokens: u32::default(),
			eggs_thrown: u32::default(),
			xp: u64::default(),
			win_streak: u32::default(),
			solo_normal: SoloNormal::default(),
			solo_insane: SoloInsane::default(),
			team_normal: TeamNormal::default(),
			team_insane: TeamInsane::default(),
			mega_double: MegaDouble::default(),
			mega_normal: MegaNormal::default(),
			ranked: Ranked::default(),
			solo_lab: SoloLab::default(),
			team_lab: TeamLab::default(),
			tourney: Tourney::default(),
		}
	}
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloNormal {
	#[serde(rename = "losses_solo_normal")]
	pub losses: u32,
	#[serde(rename = "wins_solo_normal")]
	pub wins: u32,
	#[serde(rename = "kills_solo_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_normal")]
	pub deaths: u32,
	#[serde(rename = "time_played_solo")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_shot_solo")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_solo")]
	pub arrows_hit: u32,
	#[serde(rename = "fastest_win_solo")]
	pub fastest_win: SecondsOption,
	#[serde(rename = "winstreak_solo")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloInsane {
	#[serde(rename = "losses_solo_insane")]
	pub losses: u32,
	#[serde(rename = "wins_solo_insane")]
	pub wins: u32,
	#[serde(rename = "kills_solo_insane")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_insane")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct TeamNormal {
	#[serde(rename = "losses_team_normal")]
	pub losses: u32,
	#[serde(rename = "wins_team_normal")]
	pub wins: u32,
	#[serde(rename = "kills_team_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_team_normal")]
	pub deaths: u32,
	#[serde(rename = "time_played_team")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_shot_team")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_team")]
	pub arrows_hit: u32,
	#[serde(rename = "fastest_win_team")]
	pub fastest_win: SecondsOption,
	#[serde(rename = "winstreak_team")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct TeamInsane {
	#[serde(rename = "losses_team_insane")]
	pub losses: u32,
	#[serde(rename = "wins_team_insane")]
	pub wins: u32,
	#[serde(rename = "kills_team_insane")]
	pub kills: u32,
	#[serde(rename = "deaths_team_insane")]
	pub deaths: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct MegaDouble {
	#[serde(rename = "losses_mega_doubles")]
	pub losses: u32,
	#[serde(rename = "wins_mega_doubles")]
	pub wins: u32,
	#[serde(rename = "kills_mega_doubles")]
	pub kills: u32,
	#[serde(rename = "deaths_mega_doubles")]
	pub deaths: u32,
	#[serde(rename = "time_played_mega_doubles")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_shot_mega_doubles")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_mega_doubles")]
	pub arrows_hit: u32,
	#[serde(rename = "fastest_win_mega_doubles")]
	pub fastest_win: SecondsOption,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct MegaNormal {
	#[serde(rename = "losses_mega")]
	pub losses: u32,
	#[serde(rename = "wins_mega")]
	pub wins: u32,
	#[serde(rename = "kills_mega")]
	pub kills: u32,
	#[serde(rename = "deaths_mega")]
	pub deaths: u32,
	#[serde(rename = "time_played_mega")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_shot_mega")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_mega")]
	pub arrows_hit: u32,
	#[serde(rename = "fastest_win_mega")]
	pub fastest_win: SecondsOption,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Ranked {
	#[serde(rename = "losses_ranked")]
	pub losses: u32,
	#[serde(rename = "wins_ranked")]
	pub wins: u32,
	#[serde(rename = "kills_ranked")]
	pub kills: u32,
	#[serde(rename = "deaths_ranked")]
	pub deaths: u32,
	#[serde(rename = "time_played_ranked")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_shot_ranked")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_ranked")]
	pub arrows_hit: u32,
	#[serde(rename = "fastest_win_ranked")]
	pub fastest_win: SecondsOption,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct SoloLab {
	#[serde(rename = "losses_lab_solo")]
	pub losses: u32,
	#[serde(rename = "wins_lab_solo")]
	pub wins: u32,
	#[serde(rename = "kills_lab_solo")]
	pub kills: u32,
	#[serde(rename = "deaths_lab_solo")]
	pub deaths: u32,
	#[serde(rename = "time_played_lab_solo")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_shot_lab_solo")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_lab_solo")]
	pub arrows_hit: u32,
	#[serde(rename = "fastest_win_lab_solo")]
	pub fastest_win: SecondsOption,
	#[serde(rename = "winstreak_lab_solo")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct TeamLab {
	#[serde(rename = "losses_lab_team")]
	pub losses: u32,
	#[serde(rename = "wins_lab_team")]
	pub wins: u32,
	#[serde(rename = "kills_lab_team")]
	pub kills: u32,
	#[serde(rename = "deaths_lab_team")]
	pub deaths: u32,
	#[serde(rename = "time_played_lab_team")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_shot_lab_team")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_lab_team")]
	pub arrows_hit: u32,
	#[serde(rename = "fastest_win_lab_team")]
	pub fastest_win: SecondsOption,
	#[serde(rename = "winstreak_lab_team")]
	pub win_streak: u32,
}

#[derive(serde::Deserialize, bincode::Decode, bincode::Encode, Default)]
#[serde(default)]
pub struct Tourney {
	#[serde(rename = "losses_tourney")]
	pub losses: u32,
	#[serde(rename = "wins_tourney")]
	pub wins: u32,
	#[serde(rename = "kills_tourney")]
	pub kills: u32,
	#[serde(rename = "deaths_tourney")]
	pub deaths: u32,
	#[serde(rename = "time_played_tourney")]
	pub time_played: Seconds,
	#[serde(rename = "arrows_shot_tourney")]
	pub arrows_shot: u32,
	#[serde(rename = "arrows_hit_tourney")]
	pub arrows_hit: u32,
	#[serde(rename = "fastest_win_tourney")]
	pub fastest_win: SecondsOption,
	#[serde(rename = "winstreak_tourney")]
	pub win_streak: u32,
}
