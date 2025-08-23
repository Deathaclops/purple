
// Color struct represents an RGBA color

use vello::peniko::{color::AlphaColor, BrushRef};

#[repr(C)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
} // end struct Color

// Conversions between types

impl From<[u8; 4]> for Color {
	fn from(color: [u8; 4]) -> Self {
		return Color { r: color[0], g: color[1], b: color[2], a: color[3] };
	} // end From [u8; 4]
} // end impl From [u8; 4] for Color

impl From<&[u8; 4]> for Color {
	fn from(color: &[u8; 4]) -> Self {
		return Color { r: color[0], g: color[1], b: color[2], a: color[3] };
	} // end From &[u8; 4]
} // end impl From &[u8; 4] for Color

impl From<&mut [u8; 4]> for Color {
	fn from(color: &mut [u8; 4]) -> Self {
		return Color { r: color[0], g: color[1], b: color[2], a: color[3] };
	} // end From &mut [u8; 4]
} // end impl From &mut [u8; 4] for Color

impl From<[u8; 3]> for Color {
	fn from(color: [u8; 3]) -> Self {
		return Color { r: color[0], g: color[1], b: color[2], a: 255 };
	} // end From [u8; 3]
} // end impl From [u8; 3] for Color

impl From<&[u8; 3]> for Color {
	fn from(color: &[u8; 3]) -> Self {
		return Color { r: color[0], g: color[1], b: color[2], a: 255 };
	} // end From &[u8; 3]
} // end impl From &[u8; 3] for Color

impl From<&mut [u8; 3]> for Color {
	fn from(color: &mut [u8; 3]) -> Self {
		return Color { r: color[0], g: color[1], b: color[2], a: 255 };
	} // end From &mut [u8; 3]
} // end impl From &mut [u8; 3] for Color

impl<'a> From<Color> for BrushRef<'a> {
	fn from(color: Color) -> Self {
		return AlphaColor::from_rgba8(
			color.r,
			color.g,
			color.b,
			color.a,
		).into();
	} // end From Color
} // end impl From Color for BrushRef

