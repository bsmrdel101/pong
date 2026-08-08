use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use winit::{dpi::PhysicalSize, event_loop::{EventLoop}, platform::web::WindowAttributesExtWebSys, window::WindowAttributes};
use wgpu::{Instance, Limits};
use crate::{App, error::{EngineResult, EngineError}};


pub fn spawn_event_loop(event_loop: EventLoop<()>)  {
  use winit::platform::web::EventLoopExtWebSys;
  event_loop.spawn_app(App::default());
}

pub fn window_attributes() -> WindowAttributes {
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.unchecked_into();
  canvas.set_width(800);
  canvas.set_height(600);
  canvas.style().set_property("width", "800px").unwrap();
  canvas.style().set_property("height", "600px").unwrap();

  WindowAttributes::default()
    .with_canvas(Some(canvas))
    .with_inner_size(PhysicalSize::new(800, 600))
}

pub fn get_instance() -> Instance {
  web_sys::console::log_1(&"Creating WGPU WebGL instance".into());
  Instance::new(&wgpu::InstanceDescriptor {
    backends: wgpu::Backends::GL,
    ..Default::default()
  })
}

pub fn get_device_required_limits() -> Limits {
  Limits::downlevel_webgl2_defaults()
}

pub async fn load_bytes(path: &str) -> EngineResult<Vec<u8>> {
  let window = web_sys::window()
    .ok_or_else(|| EngineError::Js("No window available".into()))?;

  let response = JsFuture::from(
    window.fetch_with_str(path)
  ).await?
  .dyn_into::<web_sys::Response>()?;

  if !response.ok() {
    return Err(EngineError::Js(
      format!("Failed to load asset '{}': HTTP {}", path, response.status()).into()
    ));
  }

  let buffer = JsFuture::from(response.array_buffer()?).await?;
  let bytes = js_sys::Uint8Array::new(&buffer);

  Ok(bytes.to_vec())
}
