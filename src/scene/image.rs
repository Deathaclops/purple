use std::sync::Arc;
use vello::peniko::{self, Blob, ImageFormat};
use crate::prim::Dimensions;

#[derive(Clone)]
pub struct Image {
	pub image: peniko::ImageData,
} // end struct Image

impl Image {
	pub fn new(rgba_bytes: Vec<u8>, width: u32, height: u32) -> Self {
		let image: peniko::ImageData = peniko::ImageData {
			width,
			height,
			data: Blob::new(Arc::new(rgba_bytes)),
			format: ImageFormat::Rgba8,
			alpha_type: peniko::ImageAlphaType::Alpha,
		}; return Self { image };
	} // end fn new_raw
	pub fn load(bytes: Vec<u8>) -> Self {
		let data = image::load_from_memory(&bytes).unwrap();
		let image_buf = data.into_rgba8(); // owned ImageBuffer<Rgba<u8>, Vec<u8>>
		let (width, height) = (image_buf.width(), image_buf.height());
		let raw_pixels: Vec<u8> = image_buf.into_raw(); // Vec<u8> we can own
		let image: peniko::ImageData = peniko::ImageData {
			width,
			height,
			data: Blob::new(Arc::new(raw_pixels.clone())),
			format: ImageFormat::Rgba8,
			alpha_type: peniko::ImageAlphaType::Alpha,
		}; return Self { image };
	} // end fn new
	pub fn get_bytes(&self) -> &[u8] {
		self.image.data.data()
	} // end fn get_bytes
	pub fn size(&self) -> Dimensions {
		Dimensions { width: self.image.width as f64, height: self.image.height as f64 }
	} // end fn size
} // end impl Image