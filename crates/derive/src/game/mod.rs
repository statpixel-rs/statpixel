mod block;
mod impls;
mod key;
mod label;
mod mode;
mod prelude;
mod structs;

use std::borrow::Cow;

use darling::{ast, FromDeriveInput, ToTokens};
use minecraft::text::parse::minecraft_string;
use prelude::*;
use quote::quote;
use structs::*;

use crate::{game::key::Access, util::ident};

use self::{
	impls::{Crates, Idents, State},
	key::Side,
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(game), supports(struct_named))]
pub(crate) struct GameInputReceiver {
	pub ident: syn::Ident,
	pub data: ast::Data<(), GameFieldReceiver>,

	/// The path to the game data in the PlayerStats struct.
	pub path: String,

	/// A pretty name for the game, coloured with Minecraft escape codes.
	pub pretty: String,

	/// A plain name for the game with no colouring.
	pub plain: String,

	/// A path to the module with level calculations.
	pub calc: Option<syn::Path>,

	/// The fields to include in the Overall mode.
	/// These must be present in all [`Mode`]s.
	///
	/// If this is empty, it is assumed that you have an
	/// Overall mode already defined.
	#[darling(multiple)]
	pub field: Vec<OverallFieldData>,

	/// The additional fields to include in the info header.
	#[darling(multiple)]
	pub label: Vec<InfoFieldData>,

	/// The path (from `stats`) to the XP field
	pub xp: Option<String>,
	/// The path (from the current mode) to the XP field
	/// This takes precedence over `xp`
	pub xp_local: Option<String>,
}

impl ToTokens for GameInputReceiver {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let pretty = self.pretty.as_str();
		let plain = self.plain.as_str();

		let state = State::new(self);
		let Crates {
			poise,
			minecraft,
			api,
			bincode,
			bitcode,
			uuid,
			translate,
			chrono,
			skia,
			serde,
			hypixel,
			redis,
			..
		} = &state.crates;
		let Idents {
			mode_enum,
			kind_enum,
			calc,
			path_to_game,
			..
		} = &state.idents;

		let game_ident = &self.ident;
		let blocks = self.blocks();
		let labels = self.labels();
		let modes = self.modes();
		let overall_modes = self.overall_modes();
		let condensed_modes = modes.iter().take(5).collect::<Vec<_>>();

		let blocks_sum = self.block_shapes_sum(&overall_modes);
		let blocks_diff_sum = self.block_shapes_diff_sum(&overall_modes);
		let labels_sum = self.label_shapes_sum(&overall_modes);
		let labels_diff_sum = self.label_shapes_diff_sum(&overall_modes);

		let block_lines = blocks.len() as u8
			+ condensed_modes.iter().fold(0u8, |a, m| {
				let blocks = m
					.blocks()
					.iter()
					.filter(|b| {
						!blocks
							.iter()
							.any(|i| i.var_id().to_string() == b.var_id().to_string())
					})
					.count();

				a.max(blocks as u8)
			});

		let label_lines_first = labels.iter().fold((0u8, 0u8), |(a, b), l| {
			if l.is_static {
				(a, b + 1)
			} else {
				(a + 1, b)
			}
		});
		let label_lines = label_lines_first.0;
		let label_lines_first = label_lines_first.0.max(label_lines_first.1);

		let condensed_labels_sum =
			self.condensed_label_shapes_sum(&overall_modes, label_lines_first);
		let condensed_labels_diff_sum =
			self.condensed_label_shapes_diff_sum(&overall_modes, label_lines_first);
		let condensed_blocks_sum = self.condensed_block_shapes_sum(&overall_modes);
		let condensed_blocks_diff_sum = self.condensed_block_shapes_diff_sum(&overall_modes);

		let overall_block_idents = blocks
			.iter()
			.map(|b| b.var_id().to_string())
			.collect::<Vec<_>>();
		#[allow(clippy::unnecessary_to_owned)]
		let mode_blocks = modes
			.iter()
			.flat_map(|m| m.blocks().into_owned().into_iter())
			.filter_map(|b| {
				let var_id = b.var_id().to_string();

				if overall_block_idents.iter().any(|id| id.eq(&var_id)) {
					None
				} else {
					Some((var_id, b))
				}
			})
			.collect::<std::collections::HashMap<_, _>>();

		let mode_blocks = mode_blocks.values().collect::<Vec<_>>();

		let overall_ident = syn::parse_str::<syn::Type>("Overall").unwrap();

		tokens.extend({
			let size = minecraft_string(pretty).count();

			quote! {
				pub const LABEL: [#minecraft::text::Text; #size] = #minecraft::text::parse::minecraft_text(#pretty);
				pub const PRETTY: &str = #pretty;
				pub const PLAIN: &str = #plain;
			}
		});

		let mode_enum_tr = modes.iter().map(|m| {
			let ty = m.ty();

			quote!(#mode_enum::#ty => #ty::tr())
		});

		let mode_from_u8_str = modes.iter().enumerate().map(|(idx, m)| {
			let idx = (idx + 1).to_string();
			let ty = m.ty();

			quote!(#idx => #mode_enum::#ty)
		});

		tokens.extend({
			let rows = modes.iter().map(|m| m.ty()).collect::<Vec<_>>();
			let from_str = modes.iter().filter_map(|m| {
				let ty = m.ty();
				let st = m.name()?;

				Some(quote!(#st => #mode_enum::#ty))
			});

			let mut slice = rows
				.iter()
				.map(|id| (quote!(#id).to_string(), quote!(#mode_enum::#id)))
				.collect::<Vec<_>>();

			slice.sort_by(|a, b| a.0.cmp(&b.0));

			let slice = slice.into_iter().map(|(_, id)| id);

			let to_u32 = rows.iter().enumerate().map(|(i, r)| {
				let i = i as u32 + 1;

				quote!(#mode_enum::#r => #i)
			});

			let from_u32 = rows.iter().enumerate().map(|(i, r)| {
				let i = i as u32 + 1;

				quote!(#i => #mode_enum::#r)
			});

			quote! {
				#[derive(::std::fmt::Debug, #poise::ChoiceParameter, Clone, Copy, #bincode::Encode, #bincode::Decode, #bitcode::Encode, #bitcode::Decode)]
				pub enum #mode_enum {
					#overall_ident,
					#(#rows),*
				}

				impl #mode_enum {
					pub fn slice() -> &'static [#mode_enum] {
						const MODES: &[#mode_enum] = &[
							#mode_enum::#overall_ident,
							#(#slice),*
						];

						MODES
					}

					pub fn tr(&self) -> &str {
						match self {
							#mode_enum::#overall_ident => #overall_ident::tr(),
							#(#mode_enum_tr,)*
						}
					}

					pub fn get_mode(mode: Option<#mode_enum>, session: &#api::player::status::Session) -> #mode_enum {
						if let Some(mode) = mode {
							mode
						} else if session.game_type == Some(#hypixel::game::r#type::Type::#game_ident) && let Some(game_mode) = session.game_mode.as_ref() {
							#mode_enum::from(game_mode.as_str())
						} else {
							#mode_enum::#overall_ident
						}
					}

					pub fn from_u8_str(value: &str) -> #mode_enum {
						match value {
							#(#mode_from_u8_str,)*
							_ => #mode_enum::#overall_ident,
						}
					}
				}

				impl From<&str> for #mode_enum {
					fn from(value: &str) -> #mode_enum {
						match value {
							#(#from_str,)*
							_ => #mode_enum::#overall_ident,
						}
					}
				}

				impl From<&#mode_enum> for u32 {
					fn from(value: &#mode_enum) -> u32 {
						match value {
							#mode_enum::#overall_ident => 0,
							#(#to_u32,)*
						}
					}
				}

				impl From<u32> for #mode_enum {
					fn from(value: u32) -> #mode_enum {
						match value {
							#(#from_u32,)*
							_ => #mode_enum::#overall_ident,
						}
					}
				}
			}
		});

		tokens.extend({
			let mut slice = blocks
				.iter()
				.map(|id| (id.var_id().to_string(), quote!(#kind_enum::#id)))
				.chain(
					labels
						.iter()
						.map(|id| (id.var_id().to_string(), quote!(#kind_enum::#id)))
				)
				.collect::<Vec<_>>();

			slice.sort_by(|a, b| a.0.cmp(&b.0));

			let slice = slice.into_iter().map(|(_, id)| id);

			let blocks_len = blocks.len();
			let labels_len = labels.len();

			let kind_enum_tr = blocks.iter()
				.map(|b| (b.var_id(), b.tr()))
				.chain(labels.iter().map(|l| (l.var_id(), l.tr())))
				.filter_map(|(id, tr)| {
					if tr == "level" {
						None
					} else {
						Some(quote!(#kind_enum::#id => #tr))
					}
				});

			let mode_kind_enum_tr = mode_blocks.iter()
				.map(|b| {
					let tr = b.tr();

					quote!(#kind_enum::#b => #tr)
				});

			let to_u32 = blocks.iter()
				.map(|b| (b.var_id(), b.tr()))
				.chain(labels.iter().map(|l| (l.var_id(), l.tr())))
				.enumerate()
				.filter_map(|(i, (id, tr))| {
					let i = i as u32 + 1;

					if tr == "level" {
						None
					} else {
						Some(quote!(#kind_enum::#id => #i))
					}
				});

			let mode_to_u32 = mode_blocks.iter()
				.enumerate()
				.map(|(i, id)| {
					let i = i as u32 + 1 + blocks_len as u32 + labels_len as u32;

					quote!(#kind_enum::#id => #i)
				});

			let from_u32 = blocks.iter()
				.map(|b| (b.var_id(), b.tr()))
				.chain(labels.iter().map(|l| (l.var_id(), l.tr())))
				.enumerate()
				.filter_map(|(i, (id, tr))| {
					let i = i as u32 + 1;

					if tr == "level" {
						None
					} else {
						Some(quote!(#i => #kind_enum::#id))
					}
				});

			let mode_from_u32 = mode_blocks.iter()
				.enumerate()
				.map(|(i, id)| {
					let i = i as u32 + 1 + blocks_len as u32 + labels_len as u32;

					quote!(#i => #kind_enum::#id)
				});

			let try_from_str_lower = blocks.iter()
				.map(|b| (b.var_id(), b.var_id().to_string()))
				.chain(labels.iter().map(|l| (l.var_id(), l.var_id().to_string())))
				.map(|(id, string)| {
					quote!(#string => #kind_enum::#id)
				});

			let labels_no_level = labels.iter().filter(|l| l.tr() != "level");

			quote! {
				#[allow(non_camel_case_types)]
				#[derive(Default, Debug, #serde::Deserialize, #serde::Serialize, #poise::ChoiceParameter, #bincode::Encode, #bincode::Decode, #bitcode::Encode, #bitcode::Decode, Clone, Copy)]
				#[serde(rename_all = "snake_case")]
				pub enum #kind_enum {
					level,
					#[default]
					#(#blocks,)*
					#(#labels_no_level,)*
					#(#mode_blocks,)*
				}

				impl #kind_enum {
					pub fn slice() -> &'static [#kind_enum] {
						const KINDS: &[#kind_enum] = &[
							#kind_enum::level,
							#(#slice),*
						];

						KINDS
					}

					pub fn tr(&self) -> &str {
						match self {
							#kind_enum::level => "level",
							#(#kind_enum_tr,)*
							#(#mode_kind_enum_tr,)*
						}
					}

					pub fn try_from_str_lower(value: &str) -> Option<Self> {
						Some(match value {
							#(#try_from_str_lower,)*
							_ => return None,
						})
					}
				}

				impl From<&#kind_enum> for u32 {
					fn from(value: &#kind_enum) -> u32 {
						match value {
							#kind_enum::level => 0,
							#(#to_u32,)*
							#(#mode_to_u32,)*
						}
					}
				}

				impl From<u32> for #kind_enum {
					fn from(value: u32) -> #kind_enum {
						match value {
							#(#from_u32,)*
							#(#mode_from_u32,)*
							_ => #kind_enum::level,
						}
					}
				}
			}
		});

		for (i, mode) in modes.iter().enumerate() {
			impls::impl_mode(
				tokens,
				&state,
				mode,
				if i < 2 {
					label_lines_first
				} else {
					label_lines
				},
				block_lines,
			);
		}

		let as_root = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #tr), #api::id::command(#api::command::Id::Root {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid,
				}))
			}
		});

		let as_snapshot = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #tr), #api::id::command(#api::command::Id::Snapshot {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid,
					past,
				}))
			}
		});

		let as_at = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #tr), #api::id::command(#api::command::Id::At {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid,
					past,
				}))
			}
		});

		let as_history = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #tr), #api::id::command(#api::command::Id::History {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid,
				}))
			}
		});

		let as_project = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #tr), #api::id::command(#api::command::Id::Project {
					kind: #api::command::ProjectMode::#game_ident(#mode_enum::#ty, kind),
					uuid,
				}))
			}
		});

		let as_compare = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #tr), #api::id::command(#api::command::Id::Compare {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid_lhs,
					uuid_rhs,
				}))
			}
		});

		let min = self.min_sum(&ident("min"), &overall_modes);
		let max = self.max_sum(&ident("max"), &overall_modes);

		tokens.extend(quote! {
			impl #api::canvas::prelude::Mode for #mode_enum {
				type Kind = #kind_enum;

				fn as_root<'c>(
					ctx: &'c #translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow<'c>, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::Root {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid,
								})),
								#(#as_root),*
							].into()
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr(ctx, selected.tr()).into_owned());
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::Root {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid,
						})
					)
				}

				fn as_snapshot<'c>(
					ctx: &'c #translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					past: i64,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow<'c>, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::Snapshot {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid,
									past,
								})),
								#(#as_snapshot),*
							].into()
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr(ctx, selected.tr()).into_owned());
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::Snapshot {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid,
							past,
						})
					)
				}

				fn as_at<'c>(
					ctx: &'c #translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					past: i64,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow<'c>, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::At {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid,
									past,
								})),
								#(#as_at),*
							].into()
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr(ctx, selected.tr()).into_owned());
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::At {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid,
							past,
						})
					)
				}

				fn as_history<'c>(
					ctx: &'c #translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow<'c>, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::History {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid,
								})),
								#(#as_history),*
							].into()
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr(ctx, selected.tr()).into_owned());
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::History {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid,
						})
					)
				}

				fn as_project<'c>(
					ctx: &'c #translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					kind: Self::Kind,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow<'c>, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::Project {
									kind: #api::command::ProjectMode::#game_ident(#mode_enum::#overall_ident, kind),
									uuid,
								})),
								#(#as_project),*
							].into()
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr(ctx, selected.tr()).into_owned());
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::Project {
							kind: #api::command::ProjectMode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident), kind),
							uuid,
						})
					)
				}

				fn as_compare<'c>(
					ctx: &'c #translate::context::Context<'_>,
					uuid_lhs: #uuid::Uuid,
					uuid_rhs: #uuid::Uuid,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow<'c>, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::Compare {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid_lhs,
									uuid_rhs,
								})),
								#(#as_compare),*
							].into()
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr(ctx, selected.tr()).into_owned());
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::Compare {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid_lhs,
							uuid_rhs,
						})
					)
				}
			}
		});

		let xp = self
			.xp(Side::None, path_to_game, None)
			.expect("an xp field should be present for all modes (xp)");

		let xp_lhs: std::borrow::Cow<'_, proc_macro2::TokenStream> = self
			.xp(Side::Lhs, path_to_game, None)
			.expect("an xp field should be present for all modes (xp_lhs)");

		let xp_rhs = self
			.xp(Side::Rhs, path_to_game, None)
			.expect("an xp field should be present for all modes (xp_rhs)");

		let diff_log = blocks
			.iter()
			.filter_map(|b| b.diff_log_sum(&overall_modes, &ident("log")));

		let add_to_pipeline = blocks
			.iter()
			.filter_map(|b| {
				if !b.is_measurable() {
					return None;
				}

				let value = b.value_sum(Side::None, &overall_modes, Access::NoneDiff)?;

				Some(quote!({
					let key = #api::leaderboard::encode(&#api::leaderboard::Kind::#game_ident(
						#mode_enum::#overall_ident,
						#kind_enum::#b,
					));
					let game = &data.stats.#path_to_game;

					pipeline.zadd(&key, data.uuid.as_bytes(), #value);
					pipeline.zremrangebyrank(&key, 15_001, -15_000);
				}))
			})
			.chain(labels.iter().filter_map(|l| {
				if !l.is_measurable() {
					return None;
				}

				let value = l.value_sum(Side::None, &overall_modes, Access::NoneDiff)?;

				Some(quote!({
					let key = #api::leaderboard::encode(&#api::leaderboard::Kind::#game_ident(
						#mode_enum::#overall_ident,
						#kind_enum::#l,
					));
					let game = &data.stats.#path_to_game;

					pipeline.zadd(&key, data.uuid.as_bytes(), #value);
					pipeline.zremrangebyrank(&key, 15_001, -15_000);
				}))
			}))
			.chain(std::iter::once(quote!({
				let key = #api::leaderboard::encode(&#api::leaderboard::Kind::#game_ident(
					#mode_enum::#overall_ident,
					#kind_enum::level,
				));
				let game = &data.stats.#path_to_game;

				pipeline.zadd(&key, data.uuid.as_bytes(), #calc::get_total_xp(#calc::convert(&#xp)));
				pipeline.zremrangebyrank(&key, 15_001, -15_000);
			})));

		let leaderboards = blocks
			.iter()
			.filter_map(|b| {
				if !b.is_measurable() {
					return None;
				}

				let tr = b.as_tr();

				Some(quote!({
					#api::leaderboard::Leaderboard {
						kind: #api::leaderboard::Kind::#game_ident(
							#mode_enum::#overall_ident,
							#kind_enum::#b,
						),
						game: #hypixel::game::r#type::Type::#game_ident,
						name: format!("{}", #tr),
						display_name: format!(
							"{} {}",
							#hypixel::game::r#type::Type::#game_ident.as_clean_name(),
							#tr,
						),
						display_name_lower: format!(
							"{} {}",
							#hypixel::game::r#type::Type::#game_ident.as_clean_name(),
							#tr,
						)
						.replace(' ', "")
						.to_lowercase(),
					}
				}))
			})
			.chain(labels.iter().filter_map(|l| {
				if !l.is_measurable() {
					return None;
				}

				let tr = l.as_tr();

				Some(quote!({
					#api::leaderboard::Leaderboard {
						kind: #api::leaderboard::Kind::#game_ident(
							#mode_enum::#overall_ident,
							#kind_enum::#l,
						),
						game: #hypixel::game::r#type::Type::#game_ident,
						name: format!(
							"{}",
							#tr,
						),
						display_name: format!(
							"{} {}",
							#hypixel::game::r#type::Type::#game_ident.as_clean_name(),
							#tr,
						),
						display_name_lower: format!(
							"{} {}",
							#hypixel::game::r#type::Type::#game_ident.as_clean_name(),
							#tr,
						)
						.replace(' ', "")
						.to_lowercase(),
					}
				}))
			}))
			.chain(std::iter::once(quote!({
				#api::leaderboard::Leaderboard {
					kind: #api::leaderboard::Kind::#game_ident(
						#mode_enum::#overall_ident,
						#kind_enum::level,
					),
					game: #hypixel::game::r#type::Type::#game_ident,
					name: format!(
						"{}",
						#translate::tr(ctx, "level"),
					),
					display_name: format!(
						"{} {}",
						#hypixel::game::r#type::Type::#game_ident.as_clean_name(),
						#translate::tr(ctx, "level"),
					),
					display_name_lower: format!(
						"{} {}",
						#hypixel::game::r#type::Type::#game_ident.as_clean_name(),
						#translate::tr(ctx, "level"),
					)
					.replace(' ', "")
					.to_lowercase(),
				}
			})));

		let leaderboard_kind_match = blocks
			.iter()
			.filter_map(|b| {
				if !b.is_measurable() {
					return None;
				}

				let value = b.value_sum(Side::None, &overall_modes, Access::None)?;
				let (top, bottom) = if let Some(bottom) =
					b.value_bottom_sum(Side::None, &overall_modes, Access::None)
				{
					let top = b.value_top_sum(Side::None, &overall_modes, Access::None)?;

					(
						quote!(canvas = canvas.push_right(
							&#api::canvas::shape::LeaderboardValue,
							#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#top),
						);),
						quote!(canvas = canvas.push_right(
							&#api::canvas::shape::LeaderboardValue,
							#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#bottom),
						);),
					)
				} else {
					(quote!(), quote!())
				};

				Some(quote!(#kind_enum::#b => {
					canvas = canvas.push_right(
						&#api::canvas::shape::LeaderboardValue,
						#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#value),
					);
					#top
					#bottom
				}))
			})
			.chain(labels.iter().filter_map(|l| {
				if !l.is_measurable() {
					return None;
				}

				let value = l.value_sum(Side::None, &overall_modes, Access::None)?;
				let (top, bottom) = if let Some(bottom) =
					l.value_bottom_sum(Side::None, &overall_modes, Access::None)
				{
					let top = l.value_top_sum(Side::None, &overall_modes, Access::None)?;

					(
						quote!(canvas = canvas.push_right(
							&#api::canvas::shape::LeaderboardValue,
							#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#top),
						);),
						quote!(canvas = canvas.push_right(
							&#api::canvas::shape::LeaderboardValue,
							#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#bottom),
						);),
					)
				} else {
					(quote!(), quote!())
				};

				Some(quote!(#kind_enum::#l => {
					canvas = canvas.push_right(
						&#api::canvas::shape::LeaderboardValue,
						#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#value),
					);
					#top
					#bottom
				}))
			}))
			.chain(std::iter::once(quote!(#kind_enum::level => {
				let xp = #calc::convert(&#xp);

				canvas = canvas.push_right(
					&#api::canvas::shape::LeaderboardValue,
					#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#calc::get_total_xp(xp)),
				);
			})));

		let leaderboard_kind_match_header = blocks
			.iter()
			.filter_map(|b| {
				if !b.is_measurable() {
					return None;
				}

				let tr = b.as_tr();
				let (top, bottom) = if let Some(bottom) = b.as_tr_bottom() {
					let top = b.as_tr_top();

					(
						quote!(canvas = canvas.push_right(
								&#api::canvas::shape::LeaderboardValue,
								#api::canvas::shape::LeaderboardValue::label(family, #top),
							);),
						quote!(canvas = canvas.push_right(
								&#api::canvas::shape::LeaderboardValue,
								#api::canvas::shape::LeaderboardValue::label(family, #bottom),
							);),
					)
				} else {
					(quote!(), quote!())
				};

				Some(quote!(#kind_enum::#b => {
					canvas = canvas.push_right(
						&#api::canvas::shape::LeaderboardValue,
						#api::canvas::shape::LeaderboardValue::label(family, #tr),
					);
					#top
					#bottom
				}))
			})
			.chain(labels.iter().filter_map(|l| {
				if !l.is_measurable() {
					return None;
				}

				let tr = l.as_tr();
				let (top, bottom) = if let Some(bottom) = l.as_tr_bottom() {
					let top = l.as_tr_top();

					(
						quote!(canvas = canvas.push_right(
								&#api::canvas::shape::LeaderboardValue,
								#api::canvas::shape::LeaderboardValue::label(family, #top),
							);),
						quote!(canvas = canvas.push_right(
								&#api::canvas::shape::LeaderboardValue,
								#api::canvas::shape::LeaderboardValue::label(family, #bottom),
							);),
					)
				} else {
					(quote!(), quote!())
				};

				Some(quote!(#kind_enum::#l => {
					canvas = canvas.push_right(
						&#api::canvas::shape::LeaderboardValue,
						#api::canvas::shape::LeaderboardValue::label(family, #tr),
					);
					#top
					#bottom
				}))
			}))
			.chain(std::iter::once(quote!(#kind_enum::level => {
				canvas = canvas.push_right(
					&#api::canvas::shape::LeaderboardValue,
					#api::canvas::shape::LeaderboardValue::label(family, #translate::tr(ctx, "experience")),
				);
			})));

		let leaderboard_kind_match_extras = blocks
			.iter()
			.filter_map(|b| {
				if !b.is_measurable() {
					return None;
				}

				let extras = if b.tr_bottom().is_some() { 2u8 } else { 0 };

				Some(quote!(#kind_enum::#b => #extras))
			})
			.chain(labels.iter().filter_map(|l| {
				if !l.is_measurable() {
					return None;
				}

				let extras = if l.tr_bottom().is_some() { 2u8 } else { 0 };

				Some(quote!(#kind_enum::#l => #extras))
			}))
			.chain(std::iter::once(quote!(#kind_enum::level => 0)));

		let embed = blocks
			.iter()
			.filter_map(|b| {
				Some((
					b.value_fmt_sum(Side::None, &overall_modes, Access::None)?,
					b.as_tr(),
				))
			})
			.enumerate()
			.map(|(i, (v, tr))| {
				let extra = if i % 3 == 0 {
					quote!(field.push('\n');)
				} else {
					quote!()
				};

				quote! {
					#extra

					field.push_str(#tr.as_ref());
					field.push_str(": **");
					field.push_str(#v.as_ref());
					field.push_str("**\n");
				}
			});

		let embed_diff = blocks
			.iter()
			.filter_map(|b| b.diff_fmt_sum(&overall_modes))
			.enumerate()
			.map(|(i, v)| {
				let extra = if i % 3 == 0 {
					quote!(field.push('\n');)
				} else {
					quote!()
				};

				quote! {
					#extra

					field.push_str(#translate::tr(ctx, Self::tr()).as_ref());
					field.push_str(": **");
					field.push_str(#v.as_ref());
					field.push_str("**\n");
				}
			});

		let chart = blocks.iter().filter_map(|b| {
			if b.skip_chart() {
				return None;
			}

			let value = b.value_trunc_sum(Side::None, &overall_modes, Access::NoneDiff)?;
			let tr = b.as_tr();

			Some(quote!(
				(
					#tr,
					snapshots.iter().map(|(created_at, data)| {
						let game = &data.stats.#path_to_game;
						let v: u32 = #value.into();

						(*created_at, v)
					})
					.collect::<::std::vec::Vec<_>>(),
				)
			))
		});

		let project = blocks
			.iter()
			.filter_map(|b| Some(({
				let value = b.value_sum(Side::None, &overall_modes, Access::NoneDiff)?;

				quote!({
					let game = &data.stats.#path_to_game;

					f64::from(#value)
				})
			}, b.tr(), b.var_id())))
			.chain(
				labels
					.iter()
					.filter_map(|l| Some(({
						let value = l.value_sum(Side::None, &overall_modes, Access::NoneDiff)?;

						quote!({
							let game = &data.stats.#path_to_game;

							f64::from(#value)
						})
					}, l.tr(), l.var_id()))),
			)
			.chain(std::iter::once((
				quote!({
					let game = &data.stats.#path_to_game;
					let xp = #calc::convert(&#xp);

					f64::from(#calc::get_level(xp)) + f64::from(#calc::get_level_progress(xp))
				}),
				Cow::Borrowed("level"),
				Cow::Owned(quote!(level)),
			)))
			.map(|(value, tr, id)| {
				quote! {
					#kind_enum::#id => {
						let mut low = f64::MAX;
						let mut high: f64 = 1.;

						for (_, data) in &snapshots {
							let val = #value;

							low = low.min(val);
							high = high.max(val);
						}

						let series = snapshots
							.iter()
							.map(|(time, data)| (time.timestamp() as f64, #value))
							.collect::<Vec<_>>();

						let line = #api::canvas::project::line::Line::from_series(&series);

						let predict_y = value.unwrap_or_else(|| #api::canvas::project::next_milestone({
							let data = &last.1;
							#value
						}));
						let predict_x = line
							.x(predict_y, last.0.timestamp() as f64)
							.map(|x| #chrono::TimeZone::timestamp_opt(&#chrono::Utc, x as i64, 0).unwrap());

						let mut buffer = #api::canvas::project::f64::create(
							ctx,
							family,
							::std::vec![(
								#translate::tr(ctx, #tr),
								snapshots
									.iter()
									.map(|(time, data)| (*time, #value))
									.collect::<Vec<_>>(),
								predict_x.unwrap_or(last.0),
								predict_y,
							)],
							first.0..predict_x.map_or(last.0, |x| x.max(last.0)),
							(low * (7. / 8.))..(predict_y.max(high) * (8. / 7.)),
							None,
							background,
						)?;

						let mut surface = #api::canvas::project::canvas(&mut buffer)?;

						#api::canvas::chart::apply_title(ctx, family, &mut surface, &last.1, &LABEL, background);

						let r = #api::percent::PercentU32((#api::canvas::project::line::compute_r(&series, &line) * 100.) as u32);

						if let Some(x) = predict_x {
							#api::canvas::project::apply_bubbles(
								&mut surface,
								ctx,
								family,
								#translate::tr(ctx, #tr).as_ref(),
								&predict_y,
								&r,
								&x,
								background,
							);
						} else {
							#api::canvas::project::apply_bubbles(
								&mut surface,
								ctx,
								family,
								#translate::tr(ctx, #tr).as_ref(),
								&predict_y,
								&r,
								&#translate::tr(ctx, "never").as_ref(),
								background,
							);
						}

						#api::canvas::project::round_corners(&mut surface);

						Ok(#api::canvas::to_png(&mut surface))
					}
				}
			});

		let label_separator = if label_lines_first != 0 {
			quote!(.append(#minecraft::text::Text::NEW_LINE))
		} else {
			quote!()
		};

		tokens.extend(quote! {
			pub struct #overall_ident;

			impl #api::canvas::diff::DiffLog for #overall_ident {
				fn diff_log<'e>(
					data_lhs: &#api::player::data::Data,
					data_rhs: &#api::player::data::Data,
					ctx: &#translate::context::Context<'_>,
					mut embed: #poise::serenity_prelude::CreateEmbed<'e>,
				) -> Result<#poise::serenity_prelude::CreateEmbed<'e>, #poise::serenity_prelude::CreateEmbed<'e>> {
					let mut log = String::new();
					let game_lhs = &data_lhs.stats.#path_to_game;
					let game_rhs = &data_rhs.stats.#path_to_game;

					#(#diff_log)*

					if !log.is_empty() {
						let mut title = String::new();

						title.push_str(PLAIN);
						title.push(' ');
						title.push_str(#translate::tr(ctx, Overall::tr()).as_ref());

						Ok(embed.field(title, log, true))
					} else {
						Err(embed)
					}
				}
			}

			impl #overall_ident {
				pub fn add_to_pipeline(pipeline: &mut #redis::Pipeline, data: &#api::player::data::Data) {
					#(#add_to_pipeline)*
				}

				pub fn leaderboards(ctx: &#translate::context::Context<'_>) -> Vec<#api::leaderboard::Leaderboard> {
					vec![
						#(#leaderboards),*
					]
				}

				pub fn get_extras(kind: &#kind_enum) -> u8 {
					match kind {
						#(#leaderboard_kind_match_extras,)*
						_ => 0,
					}
				}

				pub fn leaderboard<'c>(
					ctx: &#translate::context::Context<'_>,
					start: usize,
					players: &[::std::sync::Arc<#api::player::data::Data>],
					kind: &#kind_enum,
					family: #minecraft::style::Family,
					background: Option<#skia::Color>,
					mut canvas: #api::canvas::Canvas<'c>,
				) -> Result<#api::canvas::Canvas<'c>, #translate::Error> {
					match kind {
						#(#leaderboard_kind_match_header)*
						_ => return Err(#translate::Error::NotImplemented),
					}

					for (idx, data) in players.iter().enumerate() {
						let game = &data.stats.#path_to_game;

						let level = {
							let xp = #calc::convert(&#xp);
							let level = #calc::get_level(xp);

							level
						};

						canvas = canvas
							.push_down_start(
								&#api::canvas::shape::LeaderboardPlace,
								#api::canvas::shape::LeaderboardPlace::from_usize(family, start + idx + 1),
							)
							.push_right(
								&#api::canvas::shape::LeaderboardName,
								#api::canvas::body::Body::build_slice(
									family,
									#api::canvas::text::from_data_with_level(
										&data,
										&data.username,
										None,
										&#calc::get_level_format(level)
									).as_slice(),
									20.,
									#skia::textlayout::TextAlign::Left,
								),
							);

						match kind {
							#(#leaderboard_kind_match)*
							_ => return Err(#translate::Error::NotImplemented),
						}
					}

					Ok(canvas)
				}

				#[inline]
				pub fn tr() -> &'static str {
					"Overall"
				}

				pub fn embed<'e>(
					&self,
					ctx: &'e #translate::context::Context<'_>,
					embed: #poise::serenity_prelude::CreateEmbed<'e>,
					data: &#api::player::data::Data,
				) -> #poise::serenity_prelude::CreateEmbed<'e> {
					let game = &data.stats.#path_to_game;
					let mut field = String::new();

					#(#embed)*

					embed.field(
						#translate::tr(ctx, Self::tr()),
						field,
						true,
					)
				}

				pub fn embed_diff<'e>(
					ctx: &'e #translate::context::Context<'_>,
					embed: #poise::serenity_prelude::CreateEmbed<'e>,
					data_lhs: &#api::player::data::Data,
					data_rhs: &#api::player::data::Data,
				) -> #poise::serenity_prelude::CreateEmbed<'e> {
					let mut field = String::new();
					let game_lhs = &data_lhs.stats.#path_to_game;
					let game_rhs = &data_rhs.stats.#path_to_game;

					#(#embed_diff)*

					embed.field(#translate::tr(ctx, Self::tr()), field, true)
				}

				pub fn project(
					ctx: &#translate::context::Context<'_>,
					family: #minecraft::style::Family,
					snapshots: ::std::vec::Vec<(#chrono::DateTime<#chrono::Utc>, #api::player::data::Data)>,
					kind: #kind_enum,
					value: Option<f64>,
					background: Option<#skia::Color>,
				) -> Result<::std::vec::Vec<u8>, #translate::Error> {
					let first = snapshots.first().unwrap();
					let last = snapshots.last().unwrap();

					match kind {
						#(#project,)*
						_ => Err(#translate::Error::NotImplemented),
					}
				}

				pub fn chart(
					ctx: &#translate::context::Context<'_>,
					family: #minecraft::style::Family,
					snapshots: ::std::vec::Vec<(#chrono::DateTime<#chrono::Utc>, #api::player::data::Data)>,
					background: Option<#skia::Color>,
				) -> Result<::std::vec::Vec<u8>, #translate::Error> {
					let first = snapshots.first().unwrap();
					let last = snapshots.last().unwrap();

					let mut lower = Self::min_fields(&#overall_ident, &first.1);
					let mut upper = ::std::cmp::max(Self::max_fields(&#overall_ident, &first.1), 100);

					for (_, data) in snapshots.iter().skip(1) {
						let stats = &#overall_ident;

						lower = lower.min(Self::min_fields(stats, data));
						upper = upper.max(Self::max_fields(stats, data));
					}

					let x_range = first.0.clone()..last.0.clone();

					let v = ::std::vec![
						#(#chart,)*
					];

					let mut buffer = #api::canvas::chart::u32::create::<true>(
						ctx,
						family,
						v,
						x_range,
						((f64::from(lower) * (11. / 16.)) as u32)..((f64::from(upper) * (16. / 15.)) as u32),
						None,
						background,
					)?;

					let mut surface = #api::canvas::chart::canvas(&mut buffer)?;

					#api::canvas::chart::apply_title(ctx, family, &mut surface, &last.1, &LABEL, background);
					#api::canvas::chart::round_corners(&mut surface);

					Ok(#api::canvas::to_png(&mut surface))
				}

				pub fn min_fields(&self, data: &#api::player::data::Data) -> u32 {
					let game = &data.stats.#path_to_game;
					let mut min = ::std::u32::MAX;

					#min
					min
				}

				pub fn max_fields(&self, data: &#api::player::data::Data) -> u32 {
					let game = &data.stats.#path_to_game;
					let mut max = ::std::u32::MIN;

					#max
					max
				}

				pub fn add_header<'c>(
					&self,
					ctx: &#translate::context::Context<'_>,
					family: #minecraft::style::Family,
					mut canvas: #api::canvas::Canvas<'c>,
					data: &'c #api::player::data::Data,
				) -> #api::canvas::Canvas<'c> {
					let game = &data.stats.#path_to_game;

					canvas
						#labels_sum
				}

				#[allow(clippy::too_many_arguments)]
				pub fn canvas<'c>(
					&self,
					ctx: &#translate::context::Context<'_>,
					family: #minecraft::style::Family,
					mut canvas: #api::canvas::Canvas<'c>,
					data: &'c #api::player::data::Data,
					session: &'c #api::player::status::Session,
					status: &'c #api::canvas::shape::Status,
					progress: &'c #api::canvas::shape::WideBubbleProgress,
				) -> #api::canvas::Canvas<'c> {
					let game = &data.stats.#path_to_game;
					let xp = #calc::convert(&#xp);
					let level = #calc::get_level(xp);

					let canvas = canvas
						.push_down(
							&#api::canvas::shape::Subtitle,
							#api::canvas::shape::Subtitle::from_label(ctx, family, &LABEL, #overall_ident::tr()),
						)
						.push_down_post_draw(
							progress,
							#api::canvas::shape::WideBubbleProgress::from_level_progress(
								ctx,
								family,
								&#calc::get_level_format(level),
								&#calc::get_curr_level_xp(xp),
								&#calc::get_level_xp(xp),
							)
						);

					self.add_header(ctx, family, canvas, data)
						.push_right_post_draw(
							status,
							#api::canvas::body::Body::from_status(ctx, family, session)
						)
						#blocks_sum
				}

				pub fn condensed<'c>(
					&self,
					ctx: &#translate::context::Context<'_>,
					family: #minecraft::style::Family,
					mut canvas: #api::canvas::Canvas<'c>,
					data: &'c #api::player::data::Data,
				) -> #api::canvas::Canvas<'c> {
					use #api::canvas::label::ToFormatted;

					let game = &data.stats.#path_to_game;

					canvas
						.push_checked(
							&#api::canvas::shape::CondensedBubble {
								lines: #label_lines_first + #block_lines + if #label_lines_first == 0 { 2 } else { 3 }
							},
							#api::canvas::body::Body::new(17., None, family)
								.append(#minecraft::text::Text::NEW_LINE)
								.append(#minecraft::text::Text {
									text: #translate::tr(ctx, Self::tr()).as_ref(),
									font: #minecraft::style::MinecraftFont::Bold,
									paint: #minecraft::paint::Paint::White,
									..Default::default()
								})
								#label_separator
								#condensed_labels_sum
								.append(#minecraft::text::Text::NEW_LINE)
								.extend(&[
									#condensed_blocks_sum
								])
								.build()
						)
				}

				pub fn condensed_diff<'c>(
					ctx: &#translate::context::Context<'_>,
					family: #minecraft::style::Family,
					mut canvas: #api::canvas::Canvas<'c>,
					data_lhs: &'c #api::player::data::Data,
					data_rhs: &'c #api::player::data::Data,
				) -> #api::canvas::Canvas<'c> {
					use #api::canvas::label::ToFormatted;

					let game_lhs = &data_lhs.stats.#path_to_game;
					let game_rhs = &data_rhs.stats.#path_to_game;

					canvas
						.push_checked(
							&#api::canvas::shape::CondensedBubble {
								lines: #label_lines_first + #block_lines + if #label_lines_first == 0 { 2 } else { 3 }
							},
							#api::canvas::body::Body::new(17., None, family)
								.append(#minecraft::text::Text::NEW_LINE)
								.append(#minecraft::text::Text {
									text: #translate::tr(ctx, Self::tr()).as_ref(),
									font: #minecraft::style::MinecraftFont::Bold,
									paint: #minecraft::paint::Paint::White,
									..Default::default()
								})
								#label_separator
								#condensed_labels_diff_sum
								.append(#minecraft::text::Text::NEW_LINE)
								.extend(&[
									#condensed_blocks_diff_sum
								])
								.build()
						)
				}

				#[allow(clippy::too_many_arguments)]
				pub fn canvas_diff<'c>(
					ctx: &#translate::context::Context<'_>,
					family: #minecraft::style::Family,
					mut canvas: #api::canvas::Canvas<'c>,
					data_lhs: &'c #api::player::data::Data,
					data_rhs: &'c #api::player::data::Data,
					session: &'c #api::player::status::Session,
					status: &'c #api::canvas::shape::Status,
					progress: &'c #api::canvas::shape::WideBubbleProgress,
				) -> #api::canvas::Canvas<'c> {
					let game_lhs = &data_lhs.stats.#path_to_game;
					let game_rhs = &data_rhs.stats.#path_to_game;
					let is_different = data_lhs.uuid != data_rhs.uuid;

					let (xp, positive, level) = {
						let xp_lhs = #calc::convert(&#xp_lhs);
						let xp_rhs = #calc::convert(&#xp_rhs);
						let xp = if xp_lhs > xp_rhs {
							xp_lhs - xp_rhs
						} else {
							xp_rhs - xp_lhs
						};

						(xp, xp_rhs >= xp_lhs, #calc::get_level(xp))
					};

					canvas
						.push_down(
							&#api::canvas::shape::Subtitle,
							#api::canvas::shape::Subtitle::from_label(ctx, family, &LABEL, #overall_ident::tr()),
						)
						.push_down_post_draw(
							progress,
							#api::canvas::shape::WideBubbleProgress::from_level_diff(
								ctx,
								family,
								&#calc::get_level_format(level),
								&#calc::get_total_xp(xp),
								positive,
								is_different,
							),
						)
						#labels_diff_sum
						.push_right_post_draw(
							status,
							#api::canvas::body::Body::from_status(ctx, family, session)
						)
						#blocks_diff_sum
				}
			}
		});

		tokens.extend({
			let project_mode = modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					#mode_enum::#ty => #ty::project(
						ctx,
						family,
						snapshots,
						kind,
						value,
						background,
					),
				}
			});

			let chart_mode = modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					#mode_enum::#ty => #ty::chart(
						ctx,
						family,
						snapshots,
						background,
					),
				}
			});

			let canvas_mode = modes.iter().map(|mode| {
				let ty = mode.ty();
				let id = mode.id();

				quote! {
					#mode_enum::#ty => game.#id.canvas(
						ctx,
						family,
						canvas,
						data,
						session,
						&status,
						&progress,
					),
				}
			});

			let condensed_canvas = (0..(condensed_modes.len() / 3 + 1))
				.map(|i| {
					let canvas = ident(&format!("canvas_{}", i));

					quote!(let #canvas = #api::canvas::Canvas::new(720., family).gap(7.);)
				})
				.collect::<Vec<_>>();

			let condensed_canvas_build = (0..(condensed_modes.len() / 3 + 1))
				.map(|i| {
					let canvas = ident(&format!("canvas_{}", i));

					quote!(#canvas.build(None, background).unwrap())
				})
				.collect::<Vec<_>>();

			let condensed_mode = condensed_modes.iter().enumerate().map(|(i, mode)| {
				let id = mode.id();
				let i = (i + 1) / 3;
				let canvas = ident(&format!("canvas_{}", i));

				quote! {
					let #canvas = game.#id.condensed(
						ctx,
						family,
						#canvas,
						data,
					);
				}
			});

			let condensed_mode_diff = condensed_modes.iter().enumerate().map(|(i, mode)| {
				let ty = mode.ty();
				let i = (i + 1) / 3;
				let canvas = ident(&format!("canvas_{}", i));

				quote! {
					let #canvas = #ty::condensed_diff(
						ctx,
						family,
						#canvas,
						data_lhs,
						data_rhs,
					);
				}
			});

			let canvas_diff_mode = modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					#mode_enum::#ty => #ty::canvas_diff(
						ctx,
						family,
						canvas,
						data_lhs,
						data_rhs,
						session,
						&status,
						&progress,
					),
				}
			});

			let from_kind_match = blocks.iter()
				.filter_map(|b| {
					let id = b.var_id();
					let value = b.value_fmt_sum(Side::None, &overall_modes, Access::None)?;

					Some(quote!(#kind_enum::#id => #value.into_owned()))
				})
				.chain(
					labels.iter()
						.filter_map(|l| {
							let id = l.var_id();
							let value = l.value_fmt_sum(Side::None, &overall_modes, Access::None)?;

							Some(quote!(#kind_enum::#id => #value.into_owned()))
						})
				);

			let from_kind_diff_match = blocks.iter()
				.filter_map(|b| {
					let id = b.var_id();
					let value = b.diff_fmt_sum(&overall_modes)?;

					Some(quote!(#kind_enum::#id => #value))
				})
				.chain(
					labels.iter()
						.filter_map(|l| {
							let id = l.var_id();
							let value = l.diff_fmt_sum(&overall_modes)?;

							Some(quote!(#kind_enum::#id => #value))
						})
				);

			let diff_log = overall_modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					let embed = match <#ty as #api::canvas::diff::DiffLog>::diff_log(data_lhs, data_rhs, ctx, embed) {
						Err(embed) => embed,
						Ok(embed) => {
							is_modified = true;
							embed
						},
					};
				}
			});

			let add_to_pipeline = modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					#ty::add_to_pipeline(pipeline, data);
				}
			});

			let leaderboards = modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					leaderboards.extend(#ty::leaderboards(ctx).into_iter());
				}
			});

			let leaderboard_match = modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					#mode_enum::#ty => #ty::leaderboard(ctx, start, players, kind, family, background, canvas),
				}
			});

			let leaderboard_extras_match = modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					#mode_enum::#ty => #ty::get_extras(kind),
				}
			});

			let embed_game = overall_modes.iter().map(|mode| {
				quote!(embed = game.#mode.embed(ctx, embed, data);)
			});

			let embed_diff_game = overall_modes.iter().map(|mode| {
				let ty = mode.ty();

				quote!(embed = #ty::embed_diff(ctx, embed, data_lhs, data_rhs);)
			});

			let buffer_fields = match overall_modes.len() % 3 {
				0 => 0,
				1 => 2,
				2 => 1,
				_ => unreachable!(),
			};

			let buffer_fields = if buffer_fields == 0 {
				None
			} else {
				Some(quote!{
					for _ in 0..#buffer_fields {
						embed = embed.field("\u{200b}", "\u{200b}", true);
					}
				})
			};

			let has_compact = modes.len() > 1;

			quote! {
				impl #game_ident {
					pub fn add_to_pipeline(pipeline: &mut #redis::Pipeline, data: &#api::player::data::Data) {
						#overall_ident::add_to_pipeline(pipeline, data);
						#(#add_to_pipeline)*
					}

					pub fn leaderboards(ctx: &#translate::context::Context<'_>, leaderboards: &mut Vec<#api::leaderboard::Leaderboard>) {
						leaderboards.extend(#overall_ident::leaderboards(ctx).into_iter());
						#(#leaderboards)*
					}

					#[allow(clippy::too_many_arguments)]
					pub fn leaderboard(
						ctx: &#translate::context::Context<'_>,
						start: usize,
						players: &[::std::sync::Arc<#api::player::data::Data>],
						mode: &#mode_enum,
						kind: &#kind_enum,
						leaderboard: &#api::leaderboard::Leaderboard,
						family: #minecraft::style::Family,
						background: Option<#skia::Color>,
					) -> Result<#skia::Surface, #translate::Error> {
						let mut canvas = #api::canvas::Canvas::new(720., family).gap(7.).push_down(
							&#api::canvas::shape::LeaderboardTitle {
								extras: match mode {
									#mode_enum::#overall_ident => #overall_ident::get_extras(kind),
									#(#leaderboard_extras_match)*
								},
							},
							#api::canvas::body::Body::new(24., #skia::textlayout::TextAlign::Center, family)
								.extend(leaderboard.game.as_text())
								.append(#minecraft::text::Text::SPACE)
								.extend(&[#minecraft::text::Text {
									text: &leaderboard.name,
									paint: #minecraft::paint::Paint::White,
									..Default::default()
								}])
								.build(),
						)
						.push_down_start(
							&#api::canvas::shape::LeaderboardPlace,
							#api::canvas::shape::LeaderboardPlace::label(ctx, family),
						)
						.push_right(
							&#api::canvas::shape::LeaderboardNameLabel,
							#api::canvas::shape::LeaderboardName::label(ctx, family),
						);

						let canvas = match mode {
							#mode_enum::#overall_ident => #overall_ident::leaderboard(ctx, start, players, kind, family, background, canvas),
							#(#leaderboard_match)*
						}?;

						Ok(canvas.build(None, background).unwrap())
					}

					pub fn from_kind<'t, 'c: 't>(
						ctx: &'c #translate::context::Context<'c>,
						data: &'t #api::player::data::Data,
						kind: &#kind_enum
					) -> Result<::std::borrow::Cow<'static, str>, #translate::Error> {
						let game = &data.stats.#path_to_game;

						let value: String = match kind {
							#(#from_kind_match,)*
							_ => return Err(#translate::Error::NotImplemented),
						};

						Ok(::std::borrow::Cow::Owned(value))
					}

					pub fn from_kind_diff<'t, 'c: 't>(
						ctx: &'c #translate::context::Context<'c>,
						data_lhs: &'t #api::player::data::Data,
						data_rhs: &'t #api::player::data::Data,
						kind: &#kind_enum
					) -> Result<String, #translate::Error> {
						let game_lhs = &data_lhs.stats.#path_to_game;
						let game_rhs = &data_rhs.stats.#path_to_game;

						Ok(match kind {
							#(#from_kind_diff_match,)*
							_ => return Err(#translate::Error::NotImplemented),
						})
					}

					pub async fn autocomplete<'a>(ctx: #translate::Context<'a>, partial: String) -> impl Iterator<Item = #poise::serenity_prelude::AutocompleteChoice<'a>> + 'a {
						let locale = #translate::prelude::GetLocale::locale(&ctx);

						#mode_enum::slice()
							.iter()
							.filter_map({
								move |mode| {
									let name = #translate::tr(&locale, mode.tr());
									let mode: u32 = mode.into();
									let name: String = name.into_owned();

									if name.to_ascii_lowercase().contains(&partial) {
										Some(#poise::serenity_prelude::AutocompleteChoice::new(name, mode))
									} else {
										None
									}
								}
							})
							.take(10)
					}

					pub async fn autocomplete_kind<'a>(ctx: #translate::Context<'a>, partial: String) -> impl Iterator<Item = #poise::serenity_prelude::AutocompleteChoice<'a>> + 'a {
						let locale = #translate::prelude::GetLocale::locale(&ctx);

						#kind_enum::slice()
							.iter()
							.filter_map({
								move |kind| {
									let name = #translate::tr(&locale, kind.tr());
									let kind: u32 = kind.into();
									let name: String = name.into_owned();

									if name.to_ascii_lowercase().contains(&partial) {
										Some(#poise::serenity_prelude::AutocompleteChoice::new(name, kind))
									} else {
										None
									}
								}
							})
							.take(10)
					}
				}

				impl #api::canvas::diff::DiffLog for #game_ident {
					fn diff_log<'e>(
						data_lhs: &#api::player::data::Data,
						data_rhs: &#api::player::data::Data,
						ctx: &#translate::context::Context<'_>,
						embed: #poise::serenity_prelude::CreateEmbed<'e>,
					) -> Result<#poise::serenity_prelude::CreateEmbed<'e>, #poise::serenity_prelude::CreateEmbed<'e>> {
						let mut is_modified = false;
						let embed = match <#overall_ident as #api::canvas::diff::DiffLog>::diff_log(data_lhs, data_rhs, ctx, embed) {
							Err(embed) => embed,
							Ok(embed) => {
								is_modified = true;
								embed
							},
						};

						#(#diff_log)*

						if is_modified {
							Ok(embed)
						} else {
							Err(embed)
						}
					}
				}

				impl #api::canvas::prelude::Game for #game_ident {
					type Mode = #mode_enum;
					const HAS_COMPACT: bool = #has_compact;

					fn condensed_diff(
						ctx: &#translate::context::Context<'_>,
						family: #minecraft::style::Family,
						data_lhs: &#api::player::data::Data,
						data_rhs: &#api::player::data::Data,
						suffix: Option<&str>,
						background: Option<#skia::Color>,
					) -> Vec<#skia::Surface> {
						let game_lhs = &data_lhs.stats.#path_to_game;
						let game_rhs = &data_rhs.stats.#path_to_game;
						let is_different = data_lhs.uuid != data_rhs.uuid;

						#(#condensed_canvas)*

						let level_lhs = {
							let xp = #calc::convert(&#xp_lhs);
							let level = #calc::get_level(xp);

							level
						};

						let level_rhs = if is_different {
							let xp = #calc::convert(&#xp_rhs);
							let level = #calc::get_level(xp);

							level
						} else {
							let xp_lhs = #calc::convert(&#xp_lhs);
							let xp_rhs = #calc::convert(&#xp_rhs);
							let xp = if xp_lhs > xp_rhs {
								xp_lhs - xp_rhs
							} else {
								xp_rhs - xp_lhs
							};

							let level = #calc::get_level(xp);

							level
						};

						let mut canvas_0 = canvas_0
							.push_down(
								&#api::canvas::shape::LongTitle,
								#api::canvas::shape::Title::from_text(
									family,
									&#api::canvas::text::from_data_with_level(
										&data_rhs,
										&data_rhs.username,
										suffix,
										&#calc::get_level_format(level_rhs)
									)
								),
							);

						if is_different {
							canvas_0 = canvas_0
								.push_down(
									&#api::canvas::shape::LongSubtitle,
									#api::canvas::shape::Subtitle::from_text(
										family,
										&#api::canvas::text::from_data_with_level(
											&data_lhs,
											&data_lhs.username,
											None,
											&#calc::get_level_format(level_lhs)
										)
									),
								);
						}

						let canvas_0 = canvas_0
							.push_down(
								&#api::canvas::shape::LongSubtitle,
								#api::canvas::body::Body::build_slice(family, &LABEL, 17., #skia::textlayout::TextAlign::Center),
							);

						let canvas_0 = #overall_ident::condensed_diff(
							ctx,
							family,
							canvas_0,
							data_lhs,
							data_rhs,
						);

						#(#condensed_mode_diff)*

						vec![
							#(#condensed_canvas_build,)*
						]
					}

					fn condensed(
						ctx: &#translate::context::Context<'_>,
						family: #minecraft::style::Family,
						data: &#api::player::data::Data,
						suffix: Option<&str>,
						background: Option<#skia::Color>,
					) -> Vec<#skia::Surface> {
						let game = &data.stats.#path_to_game;

						#(#condensed_canvas)*

						let level = {
							let xp = #calc::convert(&#xp);
							let level = #calc::get_level(xp);

							level
						};

						let mut canvas_0 = canvas_0
							.push_down(
								&#api::canvas::shape::LongTitle,
								#api::canvas::shape::Title::from_text(family, &#api::canvas::text::from_data_with_level(
									&data,
									&data.username,
									suffix,
									&#calc::get_level_format(level)
								)),
							)
							.push_down(
								&#api::canvas::shape::LongSubtitle,
								#api::canvas::body::Body::build_slice(family, &LABEL, 17., #skia::textlayout::TextAlign::Center),
							);

						let canvas_0 = #overall_ident.condensed(
							ctx,
							family,
							canvas_0,
							data,
						);

						#(#condensed_mode)*

						vec![
							#(#condensed_canvas_build,)*
						]
					}

					#[allow(clippy::too_many_arguments)]
					fn canvas_diff(
						ctx: &#translate::context::Context<'_>,
						family: #minecraft::style::Family,
						data_lhs: &#api::player::data::Data,
						data_rhs: &#api::player::data::Data,
						session: &#api::player::status::Session,
						skin: &#skia::Image,
						mode: Option<Self::Mode>,
						suffix: Option<&str>,
						background: Option<#skia::Color>,
					) -> (#skia::Surface, Self::Mode) {
						let game_lhs = &data_lhs.stats.#path_to_game;
						let game_rhs = &data_rhs.stats.#path_to_game;
						let is_different = data_lhs.uuid != data_rhs.uuid;

						let mode = #mode_enum::get_mode(mode, session);
						let mut canvas = #api::canvas::Canvas::new(720., family)
							.gap(7.)
							.push_down(
								&#api::canvas::shape::Title,
								#api::canvas::shape::Title::from_text(family, &#api::canvas::text::from_data(&data_rhs, &data_rhs.username, suffix)),
							);

						if is_different {
							canvas = canvas.push_down(
								&#api::canvas::shape::Subtitle,
								#api::canvas::shape::Subtitle::from_text(family, &#api::canvas::text::from_data(&data_lhs, &data_lhs.username, None)),
							);
						}

						let (xp, level, progress) = {
							let xp_lhs = #calc::convert(&#xp_lhs);
							let xp_rhs = #calc::convert(&#xp_rhs);
							let xp = if xp_lhs > xp_rhs {
								xp_lhs - xp_rhs
							} else {
								xp_rhs - xp_lhs
							};

							let level = #calc::get_level(xp);
							let progress = #api::canvas::shape::WideBubbleProgress(
								#calc::get_level_progress(xp),
								#calc::get_colours(level),
								is_different,
							);

							(xp, level, progress)
						};

						let status = #api::canvas::shape::Status(session, skin);

						let mut canvas = match mode {
							#mode_enum::#overall_ident => {
								#overall_ident::canvas_diff(
									ctx,
									family,
									canvas,
									data_lhs,
									data_rhs,
									session,
									&status,
									&progress,
								)
							}
							#(#canvas_diff_mode)*
						};

						(canvas.build(None, background).unwrap(), mode)
					}

					#[allow(clippy::too_many_arguments)]
					fn canvas(
						ctx: &#translate::context::Context<'_>,
						family: #minecraft::style::Family,
						data: &#api::player::data::Data,
						session: &#api::player::status::Session,
						skin: &#skia::Image,
						mode: Option<Self::Mode>,
						suffix: Option<&str>,
						background: Option<#skia::Color>,
					) -> (#skia::Surface, Self::Mode) {
						let game = &data.stats.#path_to_game;

						let mode = #mode_enum::get_mode(mode, session);
						let mut canvas = #api::canvas::Canvas::new(720., family)
							.gap(7.)
							.push_down(
								&#api::canvas::shape::Title,
								#api::canvas::shape::Title::from_text(family, &#api::canvas::text::from_data(&data, &data.username, suffix)),
							);

						let (xp, level, progress) = {
							let xp = #calc::convert(&#xp);
							let level = #calc::get_level(xp);

							let progress = #api::canvas::shape::WideBubbleProgress(
								#calc::get_level_progress(xp),
								#calc::get_colours(level),
								false,
							);

							(xp, level, progress)
						};

						let status = #api::canvas::shape::Status(session, skin);

						let mut canvas = match mode {
							#mode_enum::#overall_ident => {
								#overall_ident.canvas(
									ctx,
									family,
									canvas,
									data,
									session,
									&status,
									&progress,
								)
							}
							#(#canvas_mode)*
						};

						(canvas.build(None, background).unwrap(), mode)
					}

					fn chart(
						ctx: &#translate::context::Context<'_>,
						family: #minecraft::style::Family,
						snapshots: ::std::vec::Vec<(#chrono::DateTime<#chrono::Utc>, #api::player::data::Data)>,
						session: &#api::player::status::Session,
						background: Option<#skia::Color>,
						mode: Option<Self::Mode>,
					) -> Result<(::std::vec::Vec<u8>, Self::Mode), #translate::Error> {
						let mode = #mode_enum::get_mode(mode, session);

						Ok((match mode {
							#mode_enum::#overall_ident => {
								#overall_ident::chart(
									ctx,
									family,
									snapshots,
									background,
								)
							}
							#(#chart_mode)*
						}?, mode))
					}

					#[allow(clippy::too_many_arguments)]
					fn project(
						ctx: &#translate::context::Context<'_>,
						family: #minecraft::style::Family,
						snapshots: ::std::vec::Vec<(#chrono::DateTime<#chrono::Utc>, #api::player::data::Data)>,
						session: &#api::player::status::Session,
						mode: Option<Self::Mode>,
						kind: Option<#kind_enum>,
						value: Option<f64>,
						background: Option<#skia::Color>,
					) -> Result<(::std::vec::Vec<u8>, Self::Mode), #translate::Error> {
						let mode = #mode_enum::get_mode(mode, session);
						let kind = kind.unwrap_or_default();

						Ok((match mode {
							#mode_enum::#overall_ident => {
								#overall_ident::project(
									ctx,
									family,
									snapshots,
									kind,
									value,
									background,
								)
							}
							#(#project_mode)*
						}?, mode))
					}

					fn embed<'e>(
						ctx: &'e #translate::context::Context<'_>,
						player: &#api::player::Player,
						data: &#api::player::data::Data,
					) -> #poise::serenity_prelude::CreateEmbed<'e> {
						let game = &data.stats.#path_to_game;
						let mut embed = #poise::serenity_prelude::CreateEmbed::default()
							.thumbnail(player.get_body_url());

						if let Some(prefix) = data.get_rank().as_str() {
							embed = embed.author(
								#poise::serenity_prelude::CreateEmbedAuthor::new(
									format!(concat!("{} {} :: ", #plain), prefix, &data.username)
								)
								.icon_url(player.get_head_url())
							);
						} else {
							embed = embed.author(
								::poise::serenity_prelude::CreateEmbedAuthor::new(
									format!(concat!("{} :: ", #plain), &data.username)
								)
								.icon_url(player.get_head_url())
							);
						}

						embed = #overall_ident.embed(
							ctx,
							embed,
							data,
						);

						#(#embed_game)*
						#buffer_fields

						embed
					}

					fn embed_diff<'e>(
						ctx: &'e #translate::context::Context<'_>,
						player: &#api::player::Player,
						data_lhs: &#api::player::data::Data,
						data_rhs: &#api::player::data::Data,
					) -> #poise::serenity_prelude::CreateEmbed<'e> {
						let mut embed = #poise::serenity_prelude::CreateEmbed::default()
							.thumbnail(player.get_body_url());

						if let Some(prefix) = data_rhs.get_rank().as_str() {
							embed = embed.author(
								#poise::serenity_prelude::CreateEmbedAuthor::new(
									format!(concat!("{} {} :: ", #plain), prefix, &data_rhs.username)
								)
								.icon_url(player.get_head_url())
							);
						} else {
							embed = embed.author(
								#poise::serenity_prelude::CreateEmbedAuthor::new(
									format!(concat!("{} :: ", #plain), &data_rhs.username)
								)
								.icon_url(player.get_head_url())
							);
						}

						embed = #overall_ident::embed_diff(
							ctx,
							embed,
							data_lhs,
							data_rhs,
						);

						#(#embed_diff_game)*
						#buffer_fields

						embed
					}
				}
			}
		});
	}
}
