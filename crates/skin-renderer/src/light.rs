#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
	position: [f32; 4],
	color: [f32; 4],
}

impl LightUniform {
	pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
		Self {
			position: [
				position[0],
				position[1],
				position[2],
				/* padding */ 0.0,
			],
			color: [color[0], color[1], color[2], /* padding */ 0.0],
		}
	}
}

impl Default for LightUniform {
	fn default() -> Self {
		LightUniform::new([0.0, 50.0, 310.0], [1.0, 1.0, 1.0])
	}
}
