pub struct Line {
	pub slope: f64,
	pub intercept: f64,
	pub mean_x: f64,
	pub mean_y: f64,

	pub simple_slope: f64,
	pub simple_intercept: f64,
}

impl Line {
	#[must_use]
	#[inline]
	pub fn y(&self, x: f64) -> f64 {
		self.slope * x + self.intercept
	}

	#[must_use]
	#[inline]
	pub fn x(&self, y: f64, last: f64) -> Option<f64> {
		if self.slope < f64::EPSILON {
			None
		} else {
			let x = (y - self.intercept) / self.slope;

			Some(if x < last {
				(y - self.simple_intercept) / self.simple_slope
			} else {
				x
			})
		}
	}

	/// # Panics
	/// Panics if `series` is empty.
	#[must_use]
	pub fn from_series(series: &[(f64, f64)]) -> Self {
		if series.len() == 1 {
			return Self {
				slope: 0.,
				intercept: series[0].1,
				mean_x: series[0].0,
				mean_y: series[0].1,
				simple_slope: 0.,
				simple_intercept: series[0].1,
			};
		}

		#[allow(clippy::cast_precision_loss)]
		let mean_x = series.iter().map(|s| s.0).sum::<f64>() / series.len() as f64;
		#[allow(clippy::cast_precision_loss)]
		let mean_y = series.iter().map(|s| s.1).sum::<f64>() / series.len() as f64;

		let m = series
			.iter()
			.map(|s| (s.0 - mean_x) * (s.1 - mean_y))
			.sum::<f64>()
			/ series.iter().map(|s| (s.0 - mean_x).powi(2)).sum::<f64>();

		let b = mean_y - m * mean_x;

		let first = series.first().unwrap();
		let last = series.last().unwrap();

		Self {
			slope: m,
			intercept: b,
			mean_x,
			mean_y,
			simple_slope: (last.1 - first.1) / (last.0 - first.0),
			simple_intercept: first.1 - (first.0 * (last.1 - first.1) / (last.0 - first.0)),
		}
	}
}

#[must_use]
pub fn compute_r(series: &[(f64, f64)], line: &Line) -> f64 {
	if line.slope == 0. {
		return if series.len() == 1 { 0. } else { 1. };
	}

	(1. - series
		.iter()
		.map(|s| (s.1 - (line.slope * s.0 + line.intercept)).powi(2))
		.sum::<f64>()
		/ series
			.iter()
			.map(|s| (s.1 - line.mean_y).powi(2))
			.sum::<f64>())
	.sqrt()
}
