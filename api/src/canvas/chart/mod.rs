pub mod console;
pub mod text;

use std::{borrow::Cow, ops::Range};

use crate::{canvas::label::ToFormatted, player::data::Data};
use chrono::{DateTime, Utc};
use minecraft::{
	paint::Paint,
	text::{parse::minecraft_string, rank::Rank, Text},
};
use plotters::{
	prelude::{
		BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, Rectangle,
		SeriesLabelPosition, ShapeStyle,
	},
	series::LineSeries,
	style::{self, Color, IntoTextStyle, Palette, Palette99, RGBAColor, RGBColor},
};
use skia_safe::{
	AlphaType, Borrows, ClipOp, Color4f, ColorType, ImageInfo, Point, RRect, Rect, Surface,
};
use translate::{context::Context, prelude::GetChronoLocale, tr, Error};

use super::shape;

pub const WIDTH_F: f32 = 750.;

pub const CANVAS_BACKGROUND: RGBAColor = RGBAColor(31, 48, 64, 1.);
pub const BACKGROUND: RGBAColor = RGBAColor(20, 20, 20, 1.);

macro_rules! impl_chart_create {
	($ty: ident) => {
		pub mod $ty {
			use super::*;

			/// # Errors
			/// Returns an error if the image could not be created.
			pub fn create<const BUBBLE: bool>(
				ctx: &Context<'_>,
				series: Vec<(Cow<str>, Vec<(DateTime<Utc>, $ty)>)>,
				range_x: Range<DateTime<Utc>>,
				range_y: Range<$ty>,
				colour: Option<Paint>,
				background: Option<skia_safe::Color>,
			) -> Result<Vec<u8>, Error> {
				const PIXELS: usize = 750 * 400;
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
					BitMapBackend::with_buffer(&mut buffer, (750, 400)).into_drawing_area();

				backend.fill(&background).map_err(|_| Error::Plotters)?;

				// set start time to `created_at`, and end to last time
				let mut chart = ChartBuilder::on(&backend)
					.margin_top(71)
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
						("Minecraft", 20)
							.into_text_style(&backend)
							.with_color(foreground),
					)
					.draw()
					.map_err(|_| Error::Plotters)?;

				if let Some(colour) = colour {
					let colour: ShapeStyle = colour.as_plotters().into();

					for (name, series) in series.into_iter() {
						chart
							.draw_series(if BUBBLE {
								LineSeries::new(series, colour.filled().stroke_width(2))
									.point_size(2)
							} else {
								LineSeries::new(series, colour.filled().stroke_width(2))
							})
							.map_err(|_| Error::Plotters)?
							.label(name.into_owned())
							.legend(move |(x, y)| {
								Rectangle::new([(x, y - 5), (x + 10, y + 5)], colour.filled())
							});
					}
				} else {
					for (idx, (name, series)) in series.into_iter().enumerate() {
						let colour = Palette99::pick(idx).mix(0.9);

						chart
							.draw_series(if BUBBLE {
								LineSeries::new(series, colour.filled().stroke_width(2))
									.point_size(2)
							} else {
								LineSeries::new(series, colour.filled().stroke_width(2))
							})
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
					.border_style(&style::colors::TRANSPARENT)
					.background_style(&BACKGROUND.mix(0.6))
					.label_font(
						("Minecraft", 17)
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

impl_chart_create!(u32);
impl_chart_create!(u64);

impl_chart_create!(i32);
impl_chart_create!(i64);

impl_chart_create!(f32);
impl_chart_create!(f64);

/// # Errors
/// Returns an error if the canvas could not be created.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn canvas_with_size(
	buffer: &mut [u8],
	size: (usize, usize),
) -> Result<Borrows<Surface>, Error> {
	let info = ImageInfo::new(
		(size.0 as i32, size.1 as i32),
		ColorType::RGBA8888,
		AlphaType::Premul,
		None,
	);

	skia_safe::surfaces::wrap_pixels(&info, buffer, size.0 * 4, None).ok_or(Error::Canvas)
}

/// # Errors
/// Returns an error if the canvas could not be created.
pub fn canvas(buffer: &mut [u8]) -> Result<Borrows<Surface>, Error> {
	let info = ImageInfo::new((750, 400), ColorType::RGBA8888, AlphaType::Premul, None);

	skia_safe::surfaces::wrap_pixels(&info, buffer, 750 * 4, None).ok_or(Error::Canvas)
}

pub fn apply_title(
	ctx: &Context<'_>,
	surface: &mut Surface,
	data: &Data,
	label: &[Text],
	background: Option<skia_safe::Color>,
) {
	let rank = data.get_rank();
	let username_paint = rank.get_username_paint();

	let mut text = Vec::new();
	let tr = tr(ctx, "statistics-history");

	text.extend(label);

	text.push(Text {
		text: tr.as_ref(),
		paint: Paint::White,
		..Default::default()
	});

	if let Some(rank) = rank.get_text() {
		text.extend(rank);
	} else if let Some(prefix) = data.prefix.as_ref() {
		text.extend(minecraft_string(prefix));
	}

	let username = if rank == Rank::Default {
		Cow::Borrowed(data.username.as_str())
	} else {
		Cow::Owned(format!(" {}", data.username))
	};

	text.push(Text {
		text: &username,
		paint: username_paint,
		..Default::default()
	});

	super::Canvas::new(720.)
		.push_right(&shape::LongTitle, shape::Title::from_text(&text))
		.build_with(surface, None, background);
}

pub fn round_corners(surface: &mut Surface) {
	let mut rect = RRect::new();

	rect.set_rect_radii(
		Rect::new(0., 0., WIDTH_F, 400.),
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
