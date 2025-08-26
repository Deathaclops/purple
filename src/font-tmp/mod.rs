use std::sync::Arc;

use crate::{font, scene::Scene};
use cosmic_text::{fontdb::{Database, Query}, ttf_parser::{glyf, GlyphId}, Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache};
use vello::{kurbo::Affine, peniko::color::AlphaColor, Glyph};

pub struct TextContext {
	pub font_system: FontSystem,
	pub swash_cache: SwashCache,
	pub buffer: Buffer,
	pub fonts: Vec<(cosmic_text::fontdb::ID, vello::peniko::Font)>,
}

impl TextContext {

	pub fn new() -> Self {
		let mut font_system = FontSystem::new();
		*font_system.db_mut() = Database::new();

		let bytes_1 = include_bytes!("../../fonts/Rubik-Regular.ttf").to_vec();
		let font_1 = vello::peniko::Font::new(bytes_1.clone().into(), 0);
		font_system.db_mut().load_font_data(bytes_1);
		let id_1 = font_system.db().query(&Query {
			families: &[Family::Name("Rubik")],
			weight: cosmic_text::Weight::NORMAL,
			stretch: cosmic_text::fontdb::Stretch::Normal,
			style: cosmic_text::Style::Normal,
		}).unwrap();
		let bytes_2 = include_bytes!("../../fonts/NotoSansJP-Regular.ttf").to_vec();
	let font_2 = vello::peniko::Font::new(bytes_2.clone().into(), 0);
		font_system.db_mut().load_font_data(bytes_2);
		let id_2 = font_system.db().query(&Query {
			families: &[Family::Name("Noto Sans JP")],
			weight: cosmic_text::Weight::NORMAL,
			stretch: cosmic_text::fontdb::Stretch::Normal,
			style: cosmic_text::Style::Normal,
		}).unwrap();
		let bytes_3 = include_bytes!("../../fonts/NotoColorEmoji-Regular.ttf").to_vec();
	let font_3 = vello::peniko::Font::new(bytes_3.clone().into(), 0);
		font_system.db_mut().load_font_data(bytes_3);
		let id_3 = font_system.db().query(&Query {
			families: &[Family::Name("Noto Color Emoji")],
			weight: cosmic_text::Weight::NORMAL,
			stretch: cosmic_text::fontdb::Stretch::Normal,
			style: cosmic_text::Style::Normal,
		}).unwrap();
		let query = Query {
			families: &[Family::Name("Rubik"), Family::Name("Noto Sans JP"), Family::Name("Noto Color Emoji")],
			weight: cosmic_text::Weight::NORMAL,
			stretch: cosmic_text::fontdb::Stretch::Normal,
			style: cosmic_text::Style::Normal,
		};
		let mut swash_cache = SwashCache::new();
		let metrics = Metrics::new(14.0, 20.0);
		let mut buffer = Buffer::new(&mut font_system, metrics);
		Self {
			font_system,
			swash_cache,
			buffer,
			id_1,
			id_2,
			id_3,
			font_1,
			font_2,
			font_3,
		}
	}

	pub fn render_text(&mut self, scene: &mut Scene, posis: (f64, f64)) {

		let buffer = &mut self.buffer.borrow_with(&mut self.font_system);

		let mut attrs: Attrs<'_> = Attrs::new();
		attrs = attrs.family(cosmic_text::Family::Name("Rubik"));

		buffer.set_text("Hello, everyone! (^o^)/ Let's have a great day! ✨ (こんにちは、皆さん！(^o^)/ 素晴らしい一日を送りましょう！✨)", &attrs, Shaping::Advanced);

		let color = Color::rgb(0, 0, 0);
		let mut pos: f32 = 0.0;

		let mut glyph_groups = (Vec::new(), Vec::new(), Vec::new());

		for run in buffer.layout_runs() {
			for glyph in run.glyphs {
				let g = Glyph {
					id: glyph.glyph_id as u32,
					x: pos + glyph.x,
					y: glyph.y,
				};
				if glyph.font_id == self.id_1 {
					glyph_groups.0.push(g);
				} else if glyph.font_id == self.id_2 {
					glyph_groups.1.push(g);
				} else if glyph.font_id == self.id_3 {
					glyph_groups.2.push(g);
				}
				
				pos += glyph.w * 0.1;
			}
		}

		scene.0
			.draw_glyphs(&self.font_1)
			.transform(Affine::translate(posis))
			.font_size(12.0)
			.draw(
				&vello::peniko::Style::Fill(vello::peniko::Fill::NonZero),
				glyph_groups.0.iter().cloned(),
			);

		scene.0
			.draw_glyphs(&self.font_2)
			.transform(Affine::translate(posis))
			.font_size(12.0)
			.draw(
				&vello::peniko::Style::Fill(vello::peniko::Fill::NonZero),
				glyph_groups.1.iter().cloned(),
			);

		scene.0
			.draw_glyphs(&self.font_3)
			.transform(Affine::translate(posis))
			.font_size(12.0)
			.draw(
				&vello::peniko::Style::Fill(vello::peniko::Fill::NonZero),
				glyph_groups.2.iter().cloned(),
			);

	}

}
