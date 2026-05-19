use wgpu::{ Instance, Limits };
use winit::window::WindowAttributes;


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
