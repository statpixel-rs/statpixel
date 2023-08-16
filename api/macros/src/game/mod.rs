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
	impls::{Crates, Idents, State, Streams},
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
			futures,
		} = &state.crates;
		let Idents {
			mode_enum,
			kind_enum,
			calc,
			path_to_game,
		} = &state.idents;
		let Streams {
			blocks_sum,
			blocks_diff_sum,
			labels_sum,
			labels_diff_sum,
			..
		} = &state.streams;

		let game_ident = &self.ident;
		let blocks = self.blocks();
		let labels = self.labels();
		let modes = self.modes();
		let overall_modes = self.overall_modes();

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
				const LABEL: [#minecraft::text::Text; #size] = #minecraft::text::parse::minecraft_text(#pretty);
				const PRETTY: &str = #pretty;
				const PLAIN: &str = #plain;
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

			let slice = rows.iter().map(|r| quote!(#mode_enum::#r));
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
						} else if session.game_type == Some(#api::game::r#type::Type::#game_ident) && let Some(game_mode) = session.game_mode.as_ref() {
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
			let slice = blocks.iter().map(|id| quote!(#kind_enum::#id));
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

		for mode in &modes {
			impls::impl_mode(tokens, &state, mode);
		}

		let as_root = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr!(ctx, #tr), #api::id::command(#api::command::Id::Root {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid,
					background: None,
				}))
			}
		});

		let as_snapshot = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr!(ctx, #tr), #api::id::command(#api::command::Id::Snapshot {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid,
					past,
					background: None,
				}))
			}
		});

		let as_history = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr!(ctx, #tr), #api::id::command(#api::command::Id::History {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid,
					background: None,
				}))
			}
		});

		let as_project = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr!(ctx, #tr), #api::id::command(#api::command::Id::Project {
					kind: #api::command::ProjectMode::#game_ident(#mode_enum::#ty, kind),
					uuid,
					background: None,
				}))
			}
		});

		let as_compare = modes.iter().take(24).map(|mode| {
			let ty = mode.ty();
			let tr = mode.tr();

			quote! {
				#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr!(ctx, #tr), #api::id::command(#api::command::Id::Compare {
					kind: #api::command::Mode::#game_ident(#mode_enum::#ty),
					uuid_lhs,
					uuid_rhs,
					background: None,
				}))
			}
		});

		let min = self.min_sum(&ident("min"), &overall_modes);
		let max = self.max_sum(&ident("max"), &overall_modes);

		tokens.extend(quote! {
			impl #api::prelude::Mode for #mode_enum {
				type Kind = #kind_enum;

				fn as_root(
					ctx: &#translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr!(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::Root {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid,
									background: None,
								})),
								#(#as_root),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr!(ctx, selected.tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::Root {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid,
							background: None,
						})
					)
				}

				fn as_snapshot(
					ctx: &#translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					past: i64,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::Snapshot {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid,
									past,
									background: None,
								})),
								#(#as_snapshot),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr!(ctx, selected.tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::Snapshot {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid,
							past,
							background: None,
						})
					)
				}

				fn as_history(
					ctx: &#translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::History {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid,
									background: None,
								})),
								#(#as_history),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr!(ctx, selected.tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::History {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid,
							background: None,
						})
					)
				}

				fn as_project(
					ctx: &#translate::context::Context<'_>,
					uuid: #uuid::Uuid,
					kind: Self::Kind,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr!(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::Project {
									kind: #api::command::ProjectMode::#game_ident(#mode_enum::#overall_ident, kind),
									uuid,
									background: None,
								})),
								#(#as_project),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr!(ctx, selected.tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::Project {
							kind: #api::command::ProjectMode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident), kind),
							uuid,
							background: None,
						})
					)
				}

				fn as_compare(
					ctx: &#translate::context::Context<'_>,
					uuid_lhs: #uuid::Uuid,
					uuid_rhs: #uuid::Uuid,
					selected: Option<#mode_enum>
				) -> (#poise::serenity_prelude::CreateActionRow, #api::id::Id) {
					let mut menu = #poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						#poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#poise::serenity_prelude::CreateSelectMenuOption::new(#translate::tr!(ctx, #overall_ident::tr()), #api::id::command(#api::command::Id::Compare {
									kind: #api::command::Mode::#game_ident(#mode_enum::#overall_ident),
									uuid_lhs,
									uuid_rhs,
									background: None,
								})),
								#(#as_compare),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(#translate::tr!(ctx, selected.tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						#poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						#api::id::Id::Command(#api::command::Id::Compare {
							kind: #api::command::Mode::#game_ident(selected.unwrap_or(#mode_enum::#overall_ident)),
							uuid_lhs,
							uuid_rhs,
							background: None,
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

					field.push_str(#translate::tr!(ctx, Self::tr()).as_ref());
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
							::std::vec![(
								#translate::tr!(ctx, #tr),
								snapshots
									.iter()
									.map(|(time, data)| (*time, #value))
									.collect::<Vec<_>>(),
								predict_x.unwrap_or(last.0),
								predict_y,
							)],
							first.0..predict_x.map_or(last.0, |x| x.max(last.0)),
							({
								let data = &first.1;
								#value
							} * (7. / 8.))..(predict_y.max({
								let data = &last.1;
								#value
							}) * (8. / 7.)),
							None,
							background,
						)?;

						let mut surface = #api::canvas::project::canvas(&mut buffer)?;

						#api::canvas::chart::apply_title(ctx, &mut surface, &last.1, &LABEL, background);

						let r = #api::percent::PercentU32((#api::canvas::project::line::compute_r(&series, &line) * 100.) as u32);

						if let Some(x) = predict_x {
							#api::canvas::project::apply_bubbles(
								&mut surface,
								ctx,
								#translate::tr!(ctx, #tr).as_ref(),
								&predict_y,
								&r,
								&x,
								background,
							);
						} else {
							#api::canvas::project::apply_bubbles(
								&mut surface,
								ctx,
								#translate::tr!(ctx, #tr).as_ref(),
								&predict_y,
								&r,
								&#translate::tr!(ctx, "never").as_ref(),
								background,
							);
						}

						#api::canvas::project::round_corners(&mut surface);

						Ok(#api::canvas::to_png(&mut surface))
					}
				}
			});

		tokens.extend(quote! {
			pub struct #overall_ident;

			impl #api::canvas::diff::DiffLog for #overall_ident {
				fn diff_log(
					data_lhs: &#api::player::data::Data,
					data_rhs: &#api::player::data::Data,
					ctx: &#translate::context::Context<'_>,
					mut embed: #poise::serenity_prelude::Embed,
				) -> #poise::serenity_prelude::Embed {
					let mut log = String::new();
					let game_lhs = &data_lhs.stats.#path_to_game;
					let game_rhs = &data_rhs.stats.#path_to_game;

					#(#diff_log)*

					if !log.is_empty() {
						let mut title = String::new();

						title.push_str(PLAIN);
						title.push(' ');
						title.push_str(#translate::tr!(ctx, Overall::tr()).as_ref());

						embed.fields.push(#poise::serenity_prelude::EmbedField::new(title, log, true));
						embed
					} else {
						embed
					}
				}
			}

			impl #overall_ident {
				#[inline]
				pub fn tr() -> &'static str {
					"Overall"
				}

				pub fn embed(
					&self,
					ctx: &#translate::context::Context<'_>,
					embed: #poise::serenity_prelude::CreateEmbed,
					data: &#api::player::data::Data,
				) -> #poise::serenity_prelude::CreateEmbed {
					let game = &data.stats.#path_to_game;
					let mut field = String::new();

					#(#embed)*

					embed.field(
						#translate::tr!(ctx, Self::tr()),
						field,
						true,
					)
				}

				pub fn embed_diff(
					ctx: &#translate::context::Context<'_>,
					embed: #poise::serenity_prelude::CreateEmbed,
					data_lhs: &#api::player::data::Data,
					data_rhs: &#api::player::data::Data,
				) -> #poise::serenity_prelude::CreateEmbed {
					let mut field = String::new();
					let game_lhs = &data_lhs.stats.#path_to_game;
					let game_rhs = &data_rhs.stats.#path_to_game;

					#(#embed_diff)*

					embed.field(#translate::tr!(ctx, Self::tr()), field, true)
				}

				pub fn project(
					ctx: &#translate::context::Context<'_>,
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
					snapshots: ::std::vec::Vec<(#chrono::DateTime<#chrono::Utc>, #api::player::data::Data)>,
					background: Option<#skia::Color>,
				) -> Result<::std::vec::Vec<u8>, #translate::Error> {
					let first = snapshots.first().unwrap();
					let last = snapshots.last().unwrap();

					let lower = Self::min_fields(&#overall_ident, &first.1);
					let upper = ::std::cmp::max(Self::max_fields(&#overall_ident, &last.1), 100);

					let x_range = first.0.clone()..last.0.clone();
					let last_data = last.1.clone();

					let v = ::std::vec![
						#(#chart,)*
					];

					let mut buffer = #api::canvas::chart::u32::create::<true>(
						ctx,
						v,
						x_range,
						((f64::from(lower) * (11. / 16.)) as u32)..((f64::from(upper) * (16. / 15.)) as u32),
						None,
						background,
					)?;

					let mut surface = #api::canvas::chart::canvas(&mut buffer)?;

					#api::canvas::chart::apply_title(ctx, &mut surface, &last_data, &LABEL, background);
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

				pub fn canvas<'c>(
					&self,
					ctx: &#translate::context::Context<'_>,
					mut canvas: #api::canvas::Canvas<'c>,
					data: &'c #api::player::data::Data,
					session: &'c #api::player::status::Session,
					status: &'c #api::canvas::shape::Status,
					progress: &'c #api::canvas::shape::WideBubbleProgress,
				) -> #api::canvas::Canvas<'c> {
					let game = &data.stats.#path_to_game;
					let xp = #calc::convert(&#xp);
					let level = #calc::get_level(xp);

					canvas
						.push_down(
							&#api::canvas::shape::Subtitle,
							#api::canvas::shape::Subtitle::from_label(ctx, &LABEL, #overall_ident::tr()),
						)
						.push_down_post_draw(
							progress,
							#api::canvas::shape::WideBubbleProgress::from_level_progress(
								ctx,
								&#calc::get_level_format(level),
								&#calc::get_curr_level_xp(xp),
								&#calc::get_level_xp(xp),
							)
						)
						#labels_sum
						.push_right_post_draw(
							status,
							#api::canvas::body::Body::from_status(ctx, session)
						)
						#blocks_sum
				}

				pub fn canvas_diff<'c>(
					ctx: &#translate::context::Context<'_>,
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
							#api::canvas::shape::Subtitle::from_label(ctx, &LABEL, #overall_ident::tr()),
						)
						.push_down_post_draw(
							progress,
							#api::canvas::shape::WideBubbleProgress::from_level_diff(
								ctx,
								&#calc::get_level_format(level),
								&#calc::get_total_xp(xp),
								positive,
								is_different,
							),
						)
						#labels_diff_sum
						.push_right_post_draw(
							status,
							#api::canvas::body::Body::from_status(ctx, session)
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
						canvas,
						data,
						session,
						&status,
						&progress,
					),
				}
			});

			let canvas_diff_mode = modes.iter().map(|mode| {
				let ty = mode.ty();

				quote! {
					#mode_enum::#ty => #ty::canvas_diff(
						ctx,
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
					let embed = <#ty as #api::canvas::diff::DiffLog>::diff_log(data_lhs, data_rhs, ctx, embed);
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

			quote! {
				impl #game_ident {
					pub fn from_kind<'t, 'c: 't>(
						ctx: &'c #translate::context::Context<'c>,
						data: &'t #api::player::data::Data,
						kind: &#kind_enum
					) -> ::std::borrow::Cow<'static, str> {
						let game = &data.stats.#path_to_game;

						let value: String = match kind {
							#kind_enum::level => unreachable!(),
							#(#from_kind_match,)*
							_ => panic!("unknown kind"),
						};

						::std::borrow::Cow::Owned(value)
					}

					pub fn from_kind_diff<'t, 'c: 't>(
						ctx: &'c #translate::context::Context<'c>,
						data_lhs: &'t #api::player::data::Data,
						data_rhs: &'t #api::player::data::Data,
						kind: &#kind_enum
					) -> String {
						let game_lhs = &data_lhs.stats.#path_to_game;
						let game_rhs = &data_rhs.stats.#path_to_game;

						match kind {
							#kind_enum::level => unreachable!(),
							#(#from_kind_diff_match,)*
							_ => panic!("unknown kind"),
						}
					}

					pub async fn autocomplete<'a>(ctx: #translate::Context<'a>, partial: String) -> impl #futures::Stream<Item = #poise::AutocompleteChoice<u32>> + 'a {
						use #futures::StreamExt as _;

						#futures::stream::iter(#mode_enum::slice())
							.filter_map(move |mode| {
								let name = #translate::tr!(&ctx, mode.tr());
								let mode: u32 = mode.into();

								#futures::future::ready(if name.to_ascii_lowercase().contains(&partial) {
									Some(#poise::AutocompleteChoice {
										name: name.to_string(),
										value: mode,
									})
								} else {
									None
								})
							})
							.take(10)
					}

					pub async fn autocomplete_kind<'a>(ctx: #translate::Context<'a>, partial: String) -> impl #futures::Stream<Item = #poise::AutocompleteChoice<u32>> + 'a {
						use #futures::StreamExt as _;

						#futures::stream::iter(#kind_enum::slice())
							.filter_map(move |kind| {
								let name = #translate::tr!(&ctx, kind.tr());
								let kind: u32 = kind.into();

								#futures::future::ready(if name.to_ascii_lowercase().contains(&partial) {
									Some(#poise::AutocompleteChoice {
										name: name.to_string(),
										value: kind,
									})
								} else {
									None
								})
							})
							.take(10)
					}
				}

				impl #api::canvas::diff::DiffLog for #game_ident {
					fn diff_log(
						data_lhs: &#api::player::data::Data,
						data_rhs: &#api::player::data::Data,
						ctx: &#translate::context::Context<'_>,
						embed: #poise::serenity_prelude::Embed,
					) -> #poise::serenity_prelude::Embed {
						let embed = <#overall_ident as #api::canvas::diff::DiffLog>::diff_log(data_lhs, data_rhs, ctx, embed);
						#(#diff_log)*
						embed
					}
				}

				impl #api::prelude::Game for #game_ident {
					type Mode = #mode_enum;

					#[allow(clippy::too_many_arguments)]
					fn canvas_diff(
						ctx: &#translate::context::Context<'_>,
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
						let mut canvas = #api::canvas::Canvas::new(720.)
							.gap(7.)
							.push_down(
								&#api::canvas::shape::Title,
								#api::canvas::shape::Title::from_text(&#api::canvas::text::from_data(&data_rhs, &data_rhs.username, suffix)),
							);

						if is_different {
							canvas = canvas.push_down(
								&#api::canvas::shape::Subtitle,
								#api::canvas::shape::Subtitle::from_text(&#api::canvas::text::from_data(&data_lhs, &data_lhs.username, None)),
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

					fn canvas(
						ctx: &#translate::context::Context<'_>,
						data: &#api::player::data::Data,
						session: &#api::player::status::Session,
						skin: &#skia::Image,
						mode: Option<Self::Mode>,
						suffix: Option<&str>,
						background: Option<#skia::Color>,
					) -> (#skia::Surface, Self::Mode) {
						let game = &data.stats.#path_to_game;

						let mode = #mode_enum::get_mode(mode, session);
						let mut canvas = #api::canvas::Canvas::new(720.)
							.gap(7.)
							.push_down(
								&#api::canvas::shape::Title,
								#api::canvas::shape::Title::from_text(&#api::canvas::text::from_data(&data, &data.username, suffix)),
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
									snapshots,
									background,
								)
							}
							#(#chart_mode)*
						}?, mode))
					}

					fn project(
						ctx: &#translate::context::Context<'_>,
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
									snapshots,
									kind,
									value,
									background,
								)
							}
							#(#project_mode)*
						}?, mode))
					}

					fn embed(
						ctx: &#translate::context::Context<'_>,
						player: &#api::player::Player,
						data: &#api::player::data::Data,
					) -> #poise::serenity_prelude::CreateEmbed {
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

					fn embed_diff(
						ctx: &#translate::context::Context<'_>,
						player: &#api::player::Player,
						data_lhs: &#api::player::data::Data,
						data_rhs: &#api::player::data::Data,
					) -> #poise::serenity_prelude::CreateEmbed {
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
