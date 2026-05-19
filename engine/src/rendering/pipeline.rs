use crate::rendering::mesh::Vertex;

pub struct MsaaTarget {
  pub texture: wgpu::Texture,
  pub view: wgpu::TextureView,
  pub samples: u32
}

impl MsaaTarget {
  pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> Self {
    let texture = device.create_texture(
      &wgpu::TextureDescriptor {
        label: Some("MSAA Texture"),
        size: wgpu::Extent3d {
          width: config.width,
          height: config.height,
          depth_or_array_layers: 1
        },
        mip_level_count: 1,
        sample_count: samples,
        dimension: wgpu::TextureDimension::D2,
        format: config.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[]
      }
    );

    let view = texture.create_view(
      &wgpu::TextureViewDescriptor::default()
    );

    Self {
      texture,
      view,
      samples
    }
  }
}

pub struct RenderPipeline {
  pub pipeline: wgpu::RenderPipeline
}

impl RenderPipeline {
  pub fn new(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    msaa_samples: u32,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
    shader: &wgpu::ShaderModule
  ) -> Self {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[texture_bind_group_layout],
      immediate_size: 0
    });
    
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[Vertex::desc()],
        compilation_options: wgpu::PipelineCompilationOptions::default()
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL
        })],
        compilation_options: wgpu::PipelineCompilationOptions::default()
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        count: msaa_samples,
        mask: !0,
        alpha_to_coverage_enabled: false
      },
      multiview_mask: None,
      cache: None
    });

    Self {
      pipeline
    }
  }
}
