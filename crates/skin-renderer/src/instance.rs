use bytemuck::{Pod, Zeroable};
use glam::{Mat3, Mat4, Quat, Vec3};

pub struct Instance {
	position: Vec3,
	rotation: Quat,
}

impl Instance {
	pub fn to_raw(&self) -> InstanceRaw {
		InstanceRaw {
			model: (Mat4::from_translation(self.position) * Mat4::from_quat(self.rotation))
				.to_cols_array_2d(),
			normal: Mat3::from_quat(self.rotation).to_cols_array_2d(),
		}
	}
}

impl Default for Instance {
	fn default() -> Self {
		Self {
			position: Vec3::new(0.0, -102.0, 0.0),
			// glam uses radians for rotations
			rotation: Quat::from_axis_angle(Vec3::Y, 20.0_f32.to_radians()),
		}
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct InstanceRaw {
	model: [[f32; 4]; 4],
	normal: [[f32; 3]; 3],
}

impl InstanceRaw {
	pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		use std::mem::size_of;

		wgpu::VertexBufferLayout {
			array_stride: size_of::<InstanceRaw>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Instance,
			attributes: &[
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 5,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
					shader_location: 6,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
					shader_location: 7,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: size_of::<[f32; 12]>() as wgpu::BufferAddress,
					shader_location: 8,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: size_of::<[f32; 16]>() as wgpu::BufferAddress,
					shader_location: 9,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: size_of::<[f32; 19]>() as wgpu::BufferAddress,
					shader_location: 10,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: size_of::<[f32; 22]>() as wgpu::BufferAddress,
					shader_location: 11,
					format: wgpu::VertexFormat::Float32x3,
				},
			],
		}
	}
}
