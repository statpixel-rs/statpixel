use std::borrow::Cow;

use api::{canvas, Data, Session};
use minecraft::style::Family;
use skia_safe::Color;
use translate::context;

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_arguments)]
pub fn command<G: api::canvas::prelude::Game>(
	ctx: &context::Context<'_>,
	family: Family,
	mode: Option<G::Mode>,
	background: Option<Color>,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
) -> (Cow<'static, [u8]>, G::Mode) {
	let (mut surface, mode) = G::canvas(ctx, family, data, session, skin, mode, suffix, background);

	(canvas::to_png(&mut surface).into(), mode)
}
