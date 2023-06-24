use std::ops::Range;

use crate::canvas::{
	chart::text::{PixelState, TextDrawingBackend},
	label::ToFormatted,
};
use chrono::{DateTime, Utc};
use minecraft::paint::Paint;
use plotters::{
	prelude::{ChartBuilder, IntoDrawingArea, LabelAreaPosition},
	series::LineSeries,
	style::{self, Color, IntoTextStyle},
};

use translate::{prelude::GetChronoLocale, context::Context, Error};

/// # Errors
/// Returns an error if the image could not be created.
pub fn create(
	ctx: &Context<'_>,
	series: impl IntoIterator<Item = (DateTime<Utc>, u32)>,
	colour: Paint,
	range_x: Range<DateTime<Utc>>,
	range_y: Range<u32>,
) -> Result<String, Error> {
	// Allocate a buffer large enough to hold an RGBA representation of the image
	let buffer = vec![PixelState::Empty; 5_000];

	// The BitMapBackend uses RGB, so we will need to conver it later
	let backend = TextDrawingBackend(buffer).into_drawing_area();

	// set start time to `created_at`, and end to last time
	let mut chart = ChartBuilder::on(&backend)
		.margin_top(60)
		.margin_bottom(15)
		.margin_left(30)
		.margin_right(20)
		.set_label_area_size(LabelAreaPosition::Left, 30)
		.set_label_area_size(LabelAreaPosition::Bottom, 30)
		.build_cartesian_2d(range_x, range_y)
		.map_err(|_| Error::Plotters)?;

	let locale = ctx.get_chrono_locale();

	chart
		.configure_mesh()
		.y_label_formatter(&|y| y.to_formatted_label(ctx).into_owned())
		.x_label_formatter(&|x| x.format_localized("%b %e", locale).to_string())
		.x_labels(10)
		.light_line_style(style::colors::WHITE.mix(0.1))
		.bold_line_style(style::colors::WHITE.mix(0.3))
		.axis_style(style::colors::WHITE.mix(0.5))
		.label_style(
			("Minecraft", 20)
				.into_text_style(&backend)
				.with_color(style::colors::WHITE),
		)
		.draw()
		.map_err(|_| Error::Plotters)?;

	chart
		.draw_series(LineSeries::new(series, colour.as_plotters().stroke_width(2)).point_size(2))
		.map_err(|_| Error::Plotters)?;

	// Ok(TextDrawingBackend::present(backend.get_pixel_range()))

	unimplemented!()
}
