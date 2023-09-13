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

pub(crate) fn impl_mode(
	tokens: &mut proc_macro2::TokenStream,
	state: &State,
	mode: &Mode<'_>,
	label_lines: u8,
	block_lines: u8,
) {
	let id = mode.id();
	let ty = &mode.ty();
	let tr = mode.tr();

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
		idents:
			Idents {
				kind_enum,
				calc,
				path_to_game,
				game_ident,
				mode_enum,
				..
			},
		crates:
			Crates {
				translate,
				poise,
				chrono,
				skia,
				minecraft,
				api,
				hypixel,
				extra,
				redis,
				..
			},
		..
	} = state;

	let self_blocks = mode.block_shapes(mode);
	let self_blocks_diff = mode.block_shapes_diff(mode);
	let blocks = state.receiver.block_shapes(mode);
	let blocks_diff = state.receiver.block_shapes_diff(mode);
	let labels = state.receiver.label_shapes(mode);
	let labels_diff = state.receiver.label_shapes_diff(mode);

	let condensed_self_blocks = mode.condensed_block_shapes(mode);
	let condensed_self_blocks_diff = mode.condensed_block_shapes_diff(mode);
	let condensed_blocks = state.receiver.condensed_block_shapes(mode);
	let condensed_blocks_diff = state.receiver.condensed_block_shapes_diff(mode);
	let condensed_labels = state.receiver.condensed_label_shapes(mode, label_lines);
	let condensed_labels_diff = state
		.receiver
		.condensed_label_shapes_diff(mode, label_lines);

	let mode_blocks = mode.blocks();
	let game_blocks = state.receiver.blocks();

	let game_labels = state.receiver.labels();
	let game_labels = game_labels
		.iter()
		.filter(|l| l.tr() != "level")
		.collect::<Vec<_>>();

	let diff_log = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| b.diff_log(mode, &ident("log")));

	let add_to_pipeline = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| {
			if !b.is_measurable() {
				return None;
			}

			let value = b.value(Side::None, Access::Mode(mode))?;

			Some(quote!({
				let key = #api::leaderboard::encode(&#api::leaderboard::Kind::#game_ident(
					#mode_enum::#ty,
					#kind_enum::#b,
				));
				let game = &data.stats.#path_to_game;
				let stats = &data.stats.#path_to_game.#id;

				pipeline.zadd(key, data.uuid.as_bytes(), #value);
			}))
		})
		.chain(game_labels.iter().filter_map(|l| {
			if !l.is_measurable() {
				return None;
			}

			let value = l.value(Side::None, Access::Mode(mode))?;

			Some(quote!({
				let key = #api::leaderboard::encode(&#api::leaderboard::Kind::#game_ident(
					#mode_enum::#ty,
					#kind_enum::#l,
				));
				let game = &data.stats.#path_to_game;
				let stats = &data.stats.#path_to_game.#id;

				pipeline.zadd(key, data.uuid.as_bytes(), #value);
			}))
		}));

	let leaderboards = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| {
			if !b.is_measurable() {
				return None;
			}

			let tr = b.as_tr();

			Some(quote!({
				#api::leaderboard::Leaderboard {
					kind: #api::leaderboard::Kind::#game_ident(
						#mode_enum::#ty,
						#kind_enum::#b,
					),
					game: #hypixel::game::r#type::Type::#game_ident,
					name: format!("{} {}", #translate::tr(ctx, #ty::tr()), #tr),
					display_name: format!(
						"{} {} {}",
						#hypixel::game::r#type::Type::#game_ident.as_short_clean_name(),
						#translate::tr(ctx, #ty::tr()),
						#tr,
					),
					display_name_lower: format!(
						"{} {} {}",
						#hypixel::game::r#type::Type::#game_ident.as_short_clean_name(),
						#translate::tr(ctx, #ty::tr()),
						#tr,
					)
					.replace(' ', "")
					.to_lowercase(),
				}
			}))
		})
		.chain(game_labels.iter().filter_map(|l| {
			if !l.is_measurable() {
				return None;
			}

			let tr = l.as_tr();

			Some(quote!({
				#api::leaderboard::Leaderboard {
					kind: #api::leaderboard::Kind::#game_ident(
						#mode_enum::#ty,
						#kind_enum::#l,
					),
					game: #hypixel::game::r#type::Type::#game_ident,
					name: format!(
						"{} {}",
						#translate::tr(ctx, #ty::tr()),
						#tr,
					),
					display_name: format!(
						"{} {} {}",
						#hypixel::game::r#type::Type::#game_ident.as_short_clean_name(),
						#translate::tr(ctx, #ty::tr()),
						#tr,
					),
					display_name_lower: format!(
						"{} {} {}",
						#hypixel::game::r#type::Type::#game_ident.as_short_clean_name(),
						#translate::tr(ctx, #ty::tr()),
						#tr,
					)
					.replace(' ', "")
					.to_lowercase(),
				}
			}))
		}));

	let embed = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| Some((b.value_fmt(Access::Mode(mode))?, b.as_tr())))
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

				field.push_str(#translate::tr(ctx, Self::tr()).as_ref());
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
		.chain(
			game_labels
				.iter()
				.filter_map(|l| Some(({
					let value = l.value(Side::None, Access::ModeDiff(mode))?;

					quote!({
						let stats = &data.stats.#path_to_game.#id;
						let game = &data.stats.#path_to_game;

						f64::from(#value)
					})
				}, l.tr(), l.var_id())))
		)
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

					let r = #extra::percent::PercentU32((#api::canvas::project::line::compute_r(&series, &line) * 100.) as u32);

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

	let chart = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| {
			if b.skip_chart() {
				return None;
			}

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

	let label_separator = if label_lines != 0 {
		quote!(.append(#minecraft::text::Text::NEW_LINE))
	} else {
		quote!()
	};

	let leaderboard_kind_match = game_blocks
		.iter()
		.chain(mode_blocks.iter())
		.filter_map(|b| {
			if !b.is_measurable() {
				return None;
			}

			let value = b.value(Side::None, Access::ModeDiff(mode))?;

			Some(quote!(#kind_enum::#b => {
				canvas = canvas.push_right(
					&#api::canvas::shape::LeaderboardValue,
					#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#value),
				);
			}))
		})
		.chain(game_labels.iter().filter_map(|l| {
			if !l.is_measurable() {
				return None;
			}

			let value = l.value(Side::None, Access::ModeDiff(mode))?;

			Some(quote!(#kind_enum::#l => {
				canvas = canvas.push_right(
					&#api::canvas::shape::LeaderboardValue,
					#api::canvas::shape::LeaderboardValue::from_value(ctx, family, &#value),
				);
			}))
		}));

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
					title.push_str(#translate::tr(ctx, Self::tr()).as_ref());

					embed.fields.push(#poise::serenity_prelude::EmbedField::new(title, log, true));
					embed
				} else {
					embed
				}
			}
		}

		impl #ty {
			pub fn add_to_pipeline(pipeline: &mut #redis::Pipeline, data: &#api::player::data::Data) {
				#(#add_to_pipeline)*
			}

			pub fn leaderboards(ctx: &#translate::context::Context<'_>) -> Vec<#api::leaderboard::Leaderboard> {
				vec![
					#(#leaderboards),*
				]
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
				for (idx, data) in players.iter().enumerate() {
					let game = &data.stats.#path_to_game;
					let stats = &data.stats.#path_to_game.#id;

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

				embed.field(#translate::tr(ctx, Self::tr()), field, true)
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

				let mut lower = Self::min_fields(&first.1.stats.#path_to_game.#id, &first.1);
				let mut upper = ::std::cmp::max(Self::max_fields(&first.1.stats.#path_to_game.#id, &first.1), 100);

				for (_, data) in snapshots.iter().skip(1) {
					let stats = &data.stats.#path_to_game.#id;

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

			#[allow(clippy::too_many_arguments)]
			pub fn canvas<'c>(
				&self,
				ctx: &#translate::context::Context<'_>,
				family: #minecraft::style::Family,
				mut canvas: #api::canvas::Canvas<'c>,
				data: &'c #api::player::data::Data,
				session: &'c #hypixel::player::status::Session,
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
						#api::canvas::shape::Subtitle::from_label(ctx, family, &LABEL, #tr),
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
					)
					#labels
					.push_right_post_draw(
						status,
						#api::canvas::body::Body::from_status(ctx, family, session)
					)
					#blocks
					#self_blocks
			}

			#[allow(clippy::too_many_arguments)]
			pub fn canvas_diff<'c>(
				ctx: &#translate::context::Context<'_>,
				family: #minecraft::style::Family,
				mut canvas: #api::canvas::Canvas<'c>,
				data_lhs: &'c #api::player::data::Data,
				data_rhs: &'c #api::player::data::Data,
				session: &'c #hypixel::player::status::Session,
				status: &'c #api::canvas::shape::Status,
				progress: &'c #api::canvas::shape::WideBubbleProgress,
			) -> #api::canvas::Canvas<'c> {
				let stats_lhs = &data_lhs.stats.#path_to_game.#id;
				let stats_rhs = &data_rhs.stats.#path_to_game.#id;
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
						#api::canvas::shape::Subtitle::from_label(ctx, family, &LABEL, #tr),
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
						)
					)
					#labels_diff
					.push_right_post_draw(
						status,
						#api::canvas::body::Body::from_status(ctx, family, session)
					)
					#blocks_diff
					#self_blocks_diff
			}

			pub fn condensed<'c>(
				&self,
				ctx: &#translate::context::Context<'_>,
				family: #minecraft::style::Family,
				mut canvas: #api::canvas::Canvas<'c>,
				data: &'c #api::player::data::Data,
			) -> #api::canvas::Canvas<'c> {
				use #api::canvas::label::ToFormatted;

				let stats = &data.stats.#path_to_game.#id;
				let game = &data.stats.#path_to_game;

				canvas
					.push_checked(
						&#api::canvas::shape::CondensedBubble {
							lines: #label_lines + #block_lines + if #label_lines == 0 { 2 } else { 3 }
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
							#condensed_labels
							.append(#minecraft::text::Text::NEW_LINE)
							.extend(&[
								#condensed_blocks
								#condensed_self_blocks
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

				let stats_lhs = &data_lhs.stats.#path_to_game.#id;
				let stats_rhs = &data_rhs.stats.#path_to_game.#id;
				let game_lhs = &data_lhs.stats.#path_to_game;
				let game_rhs = &data_rhs.stats.#path_to_game;

				canvas
					.push_checked(
						&#api::canvas::shape::CondensedBubble {
							lines: #label_lines + #block_lines + if #label_lines == 0 { 2 } else { 3 }
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
							#condensed_labels_diff
							.append(#minecraft::text::Text::NEW_LINE)
							.extend(&[
								#condensed_blocks_diff
								#condensed_self_blocks_diff
							])
							.build()
					)
			}
		}
	});
}
