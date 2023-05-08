use darling::{ast, FromDeriveInput, FromField, FromMeta};
use minecraft::{paint::MinecraftPaint, text::parse::parse_minecraft_string};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(game), supports(struct_named))]
pub(crate) struct GameInputReceiver {
	/// The struct ident.
	pub ident: syn::Ident,

	/// The type's generics. You'll need these any time your trait is expected
	/// to work with types that declare generics.
	pub generics: syn::Generics,

	/// Receives the body of the struct or enum. We don't care about
	/// struct fields because we previously told darling we only accept structs.
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
	pub fields: Vec<OverallFieldData>,

	/// The additional fields to include in the info header.
	#[darling(multiple)]
	pub info: Vec<InfoFieldData>,
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
			fields: ref overall_fields,
			ref info,
		} = *self;

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

		let extras = labels.iter().map(|(field, label)| {
			let name = field.ident.as_ref().unwrap();
			let tr = if let Some(tr) = &label.tr {
				quote! { #tr }
			} else {
				let name = name.to_string().replace('_', "-");

				quote! { #name }
			};

			let colour = &label.colour;
			let percent = if label.percent == Some(true) {
				quote! { true }
			} else {
				quote! { false }
			};

			let value = if let Some(div) = label.div.as_ref() {
				quote! { stats.#name * 100 / stats.#div }
			} else {
				quote! { stats.#name }
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

		let label_size = parse_minecraft_string(pretty).count();

		let mode_match_count_rows = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				Self::#ty => #ty ::get_row_count(),
			}
		});

		let first_ty = modes.first().map(|mode| &mode.ty).unwrap();

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

		let apply_items = overall_fields.iter().enumerate().map(|(i, field)| {
			let ident_parent = &field.ident;
			let tr = if let Some(tr) = &field.tr {
				quote! { #tr }
			} else if let Some(ident_parent) = ident_parent {
				let name = ident_parent.to_string().replace('_', "-");

				quote! { #name }
			} else {
				panic!("tr required when ident is not present");
			};

			let colour = &field.colour;

			let sum = if let Some(ident_parent) = ident_parent {
				let (first_sum, modes_field_value_sum_for_ident) = if modes.len() == 1 {
					let rest = modes.iter().map(|mode| {
						let ident = mode.ident.as_ref().unwrap();

						quote! {
							stats.#ident.#ident_parent
						}
					});

					(
						quote! {
							#(#rest)*
						},
						quote! {},
					)
				} else {
					let mut iter = modes.iter();
					let first = iter.next().unwrap().ident.as_ref().unwrap();

					let rest = iter.map(|mode| {
						let ident = mode.ident.as_ref().unwrap();

						quote! {
							+ stats.#ident.#ident_parent
						}
					});

					(
						quote! {
							stats.#first.#ident_parent
						},
						quote! {
							#(#rest)*
						},
					)
				};

				quote! { #first_sum #modes_field_value_sum_for_ident }
			} else {
				if field.div.len() != 2 {
					panic!("div must have exactly 2 elements");
				}

				let (top, bottom) = (field.div.get(0).unwrap(), field.div.get(1).unwrap());

				let (sum_top, sum_bottom) = if modes.len() == 1 {
					let mode = modes.first().unwrap();
					let ident = mode.ident.as_ref().unwrap();

					(
						quote! {
							stats.#ident.#top
						},
						quote! {
							stats.#ident.#bottom
						},
					)
				} else {
					let mut iter = modes.iter();
					let first = iter.next().unwrap().ident.as_ref().unwrap();

					let rest_top = iter.map(|mode| {
						let ident = mode.ident.as_ref().unwrap();

						quote! {
							+ stats.#ident.#top
						}
					});

					let rest_bottom = modes.iter().skip(1).map(|mode| {
						let ident = mode.ident.as_ref().unwrap();

						quote! {
							+ stats.#ident.#bottom
						}
					});

					(
						quote! {
							stats.#first.#top #(#rest_top)*
						},
						quote! {
							stats.#first.#bottom #(#rest_bottom)*
						},
					)
				};

				quote! {
					{
						let sum_top = #sum_top;
						let sum_bottom = #sum_bottom;

						if sum_bottom == 0 {
							sum_top as f32
						} else {
							sum_top as f32 / sum_bottom as f32
						}
					}
				}
			};

			quote! {
				let sum = #sum;

				crate::canvas::draw::apply_item(
					ctx,
					surface,
					sum,
					&::translate::tr!(ctx, #tr),
					#colour,
					#i,
				);
			}
		});

		let row_count = (overall_fields.len() + 2) as u8 / 3;

		let apply_all_modes = modes.iter().map(|mode| {
			let tr = mode.mode.as_ref().unwrap().tr.as_ref();
			let tr = if let Some(tr) = tr {
				quote! { #tr }
			} else {
				let name = &mode.ty;

				quote! { #name }
			};
			let ty = &mode.ty;

			let apply_items = overall_fields.iter().enumerate().map(|(idx, field)| {
				let ident = field.ident.as_ref();
				let tr = if let Some(tr) = &field.tr {
					quote! { #tr }
				} else if let Some(ident) = ident {
					let name = ident.to_string().replace('_', "-");
	
					quote! { #name }
				} else {
					panic!("tr required when ident is not present");
				};
	
				let colour = &field.colour;
				let value = if let Some(ident) = ident {
					quote! { self.#ident }
				} else if field.div.len() == 2 {
					let top = field.div.get(0).unwrap();
					let bottom = field.div.get(1).unwrap();

					quote! {
						{
							let top = self.#top;
							let bottom = self.#bottom;

							if bottom == 0 {
								top as f32
							} else {
								top as f32 / bottom as f32
							}
						}
					}
				} else {
					panic!("ident required when div len != 2");
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
	
			quote! {
				impl #ty {
					pub fn get_row_count() -> u8 {
						#row_count
					}

					pub fn get_tr() -> &'static str {
						#tr
					}
	
					pub fn apply(&self, ctx: ::translate::Context<'_>, surface: &mut ::skia_safe::Surface, data: &crate::player::data::PlayerData, session: &crate::player::status::PlayerSession) {
						let label = ::translate::tr!(ctx, Self::get_tr());
						
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
	
						#(#apply_items)*
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

					#(#apply_items)*
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

					let extras = &[
						#(#extras)*
					];

					crate::canvas::draw::apply_extras(
						ctx,
						&mut surface,
						extras,
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
	ident: Option<syn::Ident>,

	#[darling(multiple)]
	div: Vec<syn::Ident>,

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

	div: Option<syn::Ident>,
	percent: Option<bool>,
}
