
use winit::{event::{ElementState, KeyEvent, MouseButton}, keyboard::{KeyCode, KeyLocation, NamedKey, PhysicalKey}};
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use web_time::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

use crate::prelude::*;

pub struct State {
	pub resolution: Dimensions,
	pub delta: f64,
	pub mouse: Option<point::Point>,
	pub keys: HashMap<KeyCode, KeyState>,
	pub wheel: f32,
	pub wheel_x: f32,
	pub mouse_left: ButtonState,
	pub mouse_right: ButtonState,
	pub mouse_middle: ButtonState,
	last_time: Instant,
} // end struct State

impl State {

	pub fn new ( res: impl Into<Dimensions> ) -> Self {
		return Self {
			resolution: res.into(),
			delta: 0.0,
			mouse: None,
			keys: HashMap::new(),
			wheel: 0.0,
			wheel_x: 0.0,
			mouse_left: ButtonState::default(),
			mouse_right: ButtonState::default(),
			mouse_middle: ButtonState::default(),
			last_time: Instant::now(),
		} // end return
	} // end fn new

	pub fn update (&mut self) {
		let now = Instant::now();
		self.delta = now.duration_since(self.last_time).as_nanos() as f64 / 1_000_000_000.0;
		self.last_time = now;
		for key in &mut self.keys {
			key.1.on_pressed = false;
			key.1.on_released = false;
			key.1.on_typed = false;
		} // end for key
		self.mouse_left.on_pressed = false;
		self.mouse_left.on_released = false;
		self.mouse_right.on_pressed = false;
		self.mouse_right.on_released = false;
		self.mouse_middle.on_pressed = false;
		self.mouse_middle.on_released = false;
		self.wheel = 0.0;
		self.wheel_x = 0.0;
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
			key_state.on_typed = event.repeat || key_state.on_pressed;
			key_state.is_down = event.state == winit::event::ElementState::Pressed;
		} else {
			self.keys.insert(keycode.clone(), KeyState {
				on_pressed: event.state == winit::event::ElementState::Pressed,
				on_released: event.state == winit::event::ElementState::Released,
				on_typed: event.repeat || event.state == winit::event::ElementState::Pressed,
				is_down: event.state == winit::event::ElementState::Pressed,
			});
		} // end if self.keys.contains_key
	} // end fn keyboard_event

	pub fn key(&mut self, keycode: KeyCode) -> KeyState {
		if self.keys.contains_key(&keycode) {
			return self.keys.get(&keycode).unwrap().clone();
		} else { return KeyState::default(); }
	} // end fn key

	pub fn button_event(&mut self, button: MouseButton, state: ElementState) {
		if state == ElementState::Pressed { match button {
			MouseButton::Left => { self.mouse_left.is_down = true; self.mouse_left.on_pressed = true; }
			MouseButton::Right => { self.mouse_right.is_down = true; self.mouse_right.on_pressed = true; }
			MouseButton::Middle => { self.mouse_middle.is_down = true; self.mouse_middle.on_pressed = true; }
			_ => {}
		} } else { match button {
			MouseButton::Left => { self.mouse_left.is_down = false; self.mouse_left.on_released = true; }
			MouseButton::Right => { self.mouse_right.is_down = false; self.mouse_right.on_released = true; }
			MouseButton::Middle => { self.mouse_middle.is_down = false; self.mouse_middle.on_released = true; }
			_ => {}
		} } // end if state == Pressed
	} // end fn button_event

	pub fn fps(&self) -> f64 { return 1.0 / self.delta; }

} // end impl State

#[derive(Default, Clone, Debug)]
pub struct KeyState {
	pub on_pressed: bool,
	pub on_released: bool,
	pub on_typed: bool,
	pub is_down: bool,
} // end struct KeyState

#[derive(Default, Clone, Debug)]
pub struct ButtonState {
	pub on_pressed: bool,
	pub on_released: bool,
	pub is_down: bool,
} // end struct ButtonState

