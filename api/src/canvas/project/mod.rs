pub mod line;

use std::{borrow::Cow, ops::Range};

use crate::canvas::{
	chart::{BACKGROUND, CANVAS_BACKGROUND},
	label::ToFormatted,
	shape::{BUBBLE_HEIGHT, GAP},
};
use chrono::{DateTime, Utc};
use minecraft::{
	paint::Paint,
	style::{Family, MinecraftFont},
	text::Text,
};
use plotters::{
	prelude::{
		BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, Rectangle,
		SeriesLabelPosition,
	},
	series::LineSeries,
	style::{self, Color, IntoTextStyle, Palette, Palette99, RGBAColor, RGBColor},
};
use skia_safe::{
	textlayout::TextAlign, AlphaType, Borrows, ClipOp, Color4f, ColorType, ImageInfo, Point, RRect,
	Rect, Surface,
};
use translate::{context::Context, prelude::GetChronoLocale, tr, Error};

use super::{body, chart::WIDTH_F, shape, INSET};

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
pub const BUBBLE_HEIGHT_I: u32 = BUBBLE_HEIGHT as u32;
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
pub const GAP_I: u32 = GAP as u32;

// Get the next milestone for the given value.
//
// # Examples
// ```
// assert_eq!(next_milestone(0.), 100.);
// assert_eq!(next_milestone(100.), 200.);
// assert_eq!(next_milestone(1503.), 2000.);
// assert_eq!(next_milestone(345867.), 400000.);
// // for less than 1
// assert_eq!(next_milestone(0.45), .5);
// assert_eq!(next_milestone(0.03), 0.1);
// ```
#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn next_milestone(value: f64) -> f64 {
	if value < 1. {
		(value * 10.).ceil() / 10.
	} else {
		let digits = value.log10().floor() as u32;

		// Correct number of digits, but not the correct value yet
		let next = 10_f64.powi(digits as i32);

		// The next value multiplier
		let curr_multiplier = (value / next + 1.).floor();

		next * curr_multiplier
	}
}

/// # Errors
/// Returns an error if the canvas could not be created.
#[allow(clippy::cast_possible_wrap)]
pub fn canvas(buffer: &mut [u8]) -> Result<Borrows<Surface>, Error> {
	let info = ImageInfo::new(
		(750, 400 + BUBBLE_HEIGHT_I as i32 + GAP_I as i32),
		ColorType::RGBA8888,
		AlphaType::Premul,
		None,
	);

	skia_safe::surfaces::wrap_pixels(&info, buffer, 750 * 4, None).ok_or(Error::Canvas)
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::cast_precision_loss)]
pub fn apply_bubbles(
	surface: &mut Surface,
	ctx: &Context<'_>,
	family: Family,
	kind: &str,
	value: &impl ToFormatted,
	acc: &impl ToFormatted,
	date: &impl ToFormatted,
	background: Option<skia_safe::Color>,
) {
	crate::canvas::Canvas::new(720., family)
		.gap(7.)
		.tl((0., INSET + 50.))
		.push(
			&shape::Bubble,
			body::Body::from_bubble(ctx, family, value, kind, Paint::Aqua),
		)
		.push_right(
			&shape::Bubble,
			body::Body::from_bubble(ctx, family, acc, tr(ctx, "accuracy").as_ref(), Paint::Gold),
		)
		.push_right(
			&shape::Bubble,
			body::Body::new(30., TextAlign::Center, family)
				.extend(&[
					Text {
						text: tr(ctx, "estimate").as_ref(),
						paint: Paint::LightPurple,
						font: MinecraftFont::Normal,
						size: Some(20.),
					},
					Text {
						text: "\n",
						size: Some(20.),
						..Default::default()
					},
					Text {
						text: &date.to_formatted(ctx),
						paint: Paint::LightPurple,
						font: MinecraftFont::Normal,
						size: None,
					},
				])
				.build(),
		)
		.build_with(surface, None, background);
}

pub fn round_corners(surface: &mut Surface) {
	let mut rect = RRect::new();

	rect.set_rect_radii(
		Rect::new(0., 0., WIDTH_F, 400. + BUBBLE_HEIGHT + GAP),
		&[
			Point::new(30., 30.),
			Point::new(30., 30.),
			Point::new(30., 30.),
			Point::new(30., 30.),
		],
	);

	surface.canvas().clip_rrect(rect, ClipOp::Difference, false);
	surface.canvas().save();
	surface.canvas().clear(Color4f::new(0., 0., 0., 0.));
	surface.canvas().restore();
}

macro_rules! impl_project_create {
	($ty: ident) => {
		pub mod $ty {
			use super::*;

			#[allow(clippy::too_many_lines)]
			#[allow(clippy::missing_errors_doc)]
			pub fn create(
				ctx: &Context<'_>,
				family: Family,
				series: Vec<(Cow<str>, Vec<(DateTime<Utc>, $ty)>, DateTime<Utc>, $ty)>,
				range_x: Range<DateTime<Utc>>,
				range_y: Range<$ty>,
				colour: Option<Paint>,
				background: Option<skia_safe::Color>,
			) -> Result<Vec<u8>, Error> {
				const PIXELS: usize = 750 * (400 + BUBBLE_HEIGHT_I as usize + GAP_I as usize);
				const BUF_LEN_RGBA: usize = 4 * PIXELS;

				let foreground = background.map_or(style::colors::WHITE, |c| {
					RGBColor(255 - c.r(), 255 - c.g(), 255 - c.b())
				});
				let background = background.map_or(CANVAS_BACKGROUND, |c| {
					RGBAColor(c.r(), c.g(), c.b(), c.a() as f64 / 255.)
				});

				// Allocate a buffer large enough to hold an RGBA representation of the image
				let mut buffer = vec![u8::MAX; BUF_LEN_RGBA];

				// The BitMapBackend uses RGB, so we will need to conver it later
				let backend =
					BitMapBackend::with_buffer(&mut buffer, (750, 400 + BUBBLE_HEIGHT_I + GAP_I))
						.into_drawing_area();

				backend.fill(&background).map_err(|_| Error::Plotters)?;

				// set start time to `created_at`, and end to last time
				let mut chart = ChartBuilder::on(&backend)
					.margin_top(71 + BUBBLE_HEIGHT_I + GAP_I * 2)
					.margin_bottom(20)
					.margin_right(30)
					.set_label_area_size(LabelAreaPosition::Left, 90)
					.set_label_area_size(LabelAreaPosition::Bottom, 30)
					.build_cartesian_2d(range_x, range_y)
					.map_err(|_| Error::Plotters)?;

				let locale = ctx.get_chrono_locale();

				chart
					.configure_mesh()
					.y_label_formatter(&|y| y.to_formatted(ctx).into_owned())
					.x_label_formatter(&|x| x.format_localized("%b %e", locale).to_string())
					.x_labels(7)
					.light_line_style(foreground.mix(0.05))
					.bold_line_style(foreground.mix(0.1))
					.axis_style(foreground.mix(0.5))
					.label_style(
						(family.as_str(), 20)
							.into_text_style(&backend)
							.with_color(foreground),
					)
					.draw()
					.map_err(|_| Error::Plotters)?;

				if let Some(colour) = colour {
					let colour = colour.as_plotters();

					for (name, series, predict_x, predict_y) in series {
						if let Some(last) = series.last() {
							chart
								.draw_series(
									LineSeries::new(
										[(last.0, last.1), (predict_x, predict_y)],
										colour.mix(0.5).filled().stroke_width(2),
									)
									.point_size(2),
								)
								.map_err(|_| Error::Plotters)?;
						}

						chart
							.draw_series(
								LineSeries::new(series, colour.filled().stroke_width(2))
									.point_size(2),
							)
							.map_err(|_| Error::Plotters)?
							.label(name.into_owned())
							.legend(move |(x, y)| {
								Rectangle::new([(x, y - 5), (x + 10, y + 5)], colour.filled())
							});
					}
				} else {
					for (idx, (name, series, predict_x, predict_y)) in
						series.into_iter().enumerate()
					{
						let colour = Palette99::pick(idx).mix(0.9);

						if let Some(last) = series.last() {
							chart
								.draw_series(
									LineSeries::new(
										[(last.0, last.1), (predict_x, predict_y)],
										colour.mix(0.5).filled().stroke_width(2),
									)
									.point_size(2),
								)
								.map_err(|_| Error::Plotters)?;
						}

						chart
							.draw_series(
								LineSeries::new(series, colour.filled().stroke_width(2))
									.point_size(2),
							)
							.map_err(|_| Error::Plotters)?
							.label(name.into_owned())
							.legend(move |(x, y)| {
								Rectangle::new([(x, y - 5), (x + 10, y + 5)], colour.filled())
							});
					}
				}

				chart
					.configure_series_labels()
					.position(SeriesLabelPosition::UpperLeft)
					.border_style(style::colors::TRANSPARENT)
					.background_style(&BACKGROUND.mix(0.6))
					.label_font(
						(family.as_str(), 17)
							.into_text_style(&backend)
							.with_color(style::colors::WHITE),
					)
					.draw()
					.map_err(|_| Error::Plotters)?;

				backend.present().map_err(|_| Error::Plotters)?;

				drop(chart);
				drop(backend);

				// Convert the RGB buffer to RGBA, in place
				for i in (0..PIXELS).rev() {
					buffer.copy_within(i * 3..i * 3 + 3, i * 4);

					// Set fourth channel to max
					buffer[i * 4 + 3] = u8::MAX;
				}

				Ok(buffer)
			}
		}
	};
}

impl_project_create!(u32);
impl_project_create!(u64);

impl_project_create!(i32);
impl_project_create!(i64);

impl_project_create!(f32);
impl_project_create!(f64);
