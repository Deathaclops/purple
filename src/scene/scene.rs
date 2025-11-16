
use vello::peniko::ImageBrush;

use crate::prelude::*;

pub struct Scene(pub vello::Scene);

impl Scene {
	pub fn new() -> Self { return Self(vello::Scene::new()); }
	pub fn clear(&mut self) { self.0.reset(); }
	pub fn fill(&mut self, color: impl Into<Color>, transform: vello::kurbo::Affine, shape: &impl vello::kurbo::Shape) {
		self.0.fill(vello::peniko::Fill::NonZero, transform, color.into(), None, shape);
	} // end fn fill
	pub fn stroke(&mut self, stroke: &Stroke, color: impl Into<Color>, transform: vello::kurbo::Affine, shape: &impl vello::kurbo::Shape) {
		self.0.stroke(stroke, transform, color.into(), None, shape);
	} // end fn stroke
	pub fn draw_image(&mut self, image: &Image, transform: vello::kurbo::Affine) {
		self.0.draw_image(&ImageBrush::new(image.image.clone()), transform);
	} // end fn draw_image
	pub fn draw_scene(&mut self, scene: &Scene, transform: vello::kurbo::Affine) {
		self.0.append(&scene.0, Some(transform));
	} // end fn draw_scene
	pub fn draw_text(&mut self, text: impl Into<String>, transform: vello::kurbo::Affine) {
		
	} // end fn draw_text
} // end impl Scene
