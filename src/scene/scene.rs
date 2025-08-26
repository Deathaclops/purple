use vello::peniko::color::AlphaColor;

use crate::{font::{BUFFER, FONT_SYSTEM}, prelude::*};

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
		self.0.draw_image(&image.image, transform);
	} // end fn draw_image
	pub fn draw_scene(&mut self, scene: &Scene, transform: vello::kurbo::Affine) {
		self.0.append(&scene.0, Some(transform));
	} // end fn draw_scene
	pub fn draw_text(&mut self, text: impl Into<String>, transform: vello::kurbo::Affine, font: &crate::font::Font, formatting: crate::font::TextFormatting) {
		let s = text.into();

		let mut binding = BUFFER.lock().unwrap();
		let mut binding_2 = &mut *FONT_SYSTEM.lock().unwrap();
		let mut buffer = binding.borrow_with(binding_2);

		buffer.set_size(Some(formatting.size), Some(formatting.size * 2.0));

		let attr = cosmic_text::Attrs::new().family(cosmic_text::Family::Name(&font.name));

		buffer.set_text(s.as_str(), &attr, cosmic_text::Shaping::Advanced);

		let mut glyphs = Vec::new();
		for run in buffer.layout_runs() {
			let mut charpos = 0.0;
			for glyph in run.glyphs {
				let g = vello::Glyph {
					id: glyph.glyph_id as u32,
					x: charpos + glyph.x,
					y: glyph.y,
				};
				charpos += glyph.w * formatting.size;
				glyphs.push(g);
			}
		}
		self.0.draw_glyphs(&font.font)
			.transform(transform)
			.font_size(formatting.size)
			.brush(AlphaColor::<vello::peniko::color::Srgb>::BLACK)
			.draw(
				&vello::peniko::Style::Fill(vello::peniko::Fill::NonZero),
				glyphs.iter().cloned(),
			);
	
	} // end fn draw_text
} // end impl Scene

