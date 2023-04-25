use std::fmt::Display;

use poise::CreateReply;

pub fn success_embed<'a, 'b, S>(
	reply: &'b mut CreateReply<'a>,
	title: S,
	description: S,
) -> &'b mut CreateReply<'a>
where
	S: Into<String> + Display,
{
	reply.embed(|e| {
		e.title(title)
			.description(description)
			.colour(crate::EMBED_COLOUR)
	});

	reply
}

pub fn error_embed<'a, 'b, S>(
	reply: &'b mut CreateReply<'a>,
	title: S,
	description: S,
) -> &'b mut CreateReply<'a>
where
	S: Into<String> + Display,
{
	reply.embed(|e| {
		e.title(title)
			.description(description)
			.colour(crate::EMBED_COLOUR_ERROR)
	});

	reply
}

pub fn escape_username(username: &str) -> String {
	username.replace('_', "\\_")
}
