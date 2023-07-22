use std::sync::Arc;

use chrono::{DateTime, Utc};
use minecraft::{
	calc::pit::{Level, Prestige},
	colour::Colour,
};
use poise::serenity_prelude::Embed;

pub trait Diff {
	#[must_use]
	fn diff(&self, other: &Self) -> Self;
}

#[allow(clippy::module_name_repetitions)]
pub trait DiffLog {
	fn diff_log(
		new: &crate::player::data::Data,
		other: &crate::player::data::Data,
		ctx: &translate::context::Context<'_>,
		mut embed: Embed,
	) -> Embed;
}

macro_rules! impl_to_formatted_label_for_number {
	($int:ty) => {
		impl Diff for $int {
			fn diff(&self, other: &Self) -> Self {
				self - other
			}
		}
	};
}

impl_to_formatted_label_for_number!(u8);
impl_to_formatted_label_for_number!(u16);
impl_to_formatted_label_for_number!(u32);
impl_to_formatted_label_for_number!(u64);
impl_to_formatted_label_for_number!(u128);
impl_to_formatted_label_for_number!(usize);
impl_to_formatted_label_for_number!(i8);
impl_to_formatted_label_for_number!(i16);
impl_to_formatted_label_for_number!(i32);
impl_to_formatted_label_for_number!(i64);
impl_to_formatted_label_for_number!(i128);
impl_to_formatted_label_for_number!(isize);
impl_to_formatted_label_for_number!(f32);
impl_to_formatted_label_for_number!(f64);

impl Diff for Colour {
	fn diff(&self, _other: &Self) -> Self {
		*self
	}
}

impl Diff for bool {
	fn diff(&self, _other: &Self) -> Self {
		*self
	}
}

impl Diff for Prestige {
	fn diff(&self, other: &Self) -> Self {
		Self {
			xp: self.xp.diff(&other.xp),
		}
	}
}

impl Diff for Level {
	fn diff(&self, other: &Self) -> Self {
		Self {
			xp: self.xp.diff(&other.xp),
			prestiges: self.prestiges.diff(&other.prestiges),
		}
	}
}

impl<T> Diff for Vec<T> {
	fn diff(&self, _other: &Self) -> Self {
		Vec::new()
	}
}

impl<T> Diff for Option<T>
where
	T: Diff + Clone,
{
	fn diff(&self, other: &Self) -> Self {
		match (self, other) {
			(Some(self_), Some(other_)) => Some(self_.diff(other_)),
			(Some(self_), None) => Some(self_.clone()),
			(None, Some(other_)) => Some(other_.clone()),
			(None, None) => None,
		}
	}
}

impl Diff for String {
	fn diff(&self, _other: &Self) -> Self {
		self.clone()
	}
}

impl Diff for DateTime<Utc> {
	fn diff(&self, _other: &Self) -> Self {
		*self
	}
}

impl<T> Diff for Arc<T>
where
	T: Diff,
{
	fn diff(&self, other: &Self) -> Self {
		self.as_ref().diff(other.as_ref()).into()
	}
}
