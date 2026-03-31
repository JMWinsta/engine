use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2};
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct PrimitiveRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    viewport_size: Vec2,
}

impl PrimitiveRenderer {
    pub async fn new(window: Arc<winit::window::Window>) -> Result<Self, crate::EngineError> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // ADD THIS DEBUG LOOP:
        println!("--- DETECTED GPU ADAPTERS ---");
        for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
            println!("{:#?}", adapter.get_info());
        }
        println!("-----------------------------");

        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| crate::EngineError::RendererInit(e.to_string()))?;

        let mut adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await;

        // If High Performance failed, try literally anything else
        if adapter.is_none() {
            println!("HighPerformance adapter not found. Falling back to default...");
            adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await;
        }

        let adapter = adapter.expect("No suitable GPU adapter found. Ensure your drivers support Vulkan/DX12/OpenGL");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("Failed to request device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let camera_uniform = CameraUniform {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Primitive Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let mut renderer = Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            vertices: Vec::new(),
            indices: Vec::new(),
            camera_buffer,
            camera_bind_group,
            viewport_size: Vec2::new(size.width as f32, size.height as f32),
        };
        renderer.set_viewport(size.width as f32, size.height as f32);
        Ok(renderer)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.set_viewport(width as f32, height as f32);
        }
    }

    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport_size = Vec2::new(width, height);
        let proj = Mat4::orthographic_rh(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            -1.0,
            1.0,
        );
        let camera_uniform = CameraUniform {
            view_proj: proj.to_cols_array_2d(),
        };
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    pub fn begin_frame(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn draw_rect(&mut self, pos: Vec2, size: Vec2, color: [f32; 4]) {
        let start_idx = self.vertices.len() as u16;

        self.vertices.push(Vertex {
            position: [pos.x, pos.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [pos.x + size.x, pos.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [pos.x + size.x, pos.y + size.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [pos.x, pos.y + size.y],
            color,
        });

        self.indices.extend_from_slice(&[
            start_idx,
            start_idx + 1,
            start_idx + 2,
            start_idx,
            start_idx + 2,
            start_idx + 3,
        ]);
    }

    pub fn draw_circle(&mut self, center: Vec2, radius: f32, color: [f32; 4], segments: u32) {
        let start_idx = self.vertices.len() as u16;
        self.vertices.push(Vertex {
            position: [center.x, center.y],
            color,
        });

        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            self.vertices.push(Vertex {
                position: [
                    center.x + angle.cos() * radius,
                    center.y + angle.sin() * radius,
                ],
                color,
            });

            let next_i = (i + 1) % segments;
            self.indices.extend_from_slice(&[
                start_idx,
                start_idx + 1 + i as u16,
                start_idx + 1 + next_i as u16,
            ]);
        }
    }

    pub fn draw_line(&mut self, start: Vec2, end: Vec2, thickness: f32, color: [f32; 4]) {
        let dir = (end - start).normalize_or_zero();
        let normal = Vec2::new(-dir.y, dir.x) * (thickness / 2.0);

        let start_idx = self.vertices.len() as u16;
        self.vertices.push(Vertex {
            position: [start.x + normal.x, start.y + normal.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [start.x - normal.x, start.y - normal.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [end.x - normal.x, end.y - normal.y],
            color,
        });
        self.vertices.push(Vertex {
            position: [end.x + normal.x, end.y + normal.y],
            color,
        });

        self.indices.extend_from_slice(&[
            start_idx,
            start_idx + 1,
            start_idx + 2,
            start_idx,
            start_idx + 2,
            start_idx + 3,
        ]);
    }

    pub fn end_frame(&mut self) {
        let output = match self.surface.get_current_texture() {
            Ok(tex) => tex,
            Err(e) => {
                eprintln!("Surface error: {:?}", e);
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if !self.vertices.is_empty() {
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&self.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

            let index_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&self.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.camera_bind_group, &[]);
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
        } else {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
