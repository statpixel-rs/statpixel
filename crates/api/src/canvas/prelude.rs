use std::{borrow::Cow, fmt::Debug};

use chrono::{DateTime, Utc};
use minecraft::style::Family;
use poise::serenity_prelude as serenity;
use translate::{context, Error};
use uuid::Uuid;

use crate::player::{self, data, status};

#[allow(clippy::too_many_arguments)]
pub trait Game {
	const HAS_COMPACT: bool;
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

pub struct PlayerInput<'d> {
	pub uuid: Uuid,
	pub data: &'d data::Data,
	pub session: Option<&'d status::Session>,
	pub skin: Option<image::Image<'d>>,
}

pub struct BasicInput<'c, M: Mode> {
	pub ctx: &'c context::Context<'c>,
	pub mode: Option<M>,
}

pub trait BasicDataContainer {
	type Mode: Mode;

	fn data(
		&self,
		input: BasicInput<Self::Mode>,
		player: PlayerInput,
	) -> impl Iterator<Item = (Cow<str>, Cow<str>)>;
	fn labels(
		&self,
		input: BasicInput<Self::Mode>,
		player: PlayerInput,
	) -> impl Iterator<Item = (Cow<str>, Cow<str>)>;
}

pub trait DiffDataContainer {
	type Mode: Mode;

	fn data(
		&self,
		input: BasicInput<Self::Mode>,
		player_lhs: PlayerInput,
		player_rhs: PlayerInput,
	) -> impl Iterator<Item = (Cow<str>, Cow<str>)>;
	fn labels(
		&self,
		input: BasicInput<Self::Mode>,
		player_lhs: PlayerInput,
		player_rhs: PlayerInput,
	) -> impl Iterator<Item = (Cow<str>, Cow<str>)>;
}

pub struct ChartInput<'c, M: Mode> {
	pub ctx: &'c context::Context<'c>,
	pub mode: Option<M>,
}

pub trait ChartDataContainer {
	type Mode: Mode;

	fn points(
		&self,
		input: ChartInput<Self::Mode>,
		player: PlayerInput,
	) -> impl Iterator<Item = (DateTime<Utc>, f64)>;
}

pub struct Projection {
	pub value: f64,
	pub at: DateTime<Utc>,
}

pub struct ProjectionInput<'c, M: Mode> {
	pub ctx: &'c context::Context<'c>,
	pub mode: Option<M>,
	pub kind: M::Kind,
	pub value: Option<f64>,
}

pub trait ProjectionDataContainer {
	type Mode: Mode;

	fn project(&self, input: ProjectionInput<Self::Mode>, player: PlayerInput) -> Projection;
	fn points(
		&self,
		input: ProjectionInput<Self::Mode>,
		player: PlayerInput,
	) -> impl Iterator<Item = (DateTime<Utc>, f64)>;
}

pub struct ImageInput<'c, M: Mode> {
	pub ctx: &'c context::Context<'c>,
	pub mode: Option<M>,
	pub suffix: Option<&'c str>,
	pub background: Option<skia_safe::Color>,
}

pub trait Image {
	type Mode: Mode;

	fn general(
		input: ImageInput<<Self as Image>::Mode>,
		skin: image::Image<'_>,
		uuid: Uuid,
	) -> Result<Vec<u8>, Error>
	where
		Self: BasicDataContainer<Mode = <Self as Image>::Mode>;

	fn diff(
		input: ImageInput<<Self as Image>::Mode>,
		skin: image::Image<'_>,
		lhs: PlayerInput,
		rhs: PlayerInput,
	) -> Result<Vec<u8>, Error>
	where
		Self: DiffDataContainer<Mode = <Self as Image>::Mode>;

	fn chart(
		input: ImageInput<<Self as Image>::Mode>,
		player: PlayerInput,
	) -> Result<Vec<u8>, Error>
	where
		Self: ChartDataContainer<Mode = <Self as Image>::Mode>;

	fn project(
		input: ImageInput<<Self as Image>::Mode>,
		player: PlayerInput,
		projection: Projection,
	) -> Result<Vec<u8>, Error>
	where
		Self: ProjectionDataContainer<Mode = <Self as Image>::Mode>;
}

pub trait CondensedImage {
	type Mode: Mode;

	fn general(
		input: ImageInput<<Self as CondensedImage>::Mode>,
		uuid: Uuid,
	) -> Result<Vec<u8>, Error>
	where
		Self: BasicDataContainer<Mode = <Self as CondensedImage>::Mode>;

	fn diff(
		input: ImageInput<<Self as CondensedImage>::Mode>,
		lhs: PlayerInput,
		rhs: PlayerInput,
	) -> Result<Vec<u8>, Error>
	where
		Self: DiffDataContainer<Mode = <Self as CondensedImage>::Mode>;

	fn chart(
		input: ImageInput<<Self as CondensedImage>::Mode>,
		player: PlayerInput,
	) -> Result<Vec<u8>, Error>
	where
		Self: ChartDataContainer<Mode = <Self as CondensedImage>::Mode>;

	fn project(
		input: ImageInput<<Self as CondensedImage>::Mode>,
		player: PlayerInput,
		projection: Projection,
	) -> Result<Vec<u8>, Error>
	where
		Self: ProjectionDataContainer<Mode = <Self as CondensedImage>::Mode>;
}

pub struct PlaintextInput<'c, M: Mode> {
	pub ctx: &'c context::Context<'c>,
	pub mode: Option<M>,
}

pub trait Plaintext {
	type Mode: Mode;

	fn general(
		input: PlaintextInput<<Self as Plaintext>::Mode>,
		uuid: Uuid,
	) -> Result<Vec<u8>, Error>
	where
		Self: BasicDataContainer<Mode = <Self as Plaintext>::Mode>;

	fn diff(
		input: PlaintextInput<<Self as Plaintext>::Mode>,
		lhs: PlayerInput,
		rhs: PlayerInput,
	) -> Result<Vec<u8>, Error>
	where
		Self: DiffDataContainer<Mode = <Self as Plaintext>::Mode>;

	fn chart(
		input: PlaintextInput<<Self as Plaintext>::Mode>,
		player: PlayerInput,
	) -> Result<Vec<u8>, Error>
	where
		Self: ChartDataContainer<Mode = <Self as Plaintext>::Mode>;

	fn project(
		input: PlaintextInput<<Self as Plaintext>::Mode>,
		player: PlayerInput,
		projection: Projection,
	) -> Result<Vec<u8>, Error>
	where
		Self: ProjectionDataContainer<Mode = <Self as Plaintext>::Mode>;
}

pub trait Command {
	type Mode: Mode;

	fn general_plaintext(
		input: PlaintextInput<<Self as Command>::Mode>,
		uuid: Uuid,
	) -> Result<String, Error>
	where
		Self: Plaintext<Mode = <Self as Command>::Mode>;

	fn diff_plaintext(
		input: PlaintextInput<<Self as Command>::Mode>,
		uuid_lhs: Uuid,
		uuid_rhs: Uuid,
	) -> Result<String, Error>
	where
		Self: Plaintext<Mode = <Self as Command>::Mode>;

	fn chart_plaintext(
		input: PlaintextInput<<Self as Command>::Mode>,
		uuid: Uuid,
	) -> Result<String, Error>
	where
		Self: Plaintext<Mode = <Self as Command>::Mode>;

	fn project_plaintext(
		input: PlaintextInput<<Self as Command>::Mode>,
		uuid: Uuid,
		projection: Projection,
	) -> Result<String, Error>
	where
		Self: Plaintext<Mode = <Self as Command>::Mode>;

	fn general_image(
		input: ImageInput<<Self as Command>::Mode>,
		skin: image::Image<'_>,
		uuid: Uuid,
	) -> Result<Vec<u8>, Error>
	where
		Self: Image<Mode = <Self as Command>::Mode>;

	fn diff_image(
		input: ImageInput<<Self as Command>::Mode>,
		skin: image::Image<'_>,
		uuid_lhs: Uuid,
		uuid_rhs: Uuid,
	) -> Result<Vec<u8>, Error>
	where
		Self: Image<Mode = <Self as Command>::Mode>;

	fn chart_image(
		input: ImageInput<<Self as Command>::Mode>,
		player: PlayerInput,
	) -> Result<Vec<u8>, Error>
	where
		Self: Image<Mode = <Self as Command>::Mode>;

	fn project_image(
		input: ImageInput<<Self as Command>::Mode>,
		player: PlayerInput,
		projection: Projection,
	) -> Result<Vec<u8>, Error>
	where
		Self: Image<Mode = <Self as Command>::Mode>;

	fn general_condensed(
		input: ImageInput<<Self as Command>::Mode>,
		uuid: Uuid,
	) -> Result<Vec<u8>, Error>
	where
		Self: CondensedImage<Mode = <Self as Command>::Mode>;

	fn diff_condensed(
		input: ImageInput<<Self as Command>::Mode>,
		uuid_lhs: Uuid,
		uuid_rhs: Uuid,
	) -> Result<Vec<u8>, Error>
	where
		Self: CondensedImage<Mode = <Self as Command>::Mode>;

	fn chart_condensed(
		input: ImageInput<<Self as Command>::Mode>,
		player: PlayerInput,
	) -> Result<Vec<u8>, Error>
	where
		Self: CondensedImage<Mode = <Self as Command>::Mode>;

	fn project_condensed(
		input: ImageInput<<Self as Command>::Mode>,
		player: PlayerInput,
		projection: Projection,
	) -> Result<Vec<u8>, Error>
	where
		Self: CondensedImage<Mode = <Self as Command>::Mode>;
}

// TODO: implement defaults for all of these, so we can implement a command like so:
// impl Command for stats::bed_wars::BedWars {
// 	type Mode = stats::bed_wars::Mode;
// }
//
// then call it like so:
//
// stats::bed_wars::BedWars::general_plaintext(input, uuid)
//
// or, generically (e.g. for a "global" command):
//
// <Self as Command>::general_plaintext(input, uuid)
