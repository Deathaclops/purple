use std::sync::Arc;

use vello::peniko::{Blob, Image, ImageFormat};
use vello::wgpu;
use vello::wgpu::util::{DeviceExt, TextureDataOrder};

use crate::gpu::Gpu;


pub struct Texture {
	pub image: Image,
	pub width: u32,
	pub height: u32,
	pub texture: wgpu::Texture,
}

impl Texture {
	pub fn empty(width: u32, height: u32, gpu: &Gpu) -> Self {
		let image = Image::new(Blob::new(Arc::new(vec![0; (width * height * 4) as usize])), ImageFormat::Rgba8, width, height);
		let texture = gpu.device.create_texture_with_data(&gpu.queue, &wgpu::TextureDescriptor {
			label: None,
			size: wgpu::Extent3d {
				width,
				height,
				depth_or_array_layers: 1,
			},
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
			mip_level_count: 1,
			usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
			dimension: wgpu::TextureDimension::D2,
			sample_count: 1,
			view_formats: &[],
		}, TextureDataOrder::MipMajor, &[]);
		Self {
			image,
			width,
			height,
			texture,
		}
	} // end fn empty
} // end impl Texture
