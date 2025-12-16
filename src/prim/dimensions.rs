use winit::dpi::PhysicalSize;

// Dimensions struct represents a 2D dimension
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
	pub width: f64,
	pub height: f64,
} // end struct Dimensions

impl Dimensions {
	pub fn new(width: f64, height: f64) -> Self {
		return Self { width, height };
	} // end fn new
	pub fn zero() -> Self {
		return Self { width: 0.0, height: 0.0 };
	} // end fn zero
	pub fn is_zero(&self) -> bool {
		return self.width == 0.0 && self.height == 0.0;
	} // end fn is_zero
} // end impl Dimensions

// Conversions between types

impl From<(f32, f32)> for Dimensions {
	fn from(dim: (f32, f32)) -> Self {
		return Dimensions { width: dim.0 as f64, height: dim.1 as f64 };
	} // end From (f32, f32)
} // end impl From (f32, f32) for Dimensions

impl From<(f64, f64)> for Dimensions {
	fn from(dim: (f64, f64)) -> Self {
		return Dimensions { width: dim.0, height: dim.1 };
	} // end From (f64, f64)
} // end impl From (f64, f64) for Dimensions

impl From<(i32, i32)> for Dimensions {
	fn from(dim: (i32, i32)) -> Self {
		return Dimensions { width: dim.0 as f64, height: dim.1 as f64 };
	} // end From (i32, i32)
} // end impl From (i32, i32) for Dimensions

impl From<(u32, u32)> for Dimensions {
	fn from(dim: (u32, u32)) -> Self {
		return Dimensions { width: dim.0 as f64, height: dim.1 as f64 };
	} // end From (u32, u32)
} // end impl From (u32, u32) for Dimensions

impl From<(i64, i64)> for Dimensions {
	fn from(dim: (i64, i64)) -> Self {
		return Dimensions { width: dim.0 as f64, height: dim.1 as f64 };
	} // end From (i64, i64)
} // end impl From (i64, i64) for Dimensions

impl From<(u64, u64)> for Dimensions {
	fn from(dim: (u64, u64)) -> Self {
		return Dimensions { width: dim.0 as f64, height: dim.1 as f64 };
	} // end From (u64, u64)
} // end impl From (u64, u64) for Dimensions

impl From<(usize, usize)> for Dimensions {
	fn from(dim: (usize, usize)) -> Self {
		return Dimensions { width: dim.0 as f64, height: dim.1 as f64 };
	} // end From (usize, usize)
} // end impl From (usize, usize) for Dimensions

impl From<PhysicalSize<u32>> for Dimensions {
	fn from(size: PhysicalSize<u32>) -> Self {
		return Dimensions { width: size.width as f64, height: size.height as f64 };
	} // end From PhysicalSize<u32>
} // end impl From PhysicalSize<u32> for Dimensions

impl From<Dimensions> for PhysicalSize<u32> {
	fn from(dim: Dimensions) -> Self {
		return PhysicalSize::new(dim.width as u32, dim.height as u32);
	} // end From Dimensions
} // end impl From Dimensions for PhysicalSize<u32>

impl From<Dimensions> for (u32, u32) {
	fn from(dim: Dimensions) -> Self {
		return (dim.width as u32, dim.height as u32);
	} // end From Dimensions
} // end impl From Dimensions for (u32, u32)