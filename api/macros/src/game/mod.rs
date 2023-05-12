mod label;
mod sum;
mod tokens;

use darling::{ast, FromDeriveInput, FromField, FromMeta};
use minecraft::{paint::MinecraftPaint, text::parse::parse_minecraft_string};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::label::{
	map_game_field_to_extras_value, map_info_field_to_extras_value, parse_str_to_dot_path,
};
use crate::game::tokens::get_tr_with_fallback;

macro_rules! ident {
	($id: literal) => {
		::syn::Ident::new($id, ::proc_macro2::Span::call_site())
	};
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(game), supports(struct_named))]
pub(crate) struct GameInputReceiver {
	pub ident: syn::Ident,
	pub generics: syn::Generics,
	pub data: ast::Data<(), GameFieldReceiver>,

	/// The path to the game data in the PlayerStats struct.
	pub path: String,

	/// A pretty name for the game, coloured with Minecraft escape codes.
	pub pretty: String,

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
}

impl ToTokens for GameInputReceiver {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let GameInputReceiver {
			ref ident,
			ref generics,
			ref data,
			ref path,
			ref pretty,
			ref calc,
			field: ref overall_fields,
			label: ref info,
			xp: ref xp_path,
		} = *self;

		let path = parse_str_to_dot_path(path);

		let label_size = parse_minecraft_string(pretty).count();
		let row_count = (overall_fields.len() + 2) as u8 / 3;

		let calc = if let Some(ref calc) = calc {
			quote! { #calc }
		} else {
			quote! { ::minecraft::calc::network }
		};

		let (imp, ty, wher) = generics.split_for_impl();
		let fields = data
			.as_ref()
			.take_struct()
			.expect("should be a named struct")
			.fields;

		let mut modes = Vec::new();
		let mut labels = Vec::new();

		let mut xp_field = None;
		let mut level_fmt_field = None;

		for field in fields {
			if field.mode.is_some() {
				modes.push(field);
			}

			if let Some(label) = field.label.as_ref() {
				labels.push((field, label));
			}

			if field.xp.is_some() {
				let xp_name = field.ident.as_ref().unwrap();

				xp_field = Some(quote!(stats. #xp_name));
			}

			if field.level.is_some() {
				level_fmt_field = Some(field);
			}
		}

		if xp_field.is_none() && let Some(path) = xp_path.as_ref() {
			let path = parse_str_to_dot_path(path);

			xp_field = Some(quote!(player.stats.#path));
		}

		modes.sort_by_cached_key(|field| field.ident.as_ref().unwrap().to_string());

		let (level_fmt_field, xp_field) = match (level_fmt_field, xp_field) {
			(Some(level), Some(xp)) => {
				let level_name = level.ident.as_ref().unwrap();

				(quote!(stats. #level_name), xp)
			}
			(Some(_), None) => panic!("xp field required when level field is present"),
			(_, xp_field) => (
				quote!(#calc ::get_level_format(level)),
				if let Some(xp_field) = xp_field {
					xp_field
				} else {
					quote!(player.xp)
				},
			),
		};

		let enum_ident = syn::Ident::new(&format!("{}Mode", ident), proc_macro2::Span::call_site());
		let extras = labels.iter().map(map_game_field_to_extras_value);
		let extras_for_mode = info.iter().map(map_info_field_to_extras_value);
		let extras_for_overall = info.iter().map(|info| {
			let name = &info.ident;
			let tr = get_tr_with_fallback(info.tr.as_deref(), Some(name));

			let colour = &info.colour;
			let percent = if info.percent == Some(true) {
				quote! { true }
			} else {
				quote! { false }
			};

			let sum = if let Some(path) = info.path.as_ref() {
				let path = parse_str_to_dot_path(path);

				quote! { player.stats.#path.#name }
			} else {
				sum::sum_fields(
					modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
					Some(&ident!("stats")),
					name,
				)
			};

			let value = if let Some(div) = info.div.as_ref() {
				let sum_bottom = sum::sum_fields(
					modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
					Some(&ident!("stats")),
					div,
				);

				quote! { #sum * 100 / if #sum_bottom == 0 { 1 } else { #sum_bottom } }
			} else {
				quote! { #sum }
			};

			quote! {
				(
					::translate::tr!(ctx, #tr),
					::std::boxed::Box::new(#value),
					#colour,
					#percent,
				),
			}
		});

		let mode_enum_rows = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				#ty,
			}
		});

		let mode_match_apply_rows = modes.iter().map(|mode| {
			let ty = &mode.ty;
			let ident = mode.ident.as_ref().unwrap();

			quote! {
				#enum_ident ::#ty => player.stats. #path. #ident .apply(
					ctx,
					&mut surface,
					player,
					session,
				),
			}
		});

		let mode_match_count_rows = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				#enum_ident ::#ty => #ty ::get_row_count(),
			}
		});

		let mode_match_get_tr = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				#enum_ident ::#ty => #ty ::get_tr(),
			}
		});

		let first_ty = modes
			.first()
			.map(|mode| &mode.ty)
			.expect("there must be at least one #[game(mode(...))] attribute");

		let mode_from_str_impl =
			modes.iter().map(
				|mode| match mode.mode.as_ref().and_then(|m| m.hypixel.as_ref()) {
					Some(hypixel) => {
						let left = hypixel;
						let ty = &mode.ty;

						quote! {
							#left => Self::#ty,
						}
					}
					_ => quote! {},
				},
			);

		let label_iter = (0..label_size).map(|i| {
			quote! {
				LABEL[#i],
			}
		});

		let apply_items_overall = overall_fields.iter().enumerate().map(|(i, field)| {
			let ident_parent = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident_parent));
			let colour = &field.colour;
			let percent = match field.percent.as_ref() {
				Some(true) => quote! { Some(true) },
				_ => quote! { Some(false) },
			};

			let sum = if let Some(path) = field.path.as_ref() {
				let path = parse_str_to_dot_path(path);

				return quote! {
					player.stats.#path.#ident_parent
				};
			} else if let Some(div) = field.div.as_ref() {
				if field.percent == Some(true) {
					sum::sum_div_u32_fields(
						modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
						Some(&ident!("stats")),
						ident_parent,
						div,
					)
				} else {
					sum::sum_div_f32_fields(
						modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
						Some(&ident!("stats")),
						ident_parent,
						div,
					)
				}
			} else {
				sum::sum_fields(
					modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
					Some(&ident!("stats")),
					ident_parent,
				)
			};

			quote! {
				crate::canvas::draw::apply_item(
					ctx,
					surface,
					#sum,
					&::translate::tr!(ctx, #tr),
					#colour,
					#percent,
					#i,
				);
			}
		});

		let apply_items_mode = overall_fields.iter().enumerate().map(|(idx, field)| {
			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let percent = match field.percent.as_ref() {
				Some(true) => quote! { Some(true) },
				_ => quote! { Some(false) },
			};
			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				if field.percent == Some(true) {
					sum::div_u32_single_field(&ident!("self"), None, ident, div)
				} else {
					sum::div_f32_single_field(&ident!("self"), None, ident, div)
				}
			} else {
				quote! { self.#ident }
			};

			quote! {
				crate::canvas::draw::apply_item(
					ctx,
					surface,
					#value,
					&::translate::tr!(ctx, #tr),
					#colour,
					#percent,
					#idx,
				);
			}
		});

		let modes_len = modes.len() + 1;

		let mode_into_int_impl = modes.iter().enumerate().map(|(idx, mode)| {
			// idx = 0 is for Overall
			let idx = idx as u32 + 1;
			let ty = &mode.ty;

			quote! {
				&#enum_ident ::#ty => #idx,
			}
		});

		let mode_from_int_impl = modes.iter().enumerate().map(|(idx, mode)| {
			// idx = 0 is for Overall
			let idx = idx as u32 + 1;
			let ty = &mode.ty;

			quote! {
				#idx => #enum_ident ::#ty,
			}
		});

		let impl_mode_enum = if modes_len > 25 {
			// There can only be 25 options in a ChoiceParameter, so we need to use
			// autocomplete instead.
			quote! {
				// Implement the game mode enum.
				#[derive(::std::fmt::Debug)]
				pub enum #enum_ident {
					Overall,
					#(#mode_enum_rows)*
				}
			}
		} else {
			quote! {
				// Implement the game mode enum.
				#[derive(::std::fmt::Debug, ::poise::ChoiceParameter)]
				pub enum #enum_ident {
					Overall,
					#(#mode_enum_rows)*
				}
			}
		};

		let static_modes_iter = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				#enum_ident ::#ty,
			}
		});

		let apply_all_modes = modes.iter().map(|mode| {
			let ty = &mode.ty;
			let tr = mode.mode.as_ref().unwrap().tr.as_ref();
			let tr = if let Some(tr) = tr {
				quote! { #tr }
			} else {
				quote! { stringify!(#ty) }
			};

			let apply_items_mode = apply_items_mode.clone();
			let extras = extras.clone();
			let extras_for_mode = extras_for_mode.clone();

			quote! {
				impl #ty {
					pub fn get_row_count() -> u8 {
						#row_count
					}

					pub fn get_tr() -> &'static str {
						#tr
					}

					pub fn apply(&self, ctx: ::translate::Context<'_>, surface: &mut ::skia_safe::Surface, player: &crate::player::data::PlayerData, session: &crate::player::status::PlayerSession) {
						let label = ::translate::tr!(ctx, Self::get_tr());
						let stats = &player.stats.#path;

						crate::canvas::draw::apply_label(
							surface,
							[
								LABEL.as_slice(),
								&[::minecraft::text::Text {
									text: &::std::format!(" ({label})"),
									paint: ::minecraft::paint::MinecraftPaint::White,
									font: ::minecraft::style::MinecraftFont::Normal,
									size: ::std::option::Option::None,
								}],
							]
								.concat()
								.as_slice(),
						);

						#(#apply_items_mode)*

						crate::canvas::draw::apply_extras(
							ctx,
							surface,
							&[
								#(#extras)*
								#(#extras_for_mode)*
							],
						);
					}
				}
			}
		});

		tokens.extend(quote! {
			const LABEL: [::minecraft::text::Text; #label_size] = ::minecraft::minecraft_text!(#pretty);

			#(#apply_all_modes)*

			pub struct Overall;

			impl Overall {
				pub fn get_row_count() -> u8 {
					#row_count
				}

				pub fn get_tr() -> &'static str {
					"Overall"
				}

				pub fn label_len() -> usize {
					#label_size
				}

				pub fn apply(ctx: ::translate::Context<'_>, surface: &mut ::skia_safe::Surface, player: &crate::player::data::PlayerData, session: &crate::player::status::PlayerSession) {
					let stats = &player.stats.#path;
					let label = ::translate::tr!(ctx, Self::get_tr());

					crate::canvas::draw::apply_label(
						surface,
						&[
							#(#label_iter)*
							::minecraft::text::Text {
								text: &::std::format!(" ({label})"),
								paint: ::minecraft::paint::MinecraftPaint::White,
								font: ::minecraft::style::MinecraftFont::Normal,
								size: ::std::option::Option::None,
							},
						],
					);

					crate::canvas::draw::apply_extras(
						ctx,
						surface,
						&[
							#(#extras)*
							#(#extras_for_overall)*
						],
					);

					#(#apply_items_overall)*
				}
			}

			#impl_mode_enum

			impl From<&#enum_ident> for u32 {
				fn from(value: &#enum_ident) -> u32 {
					match value {
						&#enum_ident ::Overall => 0,
						#(#mode_into_int_impl)*
					}
				}
			}

			impl From<u32> for #enum_ident {
				fn from(value: u32) -> Self {
					match value {
						0 => #enum_ident ::Overall,
						#(#mode_from_int_impl)*
						_ => #enum_ident ::Overall,
					}
				}
			}

			// Implement the default impl for the game mode enum.
			// This should be able to get the recommended mode from the session.
			impl #enum_ident {
				pub fn slice() -> &'static [#enum_ident] {
					static MODES: [#enum_ident; #modes_len] = [
						#enum_ident ::Overall,
						#(#static_modes_iter)*
					];

					&MODES
				}

				pub fn get_mode(mode: Option<#enum_ident>, session: &crate::player::status::PlayerSession) -> #enum_ident {
					if let Some(mode) = mode {
						mode
					} else if session.game_type == Some(crate::game::r#type::GameType::#ident) && let Some(game_mode) = session.game_mode.as_ref() {
						#enum_ident ::from(game_mode.as_str())
					} else {
						#enum_ident ::Overall
					}
				}

				pub fn get_row_count(&self) -> u8 {
					match self {
						Self::Overall => #first_ty ::get_row_count(),
						#(#mode_match_count_rows)*
					}
				}

				pub fn get_tr(&self) -> &'static str {
					match self {
						Self::Overall => Overall::get_tr(),
						#(#mode_match_get_tr)*
					}
				}
			}

			impl ::std::convert::From<&str> for #enum_ident {
				fn from(s: &str) -> Self {
					match s {
						#(#mode_from_str_impl)*
						_ => Self::Overall,
					}
				}
			}

			impl #imp #ident #ty #wher {
				pub async fn autocomplete<'a>(ctx: ::translate::Context<'a>, partial: ::std::string::String) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
					::futures::StreamExt::take(
						::futures::StreamExt::filter_map(::futures::stream::iter(#enum_ident ::slice()), move |mode| {
							let name = ::translate::tr!(ctx, mode.get_tr());

							::futures::future::ready(if name.to_ascii_lowercase().contains(&partial) {
								::std::option::Option::Some(::poise::AutocompleteChoice {
									name,
									value: mode.into(),
								})
							} else {
								::std::option::Option::None
							})
						}), 25)
				}

				pub fn canvas(ctx: ::translate::Context<'_>, player: &crate::player::data::PlayerData, session: &crate::player::status::PlayerSession, mode: Option<#enum_ident>) -> ::skia_safe::Surface {
					let stats = &player.stats.#path;
					let xp = #calc ::convert(&#xp_field);
					let level = #calc ::get_level(xp);

					let mode = #enum_ident ::get_mode(mode, session);
					let mut surface = crate::canvas::create_surface(mode.get_row_count());

					match mode {
						#enum_ident ::Overall => {
							Overall::apply(
								ctx,
								&mut surface,
								player,
								session,
							);
						}
						#(#mode_match_apply_rows)*
					}

					crate::canvas::header::apply_name(&mut surface, &player);

					crate::canvas::draw::apply_data(
						ctx,
						&mut surface,
						&#level_fmt_field,
						#calc ::get_level_progress(xp),
						#calc ::get_curr_level_xp(xp),
						#calc ::get_level_xp(xp),
						&#calc ::get_colours(level),
					);

					crate::canvas::header::apply_status(
						ctx,
						&mut surface,
						&session
					);

					surface
				}
			}
		});
	}
}

#[derive(Debug, FromField)]
#[darling(attributes(game))]
pub(crate) struct GameFieldReceiver {
	/// Get the ident of the field. For fields in tuple or newtype structs or
	/// enum bodies, this can be `None`.
	pub ident: Option<syn::Ident>,

	/// This magic field name pulls the type from the input.
	pub ty: syn::Type,

	/// Field data
	pub label: Option<GameLabel>,

	/// Mode data
	pub mode: Option<ModeData>,

	/// Field that stores player xp
	pub xp: Option<bool>,

	/// Field that stores player level
	pub level: Option<bool>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct GameLabel {
	#[darling(default)]
	colour: MinecraftPaint,

	/// The translation key of the label.
	/// Defaults to the ident with underscores replaced with dashes.
	tr: Option<String>,

	div: Option<syn::Ident>,

	percent: Option<bool>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ModeData {
	hypixel: Option<String>,
	tr: Option<String>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct OverallFieldData {
	ident: syn::Ident,

	div: Option<syn::Ident>,

	tr: Option<String>,

	#[darling(default)]
	colour: MinecraftPaint,

	percent: Option<bool>,

	path: Option<String>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct InfoFieldData {
	ident: syn::Ident,

	div: Option<syn::Ident>,

	tr: Option<String>,

	#[darling(default)]
	colour: MinecraftPaint,

	percent: Option<bool>,

	path: Option<String>,
}
