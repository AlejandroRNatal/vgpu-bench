use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer};
use winit::dpi::PhysicalSize;

use crate::{
    renderer::state::{GpuGlobals, GpuPrimitive, GpuTransform},
    targets::{SVGDocument, TessellationData},
};

const WINDOW_SIZE: f32 = 800.0;

// These mush match the uniform buffer sizes in the vertex shader.
const MAX_PRIMITIVES: usize = 512;
const MAX_TRANSFORMS: usize = 512;

// Default scene has all values set to zero
#[derive(Copy, Clone, Debug)]
pub struct SceneGlobals {
    pub zoom: f32,
    pub pan: [f32; 2],
    pub window_size: PhysicalSize<u32>,
    pub wireframe: bool,
    pub size_changed: bool,
}

pub fn get_globals(file_data: &SVGDocument) -> SceneGlobals {
    let opt = usvg::Options::default();
    let content: &[u8] = file_data.content.as_bytes();
    let rtree = usvg::Tree::from_data(content, &opt.to_ref()).unwrap();
    let view_box = rtree.svg_node().view_box;

    let vb_width = view_box.rect.size().width() as f32;
    let vb_height = view_box.rect.size().height() as f32;
    let scale = vb_width / vb_height;

    let (width, height) = if scale < 1.0 {
        (WINDOW_SIZE, WINDOW_SIZE * scale)
    } else {
        (WINDOW_SIZE, WINDOW_SIZE / scale)
    };

    let pan = [vb_width / -2.0, vb_height / -2.0];
    let zoom = 2.0 / f32::max(vb_width, vb_height);
    let scene = SceneGlobals {
        zoom,
        pan,
        window_size: PhysicalSize::new(width as u32, height as u32),
        wireframe: false,
        size_changed: true,
    };
    scene
}

pub struct Buffers {
    pub ibo: Buffer,
    pub vbo: Buffer,
    pub prims_ubo: Buffer,
    pub transforms_ubo: Buffer,
    pub globals_ubo: Buffer,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
}

pub fn get_buffers(device: &wgpu::Device, data: &TessellationData) -> Buffers {
    // Create vertex buffer object
    let vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&data.mesh.vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    // Create index buffer object
    let ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&data.mesh.indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    let prim_buffer_byte_size = (MAX_PRIMITIVES * std::mem::size_of::<GpuPrimitive>()) as u64;
    let transform_buffer_byte_size = (MAX_TRANSFORMS * std::mem::size_of::<GpuTransform>()) as u64;
    let globals_buffer_byte_size = std::mem::size_of::<GpuGlobals>() as u64;

    let prims_ubo = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Prims ubo"),
        size: prim_buffer_byte_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let transforms_ubo = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Transforms ubo"),
        size: transform_buffer_byte_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let globals_ubo = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Globals ubo"),
        size: globals_buffer_byte_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(globals_buffer_byte_size),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(prim_buffer_byte_size),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(transform_buffer_byte_size),
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(globals_ubo.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(prims_ubo.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Buffer(transforms_ubo.as_entire_buffer_binding()),
            },
        ],
    });

    Buffers {
        ibo,
        vbo,
        prims_ubo,
        transforms_ubo,
        globals_ubo,
        bind_group_layout,
        bind_group,
    }
}