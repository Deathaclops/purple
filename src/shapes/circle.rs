use vello::kurbo::Shape;

pub struct Circle(pub vello::kurbo::Circle);

impl Circle {
	pub fn new(cx: impl Into<f64>, cy: impl Into<f64>, radius: impl Into<f64>) -> Self {
		return Self(vello::kurbo::Circle::new(vello::kurbo::Point::new(cx.into(), cy.into()), radius.into()));
	} // end fn new
} // end impl Circle

impl Shape for Circle {
	type PathElementsIter<'iter> = <vello::kurbo::Circle as Shape>::PathElementsIter<'iter> where Self: 'iter;
	fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> { self.0.path_elements(tolerance) }
	fn area(&self) -> f64 { self.0.area() }
	fn perimeter(&self, accuracy: f64) -> f64 { self.0.perimeter(accuracy) }
	fn winding(&self, pt: vello::kurbo::Point) -> i32 { self.0.winding(pt) }
	fn bounding_box(&self) -> vello::kurbo::Rect { self.0.bounding_box() }
} // end impl Shape for Circle