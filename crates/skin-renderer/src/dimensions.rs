#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
	pub width: u32,
	pub height: u32,
	pub unpadded_bytes_per_row: u32,
	pub padded_bytes_per_row: u32,
}

impl Dimensions {
	pub const fn new(width: u32, height: u32) -> Self {
		let bytes_per_pixel = std::mem::size_of::<u32>() as u32;
		let unpadded_bytes_per_row = width * bytes_per_pixel;
		let padded_bytes_per_row =
			Self::align_up(unpadded_bytes_per_row, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);

		Self {
			width,
			height,
			unpadded_bytes_per_row,
			padded_bytes_per_row,
		}
	}

	// Align upwards. Returns the smallest x such that x is a multiple of `align` and x >= `val`.
	const fn align_up(val: u32, align: u32) -> u32 {
		(val + align - 1) / align * align
	}
}
