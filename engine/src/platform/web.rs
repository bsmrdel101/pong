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
  let mut attrs = WindowAttributes::default();
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.unchecked_into();
  canvas.set_width(800);
  canvas.set_height(600);

  WindowAttributes::default()
    .with_canvas(Some(canvas))
    .with_inner_size(PhysicalSize::new(800, 600))
}

pub fn get_instance() -> Instance {
  Instance::default()
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

  let buffer = JsFuture::from(response.array_buffer()?).await?;
  let bytes = js_sys::Uint8Array::new(&buffer);

  Ok(bytes.to_vec())
}
