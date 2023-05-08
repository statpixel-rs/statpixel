use std::f32::consts::PI;

use skia_safe::{Path, RRect, Rect};

/// Creates a progress bar as a RRect border, returning a Path
/// with the top-left corner of the rectangle at (0, 0)
pub fn rrect_progress(rrect: RRect, progress: f32) -> Path {
	let r = rrect.simple_radii();
	let rect = Rect::from_wh(r.x * 2., r.y * 2.);

	// Progress of the horizontal strip of the rectangle
	let progress_h = rrect.width() - r.x * 2.;
	let progress_h_half = progress_h * 0.5;

	// Progress of the vertical strip of the rectangle
	let progress_v = rrect.height() - r.y * 2.;

	// Assume corners are circular
	let progress_r = 2. * PI * r.x;

	let total = progress_r * 4. + progress_h * 2. + progress_v * 2.;
	let mut progress = progress * total;

	let mut path = Path::new();

	// Start at the top middle, then go counter-clockwise
	path.move_to((rrect.width() / 2., 0.));

	// If the progress is more than the line to the top left corner,
	// draw the line and keep going
	if progress > progress_h_half {
		path.line_to((r.x, 0.));
	} else {
		// Otherwise, draw the partial line and exit
		path.line_to((r.x + progress_h_half - progress, 0.));

		return path;
	}

	progress -= progress_h_half;

	// If the progress is more than the top left corner,
	// draw the corner and keep going
	if progress > progress_r {
		path.arc_to(rect, -90., -90., true);
	} else {
		path.arc_to(rect, -90., -90. * (progress / progress_r), true);

		return path;
	}

	progress -= progress_r;

	// If the progress is more than the line to the bottom left corner,
	// draw the line and keep going
	if progress > progress_v {
		path.line_to((0., r.y + progress_v));
	} else {
		path.line_to((0., r.y + progress));

		return path;
	}

	progress -= progress_v;

	// If the progress is more than the bottom left corner,
	// draw the corner and keep going
	{
		let rect = Rect::from_xywh(0., rrect.height() - r.y * 2., r.x * 2., r.y * 2.);

		if progress > progress_r {
			path.arc_to(rect, 180., -90., true);
		} else {
			path.arc_to(rect, 180., -90. * (progress / progress_r), true);

			return path;
		}
	}

	progress -= progress_r;

	// If the progress is more than the line to the bottom right corner,
	// draw the line and keep going
	if progress > progress_h {
		path.line_to((r.x + progress_h, rrect.height()));
	} else {
		path.line_to((r.x + progress, rrect.height()));

		return path;
	}

	progress -= progress_h;

	// If the progress is more than the bottom right corner,
	// draw the corner and keep going
	{
		let rect = Rect::from_xywh(
			rrect.width() - r.x * 2.,
			rrect.height() - r.y * 2.,
			r.x * 2.,
			r.y * 2.,
		);

		if progress > progress_r {
			path.arc_to(rect, 90., -90., true);
		} else {
			path.arc_to(rect, 90., -90. * (progress / progress_r), true);

			return path;
		}
	}

	progress -= progress_r;

	// If the progress is more than the line to the top right corner,
	// draw the line and keep going
	if progress > progress_v {
		path.line_to((rrect.width(), r.y));
	} else {
		path.line_to((rrect.width(), r.y + progress_v - progress));

		return path;
	}

	progress -= progress_v;

	// If the progress is more than the top right corner,
	// draw the corner and keep going
	{
		let rect = Rect::from_xywh(rrect.width() - r.x * 2., 0., r.x * 2., r.y * 2.);

		if progress > progress_r {
			path.arc_to(rect, 0., -90., true);
		} else {
			path.arc_to(rect, 0., -90. * (progress / progress_r), true);

			return path;
		}
	}

	progress -= progress_r;

	// If the progress is more than the line to the top middle,
	// draw the line and keep going
	if progress > progress_h_half {
		path.line_to((rrect.width() / 2., 0.));
	} else {
		path.line_to((rrect.width() / 2. + progress_h_half - progress, 0.));
	}

	path
}
