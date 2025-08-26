use std::{cell::RefCell, rc::Rc, sync::Arc};
use winit::{application::ApplicationHandler, event::MouseScrollDelta, event_loop::{ControlFlow, EventLoop}, window};
use crate::prelude::*;

const FULLSCREEN: Option<window::Fullscreen> = Some(window::Fullscreen::Borderless(None));
const WINDOWED: Option<window::Fullscreen> = None;

pub struct WindowConfig {
	pub title: String,
	pub resolution: Dimensions,
	pub fullscreen: bool,
	pub canvas_id: Option<String>,
} // end struct WindowConfig

impl WindowConfig {
	pub fn new() -> Self { return Self::default(); }
	pub fn with_title(mut self, title: impl Into<String>) -> Self { self.title = title.into(); return self; }
	pub fn with_resolution(mut self, resolution: impl Into<Dimensions>) -> Self { self.resolution = resolution.into(); return self; }
	pub fn with_fullscreen(mut self, fullscreen: bool) -> Self { self.fullscreen = fullscreen; return self; }
	pub fn with_canvas_id(mut self, canvas_id: impl Into<String>) -> Self { self.canvas_id = Some(canvas_id.into()); return self; }
} // end impl WindowConfig

impl Default for WindowConfig {
	fn default() -> Self { return Self {
		title: "Application".into(),
		resolution: (960, 540).into(),
		fullscreen: false,
		canvas_id: Some("canvas".into()),
	}; } // end fn default
} // end impl Default

pub struct Purple<F> where F: FnMut(&mut Context) {
	pub window: Option<Arc<window::Window>>,
	pub eloop: F,
	pub config: WindowConfig,
	pub context: Rc<RefCell<Option<Context>>>,
} // end struct Purple

impl<F> Purple<F> where F: FnMut(&mut Context) {
	pub fn new ( config: WindowConfig, eloop: F ) {
		let event_loop: EventLoop<()> = EventLoop::new().unwrap();
		event_loop.set_control_flow(ControlFlow::Wait);
		let mut purple = Self { window: None, eloop, config, context: Rc::new(RefCell::new(None)) };
		let _ = event_loop.run_app(&mut purple);
	} // end fn new
} // end impl Purple

impl<F> ApplicationHandler for Purple<F> where F: FnMut(&mut Context) {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

		let mut window_attributes = window::WindowAttributes::default()
		.with_title(self.config.title.as_str())
		.with_fullscreen(if self.config.fullscreen { FULLSCREEN } else { WINDOWED })
		.with_resizable(true);

		let mut resolution = self.config.resolution;

		#[cfg(target_arch = "wasm32")]
		if let Some(canvas_id) = &self.config.canvas_id {
			use winit::platform::web::WindowAttributesExtWebSys;
			if let Some(canvas) = crate::web::get_canvas(canvas_id) {
				if canvas.has_attribute("width") && canvas.has_attribute("height") {
					resolution = (canvas.width(), canvas.height()).into();
				} window_attributes = window_attributes.with_canvas(Some(canvas));
			} else { panic!("Could not find specified canvas"); }
		} else { panic!("The canvas id must be set"); }

		window_attributes = window_attributes.with_inner_size(winit::dpi::PhysicalSize::new(resolution.width, resolution.height));
		let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
		self.window = Some(window.clone());

		#[cfg(target_arch = "wasm32")]
		{	let mut clone = self.context.clone();
			wasm_bindgen_futures::spawn_local(async move {
				let mut result = Context::new(window.clone()).await;
				clone.replace(Some(result));
				window.clone().request_redraw();
			}); // end spawn_local
		} // end cfg wasm32

		#[cfg(not(target_arch = "wasm32"))]
		{	let mut clone = self.context.clone();
			async_std::task::block_on ( async move {
				let mut result = Context::new(window.clone()).await;
				clone.replace(Some(result));
			}); // end block_on
		} // end cfg not wasm32

	} // end fn resumed
	fn window_event(&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		window_id: winit::window::WindowId,
		event: winit::event::WindowEvent,
	) { // begin fn window_event

		if self.context.borrow().is_none() { return; }
		let mut context_buffer = self.context.borrow_mut();
		let mut context = context_buffer.as_mut().unwrap();
		if window_id != context.window.id() { return; }

		match event {

			winit::event::WindowEvent::RedrawRequested => {
				if context.gpu.texture.width() != context.gpu.window.inner_size().width
				|| context.gpu.texture.height() != context.gpu.window.inner_size().height { log!("Incorrect texture size, resizing..."); context.resize(); }
				(self.eloop)(&mut context);
				context.state.update();
				context.render();
				context.window.request_redraw();
			} // end RedrawRequested

			winit::event::WindowEvent::CloseRequested => { drop(context); context_buffer.take(); event_loop.exit(); }
			winit::event::WindowEvent::Resized(size) => { context.resize(); }
			winit::event::WindowEvent::CursorMoved { device_id, position } => { context.state.mouse = Some((position.x, position.y).into()); }
			winit::event::WindowEvent::CursorLeft { device_id } => { context.state.mouse = None; }
			winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => { context.state.keyboard_event(event); }
			winit::event::WindowEvent::MouseInput { device_id, state, button } => { context.state.button_event(button, state); }
			winit::event::WindowEvent::MouseWheel { device_id, delta, phase } => { if let MouseScrollDelta::LineDelta(x, y) = delta { context.state.wheel = y; context.state.wheel_x = x; } }
			_ => {} // default case

		} // end match event
	} // end fn window_event
} // end impl ApplicationHandler