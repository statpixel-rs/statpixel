use std::fmt::Debug;

use chrono::{DateTime, Utc};
use minecraft::style::Family;
use poise::serenity_prelude as serenity;
use translate::{context, Error};
use uuid::Uuid;

use crate::player::{self, data, status};

#[allow(clippy::too_many_arguments)]
pub trait Game {
	type Mode: Mode + Debug;

	fn canvas_diff(
		ctx: &context::Context,
		family: Family,
		data_lhs: &data::Data,
		data_rhs: &data::Data,
		session: &status::Session,
		skin: &skia_safe::Image,
		mode: Option<Self::Mode>,
		suffix: Option<&str>,
		background: Option<skia_safe::Color>,
	) -> (skia_safe::Surface, Self::Mode);

	fn canvas(
		ctx: &context::Context,
		family: Family,
		data: &data::Data,
		session: &status::Session,
		skin: &skia_safe::Image,
		mode: Option<Self::Mode>,
		suffix: Option<&str>,
		background: Option<skia_safe::Color>,
	) -> (skia_safe::Surface, Self::Mode);

	fn condensed_diff(
		ctx: &context::Context,
		family: Family,
		data_lhs: &data::Data,
		data_rhs: &data::Data,
		suffix: Option<&str>,
		background: Option<skia_safe::Color>,
	) -> Vec<skia_safe::Surface>;

	fn condensed(
		ctx: &context::Context,
		family: Family,
		data: &data::Data,
		suffix: Option<&str>,
		background: Option<skia_safe::Color>,
	) -> Vec<skia_safe::Surface>;

	/// # Errors
	///
	/// See specific implementations for possible errors.
	fn chart(
		ctx: &context::Context,
		family: Family,
		snapshots: Vec<(DateTime<Utc>, data::Data)>,
		session: &status::Session,
		background: Option<skia_safe::Color>,
		mode: Option<Self::Mode>,
	) -> Result<(Vec<u8>, Self::Mode), Error>;

	/// # Errors
	///
	/// See specific implementations for possible errors.
	fn project(
		ctx: &context::Context,
		family: Family,
		snapshots: Vec<(DateTime<Utc>, data::Data)>,
		session: &status::Session,
		mode: Option<Self::Mode>,
		kind: Option<<<Self as Game>::Mode as Mode>::Kind>,
		value: Option<f64>,
		background: Option<skia_safe::Color>,
	) -> Result<(Vec<u8>, Self::Mode), Error>;

	fn embed(
		ctx: &context::Context,
		player: &player::Player,
		data: &data::Data,
	) -> serenity::CreateEmbed;

	fn embed_diff(
		ctx: &context::Context,
		player: &player::Player,
		data_lhs: &data::Data,
		data_rhs: &data::Data,
	) -> serenity::CreateEmbed;
}

pub trait Mode: Sized + Copy {
	type Kind: Copy + Default;

	fn as_root(
		ctx: &context::Context,
		uuid: Uuid,
		selected: Option<Self>,
	) -> (serenity::CreateActionRow, crate::id::Id);

	fn as_snapshot(
		ctx: &context::Context,
		uuid: Uuid,
		past: i64,
		selected: Option<Self>,
	) -> (serenity::CreateActionRow, crate::id::Id);

	fn as_history(
		ctx: &context::Context,
		uuid: Uuid,
		selected: Option<Self>,
	) -> (serenity::CreateActionRow, crate::id::Id);

	fn as_project(
		ctx: &context::Context,
		uuid: Uuid,
		kind: Self::Kind,
		selected: Option<Self>,
	) -> (serenity::CreateActionRow, crate::id::Id);

	fn as_compare(
		ctx: &context::Context,
		uuid_lhs: Uuid,
		uuid_rhs: Uuid,
		selected: Option<Self>,
	) -> (serenity::CreateActionRow, crate::id::Id);

	fn as_at(
		ctx: &context::Context,
		uuid: Uuid,
		past: i64,
		selected: Option<Self>,
	) -> (serenity::CreateActionRow, crate::id::Id);
}
