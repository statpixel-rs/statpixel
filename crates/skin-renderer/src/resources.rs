use crate::error::{SkinRendererError, SkinRendererResult};
use crate::model::{Material, Mesh, Model, ModelVertex};
use crate::texture::Texture;
use bytemuck::cast_slice;
use std::fmt::Debug;
use std::io::{BufReader, Cursor};
use std::path::Path;
use wgpu::util::DeviceExt;

pub async fn load_string(file_name: impl AsRef<Path>) -> SkinRendererResult<String> {
	let path = std::path::Path::new(env!("OUT_DIR"))
		.join("models")
		.join(file_name);

	let txt = std::fs::read_to_string(path)?;

	Ok(txt)
}

pub async fn load_binary(file_name: &str) -> SkinRendererResult<Vec<u8>> {
	let path = std::path::Path::new(env!("OUT_DIR"))
		.join("models")
		.join(file_name);

	std::fs::read(path).map_err(|e| e.into())
}

pub async fn load_texture(
	file_name: &str,
	device: &wgpu::Device,
	queue: &wgpu::Queue,
) -> SkinRendererResult<Texture> {
	let data = load_binary(file_name).await?;
	Texture::from_bytes(device, queue, &data)
}

pub async fn load_model(
	file_name: impl AsRef<Path> + Debug,
	device: &wgpu::Device,
	queue: &wgpu::Queue,
	layout: &wgpu::BindGroupLayout,
) -> SkinRendererResult<Model> {
	let file_name = file_name.as_ref();

	let obj_text = load_string(file_name).await?;
	let obj_cursor = Cursor::new(obj_text);
	let mut obj_reader = BufReader::new(obj_cursor);

	let (models, obj_materials) = tobj::load_obj_buf_async(
		&mut obj_reader,
		&tobj::LoadOptions {
			triangulate: true,
			single_index: true,
			..Default::default()
		},
		|p| async move {
			let mat_text = load_string(&p).await.expect("Failed to load material");
			tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
		},
	)
	.await?;

	let mut materials = Vec::new();

	for m in obj_materials? {
		let diffuse_texture = m
			.diffuse_texture
			.ok_or(SkinRendererError::MissingDiffuseTexture)?;

		let diffuse_texture = load_texture(&diffuse_texture, device, queue).await?;
		let material = Material::new(device, diffuse_texture, layout);
		materials.push(material);
	}

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
					tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
					normal: [
						m.mesh.normals[i * 3],
						m.mesh.normals[i * 3 + 1],
						m.mesh.normals[i * 3 + 2],
					],
				})
				.collect::<Vec<_>>();

			let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some(&format!("{:?} Vertex Buffer", file_name)),
				contents: cast_slice(&vertices),
				usage: wgpu::BufferUsages::VERTEX,
			});

			let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
				label: Some(&format!("{:?} Index Buffer", file_name)),
				contents: cast_slice(&m.mesh.indices),
				usage: wgpu::BufferUsages::INDEX,
			});

			Mesh {
				name: m.name,
				vertex_buffer,
				index_buffer,
				num_elements: m.mesh.indices.len() as u32,
				material: m.mesh.material_id.unwrap_or(0),
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
