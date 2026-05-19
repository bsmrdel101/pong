use wasm_bindgen::JsCast;
use winit::{ platform::web::WindowAttributesExtWebSys, window::WindowAttributes };
use wgpu::{ Instance, Limits };


pub fn window_attributes() -> WindowAttributes {
  let mut attrs = WindowAttributes::default();
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas = canvas.unchecked_into();
  attrs = attrs.with_canvas(Some(canvas));

  attrs
}

pub fn get_instance() -> Instance {
  Instance::new(&wgpu::InstanceDescriptor {
    backends: wgpu::Backends::GL,
    ..Default::default()
  })
}

pub fn get_device_required_limits() -> Limits {
  Limits::downlevel_webgl2_defaults()
}
