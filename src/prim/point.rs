

// Point struct represents a 2D point in space

use vello::kurbo::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
	pub x: f64,
	pub y: f64,
} // end struct Point

// Conversions between types

impl From<(f32, f32)> for Point { fn from(coords: (f32, f32)) -> Self { return Point { x: coords.0 as f64, y: coords.1 as f64 }; } }
impl From<(f64, f64)> for Point { fn from(coords: (f64, f64)) -> Self { return Point { x: coords.0, y: coords.1 }; } }
impl From<(i32, i32)> for Point { fn from(coords: (i32, i32)) -> Self { return Point { x: coords.0 as f64, y: coords.1 as f64 }; } }
impl From<(i64, i64)> for Point { fn from(coords: (i64, i64)) -> Self { return Point { x: coords.0 as f64, y: coords.1 as f64 }; } }

impl From<Vec2> for Point { fn from(vec: Vec2) -> Self { return Point { x: vec.x, y: vec.y }; } }
