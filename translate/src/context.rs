use std::{
	str::FromStr,
	sync::atomic::{AtomicBool, Ordering},
};

use poise::serenity_prelude as serenity;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Locale {
	bg,
	cs,
	da,
	de,
	el,
	#[default]
	en_US,
	es_ES,
	fi,
	fr,
	hi,
	hr,
	hu,
	it,
	ja,
	ko,
	lt,
	nl,
	no,
	pl,
	pt_BR,
	ro,
	ru,
	sv_SE,
	th,
	tr,
	uk,
	zh_CN,
	zh_TW,
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
			bg => "bg",
			cs => "cs",
			da => "da",
			de => "de",
			el => "el",
			en_US => "en-US",
			es_ES => "es-ES",
			fi => "fi",
			fr => "fr",
			hi => "hi",
			hr => "hr",
			hu => "hu",
			it => "it",
			ja => "ja",
			ko => "ko",
			lt => "lt",
			nl => "nl",
			no => "no",
			pl => "pl",
			pt_BR => "pt-BR",
			ro => "ro",
			ru => "ru",
			sv_SE => "sv-SE",
			th => "th",
			tr => "tr",
			uk => "uk",
			zh_CN => "zh-CN",
			zh_TW => "zh-TW",
		}
	}
}

impl FromStr for Locale {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"bg" => Locale::bg,
			"cs" => Locale::cs,
			"da" => Locale::da,
			"de" => Locale::de,
			"el" => Locale::el,
			"es-ES" => Locale::es_ES,
			"fi" => Locale::fi,
			"fr" => Locale::fr,
			"hi" => Locale::hi,
			"hr" => Locale::hr,
			"hu" => Locale::hu,
			"it" => Locale::it,
			"ja" => Locale::ja,
			"ko" => Locale::ko,
			"lt" => Locale::lt,
			"nl" => Locale::nl,
			"no" => Locale::no,
			"pl" => Locale::pl,
			"pt-BR" => Locale::pt_BR,
			"ro" => Locale::ro,
			"ru" => Locale::ru,
			"sv-SE" => Locale::sv_SE,
			"th" => Locale::th,
			"tr" => Locale::tr,
			"uk" => Locale::uk,
			"zh-CN" => Locale::zh_CN,
			"zh-TW" => Locale::zh_TW,
			_ => Locale::en_US,
		})
	}
}

pub enum ContextInteraction<'c> {
	Command(&'c super::Context<'c>),
	Component {
		interaction: &'c serenity::ComponentInteraction,
		data: &'c super::Data,
		ctx: &'c serenity::Context,
		deferred: AtomicBool,
	},
	Modal {
		interaction: &'c serenity::ModalInteraction,
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
				deferred: AtomicBool::new(false),
			},
		}
	}

	pub fn from_modal(
		ctx: &'c serenity::Context,
		data: &'c super::Data,
		interaction: &'c serenity::ModalInteraction,
	) -> Self {
		Self {
			locale: Locale::from_str(&interaction.locale).ok(),
			interaction: ContextInteraction::Modal {
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

	pub fn author(&self) -> Option<&serenity::User> {
		match self {
			Self {
				interaction: ContextInteraction::Command(ctx),
				..
			} => Some(ctx.author()),
			Self {
				interaction: ContextInteraction::Component { interaction, .. },
				..
			} => Some(&interaction.user),
			Self {
				interaction: ContextInteraction::External(..) | ContextInteraction::Modal { .. },
				..
			} => None,
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
				interaction: ContextInteraction::Modal { data, .. },
				..
			} => data,
			Self {
				interaction: ContextInteraction::External(data),
				..
			} => data,
		}
	}

	pub fn discord(&self) -> &serenity::Context {
		match self.interaction {
			ContextInteraction::Command(ctx) => ctx.discord(),
			ContextInteraction::Component { ctx, .. } => ctx,
			ContextInteraction::Modal { ctx, .. } => ctx,
			ContextInteraction::External(..) => {
				unreachable!("Context::discord() called on external context")
			}
		}
	}

	pub async fn defer(&self) -> Result<(), serenity::Error> {
		match &self.interaction {
			ContextInteraction::Command(ctx) => ctx.defer().await,
			ContextInteraction::Component {
				interaction,
				ctx,
				deferred,
				..
			} => {
				if deferred.load(Ordering::SeqCst) {
					return Ok(());
				}

				deferred.store(true, Ordering::SeqCst);
				interaction.defer(ctx).await?;

				Ok(())
			}
			ContextInteraction::External(..) | ContextInteraction::Modal { .. } => Ok(()),
		}
	}

	pub async fn send(&self, reply: poise::CreateReply) -> Result<(), serenity::Error> {
		match &self.interaction {
			ContextInteraction::Command(ctx) => ctx.send(reply).await.map(|_| ()),
			ContextInteraction::Component {
				interaction,
				ctx,
				deferred,
				..
			} => self.send_component(ctx, interaction, deferred, reply).await,
			ContextInteraction::Modal {
				interaction, ctx, ..
			} => self.send_modal(ctx, interaction, reply).await,
			ContextInteraction::External(..) => {
				unreachable!("Context::send() called on external context")
			}
		}
	}

	pub async fn reply(&self, reply: poise::CreateReply) -> Result<(), serenity::Error> {
		match &self.interaction {
			ContextInteraction::Command(ctx) => ctx.send(reply).await.map(|_| ()),
			ContextInteraction::Component {
				interaction,
				ctx,
				deferred,
				..
			} => self.send_component(ctx, interaction, deferred, reply).await,
			ContextInteraction::Modal {
				interaction, ctx, ..
			} => self.reply_modal(ctx, interaction, reply).await,
			ContextInteraction::External(..) => {
				unreachable!("Context::send() called on external context")
			}
		}
	}

	async fn send_modal(
		&self,
		ctx: &serenity::Context,
		interaction: &serenity::ModalInteraction,
		data: poise::CreateReply,
	) -> Result<(), serenity::Error> {
		let mut edit = serenity::CreateInteractionResponseMessage::new().embeds(data.embeds);

		if let Some(content) = data.content {
			edit = edit.content(content);
		}

		if let Some(components) = data.components {
			edit = edit.components(components);
		}

		edit = edit.files(data.attachments);

		interaction
			.create_response(
				ctx,
				serenity::CreateInteractionResponse::UpdateMessage(edit),
			)
			.await?;

		Ok(())
	}

	async fn reply_modal(
		&self,
		ctx: &serenity::Context,
		interaction: &serenity::ModalInteraction,
		data: poise::CreateReply,
	) -> Result<(), serenity::Error> {
		let mut edit = serenity::CreateInteractionResponseMessage::new().embeds(data.embeds);

		if let Some(content) = data.content {
			edit = edit.content(content);
		}

		if let Some(components) = data.components {
			edit = edit.components(components);
		}

		edit = edit.files(data.attachments);

		interaction
			.create_response(ctx, serenity::CreateInteractionResponse::Message(edit))
			.await?;

		Ok(())
	}

	async fn send_component(
		&self,
		ctx: &serenity::Context,
		interaction: &serenity::ComponentInteraction,
		deferred: &AtomicBool,
		data: poise::CreateReply,
	) -> Result<(), serenity::Error> {
		if deferred.load(Ordering::SeqCst) {
			let mut edit = serenity::EditInteractionResponse::new().embeds(data.embeds);

			if let Some(content) = data.content {
				edit = edit.content(content);
			}

			if let Some(components) = data.components {
				edit = edit.components(components);
			}

			if !data.attachments.is_empty() {
				edit = edit.clear_existing_attachments();
			}

			for attachment in data.attachments {
				edit = edit.new_attachment(attachment);
			}

			interaction.edit_response(ctx, edit).await?;
		} else {
			let mut edit = serenity::CreateInteractionResponseMessage::new().embeds(data.embeds);

			if let Some(content) = data.content {
				edit = edit.content(content);
			}

			if let Some(components) = data.components {
				edit = edit.components(components);
			}

			edit = edit.files(data.attachments);

			interaction
				.create_response(
					ctx,
					serenity::CreateInteractionResponse::UpdateMessage(edit),
				)
				.await?;
		}

		Ok(())
	}
}
