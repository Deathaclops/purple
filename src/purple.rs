use std::{cell::RefCell, rc::Rc, sync::Arc};
use winit::{application::ApplicationHandler, event::MouseScrollDelta, event_loop::{ControlFlow, EventLoop}, window::{self, Icon}};
use crate::prelude::*;

#[cfg(target_os = "windows")]
use winit::platform::windows::{WindowAttributesExtWindows, WindowExtWindows};

const FULLSCREEN: Option<window::Fullscreen> = Some(window::Fullscreen::Borderless(None));
const WINDOWED: Option<window::Fullscreen> = None;

pub struct WindowConfig {
	pub title: String,
	pub resolution: Dimensions,
	pub fullscreen: bool,
	pub canvas_id: Option<String>,
	pub disable_decorations: bool,
	pub icon: Option<Image>,
	pub resizeable: bool,
} // end struct WindowConfig

impl WindowConfig {
	pub fn new() -> Self { return Self::default(); }
	pub fn with_title(mut self, title: impl Into<String>) -> Self { self.title = title.into(); return self; }
	pub fn with_resolution(mut self, resolution: impl Into<Dimensions>) -> Self { self.resolution = resolution.into(); return self; }
	pub fn with_fullscreen(mut self, fullscreen: bool) -> Self { self.fullscreen = fullscreen; return self; }
	pub fn with_canvas_id(mut self, canvas_id: impl Into<String>) -> Self { self.canvas_id = Some(canvas_id.into()); return self; }
	pub fn with_icon(mut self, icon: Option<Image>) -> Self { self.icon = icon; return self; }
	pub fn with_resizable(mut self, resizeable: bool) -> Self { self.resizeable = resizeable; return self; }
	pub fn with_decorations(mut self, decorations: bool) -> Self { self.disable_decorations = !decorations; return self; }
} // end impl WindowConfig

impl Default for WindowConfig {
	fn default() -> Self { return Self {
		title: "Purple Application".into(),
		resolution: (960, 540).into(),
		fullscreen: false,
		canvas_id: Some("canvas".into()),
		disable_decorations: false,
		icon: None,
		resizeable: true,
	}; } // end fn default
} // end impl Default

pub struct Purple<F> where F: FnMut(&mut Context) {
	pub eloop: F,
	pub config: WindowConfig,
	pub context: Option<Context>,
} // end struct Purple

impl<F> Purple<F> where F: FnMut(&mut Context) {
	pub fn new ( config: WindowConfig, eloop: F ) {
		let event_loop: EventLoop<()> = EventLoop::new().unwrap();
		event_loop.set_control_flow(ControlFlow::Wait);
		let mut purple = Self { eloop, config, context: None };
		let _ = event_loop.run_app(&mut purple);
	} // end fn new
} // end impl Purple

impl<F> ApplicationHandler for Purple<F> where F: FnMut(&mut Context) {
	fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {

		let mut icon = None;
		if let Some(icon_image) = &mut self.config.icon {
			log!("Icon image found");
			let Dimensions { width, height } = icon_image.size();
			let icon_data = icon_image.get_bytes();
			icon = Some(Icon::from_rgba(icon_data, width as u32, height as u32).unwrap());
		} // end if let Some(icon_image)

		let mut window_attributes = window::WindowAttributes::default()
		.with_title(self.config.title.as_str())
		.with_fullscreen(if self.config.fullscreen { FULLSCREEN } else { WINDOWED })
		.with_resizable(true)
		.with_decorations(!self.config.disable_decorations)
		.with_min_inner_size(winit::dpi::PhysicalSize::new(40, 40))
		.with_resizable(self.config.resizeable);

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

		let ptr = &mut self.context as *mut Option<Context>;
		#[cfg(target_arch = "wasm32")]
		{	wasm_bindgen_futures::spawn_local(async move {
				let mut result = Context::new(window.clone()).await;
				result.resize();
				unsafe { *ptr = Some(result); }
				window.clone().request_redraw();
			}); // end spawn_local
		} // end cfg wasm32

		#[cfg(not(target_arch = "wasm32"))]
		{	async_std::task::block_on ( async move {
				let result = Context::new(window.clone()).await;
				unsafe { *ptr = Some(result); }
				window.set_window_icon(icon);
				window.clone().request_redraw();
			}); // end block_on
		} // end cfg not wasm32

	} // end fn resumed
	fn window_event(&mut self,
		event_loop: &winit::event_loop::ActiveEventLoop,
		window_id: winit::window::WindowId,
		event: winit::event::WindowEvent,
	) { // begin fn window_event

		if self.context.is_none() { return; }
		let mut context = self.context.as_mut().unwrap();
		if window_id != context.window.id() { return; }

		match event {

			winit::event::WindowEvent::RedrawRequested => {
				(self.eloop)(&mut context);
				context.state.update();
				context.render();
				context.window.request_redraw();
				if context.state.exiting { self.context.take(); event_loop.exit(); }
			} // end RedrawRequested

			winit::event::WindowEvent::CloseRequested => { context.state.exiting = true; }
			winit::event::WindowEvent::Resized(_size) => {
				if context.window.is_resizable() == false { context.window.set_maximized(false); }
				else { context.resize(); }
			} // end Resized
			winit::event::WindowEvent::CursorMoved { device_id: _, position } => { context.state.mouse = Some((position.x, position.y).into()); }
			winit::event::WindowEvent::CursorLeft { device_id: _ } => { context.state.mouse = None; }
			winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => { context.state.keyboard_event(event); }
			winit::event::WindowEvent::MouseInput { device_id: _, state, button } => { context.state.button_event(button, state); }
			winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
				match delta {
					MouseScrollDelta::LineDelta(x, y) => { context.state.wheel = y; context.state.wheel_x = x; }
					MouseScrollDelta::PixelDelta(pos) => { context.state.wheel = pos.y as f32 / 100.0; context.state.wheel_x = pos.x as f32 / 100.0; }
				} // end match delta
			} // end MouseWheel
			_ => {} // default case

		} // end match event
	} // end fn window_event
} // end impl ApplicationHandler