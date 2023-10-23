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
	let (models, materials) = load_obj_and_materials(bytes)?;

	let materials = load_materials(materials, device, queue, layout)?;
	let mut meshes = create_meshes(models, device)?;

	reorder_meshes(&mut meshes);

	Ok(Model { meshes, materials })
}

fn load_obj_and_materials(bytes: &[u8]) -> error::Result<(Vec<tobj::Model>, Vec<tobj::Material>)> {
	let mut reader = BufReader::new(Cursor::new(bytes));
	let load_options = tobj::LoadOptions {
		triangulate: true,
		single_index: true,
		..Default::default()
	};

	let (models, materials) = tobj::load_obj_buf(&mut reader, &load_options, |p| {
		tobj::load_mtl_buf(&mut match p.file_name().and_then(|f| f.to_str()) {
			Some("classic.mtl") => include_bytes!("../models/classic.mtl").as_slice(),
			Some("slim.mtl") => include_bytes!("../models/slim.mtl").as_slice(),
			o => unreachable!("unknown material file: {o:?}"),
		})
	})?;

	Ok((models, materials?))
}

fn load_materials(
	materials: Vec<tobj::Material>,
	device: &wgpu::Device,
	queue: &wgpu::Queue,
	layout: &wgpu::BindGroupLayout,
) -> error::Result<Vec<Material>> {
	materials
		.into_iter()
		.map(|material| {
			let texture_name = material
				.diffuse_texture
				.ok_or(error::Error::MissingDiffuseTexture)?;

			let texture_bytes = match texture_name.as_str() {
				"alex.png" => include_bytes!("../models/alex.png").as_slice(),
				"steve.png" => include_bytes!("../models/steve.png").as_slice(),
				o => unreachable!("unknown texture: {o:?}"),
			};

			let texture = Texture::try_from_bytes(device, queue, texture_bytes)?;
			let material = Material::new(device, texture, layout);

			Ok(material)
		})
		.collect()
}

fn create_meshes(models: Vec<tobj::Model>, device: &wgpu::Device) -> error::Result<Vec<Mesh>> {
	models
		.into_iter()
		.filter(|model| !model.mesh.texcoords.is_empty())
		.map(|model| {
			let vertices = model
				.mesh
				.positions
				.chunks_exact(3)
				.zip(model.mesh.texcoords.chunks_exact(2))
				.zip(model.mesh.normals.chunks_exact(3))
				.map(|((position, coords), normal)| ModelVertex {
					position: [position[0], position[1], position[2]],
					coords: [coords[0], 1.0 - coords[1]], // flip the y coordinate
					normal: [normal[0], normal[1], normal[2]],
				})
				.collect::<Vec<_>>();

			let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: cast_slice(&vertices),
				usage: wgpu::BufferUsages::VERTEX,
			});

			let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some("Index Buffer"),
				contents: cast_slice(&model.mesh.indices),
				usage: wgpu::BufferUsages::INDEX,
			});

			Ok(Mesh {
				name: model.name,
				vertex_buffer,
				index_buffer,
				num_elements: model.mesh.indices.len() as u32,
				material: model.mesh.material_id.unwrap_or_default(),
			})
		})
		.collect()
}

fn reorder_meshes(meshes: &mut [Mesh]) {
	// The meshes are in the following order:
	// ["Head", "Body", "RArm", "LArm", "LLeg", "RLeg", "HeadOut", "BodyOut", "RArmOut", "LArmOut", "LLegOut", "RLegOut"]

	// Reorder the meshes to match the desired configuration.
	meshes.swap(0, 5); // Head <-> RLeg
	meshes.swap(1, 4); // Body <-> LLeg
	meshes.swap(2, 3); // RArm <-> LArm
	meshes.swap(6, 11); // HeadOut <-> RLegOut
	meshes.swap(7, 10); // BodyOut <-> LLegOut
	meshes.swap(8, 9); // RArmOut <-> LArmOut
}
