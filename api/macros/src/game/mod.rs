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
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(&path))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
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
		let enum_kind_ident =
			syn::Ident::new(&format!("{}Kind", ident), proc_macro2::Span::call_site());
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
					modes
						.iter()
						.filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(name))
							{
								None
							} else {
								Some(m.ident.as_ref().unwrap())
							}
						})
						.peekable(),
					Some(&ident!("stats")),
					name,
				)
			};

			let value = if let Some(div) = info.div.as_ref() {
				let sum_bottom = sum::sum_fields(
					modes
						.iter()
						.filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(div))
							{
								None
							} else {
								Some(m.ident.as_ref().unwrap())
							}
						})
						.peekable(),
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
					embed = #ty ::embed(&data.stats. #path. #ident, ctx, embed, data);
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
						diff,
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
						background,
					),
				}
			})
			.collect::<Vec<_>>();

		let mode_match_apply_project = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				#enum_ident ::#ty => #ty ::project(
					ctx,
					snapshots,
					kind,
					value,
					background,
				),
			}
		});

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

		let mode_menu_root = modes.iter().take(24).map(|mode| {
			let ty = &mode.ty;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, #ty_str), crate::id::command(crate::command::Id::Root {
					kind: crate::command::Mode::#ident (#enum_ident ::#ty),
					uuid,
				}))
			}
		});

		let mode_menu_snapshot = modes.iter().take(24).map(|mode| {
			let ty = &mode.ty;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, #ty_str), crate::id::command(crate::command::Id::Snapshot {
					kind: crate::command::Mode::#ident (#enum_ident ::#ty),
					uuid,
					past,
				}))
			}
		});

		let mode_menu_history = modes.iter().take(24).map(|mode| {
			let ty = &mode.ty;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, #ty_str), crate::id::command(crate::command::Id::History {
					kind: crate::command::Mode::#ident (#enum_ident ::#ty),
					uuid,
				}))
			}
		});

		let mode_menu_project = modes.iter().take(24).map(|mode| {
			let ty = &mode.ty;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, #ty_str), crate::id::command(crate::command::Id::Project {
					kind: crate::command::ProjectMode::#ident (#enum_ident ::#ty, kind),
					uuid,
				}))
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

		let mode_diff_log = modes.iter().map(|mode| {
			let ty = &mode.ty;

			quote! {
				let embed = <#ty as crate::canvas::diff::DiffLog>::diff_log(data_new, data_old, ctx, embed);
			}
		});

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
					modes
						.iter()
						.filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(ident))
							{
								None
							} else {
								Some(m.ident.as_ref().unwrap())
							}
						})
						.peekable(),
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
					modes
						.iter()
						.filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(ident))
							{
								None
							} else {
								Some(m.ident.as_ref().unwrap())
							}
						})
						.peekable(),
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
					modes
						.iter()
						.filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(ident))
							{
								None
							} else {
								Some(m.ident.as_ref().unwrap())
							}
						})
						.peekable(),
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
					let parent = quote! { stats };
					let value = sum::sum_div_u32_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode
										.skip_field
										.iter()
										.any(|f| f.eq(div) || f.eq(ident_parent))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&parent),
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
					let parent = quote! { stats };

					sum::sum_div_f32_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode
										.skip_field
										.iter()
										.any(|f| f.eq(div) || f.eq(ident_parent))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&parent),
						ident_parent,
						div,
					)
				}
			} else if field.min.is_some() {
				let mut apply = modes.iter().filter_map(|m| {
					let mode = m.mode.as_ref().unwrap();

					if mode.skip_overall.is_some()
						|| mode.skip_field.iter().any(|f| f.eq(ident_parent))
					{
						None
					} else {
						let ident = m.ident.as_ref().unwrap();

						Some(ident)
					}
				});

				let first = apply.next().unwrap();
				let other = apply.map(|ident| {
					quote! {
						min = min.min(stats.#ident.#ident_parent);
					}
				});

				quote! {
					{
						let mut min = stats.#first.#ident_parent;

						#(#other)*

						min
					}
				}
			} else {
				sum::sum_fields(
					modes
						.iter()
						.filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(ident_parent))
							{
								None
							} else {
								Some(m.ident.as_ref().unwrap())
							}
						})
						.peekable(),
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
					let parent = quote! { stats };

					let sum = sum::sum_div_u32_fields(
						modes.iter().filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(ident_parent) || f.eq(div))
							{
								None
							} else {
								Some(m.ident.as_ref().unwrap())
							}
						}).peekable(),
						Some(&parent),
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
					let parent = quote! { stats };

					sum::sum_div_f32_fields(
						modes.iter().filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(ident_parent) || f.eq(div))
							{
								None
							} else {
								Some(m.ident.as_ref().unwrap())
							}
						}).peekable(),
						Some(&parent),
						ident_parent,
						div,
					)
				}
			} else {
				sum::sum_fields(
					modes.iter().filter_map(|m| {
						let mode = m.mode.as_ref().unwrap();

						if mode.skip_overall.is_some()
							|| mode.skip_field.iter().any(|f| f.eq(ident_parent))
						{
							None
						} else {
							Some(m.ident.as_ref().unwrap())
						}
					}).peekable(),
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

		let apply_items_mode = overall_fields.iter().filter_map(|field| {
			if field.skip_chart.is_some() {
				return None;
			};

			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::div_u32_single_field(&quote! { self }, ident, div);

					let struct_name = get_percent_ident_for_str(ty);

					return Some(quote! {
						.push_checked(
							&crate::canvas::shape::Bubble,
							crate::canvas::body::Body::from_bubble(
								ctx,
								&crate::extras::percent::#struct_name (#value),
								&::translate::tr!(ctx, #tr),
								#colour,
							),
						)
					});
				} else {
					sum::div_f32_single_field(&quote!(self), ident, div)
				}
			} else {
				quote! { self.#ident }
			};

			Some(quote! {
				.push_checked(
					&crate::canvas::shape::Bubble,
					crate::canvas::body::Body::from_bubble(
						ctx,
						&#value,
						&::translate::tr!(ctx, #tr),
						#colour,
					),
				)
			})
		});

		let apply_embed_mode = overall_fields.iter().filter(|field| field.skip_chart.is_none()).enumerate().map(|(idx, field)| {
			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let extra = if idx % 3 == 0 {
				quote! { field.push('\n'); }
			} else {
				quote! {}
			};

			let value = if let Some(div) = field.div.as_ref() {
				if let Some(ty) = field.percent.as_ref() {
					let value = sum::div_u32_single_field(&quote!(self), ident, div);

					let struct_name = get_percent_ident_for_str(ty);

					return quote! {
						#extra

						field.push_str(::translate::tr!(ctx, #tr).as_ref());
						field.push_str(": **");
						field.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&crate::extras::percent::#struct_name (#value), ctx).as_ref());
						field.push_str("**\n");
					};
				} else {
					sum::div_f32_single_field(&quote!(self), ident, div)
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

		let kind_into_int_impl = overall_fields.iter().enumerate().map(|(idx, f)| {
			let idx = idx as u32 + 1;
			let name = &f.ident;
			let id = if let Some(ref tr) = f.tr {
				let id = &tr.replace('-', "_");

				ident!(id)
			} else {
				name.clone()
			};

			quote! {
				&#enum_kind_ident ::#id => #idx,
			}
		});

		let kind_from_int_impl = overall_fields.iter().enumerate().map(|(idx, f)| {
			let idx = idx as u32 + 1;
			let name = &f.ident;
			let id = if let Some(ref tr) = f.tr {
				let id = &tr.replace('-', "_");

				ident!(id)
			} else {
				name.clone()
			};

			quote! {
				#idx => #enum_kind_ident ::#id,
			}
		});

		let impl_mode_enum = if modes_len > 25 {
			// There can only be 25 options in a ChoiceParameter, so we need to use
			// autocomplete instead.
			quote! {
				// Implement the game mode enum.
				#[derive(::std::fmt::Debug, Clone, Copy, bincode::Encode, bincode::Decode, bitcode::Encode, bitcode::Decode)]
				pub enum #enum_ident {
					Overall,
					#(#mode_enum_rows)*
				}
			}
		} else {
			quote! {
				// Implement the game mode enum.
				#[derive(::std::fmt::Debug, ::poise::ChoiceParameter, Clone, Copy, bincode::Encode, bincode::Decode, bitcode::Encode, bitcode::Decode)]
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

		let static_kinds_iter = overall_fields.iter().filter_map(|f| {
			if f.skip_chart.is_some() {
				return None;
			}

			let name = &f.ident;
			let id = if let Some(ref tr) = f.tr {
				let id = &tr.replace('-', "_");

				ident!(id)
			} else {
				name.clone()
			};

			Some(quote! {
				#enum_kind_ident ::#id,
			})
		});

		let kinds_len = overall_fields
			.iter()
			.filter(|f| f.skip_chart.is_none())
			.count();

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

			let kind_enum_match_project = overall_fields
				.iter()
				.filter_map(|f| {
					if f.skip_chart.is_some() {
						return None;
					}

					let name = &f.ident;
					let id = if let Some(ref tr) = f.tr {
						let id = &tr.replace('-', "_");

						ident!(id)
					} else {
						name.clone()
					};

					let tr = get_tr_with_fallback(f.tr.as_deref(), Some(name));
					let val = if let Some(ref div) = f.div {
						quote! { (f64::from(data.stats.#path.#ident.#name) / f64::from(data.stats.#path.#ident.#div)) }
					} else {
						quote! { f64::from(data.stats.#path.#ident.#name) }
					};

					let val_last = if let Some(ref div) = f.div {
						quote! { (f64::from(last.1.stats.#path.#ident.#name) / f64::from(last.1.stats.#path.#ident.#div)) }
					} else {
						quote! { f64::from(last.1.stats.#path.#ident.#name) }
					};

					Some(quote! {
						#enum_kind_ident ::#id => {
							let mut low = f64::MAX;
							let mut high: f64 = 1.;

							for (_, data) in &snapshots {
								low = low.min(#val);
								high = high.max(#val);
							}

							let series = snapshots
								.iter()
								.map(|(time, data)| (time.timestamp() as f64, #val))
								.collect::<Vec<_>>();

							let line = crate::canvas::project::line::Line::from_series(&series);

							let predict_y = value.unwrap_or_else(|| crate::canvas::project::next_milestone(#val_last));
							let predict_x = line
								.x(predict_y, last.0.timestamp() as f64)
								.map(|x| ::chrono::TimeZone::timestamp_opt(&::chrono::Utc, x as i64, 0).unwrap());

							let mut buffer = crate::canvas::project::f64::create(
								ctx,
								::std::vec![(
									::translate::tr!(ctx, #tr),
									snapshots
										.iter()
										.map(|(time, data)| (*time, #val))
										.collect::<Vec<_>>(),
									predict_x.unwrap_or(last.0),
									predict_y,
								)],
								first.0..predict_x.map_or(last.0, |x| x.max(last.0)),
								(f64::from(first.1.stats.#path.#ident.#name) * (7. / 8.))..(predict_y.max(#val_last) * (8. / 7.)),
								None,
								background,
							)?;

							let mut surface = crate::canvas::project::canvas(&mut buffer)?;

							crate::canvas::chart::apply_title(ctx, &mut surface, &last.1, &LABEL, background);

							let r = crate::percent::PercentU32((crate::canvas::project::line::compute_r(&series, &line) * 100.) as u32);

							if let Some(x) = predict_x {
								crate::canvas::project::apply_bubbles(
									&mut surface,
									ctx,
									::translate::tr!(ctx, #tr).as_ref(),
									&predict_y,
									&r,
									&x,
									background,
								);
							} else {
								crate::canvas::project::apply_bubbles(
									&mut surface,
									ctx,
									::translate::tr!(ctx, #tr).as_ref(),
									&predict_y,
									&r,
									&::translate::tr!(ctx, "never").as_ref(),
									background,
								);
							}

							crate::canvas::project::round_corners(&mut surface);

							Ok(crate::canvas::to_png(&mut surface))
						}
					})
				});

			let kind_level_match_project = {
				let val = {
					quote! {
						f64::from({
							let stats = &data.stats.#path;
							let xp = #calc ::convert(&#xp_field);

							f64::from(#calc ::get_level(xp)) + f64::from(#calc ::get_level_progress(xp))
						})
					}
				};

				let val_last = {
					quote! {
						f64::from({
							let data = &last.1;
							let stats = &data.stats.#path;
							let xp = #calc ::convert(&#xp_field);

							f64::from(#calc ::get_level(xp)) + f64::from(#calc ::get_level_progress(xp))
						})
					}
				};

				let val_first = {
					quote! {
						f64::from({
							let data = &first.1;
							let stats = &data.stats.#path;
							let xp = #calc ::convert(&#xp_field);

							f64::from(#calc ::get_level(xp)) + f64::from(#calc ::get_level_progress(xp))
						})
					}
				};

				let tr = "level";

				quote! {
					#enum_kind_ident ::level => {
						let mut low = f64::MAX;
						let mut high: f64 = 1.;

						for (_, data) in &snapshots {
							let val = #val;

							low = low.min(val);
							high = high.max(val);
						}

						let series = snapshots
							.iter()
							.map(|(time, data)| (time.timestamp() as f64, #val))
							.collect::<Vec<_>>();

						let line = crate::canvas::project::line::Line::from_series(&series);

						let predict_y = value.unwrap_or_else(|| crate::canvas::project::next_milestone(#val_last));
						let predict_x = line
							.x(predict_y, last.0.timestamp() as f64)
							.map(|x| ::chrono::TimeZone::timestamp_opt(&::chrono::Utc, x as i64, 0).unwrap());

						let mut buffer = crate::canvas::project::f64::create(
							ctx,
							::std::vec![(
								::translate::tr!(ctx, #tr),
								snapshots
									.iter()
									.map(|(time, data)| (*time, #val))
									.collect::<Vec<_>>(),
								predict_x.unwrap_or(last.0),
								predict_y,
							)],
							first.0..predict_x.map_or(last.0, |x| x.max(last.0)),
							(#val_first * (7. / 8.))..(predict_y.max(#val_last) * (8. / 7.)),
							None,
							background,
						)?;

						let mut surface = crate::canvas::project::canvas(&mut buffer)?;

						crate::canvas::chart::apply_title(ctx, &mut surface, &last.1, &LABEL, background);

						let r = crate::percent::PercentU32((crate::canvas::project::line::compute_r(&series, &line) * 100.) as u32);

						if let Some(x) = predict_x {
							crate::canvas::project::apply_bubbles(
								&mut surface,
								ctx,
								::translate::tr!(ctx, #tr).as_ref(),
								&predict_y,
								&r,
								&x,
								background,
							);
						} else {
							crate::canvas::project::apply_bubbles(
								&mut surface,
								ctx,
								::translate::tr!(ctx, #tr).as_ref(),
								&predict_y,
								&r,
								&::translate::tr!(ctx, "never").as_ref(),
								background,
							);
						}

						crate::canvas::project::round_corners(&mut surface);

						Ok(crate::canvas::to_png(&mut surface))
					}
				}
			};

			let diff_log_fields = overall_fields.iter().filter_map(|field| {
				if field.skip_chart.is_some() {
					return None;
				};

				let f_ident = &field.ident;
				let tr = get_tr_with_fallback(field.tr.as_deref(), Some(f_ident));

				let value_new = if let Some(div) = field.div.as_ref() {
					if field.percent.is_some() {
						sum::div_u32_single_field(&quote!(stats_new), f_ident, div)
					} else {
						sum::div_f32_single_field(&quote!(stats_new), f_ident, div)
					}
				} else {
					quote! { stats_new.#f_ident }
				};

				let value_old = if let Some(div) = field.div.as_ref() {
					if field.percent.is_some() {
						sum::div_u32_single_field(&quote!(stats_old), f_ident, div)
					} else {
						sum::div_f32_single_field(&quote!(stats_old), f_ident, div)
					}
				} else {
					quote! { stats_old.#f_ident }
				};

				Some(quote! {
					let new = #value_new;
					let old = #value_old;

					if new < old {
						log.push_str("- ");
						log.push_str(::translate::tr!(ctx, #tr).as_ref());
						log.push_str(": `");
						log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&old, ctx).as_ref());
						log.push_str("` ⇢ `");
						log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&new, ctx).as_ref());
						log.push_str("` (`-");
						log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&(old - new), ctx).as_ref());
						log.push_str("`)\n");
					} else if new > old {
						log.push_str("- ");
						log.push_str(::translate::tr!(ctx, #tr).as_ref());
						log.push_str(": `");
						log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&old, ctx).as_ref());
						log.push_str("` ⇢ `");
						log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&new, ctx).as_ref());
						log.push_str("` (`+");
						log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&(new - old), ctx).as_ref());
						log.push_str("`)\n");
					}
				})
			});

			quote! {
				impl crate::canvas::diff::DiffLog for #ty {
					#[allow(clippy::ptr_arg)]
					fn diff_log(new: &crate::player::data::Data, old: &crate::player::data::Data, ctx: &::translate::context::Context<'_>, mut embed: ::poise::serenity_prelude::Embed) -> ::poise::serenity_prelude::Embed {
						let mut log = String::new();
						let stats_new = &new.stats.#path.#ident;
						let stats_old = &old.stats.#path.#ident;

						{
							#(#diff_log_fields)*
						}

						stats_new.diff_log_own_fields(&stats_old, new, old, ctx, &mut log);

						if !log.is_empty() {
							let mut title = String::new();

							title.push_str(PLAIN);
							title.push(' ');
							title.push_str(::translate::tr!(ctx, #ty ::get_tr()).as_ref());

							embed.fields.push(::poise::serenity_prelude::EmbedField::new(title, log, true));
							embed
						} else {
							embed
						}
					}
				}

				impl #ty {
					pub fn get_tr() -> &'static str {
						#tr
					}

					#[allow(clippy::too_many_arguments)]
					pub fn apply<'c>(
						&self,
						diff: bool,
						ctx: &::translate::context::Context<'_>,
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
							.push_down_post_draw(
								progress,
								if diff {
									crate::canvas::shape::WideBubbleProgress::from_level_total(
										ctx,
										&#level_fmt_field,
										&#calc ::get_total_xp(xp),
									)
								} else {
									crate::canvas::shape::WideBubbleProgress::from_level_progress(
										ctx,
										&#level_fmt_field,
										&#calc ::get_curr_level_xp(xp),
										&#calc ::get_level_xp(xp),
									)
								},
							)
							.push_right_start(
								&crate::canvas::shape::Sidebar,
								crate::canvas::body::Body::new(17., ::std::option::Option::None)
									#(#extras)*
									#(#extras_for_mode)*
									.build()
							)
							.push_right_post_draw(
								status,
								crate::canvas::body::Body::from_status(ctx, session)
							);

						let mut canvas = canvas #(#apply_items_mode)*;

						self.apply_own_fields(ctx, canvas, data, session, stats)
					}

					pub fn embed(
						&self,
						ctx: &::translate::context::Context<'_>,
						embed: ::poise::serenity_prelude::CreateEmbed,
						data: &crate::player::data::Data,
					) -> ::poise::serenity_prelude::CreateEmbed {
						let mut field = ::std::string::String::new();
						let stats = &data.stats.#path;

						#(#apply_embed_mode)*
						self.embed_own_fields(ctx, &mut field, data, stats);

						embed.field(::translate::tr!(ctx, Self::get_tr()), field, true)
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

					pub fn project(
						ctx: &::translate::context::Context<'_>,
						snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
						kind: #enum_kind_ident,
						value: Option<f64>,
						background: Option<skia_safe::Color>,
					) -> Result<::std::vec::Vec<u8>, ::translate::Error> {
						let first = snapshots.first().unwrap();
						let last = snapshots.last().unwrap();

						match kind {
							#kind_level_match_project,
							#(#kind_enum_match_project)*
							_ => unimplemented!(),
						}
					}

					pub fn chart(
						ctx: &::translate::context::Context<'_>,
						snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
						background: Option<::skia_safe::Color>,
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

						let mut buffer = crate::canvas::chart::u32::create::<true>(
							ctx,
							v,
							x_range,
							(lower * 11 / 16)..(upper * 16 / 15),
							::std::option::Option::None,
							background,
						)?;

						let mut surface = crate::canvas::chart::canvas(&mut buffer)?;

						crate::canvas::chart::apply_title(ctx, &mut surface, &last_data, &LABEL, background);
						crate::canvas::chart::round_corners(&mut surface);

						Ok(crate::canvas::to_png(&mut surface))
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

		let kind_enum_rows = overall_fields.iter().map(|f| {
			let name = &f.ident;
			let tr = if let Some(ref tr) = f.tr {
				let id = &tr.replace('-', "_");

				ident!(id)
			} else {
				name.clone()
			};

			quote! { #tr, }
		});

		let default_kind = overall_fields
			.iter()
			.find(|f| f.skip_chart.is_none())
			.map(|f| {
				let name = &f.ident;

				if let Some(ref tr) = f.tr {
					let id = &tr.replace('-', "_");

					ident!(id)
				} else {
					name.clone()
				}
			})
			.unwrap();

		let kind_enum_match_project = overall_fields
			.iter()
			.filter_map(|f| {
				if f.skip_chart.is_some() {
					return None;
				}

				let name = &f.ident;
				let id = if let Some(ref tr) = f.tr {
					let id = &tr.replace('-', "_");

					ident!(id)
				} else {
					name.clone()
				};

				let tr = get_tr_with_fallback(f.tr.as_deref(), Some(name));
				let val = {
					let sum = if let Some(path) = f.path.as_ref() {
						let path = parse_str_to_dot_path(path);

						quote! { data.stats.#path.#name }
					} else if f.min.is_some() {
						let mut apply = modes.iter().filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(name))
							{
								None
							} else {
								let ident = m.ident.as_ref().unwrap();

								Some(ident)
							}
						});

						let first = apply.next().unwrap();
						let other = apply.map(|ident| {
							quote! {
								min = min.min(stats.#ident.#name);
							}
						});

						quote! {
							{
								let mut min = stats.#first.#name;

								#(#other)*

								min
							}
						}
					} else {
						sum::sum_fields(
							modes
								.iter()
								.filter_map(|m| {
									let mode = m.mode.as_ref().unwrap();

									if mode.skip_overall.is_some()
										|| mode.skip_field.iter().any(|f| f.eq(name))
									{
										None
									} else {
										Some(m.ident.as_ref().unwrap())
									}
								})
								.peekable(),
							Some(&ident!("stats")),
							name,
						)
					};

					if let Some(div) = f.div.as_ref() {
						let sum_bottom = sum::sum_fields(
							modes
								.iter()
								.filter_map(|m| {
									let mode = m.mode.as_ref().unwrap();

									if mode.skip_overall.is_some()
										|| mode.skip_field.iter().any(|f| f.eq(div))
									{
										None
									} else {
										Some(m.ident.as_ref().unwrap())
									}
								})
								.peekable(),
							Some(&ident!("stats")),
							div,
						);

						if f.percent.is_some() {
							quote! {
								{
									let stats = &data.stats.#path;

									(f64::from(#sum) * 100. / if #sum_bottom == 0 { 1. } else { f64::from(#sum_bottom) })
								}
							}
						} else {
							quote! {
								{
									let stats = &data.stats.#path;
	
									(f64::from(#sum) / if #sum_bottom == 0 { 1. } else { f64::from(#sum_bottom) })
								}
							}
						}
					} else {
						quote! {
							{
								let stats = &data.stats.#path;

								f64::from(#sum)
							}
						}
					}
				};

				let val_last = {
					let sum = if let Some(path) = f.path.as_ref() {
						let path = parse_str_to_dot_path(path);

						quote! { data.stats.#path.#name }
					} else if f.min.is_some() {
						let mut apply = modes.iter().filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(name))
							{
								None
							} else {
								let ident = m.ident.as_ref().unwrap();

								Some(ident)
							}
						});

						let first = apply.next().unwrap();
						let other = apply.map(|ident| {
							quote! {
								min = min.min(stats.#ident.#name);
							}
						});

						quote! {
							{
								let mut min = stats.#first.#name;

								#(#other)*

								min
							}
						}
					} else {
						sum::sum_fields(
							modes
								.iter()
								.filter_map(|m| {
									let mode = m.mode.as_ref().unwrap();

									if mode.skip_overall.is_some()
										|| mode.skip_field.iter().any(|f| f.eq(name))
									{
										None
									} else {
										Some(m.ident.as_ref().unwrap())
									}
								})
								.peekable(),
							Some(&ident!("stats")),
							name,
						)
					};

					if let Some(div) = f.div.as_ref() {
						let sum_bottom = sum::sum_fields(
							modes
								.iter()
								.filter_map(|m| {
									let mode = m.mode.as_ref().unwrap();

									if mode.skip_overall.is_some()
										|| mode.skip_field.iter().any(|f| f.eq(div))
									{
										None
									} else {
										Some(m.ident.as_ref().unwrap())
									}
								})
								.peekable(),
							Some(&ident!("stats")),
							div,
						);

						if f.percent.is_some() {
							quote! {
								{
									let stats = &last.1.stats.#path;
	
									(f64::from(#sum) * 100. / if #sum_bottom == 0 { 1. } else { f64::from(#sum_bottom) })
								}
							}
						} else {
							quote! {
								{
									let stats = &last.1.stats.#path;
	
									(f64::from(#sum) / if #sum_bottom == 0 { 1. } else { f64::from(#sum_bottom) })
								}
							}
						}
					} else {
						quote! {
							{
								let stats = &last.1.stats.#path;

								f64::from(#sum)
							}
						}
					}
				};

				let val_first = {
					let sum = if let Some(path) = f.path.as_ref() {
						let path = parse_str_to_dot_path(path);

						quote! { data.stats.#path.#name }
					} else if f.min.is_some() {
						let mut apply = modes.iter().filter_map(|m| {
							let mode = m.mode.as_ref().unwrap();

							if mode.skip_overall.is_some()
								|| mode.skip_field.iter().any(|f| f.eq(name))
							{
								None
							} else {
								let ident = m.ident.as_ref().unwrap();

								Some(ident)
							}
						});

						let first = apply.next().unwrap();
						let other = apply.map(|ident| {
							quote! {
								min = min.min(stats.#ident.#name);
							}
						});

						quote! {
							{
								let mut min = stats.#first.#name;

								#(#other)*

								min
							}
						}
					} else {
						sum::sum_fields(
							modes
								.iter()
								.filter_map(|m| {
									let mode = m.mode.as_ref().unwrap();

									if mode.skip_overall.is_some()
										|| mode.skip_field.iter().any(|f| f.eq(name))
									{
										None
									} else {
										Some(m.ident.as_ref().unwrap())
									}
								})
								.peekable(),
							Some(&ident!("stats")),
							name,
						)
					};

					if let Some(div) = f.div.as_ref() {
						let sum_bottom = sum::sum_fields(
							modes
								.iter()
								.filter_map(|m| {
									let mode = m.mode.as_ref().unwrap();

									if mode.skip_overall.is_some()
										|| mode.skip_field.iter().any(|f| f.eq(div))
									{
										None
									} else {
										Some(m.ident.as_ref().unwrap())
									}
								})
								.peekable(),
							Some(&ident!("stats")),
							div,
						);

						if f.percent.is_some() {
							quote! {
								{
									let stats = &first.1.stats.#path;
	
									(f64::from(#sum) * 100. / if #sum_bottom == 0 { 1. } else { f64::from(#sum_bottom) })
								}
							}
						} else {
							quote! {
								{
									let stats = &first.1.stats.#path;
	
									(f64::from(#sum) / if #sum_bottom == 0 { 1. } else { f64::from(#sum_bottom) })
								}
							}
						}
					} else {
						quote! {
							{
								let stats = &first.1.stats.#path;

								f64::from(#sum)
							}
						}
					}
				};

				Some(quote! {
					#enum_kind_ident ::#id => {
						let mut low = f64::MAX;
						let mut high: f64 = 1.;

						for (_, data) in &snapshots {
							low = low.min(#val);
							high = high.max(#val);
						}

						let series = snapshots
							.iter()
							.map(|(time, data)| (time.timestamp() as f64, #val))
							.collect::<Vec<_>>();

						let line = crate::canvas::project::line::Line::from_series(&series);

						let predict_y = value.unwrap_or_else(|| crate::canvas::project::next_milestone(#val_last));
						let predict_x = line
							.x(predict_y, last.0.timestamp() as f64)
							.and_then(|x| ::chrono::TimeZone::timestamp_opt(&::chrono::Utc, x as i64, 0).single());

						let mut buffer = crate::canvas::project::f64::create(
							ctx,
							::std::vec![(
								::translate::tr!(ctx, #tr),
								snapshots
									.iter()
									.map(|(time, data)| (*time, #val))
									.collect::<Vec<_>>(),
								predict_x.unwrap_or(last.0),
								predict_y,
							)],
							first.0..predict_x.map_or(last.0, |x| x.max(last.0)),
							(#val_first * (7. / 8.))..(predict_y.max(#val_last) * (8. / 7.)),
							None,
							background,
						)?;

						let mut surface = crate::canvas::project::canvas(&mut buffer)?;

						crate::canvas::chart::apply_title(ctx, &mut surface, &last.1, &LABEL, background);

						let r = crate::percent::PercentU32((crate::canvas::project::line::compute_r(&series, &line) * 100.) as u32);

						if let Some(x) = predict_x {
							crate::canvas::project::apply_bubbles(
								&mut surface,
								ctx,
								::translate::tr!(ctx, #tr).as_ref(),
								&predict_y,
								&r,
								&x,
								background,
							);
						} else {
							crate::canvas::project::apply_bubbles(
								&mut surface,
								ctx,
								::translate::tr!(ctx, #tr).as_ref(),
								&predict_y,
								&r,
								&::translate::tr!(ctx, "never").as_ref(),
								background,
							);
						}

						crate::canvas::project::round_corners(&mut surface);

						Ok(crate::canvas::to_png(&mut surface))
					}
				})
			});

		let impl_kind = overall_fields.iter().map(|f| {
			let name = &f.ident;
			let id = if let Some(ref tr) = f.tr {
				let id = &tr.replace('-', "_");

				ident!(id)
			} else {
				name.clone()
			};
			let tr = get_tr_with_fallback(f.tr.as_deref(), Some(name));

			quote! {
				#enum_kind_ident ::#id => #tr,
			}
		});

		let impl_kind_try_from_str_lower = overall_fields.iter().map(|f| {
			let name = &f.ident;
			let id = if let Some(ref tr) = f.tr {
				let id = &tr.replace('-', "_");

				ident!(id)
			} else {
				name.clone()
			};

			let string = if let Some(ref tr) = f.tr {
				tr.replace('-', " ")
			} else {
				f.ident.to_string().replace('_', " ")
			};

			quote! {
				#string => #enum_kind_ident ::#id
			}
		});

		let kind_level_match_project = {
			let val = {
				quote! {
					f64::from({
						let stats = &data.stats.#path;
						let xp = #calc ::convert(&#xp_field_overall);

						f64::from(#calc ::get_level(xp)) + f64::from(#calc ::get_level_progress(xp))
					})
				}
			};

			let val_last = {
				quote! {
					f64::from({
						let data = &last.1;
						let stats = &data.stats.#path;
						let xp = #calc ::convert(&#xp_field_overall);

						f64::from(#calc ::get_level(xp)) + f64::from(#calc ::get_level_progress(xp))
					})
				}
			};

			let val_first = {
				quote! {
					f64::from({
						let data = &first.1;
						let stats = &data.stats.#path;
						let xp = #calc ::convert(&#xp_field_overall);

						f64::from(#calc ::get_level(xp)) + f64::from(#calc ::get_level_progress(xp))
					})
				}
			};

			let tr = "level";

			quote! {
				#enum_kind_ident ::level => {
					let mut low = f64::MAX;
					let mut high: f64 = 1.;

					for (_, data) in &snapshots {
						let val = #val;

						low = low.min(val);
						high = high.max(val);
					}

					let series = snapshots
						.iter()
						.map(|(time, data)| (time.timestamp() as f64, #val))
						.collect::<Vec<_>>();

					let line = crate::canvas::project::line::Line::from_series(&series);

					let predict_y = value.unwrap_or_else(|| crate::canvas::project::next_milestone(#val_last));
					let predict_x = line
						.x(predict_y, last.0.timestamp() as f64)
						.map(|x| ::chrono::TimeZone::timestamp_opt(&::chrono::Utc, x as i64, 0).unwrap());

					let mut buffer = crate::canvas::project::f64::create(
						ctx,
						::std::vec![(
							::translate::tr!(ctx, #tr),
							snapshots
								.iter()
								.map(|(time, data)| (*time, #val))
								.collect::<Vec<_>>(),
							predict_x.unwrap_or(last.0),
							predict_y,
						)],
						first.0..predict_x.map_or(last.0, |x| x.max(last.0)),
						(#val_first * (7. / 8.))..(predict_y.max(#val_last) * (8. / 7.)),
						None,
						background,
					)?;

					let mut surface = crate::canvas::project::canvas(&mut buffer)?;

					crate::canvas::chart::apply_title(ctx, &mut surface, &last.1, &LABEL, background);

					let r = crate::percent::PercentU32((crate::canvas::project::line::compute_r(&series, &line) * 100.) as u32);

					if let Some(x) = predict_x {
						crate::canvas::project::apply_bubbles(
							&mut surface,
							ctx,
							::translate::tr!(ctx, #tr).as_ref(),
							&predict_y,
							&r,
							&x,
							background,
						);
					} else {
						crate::canvas::project::apply_bubbles(
							&mut surface,
							ctx,
							::translate::tr!(ctx, #tr).as_ref(),
							&predict_y,
							&r,
							&::translate::tr!(ctx, "never").as_ref(),
							background,
						);
					}

					crate::canvas::project::round_corners(&mut surface);

					Ok(crate::canvas::to_png(&mut surface))
				}
			}
		};

		let kind_enum_match = overall_fields.iter().map(|f| {
			let name = &f.ident;
			let id = if let Some(ref tr) = f.tr {
				let id = &tr.replace('-', "_");

				ident!(id)
			} else {
				name.clone()
			};

			let val = {
				let sum = if let Some(path) = f.path.as_ref() {
					let path = parse_str_to_dot_path(path);

					quote! { data.stats.#path.#name }
				} else if f.min.is_some() {
					let mut apply = modes.iter().filter_map(|m| {
						let mode = m.mode.as_ref().unwrap();

						if mode.skip_overall.is_some() || mode.skip_field.iter().any(|f| f.eq(name))
						{
							None
						} else {
							let ident = m.ident.as_ref().unwrap();

							Some(ident)
						}
					});

					let first = apply.next().unwrap();
					let other = apply.map(|ident| {
						quote! {
							min = min.min(stats.#ident.#name);
						}
					});

					quote! {
						{
							let mut min = stats.#first.#name;

							#(#other)*

							min
						}
					}
				} else {
					sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(name))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats")),
						name,
					)
				};

				if let Some(div) = f.div.as_ref() {
					let sum_bottom = sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(div))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats")),
						div,
					);

					if let Some(ty) = f.percent.as_ref() {
						let struct_name = get_percent_ident_for_str(ty);

						quote! {
							{
								let stats = &data.stats.#path;

								crate::extras::percent::#struct_name (#sum * 100 / if #sum_bottom == 0 { 1 } else { #sum_bottom })
							}
						}
					} else {
						quote! {
							{
								let stats = &data.stats.#path;

								(f64::from(#sum) / if #sum_bottom == 0 { 1. } else { f64::from(#sum_bottom) })
							}
						}
					}
				} else {
					quote! {
						{
							#sum
						}
					}
				}
			};

			quote! {
				#enum_kind_ident ::#id => {
					std::borrow::Cow::Owned(crate::canvas::label::ToFormatted::to_formatted_label(&#val, ctx).into_owned())
				}
			}
		});

		let kind_diff_enum_match = overall_fields.iter().map(|f| {
			let name = &f.ident;
			let id = if let Some(ref tr) = f.tr {
				let id = &tr.replace('-', "_");

				ident!(id)
			} else {
				name.clone()
			};

			let val = {
				let sum_new = if let Some(path) = f.path.as_ref() {
					let path = parse_str_to_dot_path(path);

					quote! { data_new.stats.#path.#name }
				} else if f.min.is_some() {
					let mut apply = modes.iter().filter_map(|m| {
						let mode = m.mode.as_ref().unwrap();

						if mode.skip_overall.is_some() || mode.skip_field.iter().any(|f| f.eq(name))
						{
							None
						} else {
							let ident = m.ident.as_ref().unwrap();

							Some(ident)
						}
					});

					let first = apply.next().unwrap();
					let other = apply.map(|ident| {
						quote! {
							min = min.min(stats_new.#ident.#name);
						}
					});

					quote! {
						{
							let mut min = stats_new.#first.#name;

							#(#other)*

							min
						}
					}
				} else {
					sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(name))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats_new")),
						name,
					)
				};

				let sum_old = if let Some(path) = f.path.as_ref() {
					let path = parse_str_to_dot_path(path);

					quote! { data_old.stats.#path.#name }
				} else if f.min.is_some() {
					let mut apply = modes.iter().filter_map(|m| {
						let mode = m.mode.as_ref().unwrap();

						if mode.skip_overall.is_some() || mode.skip_field.iter().any(|f| f.eq(name))
						{
							None
						} else {
							let ident = m.ident.as_ref().unwrap();

							Some(ident)
						}
					});

					let first = apply.next().unwrap();
					let other = apply.map(|ident| {
						quote! {
							min = min.min(stats_old.#ident.#name);
						}
					});

					quote! {
						{
							let mut min = stats_old.#first.#name;

							#(#other)*

							min
						}
					}
				} else {
					sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(name))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats_old")),
						name,
					)
				};

				if let Some(div) = f.div.as_ref() {
					let sum_bottom_new = sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(div))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats_new")),
						div,
					);

					let sum_bottom_old = sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(div))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats_old")),
						div,
					);

					if let Some(ty) = f.percent.as_ref() {
						let struct_name = get_percent_ident_for_str(ty);

						quote! {
							{
								let stats_new = &data_new.stats.#path;
								let stats_old = &data_old.stats.#path;
	
								crate::extras::percent::#struct_name (crate::canvas::diff::Diff::diff(
									&(#sum_new * 100 / if #sum_bottom_new == 0 { 1 } else { #sum_bottom_new }),
									&(#sum_old * 100 / if #sum_bottom_old == 0 { 1 } else { #sum_bottom_old }),
								))
							}
						}
					} else {
						quote! {
							{
								let stats_new = &data_new.stats.#path;
								let stats_old = &data_old.stats.#path;
	
								crate::canvas::diff::Diff::diff(
									&(f64::from(#sum_new) / if #sum_bottom_new == 0 { 1. } else { f64::from(#sum_bottom_new) }),
									&(f64::from(#sum_old) / if #sum_bottom_old == 0 { 1. } else { f64::from(#sum_bottom_old) }),
								)
							}
						}
					}
				} else {
					quote! {
						{
							crate::canvas::diff::Diff::diff(&#sum_new, &#sum_old)
						}
					}
				}
			};

			quote! {
				#enum_kind_ident ::#id => {
					std::borrow::Cow::Owned(crate::canvas::label::ToFormatted::to_formatted_label(&#val, ctx).into_owned())
				}
			}
		});

		let diff_log_fields = overall_fields.iter().filter_map(|f| {
			if f.skip_chart.is_some() {
				return None;
			};

			let name = &f.ident;
			let tr = get_tr_with_fallback(f.tr.as_deref(), Some(name));

			let val = {
				let sum_new = if f.path.is_some() {
					return None;
				} else if f.min.is_some() {
					let mut apply = modes.iter().filter_map(|m| {
						let mode = m.mode.as_ref().unwrap();

						if mode.skip_overall.is_some() || mode.skip_field.iter().any(|f| f.eq(name))
						{
							None
						} else {
							let ident = m.ident.as_ref().unwrap();

							Some(ident)
						}
					});

					let first = apply.next().unwrap();
					let other = apply.map(|ident| {
						quote! {
							min = min.min(stats_new.#ident.#name);
						}
					});

					quote! {
						{
							let mut min = stats_new.#first.#name;

							#(#other)*

							min
						}
					}
				} else {
					sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(name))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats_new")),
						name,
					)
				};

				let sum_old = if f.path.is_some() {
					return None;
				} else if f.min.is_some() {
					let mut apply = modes.iter().filter_map(|m| {
						let mode = m.mode.as_ref().unwrap();

						if mode.skip_overall.is_some() || mode.skip_field.iter().any(|f| f.eq(name))
						{
							None
						} else {
							let ident = m.ident.as_ref().unwrap();

							Some(ident)
						}
					});

					let first = apply.next().unwrap();
					let other = apply.map(|ident| {
						quote! {
							min = min.min(stats_old.#ident.#name);
						}
					});

					quote! {
						{
							let mut min = stats_old.#first.#name;

							#(#other)*

							min
						}
					}
				} else {
					sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(name))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats_old")),
						name,
					)
				};

				if let Some(div) = f.div.as_ref() {
					let sum_bottom_new = sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(div))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats_new")),
						div,
					);

					let sum_bottom_old = sum::sum_fields(
						modes
							.iter()
							.filter_map(|m| {
								let mode = m.mode.as_ref().unwrap();

								if mode.skip_overall.is_some()
									|| mode.skip_field.iter().any(|f| f.eq(div))
								{
									None
								} else {
									Some(m.ident.as_ref().unwrap())
								}
							})
							.peekable(),
						Some(&ident!("stats_old")),
						div,
					);

					if let Some(ty) = f.percent.as_ref() {
						let struct_name = get_percent_ident_for_str(ty);

						quote! {
							let (new, old) = {
								let stats_new = &data_new.stats.#path;
								let stats_old = &data_old.stats.#path;
	
								let sum_bottom_new = #sum_bottom_new;
								let sum_bottom_old = #sum_bottom_old;
	
								let new = crate::extras::percent::#struct_name (#sum_new * 100 / if sum_bottom_new == 0 { 1 } else { sum_bottom_new });
								let old = crate::extras::percent::#struct_name (#sum_old * 100 / if sum_bottom_old == 0 { 1 } else { sum_bottom_old });
	
								(new, old)
							};
						}
					} else {
						quote! {
							let (new, old) = {
								let stats_new = &data_new.stats.#path;
								let stats_old = &data_old.stats.#path;
	
								let sum_bottom_new = #sum_bottom_new;
								let sum_bottom_old = #sum_bottom_old;
	
								let new = f64::from(#sum_new) / if sum_bottom_new == 0 { 1. } else { f64::from(sum_bottom_new) };
								let old = f64::from(#sum_old) / if sum_bottom_old == 0 { 1. } else { f64::from(sum_bottom_old) };
	
								(new, old)
							};
						}
					}
				} else {
					quote! {
						let new = #sum_new;
						let old = #sum_old;
					}
				}
			};

			Some(quote! {
				#val

				if new < old {
					log.push_str("- ");
					log.push_str(::translate::tr!(ctx, #tr).as_ref());
					log.push_str(": `");
					log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&old, ctx).as_ref());
					log.push_str("` ⇢ `");
					log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&new, ctx).as_ref());
					log.push_str("` (`-");
					log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&(old - new), ctx).as_ref());
					log.push_str("`)\n");
				} else if new > old {
					log.push_str("- ");
					log.push_str(::translate::tr!(ctx, #tr).as_ref());
					log.push_str(": `");
					log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&old, ctx).as_ref());
					log.push_str("` ⇢ `");
					log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&new, ctx).as_ref());
					log.push_str("` (`+");
					log.push_str(crate::canvas::label::ToFormatted::to_formatted_label(&(new - old), ctx).as_ref());
					log.push_str("`)\n");
				}
			})
		});

		tokens.extend(quote! {
			const LABEL: [::minecraft::text::Text; #label_size] = ::minecraft::text::parse::minecraft_text(#pretty);
			const PRETTY: &'static str = #pretty;
			const PLAIN: &'static str = #plain;

			#(#apply_all_modes)*

			pub struct Overall;

			impl crate::canvas::diff::DiffLog for Overall {
				#[allow(clippy::ptr_arg)]
				fn diff_log(data_new: &crate::player::data::Data, data_old: &crate::player::data::Data, ctx: &::translate::context::Context<'_>, mut embed: ::poise::serenity_prelude::Embed) -> ::poise::serenity_prelude::Embed {
					let mut log = String::new();
					let stats_new = &data_new.stats.#path;
					let stats_old = &data_old.stats.#path;

					#(#diff_log_fields)*

					if !log.is_empty() {
						let mut title = String::new();

						title.push_str(PLAIN);
						title.push(' ');
						title.push_str(::translate::tr!(ctx, Overall::get_tr()).as_ref());

						embed.fields.push(::poise::serenity_prelude::EmbedField::new(title, log, true));
						embed
					} else {
						embed
					}
				}
			}

			impl Overall {
				pub fn get_tr() -> &'static str {
					"Overall"
				}

				pub fn project(
					ctx: &::translate::context::Context<'_>,
					snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
					kind: #enum_kind_ident,
					value: Option<f64>,
					background: Option<skia_safe::Color>,
				) -> Result<::std::vec::Vec<u8>, ::translate::Error> {
					let first = snapshots.first().unwrap();
					let last = snapshots.last().unwrap();

					match kind {
						#kind_level_match_project,
						#(#kind_enum_match_project)*
						_ => unimplemented!(),
					}
				}

				#[allow(clippy::too_many_arguments)]
				pub fn apply<'c>(
					diff: bool,
					ctx: &::translate::context::Context<'_>,
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
						.push_down_post_draw(
							progress,
							if diff {
								crate::canvas::shape::WideBubbleProgress::from_level_total(
									ctx,
									&#level_fmt_field_overall,
									&#calc ::get_total_xp(xp),
								)
							} else {
								crate::canvas::shape::WideBubbleProgress::from_level_progress(
									ctx,
									&#level_fmt_field_overall,
									&#calc ::get_curr_level_xp(xp),
									&#calc ::get_level_xp(xp),
								)
							},
						)
						.push_right_start(
							&crate::canvas::shape::Sidebar,
							crate::canvas::body::Body::new(17., ::std::option::Option::None)
								#(#extras)*
								#(#extras_for_overall)*
								.build()
						)
						.push_right_post_draw(
							status,
							crate::canvas::body::Body::from_status(ctx, session)
						);

					canvas #(#apply_items_overall)*
				}

				pub fn embed(
					ctx: &::translate::context::Context<'_>,
					embed: ::poise::serenity_prelude::CreateEmbed,
					data: &crate::player::data::Data
				) -> ::poise::serenity_prelude::CreateEmbed {
					let stats = &data.stats.#path;
					let mut field = ::std::string::String::new();

					#(#apply_items_embed_overall)*

					embed.field(
						::translate::tr!(ctx, Self::get_tr()),
						field,
						true,
					)
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
					ctx: &::translate::context::Context<'_>,
					snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
					background: Option<::skia_safe::Color>,
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

					let mut buffer = crate::canvas::chart::u32::create::<true>(
						ctx,
						v,
						x_range,
						(lower * 11 / 16)..(upper * 16 / 15),
						::std::option::Option::None,
						background,
					)?;

					let mut surface = crate::canvas::chart::canvas(&mut buffer)?;

					crate::canvas::chart::apply_title(ctx, &mut surface, &last_data, &LABEL, background);
					crate::canvas::chart::round_corners(&mut surface);

					Ok(crate::canvas::to_png(&mut surface))
				}
			}

			#[allow(non_camel_case_types)]
			#[derive(::serde::Deserialize, ::serde::Serialize, ::std::fmt::Debug, ::poise::ChoiceParameter, bincode::Encode, bincode::Decode, bitcode::Encode, bitcode::Decode, Clone, Copy)]
			#[serde(rename_all = "snake_case")]
			pub enum #enum_kind_ident {
				level,
				#(#kind_enum_rows)*
			}

			impl #enum_kind_ident {
				pub fn get_tr(&self) -> &'static str {
					match self {
						#enum_kind_ident ::level => "level",
						#(#impl_kind)*
					}
				}

				pub fn try_from_str_lower(value: &str) -> Option<Self> {
					Some(match value {
						#(#impl_kind_try_from_str_lower,)*
						_ => return None,
					})
				}

				pub fn slice() -> &'static [#enum_kind_ident] {
					const KINDS: [#enum_kind_ident; #kinds_len + 1] = [
						#enum_kind_ident ::level,
						#(#static_kinds_iter)*
					];

					&KINDS
				}
			}

			impl ::std::default::Default for #enum_kind_ident {
				fn default() -> Self {
					#enum_kind_ident ::#default_kind
				}
			}

			#impl_mode_enum

			impl From<&#enum_kind_ident> for u32 {
				fn from(value: &#enum_kind_ident) -> u32 {
					match value {
						#enum_kind_ident ::level => 0,
						#(#kind_into_int_impl)*
					}
				}
			}

			impl From<u32> for #enum_kind_ident {
				fn from(value: u32) -> Self {
					match value {
						0 => Self::level,
						#(#kind_from_int_impl)*
						_ => #enum_kind_ident ::default(),
					}
				}
			}

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
					const MODES: [#enum_ident; #modes_len] = [
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

				pub fn as_root(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					selected: Option<#enum_ident>
				) -> (::poise::serenity_prelude::CreateActionRow, crate::id::Id) {
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						::poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								::poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, Overall::get_tr()), crate::id::command(crate::command::Id::Root {
									kind: crate::command::Mode::#ident (#enum_ident ::Overall),
									uuid,
								})),
								#(#mode_menu_root),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(::translate::tr!(ctx, selected.get_tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						::poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						crate::id::Id::Command(crate::command::Id::Root {
							kind: crate::command::Mode::#ident (selected.unwrap_or(#enum_ident ::Overall)),
							uuid,
						})
					)
				}

				pub fn as_snapshot(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					past: i64,
					selected: Option<#enum_ident>
				) -> (::poise::serenity_prelude::CreateActionRow, crate::id::Id) {
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						::poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								::poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, Overall::get_tr()), crate::id::command(crate::command::Id::Snapshot {
									kind: crate::command::Mode::#ident (#enum_ident ::Overall),
									uuid,
									past,
								})),
								#(#mode_menu_snapshot),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(::translate::tr!(ctx, selected.get_tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						::poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						crate::id::Id::Command(crate::command::Id::Snapshot {
							kind: crate::command::Mode::#ident (selected.unwrap_or(#enum_ident ::Overall)),
							uuid,
							past,
						})
					)
				}

				pub fn as_history(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					selected: Option<#enum_ident>
				) -> (::poise::serenity_prelude::CreateActionRow, crate::id::Id) {
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						::poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								::poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, Overall::get_tr()), crate::id::command(crate::command::Id::History {
									kind: crate::command::Mode::#ident (#enum_ident ::Overall),
									uuid,
								})),
								#(#mode_menu_history),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(::translate::tr!(ctx, selected.get_tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						::poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						crate::id::Id::Command(crate::command::Id::History {
							kind: crate::command::Mode::#ident (selected.unwrap_or(#enum_ident ::Overall)),
							uuid,
						})
					)
				}

				pub fn as_project(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					kind: #enum_kind_ident,
					selected: Option<#enum_ident>
				) -> (::poise::serenity_prelude::CreateActionRow, crate::id::Id) {
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::new(
						"select",
						::poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								::poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr!(ctx, Overall::get_tr()), crate::id::command(crate::command::Id::Project {
									kind: crate::command::ProjectMode::#ident (#enum_ident ::Overall, kind),
									uuid,
								})),
								#(#mode_menu_project),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(::translate::tr!(ctx, selected.get_tr()));
					}

					menu = menu.max_values(1).min_values(1);

					(
						::poise::serenity_prelude::CreateActionRow::SelectMenu(menu),
						crate::id::Id::Command(crate::command::Id::Project {
							kind: crate::command::ProjectMode::#ident (selected.unwrap_or(#enum_ident ::Overall), kind),
							uuid,
						})
					)
				}
			}

			impl crate::prelude::Mode for #enum_ident {
				type Kind = #enum_kind_ident;

				fn as_root(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					selected: Option<#enum_ident>
				) -> (::poise::serenity_prelude::CreateActionRow, crate::id::Id) {
					Self::as_root(ctx, uuid, selected)
				}

				fn as_snapshot(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					past: i64,
					selected: Option<#enum_ident>
				) -> (::poise::serenity_prelude::CreateActionRow, crate::id::Id) {
					Self::as_snapshot(ctx, uuid, past, selected)
				}

				fn as_history(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					selected: Option<#enum_ident>
				) -> (::poise::serenity_prelude::CreateActionRow, crate::id::Id) {
					Self::as_history(ctx, uuid, selected)
				}

				fn as_project(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					kind: <Self as crate::prelude::Mode>::Kind,
					selected: Option<#enum_ident>
				) -> (::poise::serenity_prelude::CreateActionRow, crate::id::Id) {
					Self::as_project(ctx, uuid, kind, selected)
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

			impl crate::canvas::diff::DiffLog for #ident {
				#[allow(clippy::ptr_arg)]
				fn diff_log(data_new: &crate::player::data::Data, data_old: &crate::player::data::Data, ctx: &::translate::context::Context<'_>, mut embed: ::poise::serenity_prelude::Embed) -> ::poise::serenity_prelude::Embed {
					let embed = Overall::diff_log(data_new, data_old, ctx, embed);
					#(#mode_diff_log)*
					embed
				}
			}

			impl crate::prelude::Game for #ident {
				type Mode = #enum_ident;

				fn canvas_diff(
					ctx: &::translate::context::Context<'_>,
					prev: &crate::player::data::Data,
					curr: &mut crate::player::data::Data,
					session: &crate::player::status::Session,
					skin: &skia_safe::Image,
					mode: Option<<Self as crate::prelude::Game>::Mode>,
					suffix: Option<&str>,
					background: Option<::skia_safe::Color>,
				) -> (::skia_safe::Surface, <Self as crate::prelude::Game>::Mode) {
					#ident ::canvas_diff(ctx, prev, curr, session, skin, mode, suffix, background)
				}

				fn canvas(
					ctx: &::translate::context::Context<'_>,
					data: &crate::player::data::Data,
					session: &crate::player::status::Session,
					skin: &skia_safe::Image,
					mode: Option<<Self as crate::prelude::Game>::Mode>,
					suffix: Option<&str>,
					background: Option<::skia_safe::Color>,
				) -> (::skia_safe::Surface, <Self as crate::prelude::Game>::Mode) {
					#ident ::canvas(ctx, data, session, skin, mode, suffix, background)
				}

				fn chart(
					ctx: &::translate::context::Context<'_>,
					snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
					session: &crate::player::status::Session,
					background: Option<::skia_safe::Color>,
					mode: Option<#enum_ident>
				) -> Result<(::std::vec::Vec<u8>, <Self as crate::prelude::Game>::Mode), ::translate::Error> {
					#ident ::chart(ctx, snapshots, session, background, mode)
				}

				fn project(
					ctx: &::translate::context::Context<'_>,
					snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
					session: &crate::player::status::Session,
					mode: Option<#enum_ident>,
					kind: Option<#enum_kind_ident>,
					value: Option<f64>,
					background: Option<skia_safe::Color>,
				) -> Result<(::std::vec::Vec<u8>, <Self as crate::prelude::Game>::Mode), ::translate::Error> {
					#ident ::project(ctx, snapshots, session, mode, kind, value, background)
				}

				fn embed(
					ctx: &::translate::context::Context<'_>,
					player: &crate::player::Player,
					data: &crate::player::data::Data,
				) -> ::poise::serenity_prelude::CreateEmbed {
					#ident ::embed(ctx, player, data)
				}

				fn embed_diff(
					ctx: &::translate::context::Context<'_>,
					player: &crate::player::Player,
					prev: &crate::player::data::Data,
					curr: &mut crate::player::data::Data,
				) -> ::poise::serenity_prelude::CreateEmbed {
					#ident ::embed_diff(ctx, player, prev, curr)
				}
			}

			impl #imp #ident #ty #wher {
				pub fn from_kind<'t, 'c: 't>(ctx: &'c ::translate::context::Context<'c>, data: &'t crate::player::data::Data, kind: &#enum_kind_ident) -> std::borrow::Cow<'t, str> {
					let stats = &data.stats.#path;

					match kind {
						#enum_kind_ident ::level => unreachable!(),
						#(#kind_enum_match)*
					}
				}

				pub fn from_kind_diff<'t, 'c: 't>(
					ctx: &'c ::translate::context::Context<'c>,
					data_new: &'t crate::player::data::Data,
					data_old: &'t crate::player::data::Data,
					kind: &#enum_kind_ident
				) -> std::borrow::Cow<'t, str> {
					let stats_new = &data_new.stats.#path;
					let stats_old = &data_old.stats.#path;

					match kind {
						#enum_kind_ident ::level => unreachable!(),
						#(#kind_diff_enum_match)*
					}
				}

				pub async fn autocomplete<'a>(ctx: ::translate::Context<'a>, partial: ::std::string::String) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
					::futures::StreamExt::take(
						::futures::StreamExt::filter_map(::futures::stream::iter(#enum_ident ::slice()), move |mode| {
							let name = ::translate::tr!(&ctx, mode.get_tr());
							let mode: u32 = mode.into();

							::futures::future::ready(if name.to_ascii_lowercase().contains(&partial) {
								::std::option::Option::Some(::poise::AutocompleteChoice {
									name: name.to_string(),
									value: mode,
								})
							} else {
								::std::option::Option::None
							})
						}), 10)
				}

				pub async fn autocomplete_kind<'a>(ctx: ::translate::Context<'a>, partial: ::std::string::String) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
					::futures::StreamExt::take(
						::futures::StreamExt::filter_map(::futures::stream::iter(#enum_kind_ident ::slice()), move |kind| {
							let name = ::translate::tr!(&ctx, kind.get_tr());
							let kind: u32 = kind.into();

							::futures::future::ready(if name.to_ascii_lowercase().contains(&partial) {
								::std::option::Option::Some(::poise::AutocompleteChoice {
									name: name.to_string(),
									value: kind,
								})
							} else {
								::std::option::Option::None
							})
						}), 10)
				}

				#[allow(clippy::too_many_arguments)]
				pub fn canvas_diff(
					ctx: &::translate::context::Context<'_>,
					prev: &crate::player::data::Data,
					curr: &mut crate::player::data::Data,
					session: &crate::player::status::Session,
					skin: &skia_safe::Image,
					mode: Option<<Self as crate::prelude::Game>::Mode>,
					suffix: Option<&str>,
					background: Option<::skia_safe::Color>,
				) -> (::skia_safe::Surface, <Self as crate::prelude::Game>::Mode) {
					let stats = crate::canvas::diff::Diff::diff(&curr.stats.#path, &prev.stats.#path);

					curr.stats.#path = stats;

					let data = curr;
					let stats = &data.stats.#path;

					let mode = #enum_ident ::get_mode(mode, session);
					let mut canvas = crate::canvas::Canvas::new(720.)
						.gap(7.)
						.push_down(
							&crate::canvas::shape::Title,
							crate::canvas::shape::Title::from_text(&crate::canvas::text::from_data(&data, &data.username, suffix)),
						);

					let (xp, level, progress) = {
						#mode_match_xp_code
					};

					let status = crate::canvas::shape::Status(session, skin);
					let diff = true;

					let mut canvas = match mode {
						#enum_ident ::Overall => {
							Overall::apply(
								diff,
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

					(canvas.build(None, background).unwrap(), mode)
				}

				#[allow(clippy::too_many_arguments)]
				pub fn canvas(
					ctx: &::translate::context::Context<'_>,
					data: &crate::player::data::Data,
					session: &crate::player::status::Session,
					skin: &skia_safe::Image,
					mode: Option<<Self as crate::prelude::Game>::Mode>,
					suffix: Option<&str>,
					background: Option<::skia_safe::Color>,
				) -> (::skia_safe::Surface, <Self as crate::prelude::Game>::Mode) {
					let stats = &data.stats.#path;

					let mode = #enum_ident ::get_mode(mode, session);
					let mut canvas = crate::canvas::Canvas::new(720.)
						.gap(7.)
						.push_down(
							&crate::canvas::shape::Title,
							crate::canvas::shape::Title::from_text(&crate::canvas::text::from_data(&data, &data.username, suffix)),
						);

					let (xp, level, progress) = {
						#mode_match_xp_code
					};

					let status = crate::canvas::shape::Status(session, skin);
					let diff = false;

					let mut canvas = match mode {
						#enum_ident ::Overall => {
							Overall::apply(
								diff,
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

					(canvas.build(None, background).unwrap(), mode)
				}

				pub fn chart(
					ctx: &::translate::context::Context<'_>,
					snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
					session: &crate::player::status::Session,
					background: Option<::skia_safe::Color>,
					mode: Option<#enum_ident>
				) -> Result<(::std::vec::Vec<u8>, <Self as crate::prelude::Game>::Mode), ::translate::Error> {
					let mode = #enum_ident ::get_mode(mode, session);

					Ok((match mode {
						#enum_ident ::Overall => {
							Overall::chart(
								ctx,
								snapshots,
								background,
							)
						}
						#(#mode_match_apply_chart)*
					}?, mode))
				}

				pub fn project(
					ctx: &::translate::context::Context<'_>,
					snapshots: ::std::vec::Vec<(::chrono::DateTime<::chrono::Utc>, crate::player::data::Data)>,
					session: &crate::player::status::Session,
					mode: Option<#enum_ident>,
					kind: Option<#enum_kind_ident>,
					value: Option<f64>,
					background: Option<skia_safe::Color>,
				) -> Result<(::std::vec::Vec<u8>, <Self as crate::prelude::Game>::Mode), ::translate::Error> {
					let mode = #enum_ident ::get_mode(mode, session);
					let kind = kind.unwrap_or_default();

					Ok((match mode {
						#enum_ident ::Overall => {
							Overall::project(
								ctx,
								snapshots,
								kind,
								value,
								background,
							)
						}
						#(#mode_match_apply_project)*
					}?, mode))
				}

				#[allow(clippy::reversed_empty_ranges)]
				pub fn embed(
					ctx: &::translate::context::Context<'_>,
					player: &crate::player::Player,
					data: &crate::player::data::Data,
				) -> ::poise::serenity_prelude::CreateEmbed {
					let mut embed = ::poise::serenity_prelude::CreateEmbed::default()
						.thumbnail(player.get_body_url());

					if let Some(prefix) = data.get_rank().as_str() {
						embed = embed.author(
							::poise::serenity_prelude::CreateEmbedAuthor::new(
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

					embed = Overall::embed(
						ctx,
						embed,
						data,
					);

					#(#apply_modes_text)*

					for _ in 0..#buffer_fields {
						embed = embed.field("\u{200b}", "\u{200b}", true);
					}

					embed
				}

				#[allow(clippy::reversed_empty_ranges)]
				pub fn embed_diff(
					ctx: &::translate::context::Context<'_>,
					player: &crate::player::Player,
					prev: &crate::player::data::Data,
					curr: &mut crate::player::data::Data,
				) -> ::poise::serenity_prelude::CreateEmbed {
					let stats = crate::canvas::diff::Diff::diff(&curr.stats.#path, &prev.stats.#path);

					curr.stats.#path = stats;

					let data = curr;
					let stats = &data.stats.#path;

					let mut embed = ::poise::serenity_prelude::CreateEmbed::default()
						.thumbnail(player.get_body_url());

					if let Some(prefix) = data.get_rank().as_str() {
						embed = embed.author(
							::poise::serenity_prelude::CreateEmbedAuthor::new(
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

					embed = Overall::embed(
						ctx,
						embed,
						data,
					);

					#(#apply_modes_text)*

					for _ in 0..#buffer_fields {
						embed = embed.field("\u{200b}", "\u{200b}", true);
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
	skip_overall: Option<bool>,
	#[darling(default, multiple)]
	skip_field: Vec<syn::Ident>,
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
	min: Option<bool>,

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
