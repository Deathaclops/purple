
pub mod prim;
pub mod purple;
pub mod context;
pub mod scene;
pub mod shapes;

pub use crosslog;
pub use tiny_skia;

pub mod prelude {
	pub use crosslog::prelude::{println, *};
	pub use crate::prim::*;
	pub use crate::purple::*;
	pub use crate::context::*;
	pub use crate::scene::*;
	pub use crate::shapes::*;
	pub use winit::keyboard::KeyCode;
} // end mod prelude

#[cfg(target_arch = "wasm32")]
pub mod web { // Web-specific functionality
	use web_sys::{HtmlCanvasElement, wasm_bindgen::JsCast};
	use crate::prim::Dimensions;
	pub fn get_canvas(id: &String) -> Option<HtmlCanvasElement> {
		let canvas = web_sys::window()
		.and_then(|win| win.document())
		.and_then(|doc| doc.get_element_by_id(&id))?;
		return canvas.dyn_into::<HtmlCanvasElement>().ok();
	} // end fn get_canvas
	pub fn add_canvas(size: Dimensions) -> HtmlCanvasElement {
		let document = web_sys::window().unwrap().document().unwrap();
		let canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
		canvas.set_width(size.width as u32);
		canvas.set_height(size.height as u32);
		let body = document.body().unwrap();
		body.append_child(&canvas).expect("Could not append canvas to body.");
		return canvas;
	} // end fn add_canvas
} // end mod web