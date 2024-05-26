use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

use diesel_async::pooled_connection::deadpool::{self, Object};
use diesel_async::AsyncPgConnection;
use poise::serenity_prelude::{self as serenity, CacheHttp};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum Locale {
	bg,
	cs,
	da,
	de,
	el,
	#[default]
	#[cfg_attr(feature = "serde", serde(rename = "en-US"))]
	en_US,
	#[cfg_attr(feature = "serde", serde(rename = "es-ES"))]
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
	#[cfg_attr(feature = "serde", serde(rename = "pt-BR"))]
	pt_BR,
	ro,
	ru,
	#[cfg_attr(feature = "serde", serde(rename = "sv-SE"))]
	sv_SE,
	th,
	tr,
	uk,
	#[cfg_attr(feature = "serde", serde(rename = "zh-CN"))]
	zh_CN,
	#[cfg_attr(feature = "serde", serde(rename = "zh-TW"))]
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
	#[cfg(feature = "error")]
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
	Empty,
}

pub struct Context<'c> {
	locale: Option<Locale>,
	interaction: ContextInteraction<'c>,
	automated: bool,
	data: Option<&'c super::Data>,
}

impl<'c> Context<'c> {
	pub const EMPTY: Self = Self {
		locale: None,
		interaction: ContextInteraction::Empty,
		automated: true,
		data: None,
	};

	pub async fn connection(&self) -> Result<Object<AsyncPgConnection>, deadpool::PoolError> {
		self.data_opt()
			.ok_or(deadpool::PoolError::Closed)?
			.pool
			.get()
			.await
	}

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
			automated: false,
			data: Some(data),
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
			automated: false,
			data: Some(data),
		}
	}

	#[cfg(feature = "error")]
	pub fn from_poise(ctx: &'c super::Context<'c>) -> Self {
		Self {
			locale: ctx.locale().and_then(|l| Locale::from_str(l).ok()),
			interaction: ContextInteraction::Command(ctx),
			automated: false,
			data: Some(crate::DATA.get().unwrap()),
		}
	}

	pub fn external(data: &'c super::Data) -> Self {
		Self {
			locale: None,
			interaction: ContextInteraction::External(data),
			automated: false,
			data: Some(data),
		}
	}

	pub fn external_with_locale(data: &'c super::Data, locale: Option<Locale>) -> Self {
		Self {
			locale,
			interaction: ContextInteraction::External(data),
			automated: false,
			data: Some(data),
		}
	}

	pub fn automated(data: &'c super::Data) -> Self {
		Self {
			locale: None,
			interaction: ContextInteraction::External(data),
			automated: true,
			data: Some(data),
		}
	}

	pub fn is_automated(&self) -> bool {
		self.automated
	}

	pub fn locale(&self) -> Option<Locale> {
		self.locale
	}

	pub fn author(&self) -> Option<&serenity::User> {
		match self {
			#[cfg(feature = "error")]
			Self {
				interaction: ContextInteraction::Command(ctx),
				..
			} => Some(ctx.author()),
			Self {
				interaction: ContextInteraction::Component { interaction, .. },
				..
			} => Some(&interaction.user),
			Self {
				interaction:
					ContextInteraction::External(..)
					| ContextInteraction::Modal { .. }
					| ContextInteraction::Empty,
				..
			} => None,
		}
	}

	pub fn data_opt(&self) -> Option<&super::Data> {
		self.data
	}

	pub fn data(&self) -> &super::Data {
		self.data_opt()
			.expect("Context::data() called on empty context")
	}

	pub fn discord(&self) -> &serenity::Context {
		match self.interaction {
			#[cfg(feature = "error")]
			ContextInteraction::Command(ctx) => ctx.serenity_context(),
			ContextInteraction::Component { ctx, .. } => ctx,
			ContextInteraction::Modal { ctx, .. } => ctx,
			ContextInteraction::External(..) => {
				unreachable!("Context::discord() called on external context")
			}
			ContextInteraction::Empty => unreachable!("Context::discord() called on empty context"),
		}
	}

	pub async fn defer(&self) -> Result<(), serenity::Error> {
		match &self.interaction {
			#[cfg(feature = "error")]
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
				interaction.defer(ctx.http()).await?;

				Ok(())
			}
			ContextInteraction::External(..)
			| ContextInteraction::Modal { .. }
			| ContextInteraction::Empty => Ok(()),
		}
	}

	pub async fn send(&self, reply: poise::CreateReply<'c>) -> Result<(), serenity::Error> {
		match &self.interaction {
			#[cfg(feature = "error")]
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
			ContextInteraction::Empty => unreachable!("Context::send() called on empty context"),
		}
	}

	pub async fn reply(&self, reply: poise::CreateReply<'c>) -> Result<(), serenity::Error> {
		match &self.interaction {
			#[cfg(feature = "error")]
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
			ContextInteraction::Empty => unreachable!("Context::send() called on empty context"),
		}
	}

	pub fn channel_id(&self) -> Option<serenity::ChannelId> {
		match &self.interaction {
			#[cfg(feature = "error")]
			ContextInteraction::Command(ctx) => Some(ctx.channel_id()),
			ContextInteraction::Component { interaction, .. } => Some(interaction.channel_id),
			ContextInteraction::Modal { interaction, .. } => Some(interaction.channel_id),
			ContextInteraction::External(..) | ContextInteraction::Empty => None,
		}
	}

	async fn send_modal(
		&self,
		ctx: &serenity::Context,
		interaction: &serenity::ModalInteraction,
		data: poise::CreateReply<'_>,
	) -> Result<(), serenity::Error> {
		let message = data.to_slash_initial_response(
			serenity::CreateInteractionResponseMessage::new().add_files([]),
		);

		interaction
			.create_response(
				ctx.http(),
				serenity::CreateInteractionResponse::UpdateMessage(message),
			)
			.await?;

		Ok(())
	}

	async fn reply_modal(
		&self,
		ctx: &serenity::Context,
		interaction: &serenity::ModalInteraction,
		data: poise::CreateReply<'_>,
	) -> Result<(), serenity::Error> {
		let message = data.to_slash_initial_response(
			serenity::CreateInteractionResponseMessage::new().add_files([]),
		);

		interaction
			.create_response(
				ctx.http(),
				serenity::CreateInteractionResponse::Message(message),
			)
			.await?;

		Ok(())
	}

	async fn send_component(
		&self,
		ctx: &serenity::Context,
		interaction: &serenity::ComponentInteraction,
		deferred: &AtomicBool,
		data: poise::CreateReply<'_>,
	) -> Result<(), serenity::Error> {
		if deferred.load(Ordering::SeqCst) {
			let message = data.to_slash_initial_response_edit(
				serenity::EditInteractionResponse::new().clear_attachments(),
			);

			interaction.edit_response(ctx.http(), message).await?;
		} else {
			let message = data.to_slash_initial_response(
				serenity::CreateInteractionResponseMessage::new().files([]),
			);

			interaction
				.create_response(
					ctx.http(),
					serenity::CreateInteractionResponse::UpdateMessage(message),
				)
				.await?;
		}

		Ok(())
	}
}
