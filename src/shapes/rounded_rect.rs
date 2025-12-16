use vello::kurbo::{self, Shape};

pub struct RoundedRect(kurbo::RoundedRect);

impl RoundedRect {
	pub fn new(x: impl Into<f64>, y: impl Into<f64>, width: impl Into<f64>, height: impl Into<f64>, radius: impl Into<f64>) -> Self {
		let x = x.into(); let y = y.into();
		return Self(kurbo::RoundedRect::new(x, y, x + width.into(), y + height.into(), radius.into()));
	} // end fn new
	pub fn abs(x0: impl Into<f64>, y0: impl Into<f64>, x1: impl Into<f64>, y1: impl Into<f64>, radii: impl Into<f64>) -> Self {
		return Self(kurbo::RoundedRect::new(x0.into(), y0.into(), x1.into(), y1.into(), radii.into()))
	} // end fn abs
} // end impl RoundedRect

impl Shape for RoundedRect {
	type PathElementsIter<'iter> = <kurbo::RoundedRect as Shape>::PathElementsIter<'iter> where Self: 'iter;
	fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> { self.0.path_elements(tolerance) }
	fn area(&self) -> f64 { self.0.area() }
	fn perimeter(&self, accuracy: f64) -> f64 { self.0.perimeter(accuracy) }
	fn winding(&self, pt: kurbo::Point) -> i32 { self.0.winding(pt) }
	fn bounding_box(&self) -> kurbo::Rect { self.0.bounding_box() }
} // end impl Shape for Rect

