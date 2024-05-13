use crate::image_utils;
use crate::image_utils::Image;
use image::{GenericImageView, ImageBuffer, Rgba};
use std::fs;
use std::iter;
use std::ops::Range;
use wgpu::util::DeviceExt;
use wgpu::{
    self, Adapter, Device, Extent3d, Queue, SurfaceConfiguration, SurfaceTexture, Texture,
    TextureView,
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

pub async fn run() {
    env_logger::init();
    //BIG TODO: Custom error handling
    let event_loop = EventLoop::new().unwrap();
    let surface_size: (u32, u32) = (1920, 1920); //Maybe make this just one value as you pretty much always want your surface to be square?
    let clear_color = wgpu::Color {
        r: 0.3,
        g: 0.4,
        b: 0.8,
        a: 1.0,
    };
    let window = WindowBuilder::new()
        .with_title("wgpu bootstrap")
        .with_inner_size(winit::dpi::PhysicalSize::new(
            surface_size.0,
            surface_size.1,
        ))
        .build(&event_loop)
        .unwrap();

    let (surface, config, device, queue) = initialize_surface(&window, surface_size).await;
    surface.configure(&device, &config);
    let texture_bind_group_layout: wgpu::BindGroupLayout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });
    let diffuse_sampler: wgpu::Sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    //LOADING UP MULTIPLE IMAGES
    let mut emojis_vec: Vec<image_utils::Image> = vec![];
    let raw_emojis = fs::read_dir("/Users/joachimpfefferkorn/Desktop/some_emojis").unwrap();

    let mut rotation = 0.0;
    for emoji in raw_emojis {
        rotation = rotation + 0.2;
        let path = emoji.unwrap().path();
        let path_str = path.to_str().expect("Path is not a string");
        let transform = image_utils::Transform {
            scale: 0.0005,
            rotation: image_utils::Coordinates {
                x: 0.0,
                y: 0.0,
                z: rotation,
            },
            translation: image_utils::Coordinates {
                x: -0.1,
                y: 0.5,
                z: 0.0,
            },
        };
        let image = image_utils::Image::load_image(
            path_str,
            transform,
            &device,
            &queue,
            &texture_bind_group_layout,
            &diffuse_sampler,
        );
        emojis_vec.push(image);
    }

    /////////////////////
    let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    let output = surface.get_current_texture().unwrap(); //could be a better name

    let view = output //is this the viewscreen?
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&texture_bind_group_layout], // NEW!
        push_constant_ranges: &[],
    });
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //copy pasted from learnWGPU. This is verbose
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[image_utils::Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent::OVER,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
            // or Features::POLYGON_MODE_POINT
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    });
    render(
        encoder,
        &view,
        &clear_color,
        &render_pipeline,
        emojis_vec,
        &queue,
        output,
    );

    let _ = event_loop.run(move |event, window| handle_window_event(event, window));
}

//RENDER FUNCTIONS
fn render(
    mut encoder: wgpu::CommandEncoder,
    view: &TextureView,
    clear_color: &wgpu::Color,
    render_pipeline: &wgpu::RenderPipeline,

    images: Vec<Image>,

    queue: &Queue,
    output: SurfaceTexture,
) {
    let mut render_pass: wgpu::RenderPass<'_> =
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(*clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
    render_pass.set_pipeline(&render_pipeline);

    for image in &images {
        let (bind_group, vertex_buffer, index_buffer, indices) = setup_image(image);

        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(indices, 0, 0..1);
    }
    drop(render_pass);

    queue.submit(iter::once(encoder.finish()));

    output.present();
}

fn setup_image(image: &Image) -> (&wgpu::BindGroup, &wgpu::Buffer, &wgpu::Buffer, Range<u32>) {
    (
        &image.diffuse_bind_group,
        &image.vertex_buffer,
        &image.index_buffer,
        0..image.num_indices,
    )
}

//WINDOW and SURFACE FUNCTIONS
async fn initialize_surface<'a>(
    window: &'a Window,
    surface_size: (u32, u32),
) -> (wgpu::Surface<'a>, SurfaceConfiguration, Device, Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window).unwrap(); //Is not borrowing this RAM intensive?

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);

    let surface_format = surface_caps //this is so verbose, we can probably make it shorter, look into wgpu::Surface::get_default_config
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        desired_maximum_frame_latency: 2,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: surface_size.0,
        height: surface_size.1,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    (surface, config, device, queue)
    //returns surface, config, device, queue
}

fn handle_window_event(event: Event<()>, window: &EventLoopWindowTarget<()>) {
    match event {
        Event::WindowEvent {
            event: ref window_event,
            window_id: _,
        } => match window_event {
            WindowEvent::CloseRequested => {
                window.exit();
            }
            _ => {}
        },
        _ => {}
    };
}
