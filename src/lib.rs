use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};

struct State {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	window: Window,
	render_pipeline_layout: wgpu::PipelineLayout,
	render_pipeline: wgpu::RenderPipeline,
}

impl State {
	async fn new(window: Window) -> Self {
		// define size
		let size = window.inner_size();

		// define instance
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::all(),
			..Default::default()
		});
		// define surface
		let surface = unsafe { instance.create_surface(&window) }.unwrap();

		// define adapater
		let adapter = instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			},
		).await.unwrap();

		// define device, queue
		let (device, queue) = adapter.request_device(
			&wgpu::DeviceDescriptor {
				features: wgpu::Features::empty(),
				limits: if cfg!(target_arch = "wasm32") {
					wgpu::Limits::downlevel_webgl2_defaults()
				} else {
					wgpu::Limits::default()
				},
				label: None,
			},
			None,
		).await.unwrap();

		// define surface capabilities and format
		let surface_caps = surface.get_capabilities(&adapter);
		let surface_format = surface_caps.formats.iter()
			.copied()
			.find(|f| f.is_srgb())
			.unwrap_or(surface_caps.formats[0]);
		// define config

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: surface_caps.present_modes[0],
			alpha_mode: surface_caps.alpha_modes[0],
			view_formats: vec![],
		};

		// define shader, pipeline_layout and render_pipeline
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Shader"),
			source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
		});
		let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline"),
			bind_group_layouts: &[],
			push_constant_ranges: &[],
		});
		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: config.format,
					blend: Some(wgpu::BlendState::REPLACE),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: Some(wgpu::Face::Back),
				unclipped_depth: false,
				polygon_mode: wgpu::PolygonMode::Fill,
				conservative: false,
			},
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview: None,
			depth_stencil: None,
		});


		// call surface.configure using &device and &config
		surface.configure(&device, &config);

		// return self
		Self {
			window,
			surface,
			device,
			queue,
			config,
			size,
			render_pipeline,
			render_pipeline_layout,
		}
	} // End of State::new() def

	pub fn window(&self) -> &Window {
		&self.window
	} // end of State::window() def

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {

		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}

	} // End of State::resize def

	fn input(&mut self, event: &WindowEvent) -> bool {
		false
	} // End of State::input() def

	fn update(&mut self) {

	} // End of State::update() def

	fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

		let output = self.surface.get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
		});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.1,
							g: 0.2,
							b: 0.3,
							a: 1.0,
						}),
						store: wgpu::StoreOp::Store,
					},
				})],
				depth_stencil_attachment: None,
				occlusion_query_set: None,
				timestamp_writes: None,
			});

			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.draw(0..3, 0..1);
		}

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())

	} // End of State::render() def
}

pub async fn run() {

	env_logger::init();
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop).unwrap();
	let mut state = State::new(window).await;
	// define variables above

	// call run on event_loop using closure, define window functionality.
	event_loop.run(move |event, _, control_flow| {
		match event {

			Event::RedrawRequested(window_id) if window_id == state.window().id() => {
				state.update();
				match state.render() {
					Ok(_) => {}
					Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
					Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
					Err(e) => eprintln!("{:?}",e),

				}
			}
			Event::MainEventsCleared => {
				state.window().request_redraw();
			}

			Event::WindowEvent {
				window_id,
				event,
			}
			if window_id == state.window.id() => {
				match event {
					WindowEvent::CloseRequested |
					WindowEvent::KeyboardInput {
						input:
							KeyboardInput {
								state: ElementState::Pressed,
								virtual_keycode: Some(VirtualKeyCode::Escape),
								..
							},
						..
					} => *control_flow = ControlFlow::Exit,
					WindowEvent::Resized(physical_size) => {
						state.resize(physical_size);
					}
					WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
						state.resize(*new_inner_size);
					}
					_ => {}
				}
			}
			_ => {}

		}
	});
}
