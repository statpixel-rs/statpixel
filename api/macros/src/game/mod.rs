mod label;

use darling::{ast, FromDeriveInput, FromField, FromMeta};
use minecraft::{paint::Paint, text::parse::minecraft_string};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::label::{
	map_game_field_to_extras_value, map_info_field_to_extras_value, parse_str_to_dot_path,
};
use crate::{
	sum,
	tokens::{get_percent_ident_for_str, get_tr_with_fallback},
};

macro_rules! ident {
	($id: tt) => {
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

	/// A plain name for the  game with no colouring.
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
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let GameInputReceiver {
			ident,
			generics,
			data,
			path,
			pretty,
			calc,
			field: overall_fields,
			label: info,
			xp: xp_path,
			plain,
			xp_local: xp_local_path,
		} = self;

		let path = parse_str_to_dot_path(path);

		let label_size = minecraft_string(pretty).count();

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

		let (is_raw_xp, xp_field_overall) = if xp_field.is_none() {
			if let Some(path) = xp_local_path.as_ref() {
				let path = ident!(path);

				xp_field = Some(quote!(#path));
				(
					false,
					Some(sum::sum_fields(
						modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
						Some(&ident!("stats")),
						&path,
					)),
				)
			} else if let Some(path) = xp_path.as_ref() {
				let path = parse_str_to_dot_path(path);

				xp_field = Some(quote!(data.stats.#path));
				(true, None)
			} else {
				(true, None)
			}
		} else {
			(true, xp_field.clone())
		};

		modes.sort_by_cached_key(|field| field.ident.as_ref().unwrap().to_string());

		let (is_raw, level_fmt_field, ref xp_field) = match (level_fmt_field, xp_field) {
			(Some(level), Some(xp)) => {
				let level_name = level.ident.as_ref().unwrap();

				(true, quote!(stats. #level_name), xp)
			}
			(Some(_), None) => panic!("xp field required when level field is present"),
			(_, xp_field) => (
				false,
				quote!(::get_level_format(level)),
				if let Some(xp_field) = xp_field {
					xp_field
				} else {
					quote!(data.xp)
				},
			),
		};

		let level_fmt_field_overall = if is_raw {
			#[allow(clippy::redundant_clone)]
			level_fmt_field.clone()
		} else {
			quote!(#calc #level_fmt_field)
		};

		let xp_field_overall = xp_field_overall.unwrap_or_else(|| xp_field.clone());

		let enum_ident = syn::Ident::new(&format!("{}Mode", ident), proc_macro2::Span::call_site());
		let extras = labels
			.iter()
			.map(map_game_field_to_extras_value)
			.collect::<Vec<_>>();
		let extras_for_mode = info
			.iter()
			.map(map_info_field_to_extras_value)
			.collect::<Vec<_>>();
		let extras_for_overall = info.iter().map(|info| {
			let name = &info.ident;
			let tr = get_tr_with_fallback(info.tr.as_deref(), Some(name));

			let colour = &info.colour;

			let sum = if let Some(path) = info.path.as_ref() {
				let path = parse_str_to_dot_path(path);

				quote! { data.stats.#path.#name }
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

			if let Some(ty) = info.percent.as_ref() {
				let struct_name = get_percent_ident_for_str(ty);

				quote! {
					.append_item(
						&::translate::tr!(ctx, #tr),
						&crate::canvas::label::ToFormatted::to_formatted_label(
							&crate::extras::percent::#struct_name (#value),
							ctx,
						),
						&#colour
					)
				}
			} else {
				quote! {
					.append_item(
						&::translate::tr!(ctx, #tr),
						&crate::canvas::label::ToFormatted::to_formatted_label(
							&#value,
							ctx,
						),
						&#colour
					)
				}
			}
		});

		let apply_modes_text = modes
			.iter()
			.map(|mode| {
				let ty = &mode.ty;
				let ident = mode.ident.as_ref().unwrap();

				quote! {
					#ty ::embed(&data.stats. #path. #ident, ctx, &mut embed, data);
				}
			})
			.collect::<Vec<_>>();

		let mode_enum_rows = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				#ty,
			}
		});

		let mode_match_apply_rows = modes
			.iter()
			.map(|mode| {
				let ty = &mode.ty;
				let ident = mode.ident.as_ref().unwrap();

				quote! {
					#enum_ident ::#ty => data.stats. #path. #ident .apply(
						ctx,
						canvas,
						data,
						session,
						&status,
						&progress,
					),
				}
			})
			.collect::<Vec<_>>();

		let mode_match_apply_chart = modes
			.iter()
			.map(|mode| {
				let ty = &mode.ty;

				quote! {
					#enum_ident ::#ty => #ty ::chart(
						ctx,
						snapshots,
					),
				}
			})
			.collect::<Vec<_>>();

		let mode_match_get_tr = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				#enum_ident ::#ty => #ty ::get_tr(),
			}
		});

		let mode_match_from_u8_str = modes.iter().enumerate().map(|(idx, mode)| {
			let ty = &mode.ty;
			let idx = (idx + 1).to_string();
			let idx = idx.as_str();

			quote! {
				#idx => #enum_ident ::#ty,
			}
		});

		let mode_menu_option = modes.iter().take(24).enumerate().map(|(idx, mode)| {
			let idx = (idx + 1).to_string();
			let idx = idx.as_str();

			let ty = &mode.ty;
			let ty = quote!(#ty).to_string();
			let ty = ty.as_str();

			quote! {
				.add_option(poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, #ty), #idx))
			}
		});

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

		let series_tuple_mode = overall_fields
			.iter()
			.filter_map(|field| {
				if field.div.is_some() || field.skip_chart.is_some() {
					return None;
				}

				let ident = &field.ident;
				let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

				Some(quote! {
					(
						::translate::tr!(ctx, #tr),
						snapshots.iter().map(|(created_at, stats)| {
							let v: u32 = stats.#ident.into();

							(*created_at, v)
						})
						.collect::<::std::vec::Vec<_>>(),
					),
				})
			})
			.collect::<Vec<_>>();

		let min_fields_mode = overall_fields
			.iter()
			.filter_map(|field| {
				if field.div.is_some() || field.skip_chart.is_some() {
					return None;
				}

				let ident = &field.ident;

				Some(quote! {
					{
						let v: u32 = self.#ident.into();

						if v < min {
							min = v;
						}
					}
				})
			})
			.collect::<Vec<_>>();

		let max_fields_mode = overall_fields
			.iter()
			.filter_map(|field| {
				if field.div.is_some() || field.skip_chart.is_some() {
					return None;
				}

				let ident = &field.ident;

				Some(quote! {
					{
						let v: u32 = self.#ident.into();

						if v > max {
							max = v;
						}
					}
				})
			})
			.collect::<Vec<_>>();

		let series_tuple_overall = overall_fields
			.iter()
			.filter_map(|field| {
				if field.div.is_some() || field.skip_chart.is_some() {
					return None;
				}

				let ident = &field.ident;
				let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));
				let value = sum::sum_fields(
					modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
					Some(&ident!("stats")),
					ident,
				);

				Some(quote! {
					(
						::translate::tr!(ctx, #tr),
						snapshots.iter().map(|(created_at, stats)| {
							let v: u32 = (#value).into();

							(*created_at, v)
						})
						.collect::<::std::vec::Vec<_>>(),
					),
				})
			})
			.collect::<Vec<_>>();

		let min_fields_overall = overall_fields
			.iter()
			.filter_map(|field| {
				if field.div.is_some() || field.skip_chart.is_some() {
					return None;
				}

				let ident = &field.ident;
				let value = sum::sum_fields(
					modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
					Some(&ident!("stats")),
					ident,
				);

				Some(quote! {
					{
						let v: u32 = (#value).into();

						if v < min {
							min = v;
						}
					}
				})
			})
			.collect::<Vec<_>>();

		let max_fields_overall = overall_fields
			.iter()
			.filter_map(|field| {
				if field.div.is_some() || field.skip_chart.is_some() {
					return None;
				}

				let ident = &field.ident;
				let value = sum::sum_fields(
					modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
					Some(&ident!("stats")),
					ident,
				);

				Some(quote! {
					{
						let v: u32 = (#value).into();

						if v > max {
							max = v;
						}
					}
				})
			})
			.collect::<Vec<_>>();

		let apply_items_overall = overall_fields.iter().map(|field| {
			let ident_parent = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident_parent));
			let colour = &field.colour;

			let value: TokenStream = if let Some(path) = field.path.as_ref() {
				let path = parse_str_to_dot_path(path);

				return quote! {
					data.stats.#path.#ident_parent
				};
			} else if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::sum_div_u32_fields(
						modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
						Some(&ident!("stats")),
						ident_parent,
						div,
					);

					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						.push_checked(
							&crate::canvas::shape::Bubble,
							crate::canvas::body::Body::from_bubble(
								ctx,
								&crate::extras::percent::#struct_name (#value),
								&::translate::tr!(ctx, #tr),
								#colour,
							),
						)
					};
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
				.push_checked(
					&crate::canvas::shape::Bubble,
					crate::canvas::body::Body::from_bubble(
						ctx,
						&#value,
						&::translate::tr!(ctx, #tr),
						#colour,
					),
				)
			}
		});

		let apply_items_embed_overall = overall_fields.iter().enumerate().map(|(idx, field)| {
			let ident_parent = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident_parent));

			let extra = if idx % 3 == 0 {
				quote! { field.push('\n'); }
			} else {
				quote! {}
			};

			let sum = if let Some(path) = field.path.as_ref() {
				let path = parse_str_to_dot_path(path);

				return quote! {
					data.stats.#path.#ident_parent
				};
			} else if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let sum = sum::sum_div_u32_fields(
						modes.iter().map(|m| m.ident.as_ref().unwrap()).peekable(),
						Some(&ident!("stats")),
						ident_parent,
						div,
					);

					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						#extra

						field.push_str(::translate::tr!(ctx, #tr).as_ref());
						field.push_str(": **");
						field.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&crate::extras::percent::#struct_name (#sum), ctx).as_ref());
						field.push_str("**\n");
					};
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
				#extra

				field.push_str(::translate::tr!(ctx, #tr).as_ref());
				field.push_str(": **");
				field.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&#sum, ctx).as_ref());
				field.push_str("**\n");
			}
		});

		let apply_items_mode = overall_fields.iter().map(|field| {
			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::div_u32_single_field(&ident!("self"), None, ident, div);

					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						.push_checked(
							&crate::canvas::shape::Bubble,
							crate::canvas::body::Body::from_bubble(
								ctx,
								&crate::extras::percent::#struct_name (#value),
								&::translate::tr!(ctx, #tr),
								#colour,
							),
						)
					};
				} else {
					sum::div_f32_single_field(&ident!("self"), None, ident, div)
				}
			} else {
				quote! { self.#ident }
			};

			quote! {
				.push_checked(
					&crate::canvas::shape::Bubble,
					crate::canvas::body::Body::from_bubble(
						ctx,
						&#value,
						&::translate::tr!(ctx, #tr),
						#colour,
					),
				)
			}
		});

		let apply_embed_mode = overall_fields.iter().enumerate().map(|(idx, field)| {
			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let extra = if idx % 3 == 0 {
				quote! { field.push('\n'); }
			} else {
				quote! {}
			};

			let value = if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::div_u32_single_field(&ident!("self"), None, ident, div);

					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						#extra

						field.push_str(::translate::tr!(ctx, #tr).as_ref());
						field.push_str(": **");
						field.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&crate::extras::percent::#struct_name (#value), ctx).as_ref());
						field.push_str("**\n");
					};
				} else {
					sum::div_f32_single_field(&ident!("self"), None, ident, div)
				}
			} else {
				quote! { self.#ident }
			};

			quote! {
				#extra

				field.push_str(::translate::tr!(ctx, #tr).as_ref());
				field.push_str(": **");
				field.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&#value, ctx).as_ref());
				field.push_str("**\n");
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
			let ident = mode.ident.as_ref().unwrap();
			let tr = mode.mode.as_ref().unwrap().tr.as_ref();
			let tr = if let Some(tr) = tr {
				quote! { #tr }
			} else {
				quote! { stringify!(#ty) }
			};

			let apply_items_mode = apply_items_mode.clone();
			let apply_embed_mode = apply_embed_mode.clone();

			let (level_fmt_field, calc) =
				if let Some(calc) = mode.mode.as_ref().unwrap().calc.as_ref() {
					(quote! { #calc #level_fmt_field }, quote! { #calc })
				} else if is_raw {
					(quote! { #level_fmt_field }, quote! { #calc })
				} else {
					(quote! { #calc #level_fmt_field }, quote! { #calc })
				};

			let xp_field = if let Some(expr) = mode.mode.as_ref().unwrap().xp.as_ref() {
				quote! { &(#expr) }
			} else if is_raw_xp {
				quote! { #xp_field }
			} else {
				quote! { stats. #ident.#xp_field }
			};

			quote! {
				impl #ty {
					pub fn get_tr() -> &'static str {
						#tr
					}

					pub fn apply<'c>(
						&self,
						ctx: ::translate::Context<'_>,
						mut canvas: crate::canvas::Canvas<'c>,
						data: &'c crate::player::data::Data,
						session: &'c crate::player::status::Session,
						status: &'c crate::canvas::shape::Status,
						progress: &'c crate::canvas::shape::WideBubbleProgress,
					) -> crate::canvas::Canvas<'c> {
						let stats = &data.stats.#path;

						let xp = #calc ::convert(&#xp_field);
						let level = #calc ::get_level(xp);

						let mut canvas = canvas
							.push_down(
								&crate::canvas::shape::Subtitle,
								crate::canvas::shape::Subtitle::from_label(ctx, &LABEL, Self::get_tr()),
							)
							.push_down(
								progress,
								crate::canvas::shape::WideBubbleProgress::from_level_progress(
									ctx,
									&#level_fmt_field,
									&#calc ::get_curr_level_xp(xp),
									&#calc ::get_level_xp(xp),
								),
							)
							.push_right_start(
								&crate::canvas::shape::Sidebar,
								crate::canvas::body::Body::new(17., ::std::option::Option::None)
									#(#extras)*
									#(#extras_for_mode)*
									.build()
							)
							.push_right(
								status,
								crate::canvas::body::Body::from_status(ctx, session)
							);

						let mut canvas = canvas #(#apply_items_mode)*;

						self.apply_own_fields(ctx, canvas, data, session, stats)
					}

					pub fn embed(
						&self,
						ctx: ::translate::Context<'_>,
						embed: &mut ::poise::serenity_prelude::CreateEmbed,
						data: &crate::player::data::Data,
					) {
						let mut field = ::std::string::String::new();
						let stats = &data.stats.#path;

						#(#apply_embed_mode)*
						self.embed_own_fields(ctx, &mut field, data, stats);

						embed.field(::translate::tr!(ctx, Self::get_tr()), field, true);
					}

					pub fn min_fields(&self) -> u32 {
						let mut min = ::std::u32::MAX;

						#(#min_fields_mode)*

						min
					}

					pub fn max_fields(&self) -> u32 {
						let mut max = ::std::u32::MIN;

						#(#max_fields_mode)*

						max
					}

					pub fn chart(
						ctx: ::translate::Context<'_>,
						snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>
					) -> Result<::std::vec::Vec<u8>, ::translate::Error> {
						let first = snapshots.first().unwrap();
						let last = snapshots.last().unwrap();

						let lower = Self::min_fields(&first.1.stats.#path.#ident);
						let upper = ::std::cmp::max(Self::max_fields(&last.1.stats.#path.#ident), 100);

						let lower = ::std::cmp::min(Self::min_own_fields(&first.1.stats.#path.#ident), lower);
						let upper = ::std::cmp::max(Self::max_own_fields(&last.1.stats.#path.#ident), upper);

						let x_range = first.0.clone()..last.0.clone();
						let last_data = last.1.clone();

						let snapshots = snapshots.into_iter().map(|(created_at, data)| {
							(created_at, data.stats.#path.#ident)
						})
							.collect::<::std::vec::Vec<_>>();

						let v = ::std::vec![
							#(#series_tuple_mode)*
						];

						let mut buffer = crate::canvas::chart::u32::create(
							ctx,
							v,
							x_range,
							(lower * 11 / 16)..(upper * 16 / 15),
							::std::option::Option::None,
						)?;

						let mut surface = crate::canvas::chart::canvas(&mut buffer)?;

						crate::canvas::chart::apply_title(ctx, &mut surface, &last_data, &LABEL);
						crate::canvas::chart::round_corners(&mut surface);

						Ok(surface
							.image_snapshot()
							.encode_to_data(::skia_safe::EncodedImageFormat::PNG)
							.unwrap()
							.to_vec())
					}
				}
			}
		});

		let buffer_fields = match modes_len % 3 {
			0 => 0,
			1 => 2,
			2 => 1,
			_ => unreachable!(),
		};

		let mode_match_xp_code = if xp_local_path.is_some() {
			let mode_match_xp = modes.iter().map(|mode| {
				let ident = mode.ident.as_ref().unwrap();
				let ty = &mode.ty;
				let calc = if let Some(calc) = mode.mode.as_ref().unwrap().calc.as_ref() {
					quote! { #calc }
				} else {
					quote! { #calc }
				};

				let xp_field = if let Some(expr) = mode.mode.as_ref().unwrap().xp.as_ref() {
					quote! { &(#expr) }
				} else if is_raw_xp {
					quote! { #xp_field }
				} else {
					quote! { stats. #ident.#xp_field }
				};

				quote! {
					#enum_ident ::#ty => {
						let xp = #calc ::convert(&#xp_field);
						let level = #calc ::get_level(xp);

						let progress = crate::canvas::shape::WideBubbleProgress(
							#calc ::get_level_progress(xp),
							#calc ::get_colours(level),
						);

						(xp, level, progress)
					}
				}
			});

			quote! {
				match mode {
					#enum_ident ::Overall => {
						let xp = #calc ::convert(&#xp_field_overall);
						let level = #calc ::get_level(xp);

						let progress = crate::canvas::shape::WideBubbleProgress(
							#calc ::get_level_progress(xp),
							#calc ::get_colours(level),
						);

						(xp, level, progress)
					},
					#(#mode_match_xp)*
				}
			}
		} else {
			quote! {
				let xp = #calc ::convert(&#xp_field);
				let level = #calc ::get_level(xp);

				let progress = crate::canvas::shape::WideBubbleProgress(
					#calc ::get_level_progress(xp),
					#calc ::get_colours(level),
				);

				(xp, level, progress)
			}
		};

		tokens.extend(quote! {
			const LABEL: [::minecraft::text::Text; #label_size] = ::minecraft::text::parse::minecraft_text(#pretty);
			const PRETTY: &'static str = #pretty;

			#(#apply_all_modes)*

			pub struct Overall;

			impl Overall {
				pub fn get_tr() -> &'static str {
					"Overall"
				}

				pub fn apply<'c>(
					ctx: ::translate::Context<'_>,
					mut canvas: crate::canvas::Canvas<'c>,
					data: &'c crate::player::data::Data,
					session: &'c crate::player::status::Session,
					status: &'c crate::canvas::shape::Status,
					progress: &'c crate::canvas::shape::WideBubbleProgress,
				) -> crate::canvas::Canvas<'c> {
					let stats = &data.stats.#path;

					let xp = #calc ::convert(&#xp_field_overall);
					let level = #calc ::get_level(xp);

					let mut canvas = canvas
						.push_down(
							&crate::canvas::shape::Subtitle,
							crate::canvas::shape::Subtitle::from_label(ctx, &LABEL, Self::get_tr()),
						)
						.push_down(
							progress,
							crate::canvas::shape::WideBubbleProgress::from_level_progress(
								ctx,
								&#level_fmt_field_overall,
								&#calc ::get_curr_level_xp(xp),
								&#calc ::get_level_xp(xp),
							),
						)
						.push_right_start(
							&crate::canvas::shape::Sidebar,
							crate::canvas::body::Body::new(17., ::std::option::Option::None)
								#(#extras)*
								#(#extras_for_overall)*
								.build()
						)
						.push_right(
							status,
							crate::canvas::body::Body::from_status(ctx, session)
						);

					canvas #(#apply_items_overall)*
				}

				pub fn embed(ctx: ::translate::Context<'_>, embed: &mut ::poise::serenity_prelude::CreateEmbed, data: &crate::player::data::Data) {
					let stats = &data.stats.#path;
					let mut field = ::std::string::String::new();

					#(#apply_items_embed_overall)*

					embed.field(
						::translate::tr!(ctx, Self::get_tr()),
						field,
						true,
					);
				}

				pub fn min_fields(stats: &Stats) -> u32 {
					let mut min = ::std::u32::MAX;

					#(#min_fields_overall)*

					min
				}

				pub fn max_fields(stats: &Stats) -> u32 {
					let mut max = ::std::u32::MIN;

					#(#max_fields_overall)*

					max
				}

				pub fn chart(
					ctx: ::translate::Context<'_>,
					snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>
				) -> Result<::std::vec::Vec<u8>, ::translate::Error> {
					let first = snapshots.first().unwrap();
					let last = snapshots.last().unwrap();

					let lower = Self::min_fields(&first.1.stats.#path);
					let upper = ::std::cmp::max(Self::max_fields(&last.1.stats.#path), 100);

					let x_range = first.0.clone()..last.0.clone();
					let last_data = last.1.clone();

					let snapshots = snapshots.into_iter().map(|(created_at, data)| {
						(created_at, data.stats.#path)
					})
						.collect::<::std::vec::Vec<_>>();

					let v = ::std::vec![
						#(#series_tuple_overall)*
					];

					let mut buffer = crate::canvas::chart::u32::create(
						ctx,
						v,
						x_range,
						(lower * 11 / 16)..(upper * 16 / 15),
						::std::option::Option::None,
					)?;

					let mut surface = crate::canvas::chart::canvas(&mut buffer)?;

					crate::canvas::chart::apply_title(ctx, &mut surface, &last_data, &LABEL);
					crate::canvas::chart::round_corners(&mut surface);

					Ok(surface
						.image_snapshot()
						.encode_to_data(::skia_safe::EncodedImageFormat::PNG)
						.unwrap()
						.to_vec())
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

			type Stats = #ident;

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

				pub fn get_mode(mode: Option<#enum_ident>, session: &crate::player::status::Session) -> #enum_ident {
					if let Some(mode) = mode {
						mode
					} else if session.game_type == Some(crate::game::r#type::Type::#ident) && let Some(game_mode) = session.game_mode.as_ref() {
						#enum_ident ::from(game_mode.as_str())
					} else {
						#enum_ident ::Overall
					}
				}

				pub fn get_tr(&self) -> &'static str {
					match self {
						Self::Overall => Overall::get_tr(),
						#(#mode_match_get_tr)*
					}
				}

				pub fn from_u8_str(value: &str) -> Self {
					match value {
						#(#mode_match_from_u8_str)*
						_ => Self::Overall,
					}
				}

				pub fn as_components(ctx: ::translate::Context<'_>) -> ::poise::serenity_prelude::CreateComponents {
					let mut components = ::poise::serenity_prelude::CreateComponents::default();
					let mut row = ::poise::serenity_prelude::CreateActionRow::default();
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::default();
					let mut options = ::poise::serenity_prelude::CreateSelectMenuOptions::default();

					menu.options(|o| o
						.add_option(::poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, Overall::get_tr()), "0"))
						#(#mode_menu_option)*
					);

					menu.custom_id(ctx.id().to_string());
					menu.max_values(1);
					menu.min_values(1);

					row.add_select_menu(menu);
					components.set_action_row(row);
					components
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
									name: name.to_string(),
									value: mode.into(),
								})
							} else {
								::std::option::Option::None
							})
						}), 10)
				}

				pub fn canvas_diff(
					ctx: ::translate::Context<'_>,
					prev: &crate::player::data::Data,
					curr: &mut crate::player::data::Data,
					session: &crate::player::status::Session,
					skin: &[u8],
					mode: Option<#enum_ident>
				) -> ::skia_safe::Surface {
					let stats = crate::canvas::diff::Diff::diff(&curr.stats.#path, &prev.stats.#path);

					curr.stats.#path = stats;

					let data = curr;
					let stats = &data.stats.#path;

					let mode = #enum_ident ::get_mode(mode, session);
					let mut canvas = crate::canvas::Canvas::new(720.)
						.gap(7.)
						.push_down(
							&crate::canvas::shape::Title,
							crate::canvas::shape::Title::from_text(&crate::canvas::text::from_data(&data, &data.username)),
						);

					let (xp, level, progress) = {
						#mode_match_xp_code
					};

					let status = crate::canvas::shape::Status(session, skin);

					let mut canvas = match mode {
						#enum_ident ::Overall => {
							Overall::apply(
								ctx,
								canvas,
								data,
								session,
								&status,
								&progress,
							)
						}
						#(#mode_match_apply_rows)*
					};

					canvas.build(None).unwrap()
				}

				pub fn canvas(
					ctx: ::translate::Context<'_>,
					data: &crate::player::data::Data,
					session: &crate::player::status::Session,
					skin: &[u8],
					mode: Option<#enum_ident>
				) -> ::skia_safe::Surface {
					let stats = &data.stats.#path;

					let mode = #enum_ident ::get_mode(mode, session);
					let mut canvas = crate::canvas::Canvas::new(720.)
						.gap(7.)
						.push_down(
							&crate::canvas::shape::Title,
							crate::canvas::shape::Title::from_text(&crate::canvas::text::from_data(&data, &data.username)),
						);

					let (xp, level, progress) = {
						#mode_match_xp_code
					};

					let status = crate::canvas::shape::Status(session, skin);

					let mut canvas = match mode {
						#enum_ident ::Overall => {
							Overall::apply(
								ctx,
								canvas,
								data,
								session,
								&status,
								&progress,
							)
						}
						#(#mode_match_apply_rows)*
					};

					canvas.build(None).unwrap()
				}

				pub fn chart(
					ctx: ::translate::Context<'_>,
					snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
					session: &crate::player::status::Session,
					mode: Option<#enum_ident>
				) -> Result<::std::vec::Vec<u8>, ::translate::Error> {
					let mode = #enum_ident ::get_mode(mode, session);

					match mode {
						#enum_ident ::Overall => {
							Overall::chart(
								ctx,
								snapshots,
							)
						}
						#(#mode_match_apply_chart)*
					}
				}

				#[allow(clippy::reversed_empty_ranges)]
				pub fn embed(
					ctx: ::translate::Context<'_>,
					player: &crate::player::Player,
					data: &crate::player::data::Data,
				) -> ::poise::serenity_prelude::CreateEmbed {
					let mut embed = ::poise::serenity_prelude::CreateEmbed::default();

					embed.thumbnail(player.get_body_url());

					if let Some(prefix) = data.get_rank().as_str() {
						embed.author(|a| {
							a.name(format!(concat!("{} {} :: ", #plain), prefix, player.username))
								.icon_url(player.get_head_url())
						});
					} else {
						embed.author(|a| {
							a.name(format!(concat!("{} :: ", #plain), player.username))
								.icon_url(player.get_head_url())
						});
					}

					Overall::embed(
						ctx,
						&mut embed,
						data,
					);

					#(#apply_modes_text)*

					for _ in 0..#buffer_fields {
						embed.field("\u{200b}", "\u{200b}", true);
					}

					embed
				}

				#[allow(clippy::reversed_empty_ranges)]
				pub fn embed_diff(
					ctx: ::translate::Context<'_>,
					player: &crate::player::Player,
					prev: &crate::player::data::Data,
					curr: &mut crate::player::data::Data,
				) -> ::poise::serenity_prelude::CreateEmbed {
					let stats = crate::canvas::diff::Diff::diff(&curr.stats.#path, &prev.stats.#path);

					curr.stats.#path = stats;

					let data = curr;
					let stats = &data.stats.#path;

					let mut embed = ::poise::serenity_prelude::CreateEmbed::default();

					embed.thumbnail(player.get_body_url());

					if let Some(prefix) = data.get_rank().as_str() {
						embed.author(|a| {
							a.name(format!(concat!("{} {} :: ", #plain), prefix, player.username))
								.icon_url(player.get_head_url())
						});
					} else {
						embed.author(|a| {
							a.name(format!(concat!("{} :: ", #plain), player.username))
								.icon_url(player.get_head_url())
						});
					}

					Overall::embed(
						ctx,
						&mut embed,
						data,
					);

					#(#apply_modes_text)*

					for _ in 0..#buffer_fields {
						embed.field("\u{200b}", "\u{200b}", true);
					}

					embed
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
	colour: Paint,

	/// The translation key of the label.
	/// Defaults to the ident with underscores replaced with dashes.
	tr: Option<String>,

	div: Option<syn::Ident>,

	percent: Option<bool>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ModeData {
	hypixel: Option<String>,
	calc: Option<syn::Path>,
	xp: Option<syn::Expr>,
	tr: Option<String>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct OverallFieldData {
	ident: syn::Ident,

	div: Option<syn::Ident>,

	tr: Option<String>,

	#[darling(default)]
	colour: Paint,

	percent: Option<String>,

	path: Option<String>,

	skip_chart: Option<bool>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct InfoFieldData {
	ident: syn::Ident,

	div: Option<syn::Ident>,

	tr: Option<String>,

	#[darling(default)]
	colour: Paint,

	percent: Option<String>,

	path: Option<String>,
}
