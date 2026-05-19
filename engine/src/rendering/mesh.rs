use std::mem;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  position: [f32; 3],
  tex_coords: [f32; 2]
}

impl Vertex {
  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float32x3
        },
        wgpu::VertexAttribute {
          offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x2
        }
      ]
    }
  }
}

pub struct QuadMesh {
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub num_indices: u32
}

impl QuadMesh {
  pub fn new(device: &wgpu::Device) -> Self {
    let vertices = [
      Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [0.0, 0.0] },
      Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [1.0, 0.0] },
      Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },
      Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] }
    ];

    let indices: &[u16] = &[
      0, 1, 2,
      2, 3, 0
    ];

    let vertex_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Quad Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX
      }
    );

    let index_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Quad Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX
      }
    );

    Self {
      vertex_buffer,
      index_buffer,
      num_indices: indices.len() as u32
    }
  }
}
