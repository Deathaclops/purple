use vello::kurbo::Point;

pub struct Vec2 {
	pub x: f64,
	pub y: f64,
} // end struct Vec2

impl From<(f32, f32)> for Vec2 { fn from(coords: (f32, f32)) -> Self { return Vec2 { x: coords.0 as f64, y: coords.1 as f64 }; } }
impl From<(f64, f64)> for Vec2 { fn from(coords: (f64, f64)) -> Self { return Vec2 { x: coords.0, y: coords.1 }; } }
impl From<(i32, i32)> for Vec2 { fn from(coords: (i32, i32)) -> Self { return Vec2 { x: coords.0 as f64, y: coords.1 as f64 }; } }
impl From<(i64, i64)> for Vec2 { fn from(coords: (i64, i64)) -> Self { return Vec2 { x: coords.0 as f64, y: coords.1 as f64 }; } }

impl From<Point> for Vec2 { fn from(point: Point) -> Self { return Vec2 { x: point.x as f64, y: point.y as f64 }; } }

