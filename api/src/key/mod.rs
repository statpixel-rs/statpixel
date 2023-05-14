use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
	pub record: Key,
	pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct Key {
	pub limit: u32,
}
