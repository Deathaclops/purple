
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindowSizing {
	NonResizable,
	Resizable,
	Fullscreen,
} // end enum Sizing

pub struct WindowConfig {
	pub title: String,
	pub resolution: (u32, u32),
	pub sizing: WindowSizing,
	pub canvas_id: Option<String>,
} // end struct WindowConfig



