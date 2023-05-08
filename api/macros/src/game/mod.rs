mod label;
mod sum;
mod tokens;

use darling::{ast, FromDeriveInput, FromField, FromMeta};
use minecraft::{paint::MinecraftPaint, text::parse::parse_minecraft_string};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

use self::label::{map_game_field_to_extras_value, map_info_field_to_extras_value};
use crate::game::tokens::get_tr_with_fallback;

macro_rules! ident {
	($id: literal) => {
		syn::Ident::new($id, Span::call_site())
	};
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(game), supports(struct_named))]
pub(crate) struct GameInputReceiver {
	pub ident: syn::Ident,
	pub generics: syn::Generics,
	pub data: ast::Data<(), GameFieldReceiver>,

	/// The path to the game data in the PlayerStats struct.
	pub path: syn::Ident,

	/// A pretty name for the game, coloured with Minecraft escape codes.
	pub pretty: String,

	/// A path to the module with level calculations.
	pub calc: Option<syn::PatPath>,

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
		} = *self;

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
			} else if let Some(label) = field.label.as_ref() {
				labels.push((field, label));
			} else if field.xp.is_some() {
				xp_field = Some(field);
			} else if field.level.is_some() {
				level_fmt_field = Some(field);
			}
		}

		let (level_fmt_field, xp_field) = match (level_fmt_field, xp_field) {
			(Some(level), Some(xp)) => {
				let level_name = level.ident.as_ref().unwrap();
				let xp_name = xp.ident.as_ref().unwrap();

				(quote!(stats. #level_name), quote!(stats. #xp_name))
			}
			(Some(_), None) => panic!("xp field required when level field is present"),
			(_, xp_field) => (
				quote!(#calc ::get_level_format(level)),
				if let Some(xp_field) = xp_field {
					let xp_name = xp_field.ident.as_ref().unwrap();

					quote!(stats. #xp_name)
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

			let sum = sum::sum_fields(
				modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
				Some(&ident!("stats")),
				name,
			);

			let value = if let Some(div) = info.div.as_ref() {
				let sum_bottom = sum::sum_fields(
					modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
					Some(&ident!("stats")),
					div,
				);

				quote! { #sum * 100 / #sum_bottom }
			} else {
				quote! { #sum }
			};

			quote! {
				(
					::translate::tr!(ctx, #tr),
					#value,
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
				Self::#ty => #ty ::get_row_count(),
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

			let sum = if let Some(div) = field.div.as_ref() {
				sum::sum_div_f32_fields(
					modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
					Some(&ident!("stats")),
					ident_parent,
					div,
				)
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
					#i,
				);
			}
		});

		let apply_items_mode = overall_fields.iter().enumerate().map(|(idx, field)| {
			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				sum::div_f32_single_field(&ident!("self"), None, ident, div)
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
					#idx,
				);
			}
		});

		let apply_all_modes = modes.iter().map(|mode| {
			let ty = &mode.ty;
			let tr = mode.mode.as_ref().unwrap().tr.as_ref();
			let tr = if let Some(tr) = tr {
				quote! { #tr }
			} else {
				quote! { #ty }
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

						let extras = &[
							#(#extras)*
							#(#extras_for_mode)*
						];

						crate::canvas::draw::apply_extras(
							ctx,
							surface,
							extras,
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

					let extras = &[
						#(#extras)*
						#(#extras_for_overall)*
					];

					crate::canvas::draw::apply_extras(
						ctx,
						surface,
						extras,
					);

					#(#apply_items_overall)*
				}
			}

			// Implement the game mode enum.
			#[derive(::std::fmt::Debug, ::poise::ChoiceParameter)]
			pub enum #enum_ident {
				Overall,
				#(#mode_enum_rows)*
			}

			// Implement the default impl for the game mode enum.
			// This should be able to get the recommended mode from the session.
			impl #enum_ident {
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
				pub fn canvas(ctx: ::translate::Context<'_>, player: &crate::player::data::PlayerData, session: &crate::player::status::PlayerSession, mode: Option<#enum_ident>) -> ::skia_safe::Surface {
					let stats = &player.stats.#path;
					let xp = #xp_field;
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
}

#[derive(Debug, FromMeta)]
pub(crate) struct InfoFieldData {
	#[darling(default)]
	colour: MinecraftPaint,

	/// The translation key of the label.
	/// Defaults to the ident with underscores replaced with dashes.
	tr: Option<String>,

	ident: syn::Ident,

	div: Option<syn::Ident>,

	percent: Option<bool>,
}
