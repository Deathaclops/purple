
use winit::{event::KeyEvent, keyboard::{KeyCode, KeyLocation, NamedKey, PhysicalKey}, platform::{modifier_supplement::KeyEventExtModifierSupplement, scancode::PhysicalKeyExtScancode}};
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use web_time::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[derive(Default, Clone)]
pub struct KeyState {
	pub on_pressed: bool,
	pub on_released: bool,
	pub on_repeat: bool,
	pub is_down: bool,
} // end struct KeyState

pub struct State {
	pub window: Option<Arc<winit::window::Window>>,
	pub resolution: (u32, u32),
	last_time: Instant,
	pub delta: f64,
	pub mouse: Option<(f64, f64)>,
	pub keys: HashMap<KeyCode, KeyState>,
} // end struct State

impl State {
	
	pub fn new (res: (u32, u32)) -> Self {
		return Self {
			window: None,
			resolution: res,
			delta: 0.0,
			last_time: Instant::now(),
			mouse: None,
			keys: HashMap::new(),
		} // end return
	} // end fn new

	pub fn update (&mut self) {
		let now = Instant::now();
		self.delta = now.duration_since(self.last_time).as_nanos() as f64 / 1_000_000.0;
		self.last_time = now;
		for key in &mut self.keys {
			key.1.on_pressed = false;
			key.1.on_released = false;
			key.1.on_repeat = false;
		} // end for key
	} // end fn update

	pub fn keyboard_event(&mut self, event: KeyEvent) {
		let keycode = match event.physical_key {
			PhysicalKey::Code(code) => code,
			PhysicalKey::Unidentified(_) => { return; },
		}; // end match event.physical_key
		if self.keys.contains_key(&keycode) {
			let mut key_state = self.keys.get_mut(&keycode).unwrap();
			key_state.on_pressed = event.state == winit::event::ElementState::Pressed && !event.repeat;
			key_state.on_released = event.state == winit::event::ElementState::Released;
			key_state.on_repeat = event.repeat || key_state.on_pressed;
			key_state.is_down = event.state == winit::event::ElementState::Pressed;
		} else {
			self.keys.insert(keycode.clone(), KeyState {
				on_pressed: event.state == winit::event::ElementState::Pressed,
				on_released: event.state == winit::event::ElementState::Released,
				on_repeat: event.repeat || event.state == winit::event::ElementState::Pressed,
				is_down: event.state == winit::event::ElementState::Pressed,
			});
		} // end if self.keys.contains_key
	} // end fn keyboard_event

	pub fn key (&mut self, keycode: KeyCode) -> KeyState {
		if self.keys.contains_key(&keycode) {
			return self.keys.get(&keycode).unwrap().clone();
		} else {
			return KeyState::default();
		} // end if
	} // end fn key

	pub fn toggle_fullscreen(&mut self) {

		if let Some(window) = &self.window {
			let is_fullscreen = window.fullscreen().is_some();
			if is_fullscreen {
				window.set_fullscreen(None);
			} else {
				window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
			} // end if
		} // end if let Some(window)

	} // end fn toggle_fullscreen

} // end impl State

