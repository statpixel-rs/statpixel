use image::GenericImageView;

use crate::error;

pub struct Texture {
	pub texture: wgpu::Texture,
	pub view: wgpu::TextureView,
	pub sampler: wgpu::Sampler,
}

impl Texture {
	pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

	pub fn new_depth_texture(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		label: &str,
	) -> Self {
		let size = wgpu::Extent3d {
			width: config.width,
			height: config.height,
			depth_or_array_layers: 1,
		};

		let desc = wgpu::TextureDescriptor {
			label: Some(label),
			size,
			mip_level_count: 1,
			sample_count: 4,
			dimension: wgpu::TextureDimension::D2,
			format: Self::DEPTH_FORMAT,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[Self::DEPTH_FORMAT],
		};

		let texture = device.create_texture(&desc);
		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Linear,
			mipmap_filter: wgpu::FilterMode::Nearest,
			compare: Some(wgpu::CompareFunction::LessEqual),
			..Default::default()
		});

		Self {
			texture,
			view,
			sampler,
		}
	}

	pub fn try_from_bytes(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		bytes: &[u8],
	) -> error::Result<Self> {
		Self::try_from_image(device, queue, &image::load_from_memory(bytes)?)
	}

	pub fn try_from_image(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		image: &image::DynamicImage,
	) -> error::Result<Self> {
		let dimensions = image.dimensions();
		// TODO: remove this clone
		let rgba = image.to_rgba8();

		let size = wgpu::Extent3d {
			width: dimensions.0,
			height: dimensions.1,
			depth_or_array_layers: 1,
		};

		let texture = device.create_texture(&wgpu::TextureDescriptor {
			label: None,
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		});

		queue.write_texture(
			wgpu::ImageCopyTexture {
				aspect: wgpu::TextureAspect::All,
				texture: &texture,
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
			},
			&rgba,
			wgpu::ImageDataLayout {
				offset: 0,
				bytes_per_row: Some(4 * dimensions.0),
				rows_per_image: Some(dimensions.1),
			},
			size,
		);

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Nearest,
			min_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		});

		Ok(Self {
			texture,
			view,
			sampler,
		})
	}
}
