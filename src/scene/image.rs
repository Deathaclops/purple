use std::sync::Arc;

use vello::{peniko::{self, Blob, ImageFormat}, wgpu};

use crate::prim::Dimensions;

pub struct Image {
	pub image: peniko::Image,
	pub texture: Option<wgpu::Texture>,
} // end struct Image

impl Image {
	pub fn new(bytes: Vec<u8>) -> Self {
		let data = image::load_from_memory(&bytes).unwrap();
		let image_buf = data.into_rgba8(); // owned ImageBuffer<Rgba<u8>, Vec<u8>>
		let (width, height) = (image_buf.width(), image_buf.height());
		let raw_pixels: Vec<u8> = image_buf.into_raw(); // Vec<u8> we can own
		let image: peniko::Image = peniko::Image::new(Blob::new(Arc::new(raw_pixels.clone())), ImageFormat::Rgba8, width, height);
		return Self { image, texture: None };
	} // end fn new
	pub fn get_bytes(&self) -> Vec<u8> {
		self.image.data.data().to_vec()
	} // end fn get_bytes
	pub fn new_raw(bytes: Vec<u8>, width: u32, height: u32) -> Self {
		let raw_pixels = bytes;
		let image = peniko::Image::new(Blob::new(Arc::new(raw_pixels)), ImageFormat::Rgba8, width, height);
		return Self { image, texture: None };
	} // end fn new_raw
	pub fn size(&self) -> Dimensions {
		Dimensions { width: self.image.width as f64, height: self.image.height as f64 }
	} // end fn size
} // end impl Image