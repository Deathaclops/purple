use crate::scene::Scene;
use crate::context::FontContext;
use parley::{AlignmentOptions, BoundingBox, FontFamily, StyleProperty};
use vello::peniko::color::AlphaColor;
use crate::prim::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment { Left, Center, Right, Justify }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerticalAlignment { Top, Center, Bottom }

impl From<Alignment> for parley::Alignment {
	fn from(alignment: Alignment) -> Self {
		return match alignment {
			Alignment::Left => parley::Alignment::Left,
			Alignment::Center => parley::Alignment::Center,
			Alignment::Right => parley::Alignment::Right,
			Alignment::Justify => parley::Alignment::Justify,
		}; // end return
	} // end fn from
} // end impl From<Alignment> for parley::Alignment

pub struct TextOptions {
	pub font_size: f64,
	pub color: Option<Color>,
	pub bbox: Option<crate::prim::BoundingBox>,
	pub font_names: Option<Vec<String>>,
	pub alignment: Alignment,
	pub vertical_alignment: VerticalAlignment,
	pub align_from_left_of_bbox: bool,
	pub align_from_top_of_bbox: bool,
	pub quantize: bool,
} // end struct TextOptions

impl Default for TextOptions {
	fn default() -> Self {
		return Self {
			font_size: 16.0,
			color: None,
			bbox: None,
			font_names: None,
			alignment: Alignment::Left,
			vertical_alignment: VerticalAlignment::Top,
			align_from_left_of_bbox: false,
			align_from_top_of_bbox: false,
			quantize: false,
		}; // end return
	} // end fn default
} // end impl Default for TextOptions

pub fn draw_text(scene: &mut Scene, text: String, transform: Affine, font_ctx: &mut FontContext, options: TextOptions) -> crate::prim::BoundingBox {

	let text: String = text.into();
	let fonts_names = options.font_names.unwrap_or(font_ctx.fonts.clone());
	let font_families: Vec<FontFamily> = fonts_names.iter().map(|f| FontFamily::Named(std::borrow::Cow::Borrowed(f))).collect();

	let mut builder = font_ctx.layout.ranged_builder(&mut font_ctx.context, &text, 1.0, options.quantize);
	builder.push_default(parley::StyleProperty::FontSize(options.font_size as f32));
	builder.push_default(StyleProperty::FontStack(parley::FontStack::List(font_families.into())));
	let mut layout: parley::Layout<[u8; 4]> = builder.build(&text);

	if let Some(ref bbox) = options.bbox {
		let max_width = bbox.width as f32;
		layout.break_all_lines(Some(max_width));
		layout.align(Some(max_width), options.alignment.into(), AlignmentOptions { align_when_overflowing: !options.align_from_left_of_bbox });
	} else {
		layout.break_all_lines(None);
		layout.align(None, options.alignment.into(), AlignmentOptions::default());
	} // end if let Some(bbox)

	let base_y = options.bbox.as_ref().map_or(0.0, |b| b.y as f32);
		
	let mut run_y = match options.vertical_alignment {
		VerticalAlignment::Top => { base_y },
		VerticalAlignment::Center => {
			let base_height = options.bbox.as_ref().map_or(0.0, |b| b.height as f32);
			let y = (base_height - layout.height() ) / 2.0 + base_y;
			if options.align_from_top_of_bbox && layout.height() > base_height { base_y }
			else { y }
		}, VerticalAlignment::Bottom => {
			let base_height = options.bbox.as_ref().map_or(0.0, |b| b.height as f32);
			let y = base_height - layout.height() + base_y;
			if options.align_from_top_of_bbox && layout.height() > base_height { base_y }
			else { y }
		}
	};

	let mut min_y = run_y; let mut max_y = run_y;
	let mut begin = true;

	let mut min_x = match options.alignment {
		Alignment::Left => 0.0,
		Alignment::Center => -layout.width() / 2.0,
		Alignment::Right => layout.width(),
		Alignment::Justify => 0.0,
	}; // end match

	for line in layout.lines() {
		min_y = run_y.min(min_y);
		run_y += line.metrics().line_height;
		max_y = run_y.max(max_y);
		if begin { run_y -= line.metrics().descent; begin = false; }
		for item in line.items() {
			match item {
				parley::PositionedLayoutItem::GlyphRun(glyph_run) => {
					let font: &parley::FontData = glyph_run.run().font();
					let mut glyphs = Vec::new();
					let mut run_x = glyph_run.offset();
					if let Some(ref bbox) = options.bbox { run_x += bbox.x as f32; }
					if options.bbox.is_none() && options.alignment == Alignment::Center { run_x -= layout.full_width() / 2.0; }
					if options.bbox.is_none() && options.alignment == Alignment::Right { run_x -= layout.full_width(); }
					for glyph in glyph_run.glyphs() {
						let g = vello::Glyph {
							id: glyph.id as u32,
							x: run_x,
							y: run_y,
						}; glyphs.push(g);
						min_x = run_x.min(min_x);
						run_x += glyph.advance;
					} // end for glyph
					let mut drawing = scene.0.draw_glyphs(font)
					.transform(transform.0)
					.font_size(options.font_size as f32);
					if let Some(ref color) = options.color { drawing = drawing.brush(vello::peniko::Brush::Solid(AlphaColor::from_rgba8(color.r, color.g, color.b, color.a))); }
					drawing.draw(&vello::peniko::Style::Fill(vello::peniko::Fill::NonZero), glyphs.iter().cloned());
				} parley::PositionedLayoutItem::InlineBox(_inline_box) => { }
			} // end match item
		} // end for item
	} // end for line

	return crate::prim::BoundingBox {
		x: min_x as f64,
		y: min_y as f64,
		width: layout.width() as f64,
		height: layout.height() as f64,
	};

}

