use crate::prim::{Point, Dimensions};

#[derive(Clone, Debug)]
pub struct BoundingBox {
	pub x: f64,
	pub y: f64,
	pub width: f64,
	pub height: f64,
} // end struct BoundingBox

impl BoundingBox {
	pub fn new(x: impl Into<f64>, y: impl Into<f64>, width: impl Into<f64>, height: impl Into<f64>) -> Self {
		Self { x: x.into(), y: y.into(), width: width.into(), height: height.into() }
	} // end fn new
	pub fn point(&self) -> Point {
		return Point { x: self.x, y: self.y };
	} // end fn point
	pub fn dimensions(&self) -> Dimensions {
		return Dimensions { width: self.width, height: self.height };
	} // end fn dimensions
} // end impl BoundingBox

impl From<(f64, f64, f64, f64)> for BoundingBox {
	fn from(tup: (f64, f64, f64, f64)) -> Self {
		Self { x: tup.0, y: tup.1, width: tup.2, height: tup.3 }
	} // end fn from
} // end impl From<(f64, f64, f64, f64)> for BoundingBox

impl From<(f32, f32, f32, f32)> for BoundingBox {
	fn from(tup: (f32, f32, f32, f32)) -> Self {
		Self { x: tup.0 as f64, y: tup.1 as f64, width: tup.2 as f64, height: tup.3 as f64 }
	} // end fn from
} // end impl From<(f32, f32, f32, f32)> for BoundingBox

