
use std::sync::Arc;

use vello::{peniko::color::AlphaColor, wgpu};
use winit::window::Window;

use crate::context::{Gpu};
use crate::prelude::*;

pub struct Context {
	pub window: Arc<Window>,
	pub gpu: Gpu,
	pub scene: Scene,
	pub state: State,
} // end struct Context

impl Context {
	pub async fn new ( window: Arc<Window> ) -> Self {
		let gpu = Gpu::new(window.clone()).await;
		return Self { window: window.clone(), gpu, scene: Scene::new(), state: State::new(window.inner_size()) };
	} // end fn new
	pub fn resize(&mut self) {
		self.state.resolution = self.window.inner_size().into();
		self.gpu.resize();
	} // end fn resize
	pub fn render(&mut self) {

		let window_texture: wgpu::SurfaceTexture = self.gpu.surface.get_current_texture().unwrap();
		let window_view: wgpu::TextureView = window_texture.texture.create_view(&wgpu::TextureViewDescriptor {
			label: Some("Window View"),
			format: Some(self.gpu.config.format),
			..Default::default()
		}); // end let window_view

		let _ = self.gpu.renderer.render_to_texture(
				&self.gpu.device,
				&self.gpu.queue,
				&self.scene.0,
				&self.gpu.texture_view,
				&vello::RenderParams {
					base_color: AlphaColor::BLACK,
					width: self.gpu.config.width,
					height: self.gpu.config.height,
					antialiasing_method: vello::AaConfig::Msaa16,
				} // end RenderParams
			); // end render_to_texture

		let mut encoder = self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("PostProcess Encoder"),
		});

		{	let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("PostProcess Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &window_view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
						store: wgpu::StoreOp::Store,
					}, // end ops
					depth_slice: None,
				})], // end color_attachments
				depth_stencil_attachment: None,
				timestamp_writes: None,
				occlusion_query_set: None,
			}); // end let mut pass
			pass.set_pipeline(&self.gpu.pipeline);
			pass.set_bind_group(0, &self.gpu.bind_group, &[]);
			pass.draw(0..3, 0..1); // Fullscreen triangle
		} // end let mut pass

		self.gpu.queue.submit(Some(encoder.finish()));
		window_texture.present();

	} // end fn render
} // end impl Context