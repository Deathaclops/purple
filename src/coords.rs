
pub trait Vec2<T> {
	fn x(&self) -> T;
	fn y(&self) -> T;
	fn set_x(&mut self, x: T);
	fn set_y(&mut self, y: T);
}

impl Vec2<f64> for (f64, f64) {
	fn x(&self) -> f64 { self.0 }
	fn y(&self) -> f64 { self.1 }
	fn set_x(&mut self, x: f64) { self.0 = x; }
	fn set_y(&mut self, y: f64) { self.1 = y; }
} // end impl Vec2<f64> for (f64, f64)

impl Vec2<f32> for (f32, f32) {
	fn x(&self) -> f32 { self.0 }
	fn y(&self) -> f32 { self.1 }
	fn set_x(&mut self, x: f32) { self.0 = x; }
	fn set_y(&mut self, y: f32) { self.1 = y; }
} // end impl Vec2<f32> for (f32, f32)

impl Vec2<i32> for (i32, i32) {
	fn x(&self) -> i32 { self.0 }
	fn y(&self) -> i32 { self.1 }
	fn set_x(&mut self, x: i32) { self.0 = x; }
	fn set_y(&mut self, y: i32) { self.1 = y; }
} // end impl Vec2<i32> for (i32, i32)

impl Vec2<u32> for (u32, u32) {
	fn x(&self) -> u32 { self.0 }
	fn y(&self) -> u32 { self.1 }
	fn set_x(&mut self, x: u32) { self.0 = x; }
	fn set_y(&mut self, y: u32) { self.1 = y; }
} // end impl Vec2<u32> for (u32, u32)

impl Vec2<u64> for (u64, u64) {
	fn x(&self) -> u64 { self.0 }
	fn y(&self) -> u64 { self.1 }
	fn set_x(&mut self, x: u64) { self.0 = x; }
	fn set_y(&mut self, y: u64) { self.1 = y; }
} // end impl Vec2<u64> for (u64, u64)

impl Vec2<i64> for (i64, i64) {
	fn x(&self) -> i64 { self.0 }
	fn y(&self) -> i64 { self.1 }
	fn set_x(&mut self, x: i64) { self.0 = x; }
	fn set_y(&mut self, y: i64) { self.1 = y; }
} // end impl Vec2<i64> for (i64, i64)


