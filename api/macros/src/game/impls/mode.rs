use std::borrow::Cow;

use crate::{
	game::{
		key::{Access, Side},
		mode::Mode,
		prelude::{Field, FieldGroup},
	},
	util::ident,
};
use quote::quote;

use super::{Crates, Idents, State};

pub(crate) fn impl_mode(tokens: &mut proc_macro2::TokenStream, state: &State, mode: &Mode<'_>) {
	let id = mode.id();
	let ty = &mode.ty();
	let tr = mode.tr();
	let self_blocks = mode.block_shapes(mode);
	let self_blocks_diff = mode.block_shapes_diff(mode);

	let min = state.receiver.min(&ident("min"), mode);
	let self_min = mode.min(&ident("min"), mode);
	let max = state.receiver.max(&ident("max"), mode);
	let self_max = mode.max(&ident("max"), mode);

	let xp: std::borrow::Cow<'_, proc_macro2::TokenStream> = mode
		.xp(Side::None, &state.idents.path_to_game, None)
		.or_else(|| {
			state
				.receiver
				.xp(Side::None, &state.idents.path_to_game, Some(id))
		})
		.unwrap_or_else(|| panic!("an xp field was not found for mode {}", mode.ident));

	let xp_lhs = mode
		.xp(Side::Lhs, &state.idents.path_to_game, None)
		.or_else(|| {
			state
				.receiver
				.xp(Side::Lhs, &state.idents.path_to_game, Some(id))
		})
		.unwrap_or_else(|| panic!("an xp field was not found for mode {}", mode.ident));

	let xp_rhs = mode
		.xp(Side::Rhs, &state.idents.path_to_game, None)
		.or_else(|| {
			state
				.receiver
				.xp(Side::Rhs, &state.idents.path_to_game, Some(id))
		})
		.unwrap_or_else(|| panic!("an xp field was not found for mode {}", mode.ident));

	let State {
		idents: Idents {
			kind_enum,
			calc,
			path_to_game,
			..
		},
		crates: Crates {
			translate,
			api,
			poise,
			chrono,
			skia,
			..
		},
		..
	} = state;

	let blocks = state.receiver.block_shapes(mode);
	let blocks_diff = state.receiver.block_shapes_diff(mode);
	let labels = state.receiver.label_shapes(mode);
	let labels_diff = state.receiver.label_shapes_diff(mode);

	let mode_blocks = mode.blocks();
	let game_blocks = state.receiver.blocks();

	let diff_log = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| b.diff_log(mode, &ident("log")));

	let embed = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| b.value_fmt(Access::Mode(mode)))
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

	let embed_diff = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| b.diff_fmt(mode))
		.enumerate()
		.map(|(i, v)| {
			let extra = if i % 3 == 0 {
				quote! { field.push('\n'); }
			} else {
				quote! {}
			};

			quote! {
				#extra

				field.push_str(#translate::tr!(ctx, Self::tr()).as_ref());
				field.push_str(": **");
				field.push_str(#v.as_ref());
				field.push_str("**\n");
			}
		});

	let project = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| Some(({
			let value = b.value(Side::None, Access::ModeDiff(mode))?;

			quote!({
				let stats = &data.stats.#path_to_game.#id;
				let game = &data.stats.#path_to_game;

				f64::from(#value)
			})
		}, b.tr(), b.var_id())))
		.chain(std::iter::once((
			quote!({
				let stats = &data.stats.#path_to_game.#id;
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

	let chart = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| {
			let value = b.value_trunc(Side::None, Access::ModeDiff(mode))?;
			let tr = b.as_tr();

			Some(quote!(
				(
					#tr,
					snapshots.iter().map(|(created_at, data)| {
						let game = &data.stats.#path_to_game;
						let stats = &data.stats.#path_to_game.#id;
						let v: u32 = #value.into();

						(*created_at, v)
					})
					.collect::<::std::vec::Vec<_>>(),
				)
			))
		});

	tokens.extend(quote! {
		impl #api::canvas::diff::DiffLog for #ty {
			fn diff_log(
				data_lhs: &#api::player::data::Data,
				data_rhs: &#api::player::data::Data,
				ctx: &#translate::context::Context<'_>,
				mut embed: #poise::serenity_prelude::Embed,
			) -> #poise::serenity_prelude::Embed {
				let mut log = String::new();
				let game_lhs = &data_lhs.stats.#path_to_game;
				let game_rhs = &data_rhs.stats.#path_to_game;
				let stats_lhs = &data_lhs.stats.#path_to_game.#id;
				let stats_rhs = &data_rhs.stats.#path_to_game.#id;

				#(#diff_log)*

				if !log.is_empty() {
					let mut title = String::new();

					title.push_str(PLAIN);
					title.push(' ');
					title.push_str(#translate::tr!(ctx, Self::tr()).as_ref());

					embed.fields.push(#poise::serenity_prelude::EmbedField::new(title, log, true));
					embed
				} else {
					embed
				}
			}
		}

		impl #ty {
			#[inline]
			pub fn tr() -> &'static str {
				#tr
			}

			pub fn embed(
				&self,
				ctx: &#translate::context::Context<'_>,
				embed: #poise::serenity_prelude::CreateEmbed,
				data: &#api::player::data::Data,
			) -> #poise::serenity_prelude::CreateEmbed {
				let mut field = String::new();
				let game = &data.stats.#path_to_game;
				let stats = &data.stats.#path_to_game.#id;

				#(#embed)*

				embed.field(#translate::tr!(ctx, Self::tr()), field, true)
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
				let stats_lhs = &data_lhs.stats.#path_to_game.#id;
				let stats_rhs = &data_rhs.stats.#path_to_game.#id;

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
					_ => unimplemented!(),
				}
			}

			pub fn chart(
				ctx: &#translate::context::Context<'_>,
				snapshots: ::std::vec::Vec<(#chrono::DateTime<#chrono::Utc>, #api::player::data::Data)>,
				background: Option<#skia::Color>,
			) -> Result<::std::vec::Vec<u8>, #translate::Error> {
				let first = snapshots.first().unwrap();
				let last = snapshots.last().unwrap();

				let lower = Self::min_fields(&first.1.stats.#path_to_game.#id, &first.1);
				let upper = ::std::cmp::max(Self::max_fields(&last.1.stats.#path_to_game.#id, &last.1), 100);

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
				let stats = self;
				let mut min = ::std::u32::MAX;

				#min
				#self_min
				min
			}

			pub fn max_fields(&self, data: &#api::player::data::Data) -> u32 {
				let stats = self;
				let mut max = ::std::u32::MIN;

				#max
				#self_max
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
				let stats = &data.stats.#path_to_game.#id;
				let game = &data.stats.#path_to_game;
				let xp = #calc::convert(&#xp);
				let level = #calc::get_level(xp);

				canvas
					.push_down(
						&#api::canvas::shape::Subtitle,
						#api::canvas::shape::Subtitle::from_label(ctx, &LABEL, #tr),
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
					#labels
					.push_right_post_draw(
						status,
						#api::canvas::body::Body::from_status(ctx, session)
					)
					#blocks
					#self_blocks
			}

			#[allow(clippy::too_many_arguments)]
			pub fn canvas_diff<'c>(
				ctx: &#translate::context::Context<'_>,
				mut canvas: #api::canvas::Canvas<'c>,
				data_lhs: &'c #api::player::data::Data,
				data_rhs: &'c #api::player::data::Data,
				session: &'c #api::player::status::Session,
				status: &'c #api::canvas::shape::Status,
				progress: &'c #api::canvas::shape::WideBubbleProgress,
			) -> #api::canvas::Canvas<'c> {
				let stats_lhs = &data_lhs.stats.#path_to_game.#id;
				let stats_rhs = &data_rhs.stats.#path_to_game.#id;
				let game_lhs = &data_lhs.stats.#path_to_game;
				let game_rhs = &data_rhs.stats.#path_to_game;

				let (xp, level) = {
					let xp_lhs = #calc::convert(&#xp_lhs);
					let xp_rhs = #calc::convert(&#xp_rhs);
					let xp = if xp_lhs > xp_rhs {
						xp_lhs - xp_rhs
					} else {
						xp_rhs - xp_lhs
					};

					(xp, #calc::get_level(xp))
				};

				canvas
					.push_down(
						&#api::canvas::shape::Subtitle,
						#api::canvas::shape::Subtitle::from_label(ctx, &LABEL, #tr),
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
					#labels_diff
					.push_right_post_draw(
						status,
						#api::canvas::body::Body::from_status(ctx, session)
					)
					#blocks_diff
					#self_blocks_diff
			}
		}
	});
}
