use vello::kurbo;
use crate::prim::Point;

pub const AFFINE: Affine = Affine::new();

#[derive(Clone, Copy, Debug)]
pub struct Affine(pub vello::kurbo::Affine);

impl Affine {
	pub const fn new() -> Self {
		return Self(vello::kurbo::Affine::IDENTITY);
	} // end fn new
	pub fn translate(&self, x: impl Into<f64>, y: impl Into<f64>) -> Self {
		return Self(self.0.then_translate((x.into(), y.into()).into()));
	} // end fn translate
	pub fn translate_abs(&self, x: impl Into<f64>, y: impl Into<f64>) -> Self {
		let (curr_x, curr_y) = self.translation();
		return Self(
			self.0
				.then_translate((x.into() - curr_x, y.into() - curr_y).into())
		);
	} // end fn translate_abs
	pub fn rotate(&self, rel_rad: impl Into<f64>) -> Self {
		return Self(self.0.pre_rotate(rel_rad.into()));
	} // end fn rotate
	pub fn rotate_abs(&self, abs_rad: impl Into<f64>) -> Self {
		let angle = abs_rad.into();
		let current_angle = self.rotation();
		return Self(
			self.0
				.pre_rotate(angle - current_angle)
		);
	} // end fn rotate_abs
	pub fn rotate_at(&self, rel_rad: impl Into<f64>, point: impl Into<Point>) -> Self {
		let angle = rel_rad.into();
		let pivot_point = point.into();
		let pivot = kurbo::Vec2::new(pivot_point.x, pivot_point.y);
		return Self(
			self.0
				.pre_translate(pivot)
				.pre_rotate(angle)
				.pre_translate(-pivot)
		);
	} // end fn rotate_at
	pub fn rotate_at_abs(&self, abs_rad: impl Into<f64>, point: impl Into<Point>) -> Self {
		let angle = abs_rad.into();
		let current_angle = self.rotation();
		let pivot_point = point.into();
		let pivot = kurbo::Vec2::new(pivot_point.x, pivot_point.y);
		return Self(
			self.0
				.pre_translate(pivot)
				.pre_rotate(angle - current_angle)
				.pre_translate(-pivot)
		);
	} // end fn rotate_at_abs
	pub fn scale(&self, s: impl Into<f64>) -> Self {
		return Self(self.0.pre_scale(s.into()).then_translate(self.0.translation()));
	} // end fn scale
	pub fn translation(&self) -> (f64, f64) {
		let t = self.0.as_coeffs();
		return (t[4], t[5]);
	} // end fn translation
	pub fn rotation(&self) -> f64 {
		let [a, b, _c, _d, _e, _f] = self.0.as_coeffs();
		return b.atan2(a);
	} // end fn rotation
} // end impl Affine

