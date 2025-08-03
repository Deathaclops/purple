use crate::coords::Vec2;
use vello::peniko::{Blob, Image, ImageFormat};
use std::sync::Arc;

pub fn new_image(size: (u32, u32), rgba: [u8; 4]) -> Image {
    let pixel_count = (size.x() * size.y()) as usize;
    let mut data = Vec::with_capacity(pixel_count * 4);
    for _ in 0..pixel_count { data.extend_from_slice(&rgba); }
    return Image::new(
		Blob::new(Arc::new(data)),
		ImageFormat::Rgba8,
        size.x(),
        size.y(),
    ); // end Image::new
} // end fn new_image

