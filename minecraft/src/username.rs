use std::fmt::Display;

use crate::Error;

pub struct Username {
	username: String,
}

const ALLOWED_CHARS: [char; 63] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
	'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A',
	'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
	'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Username {
	pub fn parse_str(username: &str) -> Result<Self, Error> {
		if username.len() > 16 {
			return Err(Error::InvalidUsername(username.to_string()));
		}

		let valid = username.chars().all(|c| ALLOWED_CHARS.contains(&c));

		if valid {
			Ok(Self {
				username: username.to_string(),
			})
		} else {
			Err(Error::InvalidUsername(username.to_string()))
		}
	}

	pub fn as_str(&self) -> &str {
		&self.username
	}
}

impl Display for Username {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.username)
	}
}
