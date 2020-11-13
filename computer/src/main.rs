use std::time::{Duration, Instant};

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BackendBit, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferDescriptor, BufferSize, BufferUsage,
    Color, CommandEncoderDescriptor, DeviceDescriptor, Features, IndexFormat, InputStepMode,
    Instance, LoadOp, Operations, PipelineLayoutDescriptor, PowerPreference, PresentMode,
    PrimitiveTopology, ProgrammableStageDescriptor, RenderPassColorAttachmentDescriptor,
    RenderPassDescriptor, RenderPipelineDescriptor, RequestAdapterOptions, ShaderStage,
    SwapChainDescriptor, TextureFormat, TextureUsage, VertexBufferDescriptor,
    VertexStateDescriptor,
};

use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod chip;
mod computer;
use computer::Computer;
mod cpu;
mod keyboard;
use keyboard::winit::WinitKeyboard as Keyboard;
mod memory;
mod rom;
use rom::Rom;
mod screen;
use screen::wgpu::WgpuScreen as Screen;

async fn run() {
    let mut computer = Computer::<Screen, Keyboard>::new();
    computer.set_rom(Rom::from_binary("Rect.hack").unwrap());

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Nand2Tetris")
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(512, 256))
        .with_visible(true)
        .build(&event_loop)
        .unwrap();

    let instance = Instance::new(BackendBit::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::Default,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();
    println!("{:?}", adapter.get_info());
    println!("{:?}", adapter.features());
    println!("{:?}", adapter.limits());
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                features: Features::empty(),
                limits: adapter.limits(),
                shader_validation: cfg!(debug_assertions),
            },
            None,
        )
        .await
        .unwrap();

    let vertex_shader =
        device.create_shader_module(wgpu::include_spirv!("../shader/main.vert.spv"));
    let fragment_shader =
        device.create_shader_module(wgpu::include_spirv!("../shader/main.frag.spv"));
    let vertices = [[-0.5f32, 0.5], [-0.5, -0.5], [0.5, 0.5], [0.5, -0.5]];
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("vertex buffer"),
        contents: &vertices
            .concat()
            .iter()
            .flat_map(|f| f.to_ne_bytes().to_vec())
            .collect::<Vec<_>>(),
        usage: BufferUsage::VERTEX,
    });
    let instance_positions = (0..256)
        .map(|i| {
            (0..512)
                .map(|j| [j as f32 - 256.0 + 0.5, -(i as f32 - 128.0 + 0.5)])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let instance_position_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("instance position buffer"),
        contents: instance_positions
            .iter()
            .flat_map(|i| i.iter().flatten())
            .flat_map(|f| f.to_ne_bytes().to_vec())
            .collect::<Vec<_>>()
            .as_slice(),
        usage: BufferUsage::VERTEX,
    });
    let instance_color_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("instance color buffer"),
        size: computer.screen().colors().len() as _,
        usage: BufferUsage::VERTEX | BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("bind group layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStage::VERTEX,
            ty: BindingType::UniformBuffer {
                dynamic: false,
                min_binding_size: BufferSize::new(4 * 2),
            },
            count: None,
        }],
    });
    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let window_uniform_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("window uniform buffer"),
        size: 4 * 2,
        usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("bind group"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(window_uniform_buffer.slice(..)),
        }],
    });

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("render pipeline"),
        layout: Some(&pipeline_layout),
        vertex_stage: ProgrammableStageDescriptor {
            module: &vertex_shader,
            entry_point: "main",
        },
        fragment_stage: Some(ProgrammableStageDescriptor {
            module: &fragment_shader,
            entry_point: "main",
        }),
        rasterization_state: None,
        primitive_topology: PrimitiveTopology::TriangleStrip,
        color_states: &[TextureFormat::Bgra8UnormSrgb.into()],
        depth_stencil_state: None,
        vertex_state: VertexStateDescriptor {
            index_format: IndexFormat::Uint16,
            vertex_buffers: &[
                VertexBufferDescriptor {
                    stride: 4 * 2,
                    step_mode: InputStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float2],
                },
                VertexBufferDescriptor {
                    stride: 4 * 2,
                    step_mode: InputStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![1 => Float2],
                },
                VertexBufferDescriptor {
                    stride: 4,
                    step_mode: InputStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![2 => Float],
                },
            ],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let mut swap_chain_descriptor = SwapChainDescriptor {
        usage: TextureUsage::OUTPUT_ATTACHMENT,
        format: TextureFormat::Bgra8UnormSrgb,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: PresentMode::Mailbox,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

    let mut previous = Instant::now();
    let mut dt = Duration::new(0, 0);
    let mut update_counter = 0;
    let mut update_count_start = previous;
    let mut frame_counter = 0;
    let mut frame_count_start = previous;

    computer.tick(true);
    queue.write_buffer(
        &window_uniform_buffer,
        0,
        &[swap_chain_descriptor.width, swap_chain_descriptor.height]
            .iter()
            .flat_map(|size| (*size as f32).to_ne_bytes().to_vec())
            .collect::<Vec<_>>(),
    );
    event_loop.run(move |event, _, control_flow| {
        // Take ownership
        let _ = (&instance, &adapter);

        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
            } if window_id == window.id() => {
                swap_chain_descriptor.width = size.width;
                swap_chain_descriptor.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

                queue.write_buffer(
                    &window_uniform_buffer,
                    0,
                    &[swap_chain_descriptor.width, swap_chain_descriptor.height]
                        .iter()
                        .flat_map(|size| (*size as f32).to_ne_bytes().to_vec())
                        .collect::<Vec<_>>(),
                );
            }
            Event::WindowEvent {
                window_id,
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::KeyboardInput { input, .. },
            } if window_id == window.id() => {
                computer.set_keystate(input);
            }
            Event::MainEventsCleared => {
                let duration_per_update = Duration::from_secs_f64(1.0 / 60.0);
                let now = Instant::now();
                dt += now - previous;
                previous = now;
                let mut count = 0;
                while dt > duration_per_update {
                    for _ in 0..(256 * 512 / 16) {
                        computer.tick(false);
                    }
                    dt -= duration_per_update;
                    update_counter += 1;
                    count += 1;
                    if update_counter >= 100 {
                        println!(
                            "UPS: {} ({})",
                            update_counter as f64 / (now - update_count_start).as_secs_f64(),
                            count,
                        );
                        update_counter = 0;
                        update_count_start = now;
                    }
                }
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                queue.write_buffer(&instance_color_buffer, 0, computer.screen().colors());

                let frame = swap_chain.get_current_frame().unwrap().output;
                let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("command encoder"),
                });
                {
                    let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                        color_attachments: &[RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(Color::BLUE),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_bind_group(0, &bind_group, &[]);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, instance_position_buffer.slice(..));
                    render_pass.set_vertex_buffer(2, instance_color_buffer.slice(..));
                    render_pass.draw(0..(2 * 4), 0..(512 * 256));
                }

                queue.submit(Some(encoder.finish()));

                frame_counter += 1;
                if frame_counter >= 100 {
                    let now = Instant::now();
                    println!(
                        "FPS: {}",
                        frame_counter as f64 / (now - frame_count_start).as_secs_f64(),
                    );
                    frame_counter = 0;
                    frame_count_start = now;
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn main() {
    futures::executor::block_on(run());
}
