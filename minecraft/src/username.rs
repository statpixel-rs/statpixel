use std::fmt::Display;

use crate::Error;

#[derive(Debug)]
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
	/// # Errors
	/// Returns an error if the username is longer than 16 characters or contains invalid characters.
	pub fn try_from_str(username: &str) -> Result<Self, Error> {
		if username.len() > 16 || username.is_empty() {
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

	#[must_use]
	pub fn as_str(&self) -> &str {
		&self.username
	}
}

impl Display for Username {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.username)
	}
}

#[cfg(test)]
mod tests {
	use std::assert_matches::assert_matches;

	use super::*;

	#[test]
	fn test_valid_username() {
		let username = Username::try_from_str("Notch");

		assert_matches!(username, Ok(_));
		assert_eq!("Notch", username.unwrap().as_str());
	}

	#[test]
	fn test_invalid_username_chars() {
		let username = Username::try_from_str("Notch!");

		assert_matches!(username, Err(Error::InvalidUsername(_)));
	}

	#[test]
	fn test_invalid_username_length() {
		let username = Username::try_from_str("12345678901234567");

		assert_matches!(username, Err(Error::InvalidUsername(_)));
	}

	#[test]
	fn test_invalid_username_empty() {
		let username = Username::try_from_str("");

		assert_matches!(username, Err(Error::InvalidUsername(_)));
	}
}
