use std::borrow::Cow;

use wgpu::{util::DeviceExt};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};


pub async fn run(event_loop: EventLoop<()>, window: Window) {      

    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    println!("{:?}", adapter.get_info());

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");
    
    //create the vertex buffer that the shader will get vertices from
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });
    //describe the layout of the vertex buffer
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
    };


    let format = surface.get_preferred_format(&adapter).unwrap();
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    surface.configure(&device, &config);

    // Load the shaders from disk
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });
    
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[vertex_buffer_layout],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[format.into()],
        }),
        primitive: wgpu::PrimitiveState{
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    event_loop.run(move |event, _, control_flow| {       
        let _ = (&instance, &adapter, &shader, &pipeline_layout);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the surface with the new size
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
            }
            Event::RedrawRequested(_) => {
                let frame = surface.get_current_texture().unwrap();
                
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {r: 0.05, g:0.062, b:0.08, a:1.0}),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    rpass.draw(0..VERTICES.len() as u32, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            Event::MainEventsCleared => { window.request_redraw() }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
//unsafe impl bytemuck::Pod for Vertex {}
//unsafe impl bytemuck::Zeroable for Vertex {}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [0.44147372, 0.2347359], color: [0.5, 0.0, 0.5] }, // E
    
    Vertex { position: [-0.49513406, 0.06958647], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.44147372, 0.2347359], color: [0.5, 0.0, 0.5] }, // E
    
    Vertex { position: [-0.21918549, -0.44939706], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359], color: [0.5, 0.0, 0.5] }, // E
];