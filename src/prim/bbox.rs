use crate::prim::{Point, Dimensions};

#[derive(Clone)]
pub struct BoundingBox {
	pub pos: Point,
	pub dim: Dimensions,
} // end struct BoundingBox

impl From<(f64, f64, f64, f64)> for BoundingBox {
	fn from(tup: (f64, f64, f64, f64)) -> Self {
		Self { pos: Point { x: tup.0, y: tup.1 }, dim: Dimensions { width: tup.2, height: tup.3 } }
	} // end fn from
} // end impl From<(f64, f64, f64, f64)> for BoundingBox

impl From<(f32, f32, f32, f32)> for BoundingBox {
	fn from(tup: (f32, f32, f32, f32)) -> Self {
		Self { pos: Point { x: tup.0 as f64, y: tup.1 as f64 }, dim: Dimensions { width: tup.2 as f64, height: tup.3 as f64 } }
	} // end fn from
} // end impl From<(f32, f32, f32, f32)> for BoundingBox

