use crate::error;
use image::GenericImageView;

pub struct Texture {
	pub texture: wgpu::Texture,
	pub view: wgpu::TextureView,
	pub sampler: wgpu::Sampler,
}

impl Texture {
	pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
}

impl Texture {
	pub fn create_depth_texture(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		label: &str,
	) -> Self {
		let size = Self::get_depth_texture_size(config);
		let desc = Self::get_depth_texture_descriptor(size, label);
		let texture = device.create_texture(&desc);

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = Self::create_depth_texture_sampler(device);

		Self {
			texture,
			view,
			sampler,
		}
	}

	fn get_depth_texture_size(config: &wgpu::SurfaceConfiguration) -> wgpu::Extent3d {
		wgpu::Extent3d {
			width: config.width,
			height: config.height,
			depth_or_array_layers: 1,
		}
	}

	fn get_depth_texture_descriptor(size: wgpu::Extent3d, label: &str) -> wgpu::TextureDescriptor {
		wgpu::TextureDescriptor {
			label: Some(label),
			size,
			mip_level_count: 1,
			sample_count: 4,
			dimension: wgpu::TextureDimension::D2,
			format: Self::DEPTH_FORMAT,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[Self::DEPTH_FORMAT],
		}
	}

	fn create_depth_texture_sampler(device: &wgpu::Device) -> wgpu::Sampler {
		device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Linear,
			mipmap_filter: wgpu::FilterMode::Nearest,
			compare: Some(wgpu::CompareFunction::LessEqual),
			..Default::default()
		})
	}
}

impl Texture {
	pub fn try_from_bytes(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		bytes: &[u8],
	) -> error::Result<Self> {
		let image = image::load_from_memory(bytes)?;
		Self::try_from_image(device, queue, &image)
	}

	pub fn try_from_image(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		image: &image::DynamicImage,
	) -> error::Result<Self> {
		let rgba = image.to_rgba8();
		let dimensions = image.dimensions();
		let size = Self::get_image_texture_size(dimensions);
		let texture_desc = Self::get_image_texture_descriptor(size);

		let texture = device.create_texture(&texture_desc);
		Self::write_image_to_texture(queue, &texture, &rgba, dimensions, size);

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = Self::create_image_texture_sampler(device);

		Ok(Self {
			texture,
			view,
			sampler,
		})
	}

	fn get_image_texture_size(dimensions: (u32, u32)) -> wgpu::Extent3d {
		wgpu::Extent3d {
			width: dimensions.0,
			height: dimensions.1,
			depth_or_array_layers: 1,
		}
	}

	fn get_image_texture_descriptor<'a>(size: wgpu::Extent3d) -> wgpu::TextureDescriptor<'a> {
		wgpu::TextureDescriptor {
			label: None,
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		}
	}

	fn write_image_to_texture(
		queue: &wgpu::Queue,
		texture: &wgpu::Texture,
		rgba: &[u8],
		dimensions: (u32, u32),
		size: wgpu::Extent3d,
	) {
		queue.write_texture(
			wgpu::ImageCopyTexture {
				aspect: wgpu::TextureAspect::All,
				texture,
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
			},
			rgba,
			wgpu::ImageDataLayout {
				offset: 0,
				bytes_per_row: Some(4 * dimensions.0),
				rows_per_image: Some(dimensions.1),
			},
			size,
		);
	}

	fn create_image_texture_sampler(device: &wgpu::Device) -> wgpu::Sampler {
		device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Nearest,
			min_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		})
	}
}
