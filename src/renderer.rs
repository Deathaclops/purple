use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::{Fullscreen, Window}};
use vello::{kurbo::Affine, peniko::{self, color::palette::css::BLUE, Fill, Image}, wgpu::{self, Features, PipelineCompilationOptions, TextureFormat}, RendererOptions, Scene};
use crosslog::prelude::*;
use crate::WindowConfig;
use crate::coords::Vec2;

pub struct Renderer {

	pub window			: Arc<Window>					,
	pub surface			: wgpu::Surface<'static>		,
	pub device			: Arc<wgpu::Device>				,
	pub queue			: wgpu::Queue					,
	pub config			: wgpu::SurfaceConfiguration	,
	pub texture_format	: wgpu::TextureFormat			,
	pub renderer		: vello::Renderer				,
	
	pub texture				: wgpu::Texture					,
	pub texture_view		: wgpu::TextureView				,
	pub sampler				: wgpu::Sampler					,
	pub bind_group_layout	: wgpu::BindGroupLayout			,
	pub bind_group			: wgpu::BindGroup				,
	pub swizzle				: wgpu::ShaderModule			,
	pub pipeline_layout		: wgpu::PipelineLayout			,
	pub pipeline			: wgpu::RenderPipeline			,

} // end struct Renderer

impl Renderer {

	/// Creates a new Renderer instance
	pub async fn new ( window: Arc<Window>, resolution: (u32, u32)) -> Self {

		let size = PhysicalSize::new(resolution.x(), resolution.y());
		let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
			backends: wgpu::Backends::PRIMARY,
			..Default::default()
		}); // end let instance
		let surface = instance.create_surface(window.clone()).expect("could not create surface");
		let adapter: wgpu::Adapter = instance.request_adapter (
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			}, // end RequestAdapterOptions
		).await.unwrap();
		let (device, queue) = adapter.request_device (
			&wgpu::DeviceDescriptor {
				required_features: adapter.features() | Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
				required_limits: wgpu::Limits::default(),
				label: None,
				memory_hints: Default::default(),
			}, None
		).await.unwrap();
		let surface_caps = surface.get_capabilities(&adapter);
		println!("Surface capabilities: {:?}", surface_caps.formats);
		let surface_format = *surface_caps.formats.iter().find(
			|&&format| !format.is_srgb())
			.unwrap_or(&wgpu::TextureFormat::Bgra8Unorm);
		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
			desired_maximum_frame_latency: 2,
			alpha_mode: wgpu::CompositeAlphaMode::Opaque,
			view_formats: vec![surface_format],
		}; // end let config
		surface.configure(&device, &config);
		let texture_format = config.format;
		let renderer = vello::Renderer::new(&device, RendererOptions {
			use_cpu: false,
			antialiasing_support: vello::AaSupport::all(),
			num_init_threads: std::num::NonZeroUsize::new(1),
			pipeline_cache: None,
		}).expect("failed to create renderer");


		let texture = device.create_texture(&wgpu::TextureDescriptor {
			label: Some("buffer_texture"),
			size: wgpu::Extent3d {
				width: config.width,
				height: config.height,
				depth_or_array_layers: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: TextureFormat::Rgba8Unorm,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
			view_formats: &[TextureFormat::Rgba8Unorm],
		}); // end let buffer_texture
		let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
			label: Some("buffer_view"),
			format: Some(TextureFormat::Rgba8Unorm),
			..Default::default()
		}); // end let buffer_view


		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("PostProcess Sampler"),
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Linear,
			mipmap_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		}); // end let sampler
		
		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("RGBA to BGRA Bind Group Layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Texture {
						sample_type: wgpu::TextureSampleType::Float { filterable: true },
						view_dimension: wgpu::TextureViewDimension::D2,
						multisampled: false,
					}, count: None,
				}, // end wgpu::BindGroupLayoutEntry
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
					count: None,
				}, // end wgpu::BindGroupLayoutEntry
			], // end entries
		}); // end let bind_group_layout
		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("RGBA to BGRA Bind Group"),
			layout: &bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&texture_view), // Rgba8Unorm texture view
				}, // end wgpu::BindGroupEntry
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&sampler),
				}, // end wgpu::BindGroupEntry
			], // end entries
		}); // end let bind_group
		let swizzle = device.create_shader_module(wgpu::include_wgsl!("../shaders/swizzle.wgsl"));
		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("PostProcess Pipeline Layout"),
			bind_group_layouts: &[&bind_group_layout],
			push_constant_ranges: &[],
		}); // end let pipeline_layout
		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("PostProcess Pipeline"),
			layout: Some(&pipeline_layout),
			vertex: wgpu::VertexState {
				module: &swizzle,
				entry_point: Some("vs_main"),
				buffers: &[],
				compilation_options: PipelineCompilationOptions::default(),
			}, // end vertex: wgpu::VertexState
			fragment: Some(wgpu::FragmentState {
				module: &swizzle,
				entry_point: Some("fs_main"),
				targets: &[Some(wgpu::ColorTargetState {
					format: texture_format,
					blend: Some(wgpu::BlendState::REPLACE),
					write_mask: wgpu::ColorWrites::ALL,
				})], // end targets
				compilation_options: PipelineCompilationOptions::default(),
			}), // end Some(wgpu::FragmentState)
			primitive: Default::default(),
			depth_stencil: None,
			multisample: Default::default(),
			multiview: None,
			cache: None,
		}); // end let pipeline

		return Self {

			window: window.clone(),
			surface,
			device: Arc::new(device),
			queue,
			config,
			texture_format,
			renderer,

			texture,
			texture_view,
			sampler,
			bind_group_layout,
			bind_group,
			swizzle,
			pipeline_layout,
			pipeline,

		}; // end return Renderer

	} // end fn new

	pub fn render (&mut self, scene: &Scene) {

		let window_texture: wgpu::SurfaceTexture = self.surface.get_current_texture().unwrap();
		let window_view = window_texture.texture.create_view(&wgpu::TextureViewDescriptor {
			label: Some("window_view"),
			format: Some(self.texture_format),
			..Default::default()
		}); // end let window_view

		self.renderer.render_to_texture(&self.device, &self.queue, scene, &self.texture_view, &vello::RenderParams {
			base_color: peniko::color::palette::css::BLACK,
			width: self.config.width,
			height: self.config.height,
			antialiasing_method: vello::AaConfig::Msaa16,
		}).unwrap();

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("PostProcess Encoder"),
		});

		{	let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("PostProcess Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &window_view, // <- your TextureView here
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
						store: wgpu::StoreOp::Store,
					}, // end ops
				})], // end color_attachments
				depth_stencil_attachment: None,
				timestamp_writes: None,
				occlusion_query_set: None,
			}); // end let mut pass
			pass.set_pipeline(&self.pipeline);
			pass.set_bind_group(0, &self.bind_group, &[]);
			pass.draw(0..3, 0..1); // Fullscreen triangle
		} // end let mut pass

		self.queue.submit(Some(encoder.finish()));
		window_texture.present();

	} // end fn render

	pub fn resize (&mut self, resolution: (u32, u32)) {
		self.config.width = resolution.0;
		self.config.height = resolution.1;

		// Reconfigure the surface
		self.surface.configure(&self.device, &self.config);

		// Recreate the texture
		self.texture = self.device.create_texture(&wgpu::TextureDescriptor {
			label: Some("buffer_texture"),
			size: wgpu::Extent3d {
				width: self.config.width,
				height: self.config.height,
				depth_or_array_layers: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8Unorm,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
			view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
		});

		// Recreate the texture view
		self.texture_view = self.texture.create_view(&wgpu::TextureViewDescriptor {
			label: Some("buffer_view"),
			format: Some(wgpu::TextureFormat::Rgba8Unorm),
			..Default::default()
		});

		// Recreate the bind group
		self.bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("RGBA to BGRA Bind Group"),
			layout: &self.bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&self.texture_view),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&self.sampler),
				},
			],
		});
	} // end fn resize

	pub fn set_fullscreen (&mut self, fullscreen: bool) {
		self.window.set_fullscreen(if fullscreen { Some(Fullscreen::Borderless(None)) } else { None });
	} // end fn update_config

} // end impl Renderer