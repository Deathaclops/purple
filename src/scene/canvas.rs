//use futures::channel::oneshot;

use vello::wgpu::{self, TexelCopyTextureInfoBase};
use crate::prelude::*;

pub struct Canvas {
	pub image: Image,
	pub data: Vec<u8>,
	pub buffer: Option<wgpu::Buffer>,
	pub texture: wgpu::Texture,
	pub size: (u32, u32),
} // end struct Canvas

impl Canvas {
	pub fn new(size: impl Into<Dimensions>, gpu: &mut Gpu) -> Self {
		let size = size.into();
		let width = size.width as u32;
		let height = size.height as u32;
		let data = vec![0; (width * height * 4) as usize];
		let image = Image::new_raw(data.clone(), width, height);
		let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            label: Some("CPU-based Canvas"),
            view_formats: &[],
        });
		gpu.renderer.override_image(&image.image, Some(TexelCopyTextureInfoBase {
			texture: texture.clone(),
			origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
			mip_level: 0,
			aspect: wgpu::TextureAspect::All,
		}));
		return Self {
			image,
			data,
			buffer: None,
			texture,
			size: (width, height),
		};
	} // end fn new

	pub fn upload(&self, gpu: &Gpu) {
		gpu.queue.write_texture(
			self.texture.as_image_copy(),
			&self.data,
			wgpu::TexelCopyBufferLayout {
				offset: 0,
				bytes_per_row: Some(4 * self.size.0),
				rows_per_image: Some(self.size.1),
			}, // end TexelCopyBufferLayout
			wgpu::Extent3d {
				width: self.size.0,
				height: self.size.1,
				depth_or_array_layers: 1,
			}, // end Extent3d
		); // end write_texture
	} // end fn upload

	pub async fn download(&mut self, gpu: &Gpu) {
		if self.buffer.is_none() {
			let buffer_size = (self.size.0 * self.size.1 * 4) as usize;
			let staging_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
				label: Some("CPU Canvas Download Buffer"),
				size: buffer_size as u64,
				usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
				mapped_at_creation: false,
			}); // end let staging_buffer
			self.buffer = Some(staging_buffer);
		} // end if buffer is none
		let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("CPU Canvas Download Encoder") } );
		encoder.copy_texture_to_buffer(
		    self.texture.as_image_copy(),
		    wgpu::TexelCopyBufferInfo {
		        buffer: &self.buffer.as_ref().unwrap(),
		        layout: wgpu::TexelCopyBufferLayout {
		            offset: 0,
		            bytes_per_row: Some(4 * self.size.0),
		            rows_per_image: Some(self.size.1),
		        }, // end TexelCopyBufferLayout
		    }, wgpu::Extent3d { width: self.size.0, height: self.size.1, depth_or_array_layers: 1 },
		); // end copy_texture_to_buffer
		gpu.queue.submit(Some(encoder.finish()));
		let slice = self.buffer.as_ref().unwrap().slice(..);
		let (sender, receiver) = futures::channel::oneshot::channel();
		slice.map_async(wgpu::MapMode::Read, move |res| { sender.send(res).unwrap(); });
		gpu.device.poll(wgpu::PollType::Wait);
		match receiver.await { Ok(Ok(())) => { self.data = slice.get_mapped_range().to_vec(); } _ => {} }
	} // end fn download
} // end impl Canvas
