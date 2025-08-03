use std::sync::Arc;
use vello::RendererOptions;

use crosslog::prelude::*;
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;
use vello::util::RenderSurface;
use vello::wgpu;
use winit::dpi::PhysicalSize;
use winit::event_loop::ControlFlow;
use winit::window::{self, WindowButtons};
use winit::{application::ApplicationHandler, window::WindowAttributes};
use winit::{event_loop::EventLoop, window::Window};

mod web;
mod state;
use state::*;
mod renderer;
use renderer::*;
mod window_config;
pub use window_config::*;
pub mod prelude;
pub mod coords;
use coords::Vec2;
pub mod image;

pub use vello::Scene;
pub use vello::kurbo;
pub use vello::peniko;

#[cfg(target_arch = "wasm32")]
use web::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::{WindowAttributesExtWebSys, WindowExtWebSys};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

pub struct Purple<F> where F: FnMut(&mut Scene, &mut State) {
	pub eloop: F,
	pub state: State,
	pub config: WindowConfig,
	pub renderer: Rc<RefCell<Option<Renderer>>>,
	pub scene: Scene,
} // end struct PurpleApp

impl<F> Purple<F> where F: FnMut(&mut Scene, &mut State) {
	pub fn new ( config: WindowConfig, eloop: F ) {
		let event_loop: EventLoop<()> = EventLoop::new().unwrap();
		event_loop.set_control_flow(ControlFlow::Wait);
		let mut purple = Self {
			eloop,
			state: State::new(config.resolution),
			config,
			renderer: Rc::new(RefCell::new(None)),
			scene: Scene::new(),
		}; // end let purple
		let _ = event_loop.run_app(&mut purple);
	} // end fn new
} // end impl Purple

impl<F> ApplicationHandler for Purple<F> where F: FnMut(&mut Scene, &mut State) {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

		let mut resolution = self.config.resolution;
		let resizable = self.config.sizing != WindowSizing::NonResizable;
		let fullscreen = self.config.sizing == WindowSizing::Fullscreen;

		let mut window_attributes = WindowAttributes::default()
		.with_title(self.config.title.as_str())
		.with_resizable(resizable)
		.with_inner_size(PhysicalSize::new(resolution.x(), resolution.y()))
		.with_fullscreen(if fullscreen { Some(window::Fullscreen::Borderless(None)) } else { None });

		#[cfg(target_arch = "wasm32")]
		if let Some(canvas_id) = &self.config.canvas_id {
			if let Some(canvas) = get_canvas(&canvas_id) {
				if canvas.has_attribute("width") && canvas.has_attribute("height") {
					resolution = (canvas.width(), canvas.height());
				} window_attributes = window_attributes.with_canvas(Some(canvas));
			} else { panic!("Could not find specified canvas"); }
		} // end if canvas_id

		window_attributes = window_attributes.with_inner_size( PhysicalSize::new(resolution.x(), resolution.y()) );
		let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
		self.state.window = Some(window.clone());

		#[cfg(target_arch = "wasm32")]
		if self.config.canvas_id.is_none() {
			let canvas = window.as_ref().canvas().unwrap();
			web_sys::window()
				.and_then(|win| win.document())
				.and_then(|doc| doc.body())
				.and_then(|body| body.append_child(canvas.as_ref()).ok())
				.expect("couldn't append canvas to document body");
		} // Append a canvas to the body if an ID is not specified

		#[cfg(target_arch = "wasm32")]
		{	let mut clone = self.renderer.clone();
			wasm_bindgen_futures::spawn_local(async move {
				let mut result = Renderer::new(window.clone(), resolution).await;
				*clone.borrow_mut() = Some(result);
				//window.clone().request_redraw();
			}); // end spawn_local
		} // end cfg wasm32

		#[cfg(not(target_arch = "wasm32"))]
		{	let mut clone = &mut self.renderer.clone();
			async_std::task::block_on ( async move {
				let mut result = Renderer::new(window, resolution).await;
				*clone.borrow_mut() = Some(result);
			}); // end block_on
		} // end cfg not wasm32

	}

	fn window_event ( // This function is called by the event loop
			&mut self,
			event_loop: &winit::event_loop::ActiveEventLoop,
			window_id: winit::window::WindowId,
			event: winit::event::WindowEvent,
		) { // begin fn window_event

		if self.renderer.borrow().is_none() { return; }
		let mut binding = self.renderer.borrow_mut();
		let mut renderer = binding.as_mut().unwrap();
		if window_id != renderer.window.id() { return; }

		match event {

			winit::event::WindowEvent::RedrawRequested => {
				(self.eloop)(&mut self.scene, &mut self.state);
				renderer.render(&self.scene);
				self.state.update();
				renderer.window.request_redraw();
			} // end RedrawRequested

			winit::event::WindowEvent::CloseRequested => { event_loop.exit(); }
			winit::event::WindowEvent::Resized(size) => { self.state.resolution = (size.width, size.height); renderer.resize(size.into()); }
			winit::event::WindowEvent::CursorMoved { device_id, position } => { self.state.mouse = Some((position.x, position.y)); }
			winit::event::WindowEvent::CursorLeft { device_id } => { self.state.mouse = None; }
			winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => { self.state.keyboard_event(event); }
			_ => {} // default case

		} // end match event

	} // end fn window_event

}


