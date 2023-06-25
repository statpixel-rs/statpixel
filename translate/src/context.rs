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
	Component(&'c serenity::ComponentInteraction),
}

pub struct Context<'c> {
	data: &'c super::Data,
	author: &'c serenity::User,
	locale: Option<Locale>,
	serenity: &'c serenity::Context,
	interaction: ContextInteraction<'c>,
}

impl<'c> Context<'c> {
	pub fn from_component(
		ctx: &'c serenity::Context,
		data: &'c super::Data,
		interaction: &'c serenity::ComponentInteraction,
	) -> Self {
		Self {
			data,
			author: &interaction.user,
			locale: Locale::from_str(&interaction.locale).ok(),
			serenity: ctx,
			interaction: ContextInteraction::Component(interaction),
		}
	}

	pub fn from_poise(ctx: &'c super::Context<'c>) -> Self {
		Self {
			data: ctx.data(),
			author: ctx.author(),
			locale: ctx.locale().and_then(|l| Locale::from_str(l).ok()),
			serenity: ctx.discord(),
			interaction: ContextInteraction::Command(ctx),
		}
	}

	pub fn locale(&self) -> Option<&Locale> {
		self.locale.as_ref()
	}

	pub fn author(&self) -> &serenity::User {
		self.author
	}

	pub fn data(&self) -> &super::Data {
		self.data
	}

	pub fn id(&self) -> u64 {
		match self.interaction {
			ContextInteraction::Command(ctx) => ctx.id(),
			ContextInteraction::Component(interaction) => interaction.id.0.get(),
		}
	}

	pub async fn defer(&self) -> Result<(), serenity::Error> {
		match self.interaction {
			ContextInteraction::Command(ctx) => ctx.defer().await,
			ContextInteraction::Component(interaction) => {
				interaction.defer(self.serenity).await?;

				Ok(())
			}
		}
	}

	pub async fn send(&self, reply: poise::CreateReply) -> Result<(), serenity::Error> {
		match self.interaction {
			ContextInteraction::Command(ctx) => ctx.send(reply).await.map(|_| ()),
			ContextInteraction::Component(interaction) => self._send(interaction, reply).await,
		}
	}

	async fn _send(
		&self,
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

		interaction.edit_response(self.serenity, edit).await?;

		Ok(())
	}
}
