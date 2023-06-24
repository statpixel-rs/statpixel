use std::str::FromStr;

use poise::serenity_prelude::{self as serenity, MessageComponentInteraction};

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
	Component(&'c serenity::MessageComponentInteraction),
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
		interaction: &'c serenity::MessageComponentInteraction,
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
			serenity: ctx.serenity_context(),
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
			ContextInteraction::Component(interaction) => interaction.id.0,
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

	pub async fn send<'a>(
		&self,
		builder: impl for<'b> FnOnce(&'b mut poise::CreateReply<'a>) -> &'b mut poise::CreateReply<'a>,
	) -> Result<(), serenity::Error> {
		match self.interaction {
			ContextInteraction::Command(ctx) => ctx.send(builder).await.map(|_| ()),
			ContextInteraction::Component(interaction) => {
				let mut reply = poise::CreateReply::default();

				builder(&mut reply);

				self._send(interaction, reply).await
			}
		}
	}

	async fn _send(
		&self,
		interaction: &MessageComponentInteraction,
		data: poise::CreateReply<'_>,
	) -> Result<(), serenity::Error> {
		interaction
			.edit_followup_message(self.serenity, interaction.message.id, |f| {
				data.to_slash_followup_response(f);
				f
			})
			.await?;

		Ok(())
	}
}
