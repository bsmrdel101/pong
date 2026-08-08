use wgpu::{Instance, Limits};
use winit::{event_loop::{ControlFlow, EventLoop}, window::WindowAttributes};
use crate::{App, error::EngineResult};


pub fn spawn_event_loop(event_loop: EventLoop<()>)  {
  event_loop.set_control_flow(ControlFlow::Poll);

  if let Err(error) = event_loop.run_app(&mut App::default()) {
    eprintln!("Event loop error: {error}");
  }
}

pub fn window_attributes() -> WindowAttributes {
  WindowAttributes::default()
}

pub fn get_instance() -> Instance {
  Instance::new(&wgpu::InstanceDescriptor {
    backends: wgpu::Backends::PRIMARY,
    ..Default::default()
  })
}

pub fn get_device_required_limits() -> Limits {
  Limits::default()
}

pub async fn load_bytes(path: &str) -> EngineResult<Vec<u8>> {
  let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
  Ok(std::fs::read(path)?)
}
