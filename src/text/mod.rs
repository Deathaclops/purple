pub mod font; pub use font::*;
use parley::{AlignmentOptions, FontFamily, StyleProperty};

use crate::scene::Scene;

pub fn draw_text(ctx: &mut FontContext, scene: &mut Scene, transform: vello::kurbo::Affine, text: String, fonts: &[String], max_width: Option<f32>) {

	let text: String = text.into();
	let scale = 1.0;
	
	let mut ffs = Vec::new();
	for ff in fonts { ffs.push(FontFamily::Named(std::borrow::Cow::Borrowed(ff.as_str()))) }

	let mut builder = ctx.layout.ranged_builder(&mut ctx.context, &text, scale, true);
	builder.push_default(parley::StyleProperty::FontSize(16.0));
	builder.push_default(StyleProperty::FontStack(parley::FontStack::List(ffs.into())));
	let mut layout: parley::Layout<[u8; 4]> = builder.build(&text);

	layout.break_all_lines(max_width);
	layout.align(max_width, parley::Alignment::Left, AlignmentOptions::default());

	let mut char_y = 0.0;
	for line in layout.lines() {
		let mut char_x = 0.0;
		for item in line.items() { match item {
			parley::PositionedLayoutItem::GlyphRun(glyph_run) => {
				let font: &parley::Font = glyph_run.run().font();
				let mut glyphs = Vec::new();
				for glyph in glyph_run.glyphs() {
					let g = vello::Glyph {
						id: glyph.id as u32,
						x: char_x,
						y: char_y,
					};
					glyphs.push(g);
					char_x += glyph.advance;
				}
				scene.0.draw_glyphs(font)
				.transform(transform)
				.draw(&vello::peniko::Style::Fill(vello::peniko::Fill::NonZero), glyphs.iter().cloned());

			} parley::PositionedLayoutItem::InlineBox(_inline_box) => { }
		} } char_y += line.metrics().line_height;
	}
}