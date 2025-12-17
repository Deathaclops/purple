
use vello::peniko::ImageBrush;
use vello::kurbo::Stroke;

use crate::prelude::*;

pub struct Scene(pub vello::Scene);

impl Scene {
	pub fn new() -> Self { return Self(vello::Scene::new()); }
	pub fn clear(&mut self) { self.0.reset(); }
	pub fn fill(&mut self, color: impl Into<Color>, transform: Affine, shape: &impl vello::kurbo::Shape) {
		self.0.fill(vello::peniko::Fill::NonZero, transform.0, color.into(), None, shape);
	} // end fn fill
	pub fn stroke(&mut self, stroke_width: impl Into<f64>, color: impl Into<Color>, transform: Affine, shape: &impl vello::kurbo::Shape) {
		self.0.stroke(&Stroke::new(stroke_width.into()), transform.0, color.into(), None, shape);
	} // end fn stroke
	pub fn draw_image(&mut self, image: &Image, transform: Affine) {
		self.0.draw_image(&ImageBrush::new(image.image.clone()), transform.0);
	} // end fn draw_image
	pub fn draw_scene(&mut self, scene: &Scene, transform: Affine) {
		self.0.append(&scene.0, Some(transform.0));
	} // end fn draw_scene
	pub fn draw_text(&mut self, text: impl Into<String>, transform: Affine, font_ctx: &mut FontContext, options: TextOptions) -> BoundingBox {
		return crate::scene::text::draw_text(self, text.into(), transform, font_ctx, options);
	} // end fn draw_text
} // end impl Scene
