use std::borrow::Cow;

use api::{canvas, Data, Session};
use skia_safe::Color;
use translate::context;

#[allow(clippy::needless_pass_by_value)]
pub fn command<G: api::prelude::Game>(
	ctx: &context::Context<'_>,
	mode: Option<G::Mode>,
	background: Option<Color>,
	data: &Data,
	session: &Session,
	skin: &skia_safe::Image,
	suffix: Option<&str>,
) -> Cow<'static, [u8]> {
	let mut surface = G::canvas(ctx, data, session, skin, mode, suffix, background);

	canvas::to_png(&mut surface).into()
}
