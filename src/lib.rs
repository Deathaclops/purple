
pub mod prim;
pub mod purple;
pub mod context;
pub mod scene;
pub mod text;

pub mod prelude {
	pub use crosslog::prelude::{println, *};
	pub use crate::prim::*;
	pub use crate::purple::*;
	pub use crate::context::*;
	pub use crate::scene::*;
	pub use vello::kurbo::*;
	pub use winit::keyboard::KeyCode;
} // end mod prelude

#[cfg(target_arch = "wasm32")]
pub mod web { use std::sync::Arc;

// Web-specific functionality
	use web_sys::{HtmlCanvasElement, wasm_bindgen::JsCast};
	use winit::{window::Window, platform::web::WindowExtWebSys};
	pub fn get_canvas(id: &String) -> Option<HtmlCanvasElement> {
		let canvas = web_sys::window()
		.and_then(|win| win.document())
		.and_then(|doc| doc.get_element_by_id(&id))?;
		return canvas.dyn_into::<HtmlCanvasElement>().ok();
	} // end fn get_canvas
	pub fn add_canvas(window: Arc<Window>) {
		let canvas = window.canvas().unwrap().dyn_into().unwrap();
		let document = web_sys::window().unwrap().document().unwrap();
    	let body = document.body().unwrap();
		body.append_child(&canvas).expect("Could not append canvas to body.");
	} // end fn add_canvas
} // end mod web