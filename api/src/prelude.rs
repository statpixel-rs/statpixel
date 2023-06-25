use std::fmt::Debug;

use chrono::{DateTime, Utc};
use poise::serenity_prelude as serenity;
use translate::{context, Error};
use uuid::Uuid;

use crate::player::{self, data, status};

#[allow(clippy::too_many_arguments)]
pub trait Game {
	type Mode: Mode + Debug;

	fn canvas_diff(
		ctx: &context::Context,
		prev: &data::Data,
		curr: &mut data::Data,
		session: &status::Session,
		skin: &skia_safe::Image,
		mode: Option<Self::Mode>,
		suffix: Option<&str>,
		background: Option<skia_safe::Color>,
	) -> skia_safe::Surface;

	fn canvas(
		ctx: &context::Context,
		data: &data::Data,
		session: &status::Session,
		skin: &skia_safe::Image,
		mode: Option<Self::Mode>,
		suffix: Option<&str>,
		background: Option<skia_safe::Color>,
	) -> skia_safe::Surface;

	/// # Errors
	///
	/// See specific implementations for possible errors.
	fn chart(
		ctx: &context::Context,
		snapshots: Vec<(DateTime<Utc>, data::Data)>,
		session: &status::Session,
		mode: Option<Self::Mode>,
	) -> Result<Vec<u8>, Error>;

	/// # Errors
	///
	/// See specific implementations for possible errors.
	fn project(
		ctx: &context::Context,
		snapshots: Vec<(DateTime<Utc>, data::Data)>,
		session: &status::Session,
		mode: Option<Self::Mode>,
		kind: Option<<<Self as Game>::Mode as Mode>::Kind>,
		value: Option<f64>,
	) -> Result<Vec<u8>, Error>;

	fn embed(
		ctx: &context::Context,
		player: &player::Player,
		data: &data::Data,
	) -> serenity::CreateEmbed;

	fn embed_diff(
		ctx: &context::Context,
		player: &player::Player,
		prev: &data::Data,
		curr: &mut data::Data,
	) -> serenity::CreateEmbed;
}

pub trait Mode: Sized + Copy {
	type Kind: Copy + Default;

	fn as_root(
		ctx: &context::Context,
		uuid: Uuid,
		selected: Option<Self>,
	) -> serenity::CreateActionRow;

	fn as_snapshot(
		ctx: &context::Context,
		uuid: Uuid,
		from: DateTime<Utc>,
		selected: Option<Self>,
	) -> serenity::CreateActionRow;

	fn as_history(
		ctx: &context::Context,
		uuid: Uuid,
		selected: Option<Self>,
	) -> serenity::CreateActionRow;

	fn as_project(
		ctx: &context::Context,
		uuid: Uuid,
		kind: Self::Kind,
		selected: Option<Self>,
	) -> serenity::CreateActionRow;
}
