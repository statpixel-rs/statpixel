use std::str::FromStr;

use poise::serenity_prelude as serenity;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Locale {
	de,
	#[default]
	en_US,
	es_ES,
	fr,
	ja,
	ru,
}

impl Default for &'_ Locale {
	fn default() -> Self {
		&Locale::en_US
	}
}

impl Locale {
	pub fn as_str(&self) -> &'static str {
		use Locale::*;

		match self {
			de => "de",
			en_US => "en-US",
			es_ES => "es-ES",
			fr => "fr",
			ja => "ja",
			ru => "ru",
		}
	}
}

impl FromStr for Locale {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"de" => Self::de,
			"es-ES" => Self::es_ES,
			"fr" => Self::fr,
			"ja" => Self::ja,
			"ru" => Self::ru,
			_ => Self::en_US,
		})
	}
}

#[derive(Clone, Copy)]
pub enum ContextInteraction<'c> {
	Command(&'c super::Context<'c>),
	Component {
		interaction: &'c serenity::ComponentInteraction,
		data: &'c super::Data,
		ctx: &'c serenity::Context,
	},
	External(&'c super::Data),
}

pub struct Context<'c> {
	locale: Option<Locale>,
	interaction: ContextInteraction<'c>,
}

impl<'c> Context<'c> {
	pub fn from_component(
		ctx: &'c serenity::Context,
		data: &'c super::Data,
		interaction: &'c serenity::ComponentInteraction,
	) -> Self {
		Self {
			locale: Locale::from_str(&interaction.locale).ok(),
			interaction: ContextInteraction::Component {
				interaction,
				data,
				ctx,
			},
		}
	}

	pub fn from_poise(ctx: &'c super::Context<'c>) -> Self {
		Self {
			locale: ctx.locale().and_then(|l| Locale::from_str(l).ok()),
			interaction: ContextInteraction::Command(ctx),
		}
	}

	pub fn external(data: &'c super::Data) -> Self {
		Self {
			locale: None,
			interaction: ContextInteraction::External(data),
		}
	}

	pub fn locale(&self) -> Option<&Locale> {
		self.locale.as_ref()
	}

	pub fn author(&self) -> &serenity::User {
		match self {
			Self {
				interaction: ContextInteraction::Command(ctx),
				..
			} => ctx.author(),
			Self {
				interaction: ContextInteraction::Component { interaction, .. },
				..
			} => &interaction.user,
			Self {
				interaction: ContextInteraction::External(..),
				..
			} => {
				unreachable!("Context::author() called on external context")
			}
		}
	}

	pub fn data(&self) -> &super::Data {
		match self {
			Self {
				interaction: ContextInteraction::Command(ctx),
				..
			} => ctx.data(),
			Self {
				interaction: ContextInteraction::Component { data, .. },
				..
			} => data,
			Self {
				interaction: ContextInteraction::External(data),
				..
			} => data,
		}
	}

	pub fn id(&self) -> u64 {
		match self.interaction {
			ContextInteraction::Command(ctx) => ctx.id(),
			ContextInteraction::Component { interaction, .. } => interaction.id.0.get(),
			ContextInteraction::External(..) => {
				unreachable!("Context::id() called on external context")
			}
		}
	}

	pub async fn defer(&self) -> Result<(), serenity::Error> {
		match self.interaction {
			ContextInteraction::Command(ctx) => ctx.defer().await,
			ContextInteraction::Component {
				interaction, ctx, ..
			} => {
				interaction.defer(ctx).await?;

				Ok(())
			}
			ContextInteraction::External(..) => Ok(()),
		}
	}

	pub async fn send(&self, reply: poise::CreateReply) -> Result<(), serenity::Error> {
		match self.interaction {
			ContextInteraction::Command(ctx) => ctx.send(reply).await.map(|_| ()),
			ContextInteraction::Component {
				interaction, ctx, ..
			} => self._send(ctx, interaction, reply).await,
			ContextInteraction::External(..) => {
				unreachable!("Context::send() called on external context")
			}
		}
	}

	async fn _send(
		&self,
		ctx: &serenity::Context,
		interaction: &serenity::ComponentInteraction,
		data: poise::CreateReply,
	) -> Result<(), serenity::Error> {
		let mut edit = serenity::EditInteractionResponse::new().embeds(data.embeds);

		if let Some(content) = data.content {
			edit = edit.content(content);
		}

		if let Some(components) = data.components {
			edit = edit.components(components);
		}

		edit = edit.clear_existing_attachments();

		for attachment in data.attachments {
			edit = edit.new_attachment(attachment);
		}

		interaction.edit_response(ctx, edit).await?;

		Ok(())
	}
}
