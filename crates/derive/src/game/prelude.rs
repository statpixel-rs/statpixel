use quote::quote;
use std::{borrow::Cow, fmt::Debug};

use crate::util::crate_ident;

use super::{
	block::Block,
	key::{Access, Key, Side},
	label::{Collection, Label, Percent},
	mode::Mode,
};

pub trait FieldGroup {
	/// Computes the value of the xp field for use in progress calculations.
	/// This should be relative to the root of Data.
	fn xp(
		&self,
		side: Side,
		game_stats_ident: &proc_macro2::TokenStream,
		mode: Option<&syn::Ident>,
	) -> Option<Cow<proc_macro2::TokenStream>>;

	fn blocks(&self) -> Cow<Vec<Block>>;

	fn block_shapes(&self, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|block| block.shape(mode))
			.collect()
	}

	fn block_shapes_sum(&self, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|block| block.shape_sum(modes))
			.collect()
	}

	fn block_shapes_diff(&self, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|block| block.shape_diff(mode))
			.collect()
	}

	fn block_shapes_diff_sum(&self, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|block| block.shape_diff_sum(modes))
			.collect()
	}

	fn condensed_block_shapes(&self, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|block| block.condensed_shape(mode))
			.collect()
	}

	fn condensed_block_shapes_sum(&self, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|block| block.condensed_shape_sum(modes))
			.collect()
	}

	fn condensed_block_shapes_diff(&self, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|block| block.condensed_shape_diff(mode))
			.collect()
	}

	fn condensed_block_shapes_diff_sum(&self, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|block| block.condensed_shape_diff_sum(modes))
			.collect()
	}

	fn labels(&self) -> Cow<Vec<Label>>;

	fn label_shapes(&self, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		Collection {
			labels: self.labels(),
		}
		.shape(mode)
	}

	fn label_shapes_sum(&self, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		Collection {
			labels: self.labels(),
		}
		.shape_sum(modes)
	}

	fn label_shapes_diff(&self, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		Collection {
			labels: self.labels(),
		}
		.shape_diff(mode)
	}

	fn label_shapes_diff_sum(&self, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		Collection {
			labels: self.labels(),
		}
		.shape_diff_sum(modes)
	}

	fn condensed_label_shapes(&self, mode: &Mode<'_>, lines: u8) -> proc_macro2::TokenStream {
		Collection {
			labels: self.labels(),
		}
		.condensed(mode, lines)
	}

	fn condensed_label_shapes_sum(
		&self,
		modes: &[Mode<'_>],
		lines: u8,
	) -> proc_macro2::TokenStream {
		Collection {
			labels: self.labels(),
		}
		.condensed_sum(modes, lines)
	}

	fn condensed_label_shapes_diff(&self, mode: &Mode<'_>, lines: u8) -> proc_macro2::TokenStream {
		Collection {
			labels: self.labels(),
		}
		.condensed_diff(mode, lines)
	}

	fn condensed_label_shapes_diff_sum(
		&self,
		modes: &[Mode<'_>],
		lines: u8,
	) -> proc_macro2::TokenStream {
		Collection {
			labels: self.labels(),
		}
		.condensed_diff_sum(modes, lines)
	}

	fn min(&self, variable: &syn::Ident, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|b| {
				if b.skip_chart() {
					return None;
				}

				let value = b.value_trunc(Side::None, Access::ModeDiff(mode))?;

				Some(quote!({
					let v: u32 = #value.into();

					if v < #variable {
						#variable = v;
					}
				}))
			})
			.collect()
	}

	fn min_sum(&self, variable: &syn::Ident, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|b| {
				if b.skip_chart() {
					return None;
				}

				let value = b.value_trunc_sum(Side::None, modes, Access::NoneDiff)?;

				Some(quote!({
					let v: u32 = #value.into();

					if v < #variable {
						#variable = v;
					}
				}))
			})
			.collect()
	}

	fn max(&self, variable: &syn::Ident, mode: &Mode<'_>) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|b| {
				if b.skip_chart() {
					return None;
				}

				let value = b.value_trunc(Side::None, Access::ModeDiff(mode))?;

				Some(quote!({
					let v: u32 = #value.into();

					if v > #variable {
						#variable = v;
					}
				}))
			})
			.collect()
	}

	fn max_sum(&self, variable: &syn::Ident, modes: &[Mode<'_>]) -> proc_macro2::TokenStream {
		self.blocks()
			.iter()
			.filter_map(|b| {
				if b.skip_chart() {
					return None;
				}

				let value = b.value_trunc_sum(Side::None, modes, Access::NoneDiff)?;

				Some(quote!({
					let v: u32 = #value.into();

					if v > #variable {
						#variable = v;
					}
				}))
			})
			.collect()
	}
}

#[derive(Debug, Clone)]
pub enum FieldKind {
	None,
	Percent(Percent),
	Min,
}

impl FieldKind {
	pub fn is_percent(&self) -> bool {
		matches!(self, FieldKind::Percent(_))
	}
}

pub trait Field: Debug {
	/// Translates the field according to the language in the assumed [`ctx`].
	fn as_tr(&self) -> proc_macro2::TokenStream {
		let translate = crate_ident("translate");
		let tr = self.tr().into_owned();

		quote!(#translate ::tr(ctx, #tr))
	}

	fn as_tr_top(&self) -> proc_macro2::TokenStream {
		let translate = crate_ident("translate");
		let tr = self.tr_top().into_owned();

		quote!(#translate ::tr(ctx, #tr))
	}

	fn as_tr_bottom(&self) -> Option<proc_macro2::TokenStream> {
		let translate = crate_ident("translate");
		let tr = self.tr_bottom()?.into_owned();

		Some(quote!(#translate ::tr(ctx, #tr)))
	}

	fn var_id(&self) -> Cow<proc_macro2::TokenStream> {
		if self.div().is_none() {
			Cow::Borrowed(self.id())
		} else {
			Cow::Owned(syn::parse_str(&self.tr().replace('-', "_")).unwrap())
		}
	}

	/// Retrieves the translation key for the field.
	fn tr(&self) -> Cow<str>;
	fn tr_top(&self) -> Cow<str> {
		self.tr()
	}
	fn tr_bottom(&self) -> Option<Cow<str>>;
	fn id(&self) -> &proc_macro2::TokenStream;
	fn div(&self) -> Option<&proc_macro2::TokenStream>;
	fn kind(&self) -> &FieldKind;
	/// `mode` is only present for the current mode in a sum.
	/// For fields that are not sums, it is [`None`]
	fn key<'m>(&self, side: Side, access: Access<'m>) -> Option<Key<'m>>;
	fn skip_chart(&self) -> bool;

	/// Determines whether the field's value is measurable.
	///
	/// If it cannot be measured, calling [`Field::diff`] will return the left-hand side only,
	/// and using it for measurements (e.g. in a historical graph) would not make sense.
	fn is_measurable(&self) -> bool;

	fn diff_log(&self, mode: &Mode<'_>, log: &syn::Ident) -> Option<proc_macro2::TokenStream> {
		let lhs = self.value(Side::Lhs, Access::ModeDiff(mode))?;
		let rhs = self.value(Side::Rhs, Access::ModeDiff(mode))?;
		let tr = self.as_tr();

		let (lhs_f, rhs_f) = if self.div().is_some() && !self.kind().is_percent() {
			(quote!(lhs.0 / lhs.1), quote!(rhs.0 / rhs.1))
		} else {
			(quote!(lhs), quote!(rhs))
		};

		Some(quote!({
			use crate::canvas::label::ToFormatted;

			let lhs = #lhs;
			let rhs = #rhs;

			let lhs = #lhs_f;
			let rhs = #rhs_f;

			if rhs > lhs {
				#log.push_str("- ");
				#log.push_str(#tr.as_ref());
				#log.push_str(": `");
				#log.push_str(rhs.to_formatted(ctx).as_ref());
				#log.push_str("` (`+");
				#log.push_str((rhs - lhs).to_formatted(ctx).as_ref());
				#log.push_str("`)\n");
			} else if rhs < lhs {
				#log.push_str("- ");
				#log.push_str(#tr.as_ref());
				#log.push_str(": `");
				#log.push_str(rhs.to_formatted(ctx).as_ref());
				#log.push_str("` (`-");
				#log.push_str((lhs - rhs).to_formatted(ctx).as_ref());
				#log.push_str("`)\n");
			}
		}))
	}

	fn diff_log_sum(
		&self,
		modes: &[Mode<'_>],
		log: &syn::Ident,
	) -> Option<proc_macro2::TokenStream> {
		let lhs = self.value_sum(Side::Lhs, modes, Access::NoneDiff)?;
		let rhs = self.value_sum(Side::Rhs, modes, Access::NoneDiff)?;
		let tr = self.as_tr();

		let (lhs_f, rhs_f) = if self.div().is_some() && !self.kind().is_percent() {
			(quote!(lhs.0 / lhs.1), quote!(rhs.0 / rhs.1))
		} else {
			(quote!(lhs), quote!(rhs))
		};

		Some(quote!({
			use crate::canvas::label::ToFormatted;

			let lhs = #lhs;
			let rhs = #rhs;

			let lhs = #lhs_f;
			let rhs = #rhs_f;

			if rhs > lhs {
				#log.push_str("- ");
				#log.push_str(#tr.as_ref());
				#log.push_str(": `");
				#log.push_str(rhs.to_formatted(ctx).as_ref());
				#log.push_str("` (`+");
				#log.push_str((rhs - lhs).to_formatted(ctx).as_ref());
				#log.push_str("`)\n");
			} else if rhs < lhs {
				#log.push_str("- ");
				#log.push_str(#tr.as_ref());
				#log.push_str(": `");
				#log.push_str(rhs.to_formatted(ctx).as_ref());
				#log.push_str("` (`-");
				#log.push_str((lhs - rhs).to_formatted(ctx).as_ref());
				#log.push_str("`)\n");
			}
		}))
	}

	/// Diffs the field against another field of the same type,
	/// formatting it according to the language in the provided [`ctx`].
	///
	/// If the field is smaller, `-` will be prepended. Otherwise, a `+` will be prepended.
	///
	/// This assumes access to the following variables:
	/// - `ctx`: The context object from [`translate::context::Context`].
	/// - `stats_lhs` and `stats_rhs`
	///
	/// Returns a Cow<str> of the formatted value.
	fn diff_fmt(&self, mode: &Mode<'_>) -> Option<proc_macro2::TokenStream> {
		let lhs = self.value(Side::Lhs, Access::ModeDiff(mode))?;
		let rhs = self.value(Side::Rhs, Access::ModeDiff(mode))?;

		let ratio = if self.div().is_some() && !self.kind().is_percent() {
			quote! {
				if !relative_ratios {
					let bottom = rhs.1 - lhs.1;
					let value = (rhs.0 - lhs.0) / if bottom == 0. { 1. } else { bottom };

					format!("{}", value.to_formatted(ctx))
				} else {
					let (lhs, rhs) = (lhs.0 / lhs.1, rhs.0 / rhs.1);

					if rhs > lhs {
						format!("+{}", (rhs - lhs).to_formatted(ctx))
					} else if rhs < lhs {
						format!("-{}", (lhs - rhs).to_formatted(ctx))
					} else {
						format!("±{}", (rhs - lhs).to_formatted(ctx))
					}
				}
			}
		} else {
			quote! {
				if rhs > lhs {
					format!("+{}", (rhs - lhs).to_formatted(ctx))
				} else if rhs < lhs {
					format!("-{}", (lhs - rhs).to_formatted(ctx))
				} else {
					format!("±{}", (rhs - lhs).to_formatted(ctx))
				}
			}
		};

		Some(quote!({
			use crate::canvas::label::ToFormatted;

			let lhs = #lhs;
			let rhs = #rhs;

			#ratio
		}))
	}

	fn diff_fmt_sum(&self, modes: &[Mode<'_>]) -> Option<proc_macro2::TokenStream> {
		let lhs = self.value_sum(Side::Lhs, modes, Access::NoneDiff)?;
		let rhs = self.value_sum(Side::Rhs, modes, Access::NoneDiff)?;

		let ratio = if self.div().is_some() && !self.kind().is_percent() {
			quote! {
				if !relative_ratios {
					let bottom = rhs.1 - lhs.1;
					let value = (rhs.0 - lhs.0) / if bottom == 0. { 1. } else { bottom };

					format!("{}", value.to_formatted(ctx))
				} else {
					let (lhs, rhs) = (lhs.0 / lhs.1, rhs.0 / rhs.1);

					if rhs > lhs {
						format!("+{}", (rhs - lhs).to_formatted(ctx))
					} else if rhs < lhs {
						format!("-{}", (lhs - rhs).to_formatted(ctx))
					} else {
						format!("±{}", (rhs - lhs).to_formatted(ctx))
					}
				}
			}
		} else {
			quote! {
				if rhs > lhs {
					format!("+{}", (rhs - lhs).to_formatted(ctx))
				} else if rhs < lhs {
					format!("-{}", (lhs - rhs).to_formatted(ctx))
				} else {
					format!("±{}", (rhs - lhs).to_formatted(ctx))
				}
			}
		};

		Some(quote!({
			use crate::canvas::label::ToFormatted;

			let lhs = #lhs;
			let rhs = #rhs;

			#ratio
		}))
	}

	/// [`stats`] should point to the mode's struct
	/// The TokenStream results in a value that can be formatted with ToFormatted::to_formatted
	fn value(&self, side: Side, access: Access<'_>) -> Option<proc_macro2::TokenStream> {
		// if `div` is present, divide the value by the value of the field,
		// casting both to f64 beforehand
		if let Some(bottom) = self.value_bottom(side, access) {
			let api = crate_ident("api");

			let top = self.value_top(side, access)?;
			let value = quote!({
				let bottom = #bottom as f64;

				#api::canvas::prelude::Div((#top) as f64, if bottom == 0. { 1. } else { bottom })
			});

			Some(if let FieldKind::Percent(percent) = self.kind() {
				quote!({
					let v = #value;

					#percent ((v.0 / v.1 * 100.) as u32)
				})
			} else {
				value
			})
		} else {
			self.value_top(side, access)
		}
	}

	fn value_trunc(&self, side: Side, access: Access<'_>) -> Option<proc_macro2::TokenStream> {
		// if `div` is present, divide the value by the value of the field,
		// casting both to f64 beforehand
		if let Some(bottom) = self.value_bottom(side, access) {
			let api = crate_ident("api");
			let top = self.value_top(side, access)?;
			let value = quote!({
				let bottom = #bottom as f64;

				#api::canvas::prelude::Div((#top) as f64, if bottom == 0. { 1. } else { bottom })
			});

			Some(if let FieldKind::Percent(percent) = self.kind() {
				quote!({
					let v = #value;

					#percent ((v.0 / v.1 * 100.) as u32)
				})
			} else {
				quote!({ u32::from(#value) })
			})
		} else {
			self.value_top(side, access)
		}
	}

	/// [`stats`] should point to the game's struct
	/// The TokenStream results in a value that can be formatted with ToFormatted::to_formatted
	fn value_sum(
		&self,
		side: Side,
		modes: &[Mode<'_>],
		access: Access<'_>,
	) -> Option<proc_macro2::TokenStream> {
		if self.key(side, access).is_some() {
			return self.value(side, access);
		}

		if let FieldKind::Min = self.kind() {
			let mut values = modes
				.iter()
				.filter_map(|m| self.value(side, access.with_mode(m)));

			let mut stream = {
				let Some(value) = values.next() else {
					return self.value(side, access);
				};

				quote! {
					let mut min = #value;
				}
			};

			stream.extend(values.map(|value| {
				quote!({
					let v = #value;

					if v < min {
						min = v;
					}
				})
			}));

			Some(quote!({
				#stream
				min
			}))
		}
		// if `div` is present, divide the value by the value of the field,
		// casting both to f64 beforehand
		else if let Some(bottom) = self.value_bottom_sum(side, modes, access) {
			let api = crate_ident("api");

			let top = self.value_top_sum(side, modes, access)?;
			let value = quote!({
				let bottom = #bottom as f64;

				#api::canvas::prelude::Div((#top) as f64, if bottom == 0. { 1. } else { bottom })
			});

			Some(if let FieldKind::Percent(percent) = self.kind() {
				quote!({
					let v = #value;

					#percent ((v.0 / v.1 * 100.) as u32)
				})
			} else {
				value
			})
		} else {
			self.value_top_sum(side, modes, access)
		}
	}

	fn value_trunc_sum(
		&self,
		side: Side,
		modes: &[Mode<'_>],
		access: Access<'_>,
	) -> Option<proc_macro2::TokenStream> {
		if self.key(side, access).is_some() {
			return self.value_trunc(side, access);
		}

		if let FieldKind::Min = self.kind() {
			let mut values = modes
				.iter()
				.filter_map(|m| self.value(side, access.with_mode(m)));

			let mut stream = {
				let Some(value) = values.next() else {
					return self.value_trunc(side, access);
				};

				quote! {
					let mut min = #value;
				}
			};

			stream.extend(values.map(|value| {
				quote!({
					let v = #value;

					if v < min {
						min = v;
					}
				})
			}));

			Some(quote!({
				#stream
				min
			}))
		}
		// if `div` is present, divide the value by the value of the field,
		// casting both to f64 beforehand
		else if let Some(bottom) = self.value_bottom_sum(side, modes, access) {
			let top = self.value_top_sum(side, modes, access)?;
			let value = quote!({
				let bottom = #bottom as f64;

				((#top) as f64) / if bottom == 0. { 1. } else { bottom }
			});

			Some(if let FieldKind::Percent(percent) = self.kind() {
				quote!({ #percent ((#value * 100.) as u32) })
			} else {
				quote!({ (#value) as u32 })
			})
		} else {
			self.value_top_sum(side, modes, access)
		}
	}

	/// [`stats`] should point to the mode's struct
	/// Computes the value of the field, formatting it according to the language in the provided [`ctx`].
	/// The TokenStream results in a Cow<str>
	fn value_fmt(&self, access: Access<'_>) -> Option<proc_macro2::TokenStream> {
		let value = self.value(Side::None, access)?;

		Some(quote!({
			use crate::canvas::label::ToFormatted;

			#value.to_formatted(ctx)
		}))
	}

	/// [`stats`] should point to the game's struct
	/// Computes the sum of all values of the field over all given [`modes`], formatting it according to the language in the provided [`ctx`].
	/// The TokenStream results in a Cow<str>
	fn value_fmt_sum(
		&self,
		side: Side,
		modes: &[Mode<'_>],
		access: Access<'_>,
	) -> Option<proc_macro2::TokenStream> {
		let value = self.value_sum(side, modes, access)?;

		Some(quote!({
			use crate::canvas::label::ToFormatted;

			#value.to_formatted(ctx)
		}))
	}

	/// [`stats`] should point to the mode's struct
	fn value_top(&self, side: Side, access: Access<'_>) -> Option<proc_macro2::TokenStream> {
		let key = self.key(side, access)?;
		let self_id = self.id();

		Some(quote!(#key.#self_id))
	}

	/// [`stats`] should point to the game's struct
	fn value_top_sum(
		&self,
		side: Side,
		modes: &[Mode<'_>],
		access: Access<'_>,
	) -> Option<proc_macro2::TokenStream> {
		if self.key(side, access).is_some() {
			return self.value_top(side, access);
		}

		let self_id = self.id();

		let mut values = modes
			.iter()
			.filter_map(|m| self.key(side, access.with_mode(m)));

		let Some(key) = values.next() else {
			return self.value_top(side, access);
		};
		let mut stream = quote!(#key.#self_id);

		stream.extend(values.map(|key| quote!(+ #key.#self_id)));

		Some(quote!({ #stream }))
	}

	/// [`stats`] should point to the mode's struct
	fn value_bottom(&self, side: Side, access: Access<'_>) -> Option<proc_macro2::TokenStream> {
		self.div().and_then(|id| {
			let key = self.key(side, access)?;

			Some(quote!(#key.#id))
		})
	}

	/// [`stats`] should point to the game's struct
	fn value_bottom_sum(
		&self,
		side: Side,
		modes: &[Mode<'_>],
		access: Access<'_>,
	) -> Option<proc_macro2::TokenStream> {
		if self.key(side, access).is_some() {
			return self.value_bottom(side, access);
		}

		if let Some(id) = self.div() {
			let mut values = modes
				.iter()
				.filter_map(|m| self.key(side, access.with_mode(m)));

			let Some(key) = values.next() else {
				return self.value_bottom(side, access);
			};
			let mut stream = quote!(#key.#id);

			stream.extend(values.map(|key| quote!(+ #key.#id)));

			Some(quote!({ #stream }))
		} else {
			None
		}
	}
}
