


#[cfg(target_arch = "wasm32")]
mod web {

	use web_sys::HtmlCanvasElement;
	use web_sys::wasm_bindgen::JsCast;

	pub fn get_canvas ( id: &String ) -> Option<HtmlCanvasElement> {

		let canvas = web_sys::window()
		.and_then(|win| win.document())
		.and_then(|doc| doc.get_element_by_id(&id))?;
		return canvas.dyn_into::<HtmlCanvasElement>().ok();

	} // end fn get_canvas
}

#[cfg(target_arch = "wasm32")]
pub use web::*;