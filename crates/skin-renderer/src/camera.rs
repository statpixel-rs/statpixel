//! Partially derived from <https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera>

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

#[derive(Debug)]
pub struct Camera {
	eye: Vec3,
	target: Vec3,
	up: Vec3,
	aspect_ratio: f32,
	fovy: f32,
	znear: f32,
	zfar: f32,
}

impl Camera {
	pub fn new(width: f32, height: f32) -> Self {
		Self {
			eye: Vec3::new(0.0, 50.0, 310.0),
			target: Vec3::ZERO,
			up: Vec3::Y,
			aspect_ratio: width / height,
			fovy: 40.0,
			znear: 0.1,
			zfar: 1000.0,
		}
	}

	pub fn build_view_projection_matrix(&self) -> Mat4 {
		let view = Mat4::look_at_rh(self.eye, self.target, self.up);
		let proj = Mat4::perspective_rh_gl(
			self.fovy.to_radians(),
			self.aspect_ratio,
			self.znear,
			self.zfar,
		);

		proj * view
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CameraUniform {
	view_position: [f32; 4],
	view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
	pub fn new(camera: &Camera) -> Self {
		let vp_matrix = camera.build_view_projection_matrix();
		Self {
			view_position: camera.eye.extend(1.0).into(),
			view_proj: vp_matrix.to_cols_array_2d(),
		}
	}
}
