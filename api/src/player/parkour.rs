#[derive(serde::Deserialize, bincode::Encode, bincode::Decode)]
pub struct Completion {
	#[serde(rename = "timeTook")]
	pub time: extra::milliseconds::Milliseconds,
}
