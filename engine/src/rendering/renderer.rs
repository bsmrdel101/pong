use std::sync::Arc;
use winit::window::Window;
use crate::{
  error::EngineResult,
  platform::desktop::{ get_device_required_limits, get_instance },
  rendering::{ mesh::QuadMesh, pipeline::{ MsaaTarget, RenderPipeline }, texture::Texture }
};

pub struct Renderer {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  pipeline: RenderPipeline,
  msaa: MsaaTarget,
  mesh: QuadMesh,
  texture: Texture
}

impl Renderer {
  pub async fn new(window: Arc<Window>) -> EngineResult<Self> {
    let size = window.inner_size();
    let instance = get_instance();
    let surface = instance.create_surface(window.clone()).unwrap();
    
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false
      }).await?;

    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        required_limits: get_device_required_limits(),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off
      }).await?;

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter()
      .find(|f| f.is_srgb())
      .copied()
      .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2
    };

    surface.configure(&device, &config);

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into())
    });

    let texture_bind_group_layout = device.create_bind_group_layout(
      &wgpu::BindGroupLayoutDescriptor {
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
              multisampled: false,
              view_dimension: wgpu::TextureViewDimension::D2,
              sample_type: wgpu::TextureSampleType::Float { filterable: true }
            },
            count: None
          },
          wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None
          },
        ],
        label: Some("texture_bind_group_layout")
      }
    );

    const MSAA_SAMPLES: u32 = 4;
    let pipeline = RenderPipeline::new(&device, &config, MSAA_SAMPLES, &texture_bind_group_layout, &shader);
    let msaa = MsaaTarget::new(&device, &config, MSAA_SAMPLES);

    let texture = Texture::new(
      &device,
      &queue,
      "assets/tree.png",
      &texture_bind_group_layout
    )?;
    let mesh = QuadMesh::new(&device);

    Ok(Self {
      surface,
      device,
      queue,
      config,
      pipeline,
      msaa,
      mesh,
      texture
    })
  }

  pub fn render(&mut self) -> EngineResult<()> {
    let output = self.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
      self.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
          label: Some("Render Encoder")
        }
      );

    {
      let mut render_pass = encoder.begin_render_pass(
        &wgpu::RenderPassDescriptor {
          label: Some("Render Pass"),
          color_attachments: &[Some(
            wgpu::RenderPassColorAttachment {
              view: if self.msaa.samples > 1 { &self.msaa.view } else { &view },
              resolve_target: if self.msaa.samples > 1 { Some(&view) } else { None },
              depth_slice: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(
                  wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.7,
                    a: 1.0
                  }
                ),
                store: wgpu::StoreOp::Store
              }
            }
          )],
          depth_stencil_attachment: None,
          occlusion_query_set: None,
          timestamp_writes: None,
          multiview_mask: None
        }
      );

      render_pass.set_pipeline(&self.pipeline.pipeline);
      render_pass.set_bind_group(0, &self.texture.bind_group, &[]);

      render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
      render_pass.set_index_buffer(
        self.mesh.index_buffer.slice(..),
        wgpu::IndexFormat::Uint16
      );

      render_pass.draw_indexed(0..self.mesh.num_indices, 0, 0..1);
    }

    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  } 
}
