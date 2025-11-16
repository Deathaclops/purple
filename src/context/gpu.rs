use std::{num::NonZeroUsize, sync::Arc};

use vello::{wgpu::{self, PipelineCache, PipelineCompilationOptions, TextureFormat}, Renderer};
use winit::window::Window;

use crate::prelude::*;

pub struct Gpu {
	pub window: Arc<Window>,
	pub surface: wgpu::Surface<'static>,
	pub device: Arc<wgpu::Device>,
	pub queue: wgpu::Queue,
	pub config: wgpu::SurfaceConfiguration,
	pub renderer: vello::Renderer,
	pub texture: wgpu::Texture,
	pub texture_view: wgpu::TextureView,
	pub bind_group_layout	: wgpu::BindGroupLayout			,
	pub bind_group			: wgpu::BindGroup				,	
	pub pipeline_layout		: wgpu::PipelineLayout			,
	pub pipeline			: wgpu::RenderPipeline			,
	pub swizzle_shader		: wgpu::ShaderModule			,
	pub sampler				: wgpu::Sampler					,
} // end struct Gpu

impl Gpu {
	pub async fn new(window: Arc<Window>) -> Self {

		let resolution: Dimensions = window.inner_size().into();
		let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
		let surface = instance.create_surface(window.clone()).expect("could not create surface");
		let adapter: wgpu::Adapter = instance.request_adapter (
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			}, // end RequestAdapterOptions
		).await.unwrap();
		let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
			required_features: wgpu::Features::empty(),
			required_limits: adapter.limits(),
			label: None,
			memory_hints: Default::default(),
			trace: wgpu::Trace::Off,
		}).await.unwrap();
		let surface_caps = surface.get_capabilities(&adapter);
		let surface_format = TextureFormat::Bgra8Unorm;
		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: (resolution.width as u32).max(1),
			height: (resolution.height as u32).max(1),
			present_mode: wgpu::PresentMode::Fifo,
			desired_maximum_frame_latency: 2,
			alpha_mode: wgpu::CompositeAlphaMode::Opaque,
			view_formats: vec![],
		}; // end let config
		surface.configure(&device, &config);
		let texture = device.create_texture(&wgpu::TextureDescriptor {
			label: Some("Buffer Texture"),
			size: wgpu::Extent3d {
				width: config.width.max(1),
				height: config.height.max(1),
				depth_or_array_layers: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: TextureFormat::Rgba8Unorm,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
			view_formats: &[],
		}); // end let buffer_texture
		let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
			label: Some("Buffer View"),
			format: Some(TextureFormat::Rgba8Unorm),
			..Default::default()
		}); // end let buffer_view

		log!("Loading Vello renderer... This might take a while.");
		let renderer = Renderer::new(
			&device,
			vello::RendererOptions {
				use_cpu: false,
				antialiasing_support: vello::AaSupport::all(),
				num_init_threads: None,
				pipeline_cache: None,
			}, // end RendererOptions
		).unwrap(); // end let renderer
		log!("Vello renderer loaded!");

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
		let swizzle_shader = device.create_shader_module(wgpu::include_wgsl!("../../shaders/swizzle.wgsl"));
		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("PostProcess Pipeline Layout"),
			bind_group_layouts: &[&bind_group_layout],
			push_constant_ranges: &[],
		}); // end let pipeline_layout
		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("PostProcess Pipeline"),
			layout: Some(&pipeline_layout),
			vertex: wgpu::VertexState {
				module: &swizzle_shader,
				entry_point: Some("vs_main"),
				buffers: &[],
				compilation_options: PipelineCompilationOptions::default(),
			}, // end vertex: wgpu::VertexState
			fragment: Some(wgpu::FragmentState {
				module: &swizzle_shader,
				entry_point: Some("fs_main"),
				targets: &[Some(wgpu::ColorTargetState {
					format: surface_format,
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
			
			window,
			surface,
			device: Arc::new(device),
			queue,
			config,
			renderer,
			texture,
			texture_view,

			bind_group_layout,
			bind_group,
			pipeline_layout,
			pipeline,
			sampler,
			swizzle_shader,

		}; // end return Self
	} // end fn new

	pub async fn resize(&mut self, size: impl Into<Dimensions>) {
		info!("Resizing GPU context");
		let resolution = size.into();
		self.config.width = (resolution.width as u32).max(1);
		self.config.height = (resolution.height as u32).max(1);
		self.surface.configure(&self.device, &self.config);
		self.texture = self.device.create_texture(&wgpu::TextureDescriptor {
			label: Some("Buffer Texture"),
			size: wgpu::Extent3d {
				width: self.config.width,
				height: self.config.height,
				depth_or_array_layers: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: TextureFormat::Rgba8Unorm,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
			view_formats: &[],
		}); // end let buffer_texture
		self.texture_view = self.texture.create_view(&wgpu::TextureViewDescriptor {
			label: Some("Buffer Texture View"),
			..Default::default()
		}); // end let buffer_texture_view
		self.bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("RGBA to BGRA Bind Group"),
			layout: &self.bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&self.texture_view), // Rgba8Unorm texture view
				}, // end wgpu::BindGroupEntry
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&self.sampler),
				}, // end wgpu::BindGroupEntry
			], // end entries
		}); // end let bind_group
	} // end fn resize
} // end impl Gpu

