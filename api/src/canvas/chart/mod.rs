pub mod console;
pub mod text;

use std::{borrow::Cow, ops::Range};

use crate::{canvas::label::ToFormatted, player::data::Data};
use chrono::{DateTime, Utc};
use minecraft::{
	paint::{self, Paint},
	text::{parse::minecraft_string, rank::Rank, Text},
};
use plotters::{
	prelude::{
		BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition, Rectangle,
		SeriesLabelPosition, ShapeStyle,
	},
	series::LineSeries,
	style::{self, Color, IntoTextStyle, Palette, Palette99, RGBColor},
};
use skia_safe::{
	textlayout::TextAlign, AlphaType, Borrows, ClipOp, Color4f, ColorType, ImageInfo, Point, RRect,
	Rect, Surface,
};
use translate::{prelude::GetChronoLocale, tr, Context, Error};

use super::WIDTH_F;

const CANVAS_BACKGROUND: RGBColor = RGBColor(31, 48, 64);
const BACKGROUND: RGBColor = RGBColor(21, 33, 43);

macro_rules! impl_chart_create {
	($ty: ident) => {
		pub mod $ty {
			use super::*;

			/// # Errors
			/// Returns an error if the image could not be created.
			pub fn create(
				ctx: Context<'_>,
				series: Vec<(Cow<str>, Vec<(DateTime<Utc>, $ty)>)>,
				range_x: Range<DateTime<Utc>>,
				range_y: Range<$ty>,
				colour: Option<Paint>,
			) -> Result<Vec<u8>, Error> {
				const PIXELS: usize = 750 * 389;
				const BUF_LEN_RGBA: usize = 4 * PIXELS;

				// Allocate a buffer large enough to hold an RGBA representation of the image
				let mut buffer = vec![u8::MAX; BUF_LEN_RGBA];

				// The BitMapBackend uses RGB, so we will need to conver it later
				let backend =
					BitMapBackend::with_buffer(&mut buffer, (750, 389)).into_drawing_area();

				backend
					.fill(&CANVAS_BACKGROUND)
					.map_err(|_| Error::Plotters)?;

				// set start time to `created_at`, and end to last time
				let mut chart = ChartBuilder::on(&backend)
					.margin_top(60)
					.margin_bottom(20)
					.margin_right(30)
					.set_label_area_size(LabelAreaPosition::Left, 90)
					.set_label_area_size(LabelAreaPosition::Bottom, 30)
					.build_cartesian_2d(range_x, range_y)
					.map_err(|_| Error::Plotters)?;

				let locale = ctx.get_chrono_locale();

				chart
					.configure_mesh()
					.y_label_formatter(&|y| y.to_formatted_label(ctx).into_owned())
					.x_label_formatter(&|x| x.format_localized("%b %e", locale).to_string())
					.x_labels(7)
					.light_line_style(style::colors::WHITE.mix(0.05))
					.bold_line_style(style::colors::WHITE.mix(0.1))
					.axis_style(style::colors::WHITE.mix(0.5))
					.label_style(
						("Minecraft", 20)
							.into_text_style(&backend)
							.with_color(style::colors::WHITE),
					)
					.draw()
					.map_err(|_| Error::Plotters)?;

				if let Some(colour) = colour {
					let colour: ShapeStyle = colour.as_plotters().into();

					for (name, series) in series.into_iter() {
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
					for (idx, (name, series)) in series.into_iter().enumerate() {
						let colour = Palette99::pick(idx).mix(0.9);

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
					.border_style(&style::colors::TRANSPARENT)
					.background_style(&BACKGROUND.mix(0.8))
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
pub fn canvas(buffer: &mut [u8]) -> Result<Borrows<Surface>, Error> {
	let info = ImageInfo::new((750, 389), ColorType::RGBA8888, AlphaType::Premul, None);

	skia_safe::Surface::new_raster_direct(&info, buffer, 750 * 4, None).ok_or(Error::Canvas)
}

pub fn apply_title(ctx: Context<'_>, surface: &mut Surface, data: &Data, label: &[Text]) {
	let rank = data.get_rank();
	let username_paint = rank.get_username_paint();

	let mut text = Vec::new();
	let tr = tr!(ctx, "statistics-history");

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

	let rect = Rect::from_xywh(0., 0., 750., 50.);

	surface.canvas().draw_rect(rect, &paint::BACKGROUND);

	minecraft::text::draw(surface, text.as_slice(), 20., rect, TextAlign::Center, true);
}

pub fn round_corners(surface: &mut Surface) {
	let mut rect = RRect::new();

	rect.set_rect_radii(
		Rect::new(0., 0., WIDTH_F, 389.),
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
