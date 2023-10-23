use crate::{
	error,
	model::{Material, Mesh, Model, ModelVertex},
	texture::Texture,
};

use bytemuck::cast_slice;
use std::io::{BufReader, Cursor};
use wgpu::util::DeviceExt;

pub fn load_model(
	bytes: &[u8],
	device: &wgpu::Device,
	queue: &wgpu::Queue,
	layout: &wgpu::BindGroupLayout,
) -> error::Result<Model> {
	let mut reader = BufReader::new(Cursor::new(bytes));

	let (models, materials) = tobj::load_obj_buf(
		&mut reader,
		&tobj::LoadOptions {
			triangulate: true,
			single_index: true,
			..Default::default()
		},
		|p| {
			tobj::load_mtl_buf(&mut match p.file_name().and_then(|f| f.to_str()) {
				Some("classic.mtl") => include_bytes!("../models/classic.mtl").as_slice(),
				Some("slim.mtl") => include_bytes!("../models/slim.mtl").as_slice(),
				o => unreachable!("unknown material file: {o:?}"),
			})
		},
	)?;

	let materials = materials?
		.into_iter()
		.map(|m| {
			let texture = m
				.diffuse_texture
				.ok_or(error::Error::MissingDiffuseTexture)?;

			let texture = Texture::try_from_bytes(
				device,
				queue,
				match texture.as_str() {
					"alex.png" => include_bytes!("../models/alex.png").as_slice(),
					"steve.png" => include_bytes!("../models/steve.png").as_slice(),
					o => unreachable!("unknown texture: {o:?}"),
				},
			)?;
			let material = Material::new(device, texture, layout);

			Ok(material)
		})
		.collect::<error::Result<Vec<_>>>()?;

	let mut meshes = models
		.into_iter()
		.filter(|m| !m.mesh.texcoords.is_empty())
		.map(|m| {
			let vertices = (0..m.mesh.positions.len() / 3)
				.map(|i| ModelVertex {
					position: [
						m.mesh.positions[i * 3],
						m.mesh.positions[i * 3 + 1],
						m.mesh.positions[i * 3 + 2],
					],
					coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
					normal: [
						m.mesh.normals[i * 3],
						m.mesh.normals[i * 3 + 1],
						m.mesh.normals[i * 3 + 2],
					],
				})
				.collect::<Vec<_>>();

			let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: cast_slice(&vertices),
				usage: wgpu::BufferUsages::VERTEX,
			});

			let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some("Index Buffer"),
				contents: cast_slice(&m.mesh.indices),
				usage: wgpu::BufferUsages::INDEX,
			});

			Mesh {
				name: m.name,
				vertex_buffer,
				index_buffer,
				num_elements: m.mesh.indices.len() as u32,
				material: m.mesh.material_id.unwrap_or_default(),
			}
		})
		.collect::<Vec<_>>();

	// The meshes are in the following order:
	// ["Head", "Body", "RArm", "LArm", "LLeg", "RLeg", "HeadOut", "BodyOut", "RArmOut", "LArmOut", "LLegOut", "RLegOut"]

	meshes.swap(0, 5); // Head <-> RLeg
	meshes.swap(1, 4); // Body <-> LLeg
	meshes.swap(2, 3); // RArm <-> LArm
	meshes.swap(6, 11); // HeadOut <-> RLegOut
	meshes.swap(7, 10); // BodyOut <-> LLegOut
	meshes.swap(8, 9); // RArmOut <-> LArmOut

	Ok(Model { meshes, materials })
}
